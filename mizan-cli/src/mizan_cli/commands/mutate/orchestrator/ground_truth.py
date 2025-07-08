import json
import os
from typing import Dict, List
from mizan_cli.utils.logging import get_logger


logger = get_logger()


class MutationMetadata:
    def __init__(self, base_dir: str):
        self.base_dir = base_dir
        self.metadata_path = os.path.join(base_dir, "mizan_mutations.json")
        self.mutations_applied: List[str] = []
        self.failures: Dict[str, List[str]] = {}
        self.partial_applications: Dict[str, List[str]] = {}
        self._load_existing()

    def _load_existing(self):
        if os.path.exists(self.metadata_path):
            try:
                with open(self.metadata_path, "r") as f:
                    data = json.load(f)
                self.mutations_applied = data.get("mutations_applied", [])
                self.failures = data.get("failures", {})
                self.partial_applications = data.get("partial_applications", {})
            except Exception as e:
                logger.warning(f"Could not load existing metadata: {e}")

    def add_successful_mutation(self, mutation_name: str):
        if mutation_name not in self.mutations_applied:
            self.mutations_applied.append(mutation_name)

    def add_failed_mutation(self, mutation_name: str, error: str):
        # For complete failures (mutation couldn't be applied at all)
        if mutation_name not in self.failures:
            self.failures[mutation_name] = []
        self.failures[mutation_name].append(f"Failed to apply: {error}")

    def add_partial_mutation(
        self, mutation_name: str, failed_samples: List[Dict[str, str]]
    ):
        """Record a mutation that was partially successful (some samples failed)."""
        if mutation_name not in self.failures:
            self.failures[mutation_name] = []

        # Extract just the sample paths from the failed samples
        sample_paths = [sample["sample_path"] for sample in failed_samples]
        self.failures[mutation_name].extend(sample_paths)

        # Also add to mutations_applied since it was partially successful
        if mutation_name not in self.mutations_applied:
            self.mutations_applied.append(mutation_name)

    def add_partial_applications(self, mutation_name: str, partial_samples: List[str]):
        """Add samples that were partially mutated for a specific mutation."""
        if partial_samples:
            self.partial_applications[mutation_name] = partial_samples

    def save(self):
        import datetime

        data = {
            "mutations_applied": self.mutations_applied,
            "failures": self.failures,
            "partial_applications": self.partial_applications,
            "timestamp": datetime.datetime.now().isoformat(),
        }

        try:
            with open(self.metadata_path, "w") as f:
                json.dump(data, f, indent=2)
            logger.info(f"Saved mutation metadata to {self.metadata_path}")
        except Exception as e:
            logger.error(f"Failed to save metadata: {e}")
