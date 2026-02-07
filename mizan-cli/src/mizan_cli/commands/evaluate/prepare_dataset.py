import json
from pathlib import Path
from typing import List, Dict, Any
import pyarrow as pa
import pyarrow.parquet as pq

from mizan_cli.commands.checkout.models import MizanDataset
from mizan_cli.utils.logging import get_logger

logger = get_logger()


class PrepareDatasetCommand:
    def __init__(self, output_path: Path, tag: str | None = None):
        self.output_path = output_path
        self.tag = tag
        self.current_dir = Path.cwd()
        self.samples_dir = self.current_dir / "samples"

    def execute(self):
        logger.info("Preparing dataset for evaluation")

        mizan_path = self.current_dir / "mizan.json"
        dataset = MizanDataset.from_file(mizan_path)

        with open(mizan_path, "r") as f:
            mizan_data = json.load(f)
        general_info = mizan_data.get("general_information", {})
        mutations_metadata = self._get_mutations_metadata()

        rows = self._create_rows(dataset)

        schema = self._create_schema()
        table = pa.Table.from_pylist(rows, schema=schema)

        file_metadata = {
            "rust_version": general_info.get("rust_version", ""),
            "dataset_version": general_info.get("dataset_version", ""),
            "benchmark_name": general_info.get("benchmark_name", ""),
            "tag": self.tag if self.tag else "",
            "mutations_metadata": json.dumps(mutations_metadata),
        }
        existing_metadata = table.schema.metadata or {}
        merged_metadata = {
            **existing_metadata,
            **{k.encode(): v.encode() for k, v in file_metadata.items()},
        }
        table = table.replace_schema_metadata(merged_metadata)

        self.output_path.parent.mkdir(parents=True, exist_ok=True)
        pq.write_table(table, self.output_path)

        logger.info(f"Dataset saved to: {self.output_path}")

    def _get_mutations_metadata(self) -> Dict[str, Any]:
        mutations_file = self.current_dir / "mizan_mutations.json"
        if not mutations_file.exists():
            return {}

        try:
            with open(mutations_file, "r") as f:
                return json.load(f)
        except Exception as e:
            logger.warning(f"Could not read mutations file: {e}")
            return {}

    def _create_schema(self):
        return pa.schema(
            [
                ("sample_id", pa.string()),
                ("vuln_id", pa.string()),
                ("crate_name", pa.string()),
                ("granularity", pa.string()),
                ("year", pa.int64()),
                ("is_vulnerable", pa.bool_()),
                ("cwe_type", pa.list_(pa.string())),
                ("vulnerable_functions", pa.map_(pa.string(), pa.list_(pa.string()))),
                ("vulnerable_lines", pa.map_(pa.string(), pa.list_(pa.int64()))),
                (
                    "files",
                    pa.list_(
                        pa.struct([("path", pa.string()), ("content", pa.string())])
                    ),
                ),
            ]
        )

    def _create_rows(self, dataset: MizanDataset) -> List[Dict[str, Any]]:
        rows = []

        for vuln in dataset.vulnerabilities:
            for sample in vuln.code_samples:
                sample_path = self.samples_dir / sample.path_to_crate
                files = self._collect_files(sample_path)

                row = {
                    "sample_id": sample.path_to_crate,
                    "vuln_id": vuln.id,
                    "crate_name": vuln.crate_name,
                    "granularity": sample.level,
                    "year": vuln.year,
                    "is_vulnerable": sample.is_vulnerability,
                    "cwe_type": sample.cwe_type,
                    "vulnerable_functions": sample.vulnerable_functions,
                    "vulnerable_lines": sample.vulnerable_lines,
                    "files": files,
                }

                rows.append(row)

        return rows

    def _collect_files(self, sample_path: Path) -> List[Dict[str, str]]:
        files = []

        cargo_toml = sample_path / "Cargo.toml"
        if cargo_toml.exists():
            try:
                files.append(
                    {
                        "path": "Cargo.toml",
                        "content": cargo_toml.read_text(encoding="utf-8"),
                    }
                )
            except Exception as e:
                logger.warning(f"Could not read file {cargo_toml}: {e}")

        for rs_file in sorted(sample_path.rglob("*.rs")):
            try:
                relative_path = rs_file.relative_to(sample_path)
                files.append(
                    {
                        "path": str(relative_path),
                        "content": rs_file.read_text(encoding="utf-8"),
                    }
                )
            except Exception as e:
                logger.warning(f"Could not read file {rs_file}: {e}")

        return files
