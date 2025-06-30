#!/usr/bin/env python
from __future__ import annotations

import argparse
import json
import sys
from datetime import datetime, timezone
from pathlib import Path
from typing import Dict, List, Tuple, Any, Optional

import pandas as pd
from tqdm import tqdm

from sprout_pipeline.runner import run_pipeline, Provider, LLMError
from sprout_pipeline.tasks import TASK_REGISTRY


def parse_args() -> argparse.Namespace:
    """
    Parse command line arguments.

    Returns:
        Parsed arguments
    """
    parser = argparse.ArgumentParser(
        description="Run LLM-based analysis tasks on Rust code samples"
    )

    # Required arguments
    parser.add_argument(
        "--task",
        choices=TASK_REGISTRY.keys(),
        required=True,
        help="Analysis task to run",
    )
    parser.add_argument(
        "--input",
        type=Path,
        required=True,
        help="Path to mizan.json file (Rust Mizan dataset)",
    )

    # Optional arguments
    parser.add_argument(
        "--provider",
        choices=list(Provider.__args__),
        default="openai",
        help="LLM provider (default: openai)",
    )
    parser.add_argument(
        "--model",
        default="gpt-4o-mini",
        help="Model name to use (default: gpt-4o-mini)",
    )
    parser.add_argument("--prompt", type=Path, help="Override default prompt file")
    parser.add_argument(
        "--out", type=Path, help="Override output JSON path ('-' for stdout)"
    )

    return parser.parse_args()


def setup_output_paths(task: str, provider: str, model: str) -> Tuple[Path, Path, Path]:
    """
    Create output directories and determine output file paths.

    Args:
        task: The task name
        provider: The LLM provider name
        model: The model name

    Returns:
        Tuple of (json_path, csv_path, failures_path)
    """
    timestamp = datetime.now(timezone.utc).strftime("%Y-%m-%dT%H-%M-%S")
    model_dir = f"{provider}-{model}"

    # Set up paths
    json_path = Path("output") / task / model_dir / f"{timestamp}.json"
    csv_path = Path("datasets") / task / model_dir / f"{timestamp}.csv"
    failures_path = Path("output") / task / model_dir / f"{timestamp}_failures.json"

    # Create directories
    json_path.parent.mkdir(parents=True, exist_ok=True)
    csv_path.parent.mkdir(parents=True, exist_ok=True)

    return json_path, csv_path, failures_path


def save_results(
    record: Dict,
    row_dict: Dict,
    json_path: Path,
    csv_path: Path,
    is_first: bool,
    stdout_mode: bool = False,
) -> None:
    """
    Save results to JSON and CSV files.

    Args:
        record: Full result record for JSON
        row_dict: Flattened row for CSV
        json_path: Path to JSON output file
        csv_path: Path to CSV output file
        is_first: Whether this is the first record being saved
        stdout_mode: Whether to print to stdout instead of files
    """
    if stdout_mode:
        json.dump(record, sys.stdout, indent=2)
        print(file=sys.stdout)
        return

    # Save to JSON
    if is_first:
        json_path.write_text(json.dumps([record], indent=2))
    else:
        data = json.loads(json_path.read_text())
        data.append(record)
        json_path.write_text(json.dumps(data, indent=2))

    # Save to CSV
    df = pd.DataFrame([row_dict])
    df.to_csv(csv_path, mode="w" if is_first else "a", header=is_first, index=False)


def save_failure(
    failure_record: Dict, failures_path: Path, is_first_failure: bool
) -> None:
    """
    Save failure information to the failures JSON file.

    Args:
        failure_record: Failure information to save
        failures_path: Path to failures JSON file
        is_first_failure: Whether this is the first failure being saved
    """
    if is_first_failure:
        failures_path.write_text(json.dumps([failure_record], indent=2))
    else:
        data = json.loads(failures_path.read_text())
        data.append(failure_record)
        failures_path.write_text(json.dumps(data, indent=2))


def analyze_sample(
    sample_data: Tuple[Path, Any, Optional[Dict]],
    task_instance: Any,
    prompt_input: Path | str,
    system_prompt_input: Path | str,
    model_name: str,
    provider: Provider,
    sample_index: int,
    total_samples: int,
) -> Tuple[Optional[Dict], Optional[Dict], str, Optional[Dict]]:
    """
    Analyze a single code sample using the specified task and model.

    Args:
        sample_data: Tuple of (code_dir, ground_truth, context)
        task_instance: Task instance to use
        prompt_input: Prompt text or path
        model_name: Model name to use
        provider: LLM provider
        sample_index: Current sample index (1-based)
        total_samples: Total number of samples

    Returns:
        Tuple of (record, csv_row, code_label, failure_info)
        If failure_info is not None, then record and csv_row will be None
    """
    # Unpack sample data
    if len(sample_data) == 3:
        code_dir, ground_truth, ctx = sample_data
    else:
        code_dir, ground_truth = sample_data
        ctx = None
    # Build per-sample prompt if task provides helper
    if hasattr(task_instance, "build_prompt") and ctx is not None:
        prompt_text = task_instance.build_prompt(prompt_input, ctx)
    else:
        prompt_text = prompt_input
    # Create a label for display and logging
    code_label = (
        str(code_dir)
        if code_dir is not None
        else f'{ctx["crate_name"]}:{ctx["year"]}'
    )
    
    # Display progress
    print(f"• [{sample_index}/{total_samples}] analyzing {code_label} …", file=sys.stderr)
    
    # Run the model
    prediction = run_pipeline(
        prompt=prompt_text,
        system_prompt=system_prompt_input,
        schema_model=task_instance.schema,
        code_dir=code_dir,
        model_name=model_name,
        provider=provider,
    )

    # Check if the prediction is an error
    if isinstance(prediction, LLMError):
        failure_info = {
            "timestamp": datetime.now(timezone.utc).isoformat(),
            "llm_provider": provider,
            "llm_model": model_name,
            "code_sample": code_label,
            "error_type": prediction.error_type,
            "error_message": prediction.error_message,
            "context": ctx if ctx else None,
            "ground_truth": ground_truth,
        }
        return None, None, code_label, failure_info

    # Score the prediction
    score = task_instance.score(prediction, ground_truth)

    # Create metadata and record
    meta = {
        "timestamp": datetime.now(timezone.utc).isoformat(),
        "llm_provider": provider,
        "llm_model": model_name,
        "code_sample": code_label,
    }
    record = {**meta, "answer": prediction, "scoring": score}
    csv_row = task_instance.dataset_row(meta, score)

    return record, csv_row, code_label, None


def main():
    """Run the CLI application."""
    args = parse_args()

    # Initialize task
    task_cls = TASK_REGISTRY[args.task]
    task = task_cls()
    prompt_file = args.prompt or task.prompt_path

    # Reading in the system_prompt based on the task.prompt_path
    if isinstance(prompt_file, str):
        prompt_path = Path(prompt_file)
    else:
        prompt_path = prompt_file

    system_prompt_path = prompt_path.with_name(prompt_path.stem + "_system_prompt.md")

    # Read samples
    samples = list(task.iterate_samples(args.input))
    if not samples:
        sys.exit("No samples found in the input file")

    # Set up output paths
    json_path, csv_path, failures_path = setup_output_paths(
        args.task, args.provider, args.model
    )

    # Track successful results and failures
    successful_count = 0
    failure_count = 0
    first_failure = True

    # Process samples with progress bar
    for idx, sample in enumerate(
        tqdm(samples, desc="Processing samples", unit="sample", colour="green"), 1
    ):
        # Analyze sample
        record, csv_row, _, failure_info = analyze_sample(
            sample_data=sample,
            task_instance=task,
            prompt_input=prompt_file,
            system_prompt_input=system_prompt_path,
            model_name=args.model,
            provider=args.provider,
            sample_index=idx,
            total_samples=len(samples),
        )
        
        # Handle failure
        if failure_info is not None:
            failure_count += 1
            if args.out != "-":  # Don't save failures in stdout mode
                save_failure(failure_info, failures_path, first_failure)
                first_failure = False
            print(f"  → Failed: {failure_info['error_type']}", file=sys.stderr)
            continue

        # Save successful results
        successful_count += 1
        is_first = successful_count == 1 and (not args.out or args.out != "-")
        stdout_mode = args.out == "-"
        save_results(
            record=record,
            row_dict=csv_row,
            json_path=args.out or json_path if not stdout_mode else None,
            csv_path=csv_path,
            is_first=is_first,
            stdout_mode=stdout_mode,
        )
    # Show summary
    if args.out != "-":
        print(f"\nFinished. Successful: {successful_count}, Failed: {failure_count}")
        if successful_count > 0:
            print(f"Full JSON → {(args.out or json_path).resolve()}")
            print(f"Dataset CSV → {csv_path.resolve()}")
        if failure_count > 0:
            print(f"Failures → {failures_path.resolve()}")


if __name__ == "__main__":
    main()