#!/usr/bin/env python3

from pathlib import Path
from inspect_ai import eval
from mizan_cli.inspect_benchmark import rustmizan

# Configuration - modify these constants as needed
MODELS = [
    "anthropic/claude-sonnet-4-5-20250929",
    "openai/gpt-5.2-2025-12-11",
    "google/gemini-3-pro-preview",
    "openai-api/deepseek/deepseek-chat",
]

MESSAGE_LIMIT = 60  # Limit the number of messages the agent can send during evaluation, set to None for no limit
TIME_LIMIT = 3600  # Time limit for each task run in seconds
LIMIT = None  # Set to an integer to limit the number of samples evaluated
SAMPLE_IDS = None  # Set to a list of sample IDs to evaluate specific samples, e.g., ["sample-001", "sample-002"]
DATASET_PATH = "/Users/tareknasser/Documents/workspace/sfu/mizan/rust-mizan/logs/ten_vulnerable_functions.parquet"

if __name__ == "__main__":
    dataset_path = Path(DATASET_PATH)

    task = rustmizan(
        dataset_path=dataset_path,
        sample_ids=SAMPLE_IDS,
    )

    eval(
        tasks=task,
        model=MODELS,
        limit=LIMIT,
        fail_on_error=False,
        message_limit=MESSAGE_LIMIT,
        time_limit=TIME_LIMIT,
        # only affects Gemini 3 Pro
        # See https://inspect.aisi.org.uk/reasoning.html#google-gemini
        reasoning_effort="low",
    )
