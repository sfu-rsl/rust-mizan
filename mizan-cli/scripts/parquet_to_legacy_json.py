#!/usr/bin/env python3
"""Convert RustMizan parquet datasets to the legacy LangSmith-style JSON format
used on the `evaluation` branch.

Reads every *.parquet in the input directory and writes a corresponding *.json
with the same stem to the output directory. The JSON shape matches what
`PrepareDatasetCommand` used to emit:

{
  "dataset": {"name": ..., "id": <uuid>, "created_at": <iso>},
  "mutations_metadata": {...},
  "examples": [
    {
      "inputs": {"system_prompt": str, "prompt": str},   # code inlined into prompt
      "outputs": {"is_vulnerable": bool, "cwe_type": [...],
                   "vulnerable_functions": {...}, "vulnerable_lines": {...}},
      "metadata": {"id": sample_id, "vuln_id": ..., "granularity": ...,
                    "crate_name": ..., "is_vulnerable": ..., "year": ...,
                    "cwe_types": [...]}
    }, ...
  ]
}
"""

from __future__ import annotations

import argparse
import json
from datetime import datetime
from pathlib import Path
from typing import Any, Iterable
from uuid import uuid4

import pyarrow.parquet as pq

SCRIPT_DIR = Path(__file__).resolve().parent
PROMPT_TEMPLATE_PATH = SCRIPT_DIR / "prompt_template.md"
SYSTEM_PROMPT_PATH = SCRIPT_DIR / "system_prompt.md"


def format_code(files: Iterable[dict[str, str]]) -> str:
    """Format files for prompt inlining: Cargo.toml first, then .rs sorted."""
    entries = list(files)
    # Sort: Cargo.toml first, then .rs files alphabetically by path.
    entries.sort(
        key=lambda f: (0 if f["path"].endswith("Cargo.toml") else 1, f["path"])
    )
    parts: list[str] = []
    for entry in entries:
        path = entry["path"]
        content = entry["content"]
        numbered = "\n".join(
            f"{i:04d}: {line}" for i, line in enumerate(content.split("\n"), 1)
        )
        parts.append(f"## File: {path}\n\n```rust\n{numbered}\n```\n")
    return "\n".join(parts)


def row_to_example(row: dict[str, Any], prompt_template: str, system_prompt: str) -> dict[str, Any]:
    # Map/list-of-tuples fields may come back as list[tuple] from pyarrow.
    vuln_funcs = row["vulnerable_functions"]
    if isinstance(vuln_funcs, list):
        vuln_funcs = dict(vuln_funcs)
    vuln_lines = row["vulnerable_lines"]
    if isinstance(vuln_lines, list):
        vuln_lines = dict(vuln_lines)

    code_block = format_code(row["files"])
    prompt = prompt_template.replace("{CODE}", code_block)

    return {
        "inputs": {"system_prompt": system_prompt, "prompt": prompt},
        "outputs": {
            "is_vulnerable": row["is_vulnerable"],
            "cwe_type": list(row["cwe_type"] or []),
            "vulnerable_functions": vuln_funcs,
            "vulnerable_lines": vuln_lines,
        },
        "metadata": {
            "id": row["sample_id"],
            "vuln_id": row["vuln_id"],
            "granularity": row["granularity"],
            "crate_name": row["crate_name"],
            "is_vulnerable": row["is_vulnerable"],
            "year": row["year"],
            "cwe_types": list(row["cwe_type"] or []),
        },
    }


def convert(parquet_path: Path, output_path: Path, prompt_template: str, system_prompt: str) -> int:
    table = pq.read_table(parquet_path)

    # Parquet header metadata -> mutations_metadata + tag for dataset name.
    header = {
        k.decode() if isinstance(k, bytes) else k: v.decode() if isinstance(v, bytes) else v
        for k, v in (table.schema.metadata or {}).items()
    }
    try:
        mutations_metadata = json.loads(header.get("mutations_metadata", "{}")) or {}
    except json.JSONDecodeError:
        mutations_metadata = {}

    mutations_applied = mutations_metadata.get("mutations_applied") or []
    name_suffix = "-".join(mutations_applied) if mutations_applied else (header.get("tag") or "vanilla")
    dataset_name = f"mizan-evaluation-{name_suffix}"

    examples = [row_to_example(row, prompt_template, system_prompt) for row in table.to_pylist()]

    payload = {
        "dataset": {
            "name": dataset_name,
            "id": str(uuid4()),
            "created_at": datetime.now().isoformat(),
        },
        "mutations_metadata": mutations_metadata,
        "examples": examples,
    }

    output_path.parent.mkdir(parents=True, exist_ok=True)
    with open(output_path, "w") as f:
        json.dump(payload, f, indent=2)

    return len(examples)


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--input-dir",
        type=Path,
        default=Path("publication_artifacts/datasets"),
        help="Directory containing *.parquet datasets (default: publication_artifacts/datasets).",
    )
    parser.add_argument(
        "--output-dir",
        type=Path,
        default=None,
        help="Directory to write *.json datasets (default: same as --input-dir).",
    )
    args = parser.parse_args()

    input_dir: Path = args.input_dir.resolve()
    output_dir: Path = (args.output_dir or args.input_dir).resolve()

    prompt_template = PROMPT_TEMPLATE_PATH.read_text()
    system_prompt = SYSTEM_PROMPT_PATH.read_text()

    parquet_files = sorted(input_dir.glob("*.parquet"))
    if not parquet_files:
        raise SystemExit(f"No parquet files found in {input_dir}")

    for parquet_path in parquet_files:
        output_path = output_dir / f"{parquet_path.stem}.json"
        n = convert(parquet_path, output_path, prompt_template, system_prompt)
        print(f"{parquet_path.name} -> {output_path} ({n} examples)")


if __name__ == "__main__":
    main()
