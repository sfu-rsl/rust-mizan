#!/usr/bin/env python3

from pathlib import Path
from inspect_ai import eval
from mizan_cli.inspect_benchmark import rustmizan

# Configuration - modify these constants as needed
MODELS = [
    "anthropic/claude-haiku-4-5",
    "anthropic/claude-sonnet-4-5",
]

MAX_TURNS = 50
LIMIT = None  # Set to an integer to limit the number of samples evaluated
SAMPLE_IDS = None  # Set to a list of sample IDs to evaluate specific samples, e.g., ["sample-001", "sample-002"]
DATASET_PATH = "/Users/tareknasser/Documents/workspace/sfu/mizan/rust-mizan/logs/ten_vulnerable_functions.parquet"

if __name__ == "__main__":
    dataset_path = Path(DATASET_PATH)

    task = rustmizan(
        dataset_path=dataset_path,
        sample_ids=SAMPLE_IDS,
        max_turns=MAX_TURNS,
    )
    eval(tasks=task, model=MODELS, limit=LIMIT)
