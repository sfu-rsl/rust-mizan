import argparse
import json
import sys
import re
from datetime import datetime
from pathlib import Path
import pandas as pd

sys.path.append(str(Path(__file__).parent.parent))

from common.metrics import compute_sample_metrics
from common.data_utils import get_short_model_name


def parse_json_response(response_text):
    """Parse JSON from LLM output, handling markdown code blocks."""
    json_pattern = r"```json\s*(.*?)\s*```"
    match = re.search(json_pattern, response_text, re.DOTALL)
    json_text = match.group(1) if match else response_text.strip()

    try:
        return json.loads(json_text)
    except json.JSONDecodeError:
        return None


def validate_response_structure(parsed_response):
    """Check if response has required fields."""
    if not parsed_response:
        return False
    required_fields = [
        "is_vulnerable",
        "cwe_type",
        "vulnerable_functions",
        "vulnerable_lines",
    ]
    return all(field in parsed_response for field in required_fields)


def create_results_json(results_jsonl, dataset):
    """Create results.json format from JSONL results and dataset."""
    dataset_map = {
        example["metadata"]["id"]: example for example in dataset["examples"]
    }
    results = []

    for entry in results_jsonl:
        ground_truth = json.loads(entry["ground_truth"])
        metadata = entry["metadata"]
        example_id = metadata["id"]

        if example_id not in dataset_map:
            print(f"Warning: Example {example_id} not found in dataset, skipping")
            continue

        dataset_example = dataset_map[example_id]
        prediction_text = entry["prediction"]
        parsed_response = parse_json_response(prediction_text)
        is_valid_json = validate_response_structure(parsed_response)

        result_entry = {
            "example_id": example_id,
            "vuln_id": metadata["vuln_id"],
            "granularity": metadata["granularity"],
            "crate_name": metadata["crate_name"],
            "is_vulnerable": metadata["is_vulnerable"],
            "year": metadata["year"],
            "cwe_types": metadata["cwe_types"],
            "inputs": dataset_example["inputs"],
            "outputs": {
                "raw_response": prediction_text,
                "parsed_response": parsed_response if is_valid_json else {},
            },
            "reference_outputs": ground_truth,
        }

        results.append(result_entry)

    return results


def process_results_and_compute_metrics(results):
    """Compute metrics for each sample."""
    processed_results = []

    for result in results:
        is_valid_json = bool(result["outputs"]["parsed_response"])

        if is_valid_json:
            try:
                metrics = compute_sample_metrics(result)
            except Exception as e:
                print(
                    f"Warning: Failed to compute metrics for {result['example_id']}: {e}"
                )
                is_valid_json = False
                metrics = {}
        else:
            metrics = {}

        processed_entry = {
            "example_id": result["example_id"],
            "vuln_id": result["vuln_id"],
            "granularity": result["granularity"],
            "crate_name": result["crate_name"],
            "is_vulnerable_gt": result["reference_outputs"]["is_vulnerable"],
            "is_vulnerable_pred": (
                result["outputs"]["parsed_response"].get("is_vulnerable", None)
                if is_valid_json
                else None
            ),
            "is_valid_json": is_valid_json,
            **metrics,
        }

        processed_results.append(processed_entry)

    return processed_results


def main():
    parser = argparse.ArgumentParser(description="Preprocess fine-tuning results")
    parser.add_argument("--results-file", required=True, help="JSONL results file")
    parser.add_argument("--dataset-file", required=True, help="Dataset JSON file")
    parser.add_argument("--output-dir", required=True, help="Output directory")
    parser.add_argument("--experiment-name", required=True, help="Experiment name")
    parser.add_argument(
        "--model-name", default="Qwen2.5-7B-128k-Instruct", help="Model name"
    )
    args = parser.parse_args()

    results_path = Path(args.results_file)
    dataset_path = Path(args.dataset_file)

    if not results_path.exists():
        print(f"Error: Results file not found: {results_path}")
        sys.exit(1)

    if not dataset_path.exists():
        print(f"Error: Dataset file not found: {dataset_path}")
        sys.exit(1)

    output_dir = Path(args.output_dir)
    output_dir.mkdir(parents=True, exist_ok=True)

    print(f"Loading dataset from {dataset_path}...")
    with open(dataset_path) as f:
        dataset = json.load(f)

    print(f"Loading results from {results_path}...")
    with open(results_path) as f:
        results_jsonl = [json.loads(line) for line in f]

    print(f"Loaded {len(results_jsonl)} results")

    print("Creating results.json...")
    results = create_results_json(results_jsonl, dataset)
    print(f"Processed {len(results)} results")

    metadata = {
        "dataset": dataset["dataset"],
        "mutations_metadata": dataset.get("mutations_metadata", {}),
        "experiment_name": args.experiment_name,
        "model_name": args.model_name,
        "created_at": datetime.now().isoformat(),
    }

    with open(output_dir / "results.json", "w") as f:
        json.dump(results, f, indent=2)

    with open(output_dir / "metadata.json", "w") as f:
        json.dump(metadata, f, indent=2)

    print("Computing metrics and creating processed_results.csv...")
    processed_results = process_results_and_compute_metrics(results)

    df = pd.DataFrame(processed_results)
    df["model"] = get_short_model_name(args.model_name)
    df.to_csv(output_dir / "processed_results.csv", index=False)

    print("\n" + "=" * 60)
    print("Summary Statistics:")
    print("=" * 60)
    print(f"Total samples: {len(df)}")
    print(
        f"Valid JSON responses: {df['is_valid_json'].sum()} ({df['is_valid_json'].mean()*100:.1f}%)"
    )
    print(f"Vulnerable samples (ground truth): {df['is_vulnerable_gt'].sum()}")

    if df["is_valid_json"].sum() > 0:
        valid_df = df[df["is_valid_json"] == True]
        print(f"\nMetrics on valid samples:")
        print(f"  Binary accuracy: {valid_df['binary_accuracy'].mean()*100:.1f}%")
        if "success_at_1_line" in valid_df.columns:
            vuln_df = valid_df[valid_df["is_vulnerable_gt"] == True]
            if len(vuln_df) > 0:
                print(
                    f"  Success@1-Line (vulnerable only): {vuln_df['success_at_1_line'].mean()*100:.1f}%"
                )
    print("=" * 60)
    print(f"\nDone! Experiment files created in: {output_dir}")


if __name__ == "__main__":
    main()
