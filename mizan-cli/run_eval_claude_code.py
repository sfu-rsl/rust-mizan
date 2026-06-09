#!/usr/bin/env python3


from pathlib import Path
from inspect_ai import eval
from inspect_swe import claude_code
from mizan_cli.inspect_benchmark import rustmizan


MESSAGE_LIMIT = 60
LIMIT = None
SAMPLE_IDS = None
# Point this at the parquet produced by `mizan evaluate prepare-dataset`
DATASET_PATH = "dataset.parquet"

# Model for Claude Code to use
MODEL = "anthropic/claude-sonnet-4-5"

# Claude Code configuration
SYSTEM_PROMPT = """You are a security auditor analyzing Rust code for vulnerabilities.
Focus on memory safety issues and follow the task instructions carefully."""
DISALLOWED_TOOLS = ["WebSearch"]


if __name__ == "__main__":
    dataset_path = Path(DATASET_PATH)

    task = rustmizan(
        dataset_path=dataset_path,
        sample_ids=SAMPLE_IDS,
        agent=claude_code(
            system_prompt=SYSTEM_PROMPT,
            disallowed_tools=DISALLOWED_TOOLS,
        ),
        message_limit=MESSAGE_LIMIT,
    )

    eval(tasks=task, model=MODEL, limit=LIMIT)
