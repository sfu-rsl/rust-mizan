import json
from pathlib import Path
from typing import List, Any, Dict
from uuid import uuid4
from datetime import datetime

from langsmith.schemas import Example, Dataset

from mizan_cli.commands.checkout.models import MizanDataset
from mizan_cli.utils.logging import get_logger

logger = get_logger()


class PrepareDatasetCommand:
    def __init__(self, output_path: Path):
        self.output_path = output_path
        self.current_dir = Path.cwd()
        self.samples_dir = self.current_dir / "samples"
        self.prompt_template_path = self._get_prompt_template_path()
        self.system_prompt_path = self._get_system_prompt_path()

    def _get_prompt_template_path(self) -> Path:
        assets_dir = Path(__file__).parent.parent.parent / "assets" / "evaluate"
        return assets_dir / "prompt_template.md"

    def _get_system_prompt_path(self) -> Path:
        assets_dir = Path(__file__).parent.parent.parent / "assets" / "evaluate"
        return assets_dir / "system_prompt.md"

    def execute(self):
        logger.info("Preparing dataset for evaluation")

        mizan_path = self.current_dir / "mizan.json"
        dataset = MizanDataset.from_file(mizan_path)
        mutations_metadata = self._get_mutations_metadata()
        prompt_template = self._load_prompt_template()
        system_prompt = self._load_system_prompt()

        examples = self._create_examples(dataset, prompt_template, system_prompt)

        # A dataset is a collection of examples with metadata
        dataset_name = f"mizan-evaluation-{'-'.join(mutations_metadata.get('mutations_applied', [])) if mutations_metadata.get('mutations_applied') else 'vanilla'}"
        evaluation_dataset = Dataset(
            name=dataset_name,
            id=uuid4(),
            created_at=datetime.now(),
        )

        dataset_data = {
            "dataset": {
                "name": evaluation_dataset.name,
                "id": str(evaluation_dataset.id),
                "created_at": evaluation_dataset.created_at.isoformat(),
            },
            "mutations_metadata": mutations_metadata,
            "examples": [
                {
                    "inputs": example.inputs,
                    "outputs": example.outputs,
                    "metadata": example.metadata,
                }
                for example in examples
            ],
        }

        self.output_path.parent.mkdir(parents=True, exist_ok=True)
        with open(self.output_path, "w") as f:
            json.dump(dataset_data, f, indent=2)

        logger.info(f"Dataset saved to: {self.output_path}")

    def _get_mutations_metadata(self) -> Dict[str, Any]:
        # We expect to run this command in the output directory of 'mizan checkout'
        mutations_file = self.current_dir / "mizan_mutations.json"
        if not mutations_file.exists():
            return {}

        try:
            with open(mutations_file, "r") as f:
                return json.load(f)
        except Exception as e:
            logger.warning(f"Could not read mutations file: {e}")
            return {}

    def _load_prompt_template(self) -> str:
        with open(self.prompt_template_path, "r") as f:
            return f.read()

    def _load_system_prompt(self) -> str:
        with open(self.system_prompt_path, "r") as f:
            return f.read()

    def _create_examples(
        self, dataset: MizanDataset, prompt_template: str, system_prompt: str
    ) -> List[Example]:
        examples = []

        for vuln in dataset.vulnerabilities:
            for sample_idx, sample in enumerate(vuln.code_samples):
                prompt = self._generate_prompt(sample, prompt_template)

                example = Example(
                    id=uuid4(),
                    dataset_id=uuid4(),
                    inputs={
                        "system_prompt": system_prompt,
                        "prompt": prompt,
                    },
                    outputs={
                        "is_vulnerable": sample.is_vulnerability,
                        "cwe_type": sample.cwe_type,
                        "vulnerable_functions": sample.vulnerable_functions,
                        "vulnerable_lines": sample.vulnerable_lines,
                    },
                    metadata={
                        "id": sample.path_to_crate,
                        "vuln_id": vuln.id,
                        "granularity": sample.level,
                        "crate_name": vuln.crate_name,
                        "is_vulnerable": sample.is_vulnerability,
                        "year": vuln.year,
                        "cwe_types": sample.cwe_type,
                    },
                    created_at=datetime.now(),
                )

                examples.append(example)

        return examples

    def _generate_prompt(self, sample: Any, prompt_template: str) -> str:
        sample_path = self.samples_dir / sample.path_to_crate
        code_content = self._collect_code_files(sample_path)

        # The prompt has a placeholder for the code content
        return prompt_template.replace("{CODE}", code_content)

    def _collect_code_files(self, sample_path: Path) -> str:
        """Collect all code files from a sample directory. We only include Rust files and their Cargo.toml if it exists."""
        code_parts = []
        files_to_include = []

        cargo_toml = sample_path / "Cargo.toml"
        if cargo_toml.exists():
            files_to_include.append(cargo_toml)

        for rs_file in sample_path.rglob("*.rs"):
            files_to_include.append(rs_file)

        files_to_include.sort()

        for file_path in files_to_include:
            try:
                relative_path = file_path.relative_to(sample_path)

                with open(file_path, "r", encoding="utf-8") as f:
                    content = f.read()

                lines = content.split("\n")
                numbered_lines = [f"{i:04d}: {line}" for i, line in enumerate(lines, 1)]
                numbered_content = "\n".join(numbered_lines)

                code_parts.append(
                    f"## File: {relative_path}\n\n```rust\n{numbered_content}\n```\n"
                )

            except Exception as e:
                logger.warning(f"Could not read file {file_path}: {e}")

        return "\n".join(code_parts)
