#!/usr/bin/env python3

from pathlib import Path
from inspect_ai import eval
from mizan_cli.inspect_benchmark import rustmizan

# Configuration - modify these constants as needed
MODELS = [
    "anthropic/claude-sonnet-4-5",
]

MAX_TURNS = 30
LIMIT = None  # Set to an integer to limit the number of samples evaluated
SAMPLE_IDS = None  # Set to a list of sample IDs to evaluate specific samples, e.g., ["sample-001", "sample-002"]
DATASET_PATH = "/Users/tareknasser/Documents/workspace/sfu/mizan/rust-mizan/output/dataset.parquet"  # Absolute path to dataset


if __name__ == "__main__":
    dataset_path = Path(DATASET_PATH)

    for model in MODELS:
        print(f"Running evaluation with model: {model}")
        task = rustmizan(
            dataset_path=dataset_path,
            sample_ids=SAMPLE_IDS,
            max_turns=MAX_TURNS,
        )
        eval(tasks=task, model=model, limit=LIMIT)
