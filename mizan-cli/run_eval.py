#!/usr/bin/env python3

from pathlib import Path

from mizan_cli.inspect_benchmark import run_evaluation

MODELS = None  # None uses the agent's default models (or DEFAULT_MODELS), or e.g. ["anthropic/claude-sonnet-4-6"]

AGENT = "codex_cli"     # See `mizan evaluate agents`
AGENT_ARGS = {}         # Overrides for the agent's default settings, e.g. {"web_search": "live"}
MESSAGE_LIMIT = None        # Limit the number of messages the agent can send during evaluation, set to None for no limit
TIME_LIMIT = None           # Time limit for each task run in seconds
LIMIT = 1                   # Set to an integer to limit the number of samples evaluated
SAMPLE_IDS = None           # Set to a list of sample IDs to evaluate specific samples, e.g., ["sample-001", "sample-002"]
REASONING_EFFORT = "low"    # e.g. "medium", "high", or None for the model's default

DATASET_DIR = Path(__file__).resolve().parents[1] / "datasets"
# The benign runs used fail_on_error=True; other splits used False.
FAIL_ON_ERROR = False

DATASET_PATHS = [
    DATASET_DIR / "mizan-vanilla.parquet",
    # DATASET_DIR / "mizan-benign.parquet",
    # DATASET_DIR / "mizan-malignant.parquet",
    # DATASET_DIR / "mizan-rust-specific.parquet",
]

if __name__ == "__main__":
    try:
        run_evaluation(
            dataset_paths=DATASET_PATHS,
            agent=AGENT,
            agent_args=AGENT_ARGS,
            models=MODELS,
            sample_ids=SAMPLE_IDS,
            limit=LIMIT,
            message_limit=MESSAGE_LIMIT,
            time_limit=TIME_LIMIT,
            fail_on_error=FAIL_ON_ERROR,
            reasoning_effort=REASONING_EFFORT,
        )
    except ValueError as e:
        raise SystemExit(str(e))
