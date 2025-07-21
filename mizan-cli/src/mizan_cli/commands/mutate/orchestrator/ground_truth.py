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
        self.skipped: Dict[str, List[str]] = {}
        self.partial_mutations: Dict[str, List[str]] = {}
        self._load_existing()

    def _load_existing(self):
        if os.path.exists(self.metadata_path):
            try:
                with open(self.metadata_path, "r") as f:
                    data = json.load(f)
                self.mutations_applied = data.get("mutations_applied", [])
                self.skipped = data.get("skipped", data.get("failures", {}))
                self.partial_mutations = data.get("partial_mutations", data.get("partial_applications", {}))
            except Exception as e:
                logger.warning(f"Could not load existing metadata: {e}")

    def add_successful_mutation(self, mutation_name: str):
        if mutation_name not in self.mutations_applied:
            self.mutations_applied.append(mutation_name)

    def add_skipped_mutation(self, mutation_name: str, error: str):
        # For mutations that were skipped (couldn't be applied at all)
        if mutation_name not in self.skipped:
            self.skipped[mutation_name] = []
        self.skipped[mutation_name].append(f"Skipped: {error}")

    def add_partial_mutation(
        self, mutation_name: str, skipped_samples: List[Dict[str, str]]
    ):
        """Record a mutation that was partially successful (some samples were skipped)."""
        if mutation_name not in self.skipped:
            self.skipped[mutation_name] = []

        # Extract just the sample paths from the skipped samples
        sample_paths = [sample["sample_path"] for sample in skipped_samples]
        self.skipped[mutation_name].extend(sample_paths)

        # Also add to mutations_applied since it was partially successful
        if mutation_name not in self.mutations_applied:
            self.mutations_applied.append(mutation_name)

    def add_partial_mutations(self, mutation_name: str, partial_samples: List[str]):
        """Add samples where the mutation was applied to other files except the one with vulnerable lines."""
        if partial_samples:
            self.partial_mutations[mutation_name] = partial_samples

    def save(self):
        import datetime

        data = {
            "mutations_applied": self.mutations_applied,
            "skipped": self.skipped,
            "partial_mutations": self.partial_mutations,
            "timestamp": datetime.datetime.now().isoformat(),
        }

        try:
            with open(self.metadata_path, "w") as f:
                json.dump(data, f, indent=2)
            logger.info(f"Saved mutation metadata to {self.metadata_path}")
        except Exception as e:
            logger.error(f"Failed to save metadata: {e}")
