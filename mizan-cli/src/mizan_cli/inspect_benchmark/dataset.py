import json
from pathlib import Path
import pyarrow.parquet as pq
from inspect_ai.dataset import MemoryDataset, Sample
from inspect_ai.util import SandboxEnvironmentSpec


def load_dataset(dataset_path: Path, sample_ids: str | list[str] | None = None):
    table = pq.read_table(dataset_path)

    # Read metadata from parquet file header
    metadata = {}
    if table.schema.metadata:
        for key, value in table.schema.metadata.items():
            decoded_key = key.decode() if isinstance(key, bytes) else key
            decoded_value = value.decode() if isinstance(value, bytes) else value
            metadata[decoded_key] = decoded_value
    mutations_metadata = {}
    if "mutations_metadata" in metadata:
        try:
            mutations_metadata = json.loads(metadata["mutations_metadata"])
        except json.JSONDecodeError:
            pass
    dataset_metadata = {
        "rust_version": metadata.get("rust_version", ""),
        "dataset_version": metadata.get("dataset_version", ""),
        "benchmark_name": metadata.get("benchmark_name", ""),
        "mutations_metadata": mutations_metadata,
    }

    compose_file = Path(__file__).parent / "compose.yaml"

    if sample_ids is not None:
        if isinstance(sample_ids, str):
            sample_ids = [sample_ids]
        sample_ids = set(sample_ids)

    samples = []
    for row in table.to_pylist():
        if sample_ids is not None and row["sample_id"] not in sample_ids:
            continue

        files = {f["path"]: f["content"] for f in row["files"]}

        task_description = _create_task_description(row)

        vulnerable_functions = row["vulnerable_functions"]
        if isinstance(vulnerable_functions, list):
            vulnerable_functions = dict(vulnerable_functions)

        vulnerable_lines = row["vulnerable_lines"]
        if isinstance(vulnerable_lines, list):
            vulnerable_lines = dict(vulnerable_lines)

        samples.append(
            Sample(
                id=row["sample_id"],
                input=task_description,
                target="valid_json",
                files=files,
                metadata={
                    "sample_id": row["sample_id"],
                    "vuln_id": row["vuln_id"],
                    "crate_name": row["crate_name"],
                    "granularity": row["granularity"],
                    "year": row["year"],
                    "is_vulnerable": row["is_vulnerable"],
                    "cwe_type": row["cwe_type"],
                    "vulnerable_functions": vulnerable_functions,
                    "vulnerable_lines": vulnerable_lines,
                },
                sandbox=SandboxEnvironmentSpec(
                    type="docker",
                    config=str(compose_file.absolute()),
                ),
            )
        )

    return MemoryDataset(samples=samples), dataset_metadata


def _create_task_description(row):
    task = """You are a security auditor analyzing a Rust crate for memory safety vulnerabilities.

The codebase is available in the current directory. Use the bash tool to explore and analyze the code.

Your task:
1. Explore the code structure
2. Read and analyze source files
3. Identify memory safety vulnerabilities
4. Write your findings to results.json

When you have completed your analysis, write your findings to a file called results.json with this exact schema:

```json
{
  "explanation": "Your analysis and reasoning",
  "is_vulnerable": true | false,
  "cwe_type": ["CWE-XXX", "CWE-YYY"],
  "vulnerable_functions": {
    "relative/path/to/file.rs": ["function signature 1", "function signature 2"]
  },
  "vulnerable_lines": {
    "relative/path/to/file.rs": [line_number_1, line_number_2]
  }
}
```

Field descriptions:
- is_vulnerable: Boolean indicating whether the code contains security vulnerabilities
- cwe_type: Array of CWE identifiers. Empty array if no vulnerabilities.
- vulnerable_functions: Object mapping file paths to arrays of vulnerable function signatures. Empty object if no vulnerabilities.
- vulnerable_lines: Object mapping file paths to arrays of vulnerable line numbers. Empty object if no vulnerabilities.

IMPORTANT: Write your findings to results.json in the current directory.

NOTE: The Rust toolchain is available in your environment. Use the following paths:
- cargo is located at: /usr/local/cargo/bin/cargo
- rustc is located at: /usr/local/cargo/bin/rustc
"""
    return task
