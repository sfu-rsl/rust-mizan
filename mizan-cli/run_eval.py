#!/usr/bin/env python3

from inspect_ai import eval
from inspect_ai.model import get_model
from mizan_cli.inspect_benchmark import rustmizan

MODELS = [
    "anthropic/claude-sonnet-4-6",
    "openai/gpt-5.4",
    "google/gemini-3.1-pro-preview",
    get_model("openrouter/qwen/qwen3.6-plus", reasoning_enabled=False),
]

MESSAGE_LIMIT = 100     # Limit the number of messages the agent can send during evaluation, set to None for no limit
TIME_LIMIT = 3600       # Time limit for each task run in seconds
LIMIT = None            # Set to an integer to limit the number of samples evaluated
SAMPLE_IDS = None       # Set to a list of sample IDs to evaluate specific samples, e.g., ["sample-001", "sample-002"]

DATASET_PATHS = [
    "/home/tarek/Documents/workspace/rust-mizan/datasets/mizan-vanilla.parquet",
    # "/home/tarek/Documents/workspace/rust-mizan/datasets/mizan-benign.parquet",
    # "/home/tarek/Documents/workspace/rust-mizan/datasets/mizan-malignant.parquet",
    # "/home/tarek/Documents/workspace/rust-mizan/datasets/mizan-rust-specific.parquet",
]

if __name__ == "__main__":
    tasks = rustmizan(
        dataset_paths=DATASET_PATHS,
        sample_ids=SAMPLE_IDS,
    )

    eval(
        tasks=tasks,
        model=MODELS,
        limit=LIMIT,
        fail_on_error=False,
        message_limit=MESSAGE_LIMIT,
        time_limit=TIME_LIMIT,
        # Keeps concurrent compose networks under Docker Desktop's default
        # pool (~15 /16 slots). Also serves as the primary memory lever —
        # raise/lower to trade throughput for RAM.
        max_sandboxes=10,
        # only affects Gemini 3 Pro
        # See https://inspect.aisi.org.uk/reasoning.html#google-gemini
        reasoning_effort="low",
    )
