import os
import json
import shutil
import subprocess
import copy
from abc import ABC, abstractmethod
from typing import Dict, Any, Optional, List
from mizan_cli.utils.logging import get_logger
from .ground_truth import MutationMetadata


logger = get_logger()


class MutationOrchestrator:
    """Simplified orchestrator for applying mutations to code samples."""

    def __init__(self, base_dir: str):
        self.base_dir = base_dir
        self.backup_dir = os.path.join(base_dir, ".mizan_backup")
        self.metadata_manager = MutationMetadata(base_dir)

    @staticmethod
    def normalize_whitespace(text: str) -> str:
        """Normalize whitespace in a string by removing all whitespace characters."""
        return "".join(text.split())

    def create_backup(self) -> bool:
        if os.path.exists(self.backup_dir):
            shutil.rmtree(self.backup_dir)

        try:
            shutil.copytree(
                os.path.join(self.base_dir, "samples"),
                os.path.join(self.backup_dir, "samples"),
            )
            shutil.copy2(
                os.path.join(self.base_dir, "mizan.json"),
                os.path.join(self.backup_dir, "mizan.json"),
            )
            logger.debug("Created backup")
            return True
        except Exception as e:
            logger.error(f"Failed to create backup: {e}")
            return False

    def restore_backup(self) -> bool:
        if not os.path.exists(self.backup_dir):
            logger.error("No backup found")
            return False

        try:
            shutil.rmtree(os.path.join(self.base_dir, "samples"))
            shutil.copytree(
                os.path.join(self.backup_dir, "samples"),
                os.path.join(self.base_dir, "samples"),
            )
            shutil.copy2(
                os.path.join(self.backup_dir, "mizan.json"),
                os.path.join(self.base_dir, "mizan.json"),
            )
            logger.info("Restored from backup")
            return True
        except Exception as e:
            logger.error(f"Failed to restore backup: {e}")
            return False

    def cleanup_backup(self):
        if os.path.exists(self.backup_dir):
            shutil.rmtree(self.backup_dir)

    def validate_sample(
        self,
        sample_path: str,
        vuln: Dict[str, Any],
        sample: Dict[str, Any],
        backup_sample: Optional[Dict[str, Any]] = None,
    ) -> tuple[bool, Optional[str]]:
        """Validate a single code sample after mutation.

        Returns:
            tuple[bool, Optional[str]]: (success, error_message)
        """
        try:
            # Run cargo check on the specific sample
            full_sample_path = os.path.join(self.base_dir, "samples", sample_path)
            result = subprocess.run(
                ["cargo", "check"],
                cwd=full_sample_path,
                capture_output=True,
                text=True,
                timeout=60,
            )
            if result.returncode != 0:
                return False, f"Cargo check failed: {result.stderr}"

            # Validate vulnerable functions still exist
            if sample["is_vulnerability"]:
                for file_name, functions in sample.get(
                    "vulnerable_functions", {}
                ).items():
                    file_path = os.path.join(full_sample_path, file_name)
                    if not os.path.exists(file_path):
                        return False, f"File not found: {file_path}"

                    with open(file_path, "r") as f:
                        content = f.read()

                    for func in functions:
                        if func not in content:
                            return False, f"Function '{func}' not found in {file_name}"

                # Compare vulnerable lines with backup if available
                if backup_sample:
                    current_vuln_lines = sample.get("vulnerable_lines", {})
                    backup_vuln_lines = backup_sample.get("vulnerable_lines", {})

                    for file_name, backup_line_numbers in backup_vuln_lines.items():
                        if file_name not in current_vuln_lines:
                            return (
                                False,
                                f"File '{file_name}' missing from vulnerable_lines after mutation",
                            )

                        # Load the backup file to get original line content
                        backup_file_path = os.path.join(
                            self.backup_dir, "samples", sample_path, file_name
                        )
                        current_file_path = os.path.join(full_sample_path, file_name)

                        if os.path.exists(backup_file_path) and os.path.exists(
                            current_file_path
                        ):
                            with open(backup_file_path, "r") as f:
                                backup_lines = f.readlines()
                            with open(current_file_path, "r") as f:
                                current_lines = f.readlines()

                            current_line_numbers = current_vuln_lines[file_name]

                            # Compare each vulnerable line content
                            for backup_line_num in backup_line_numbers:
                                if backup_line_num <= len(backup_lines):
                                    backup_line_content = backup_lines[
                                        backup_line_num - 1
                                    ].strip()

                                    # Find if this line content exists in the current file
                                    found_match = False
                                    backup_normalized = self.normalize_whitespace(
                                        backup_line_content
                                    )
                                    for current_line_num in current_line_numbers:
                                        if current_line_num <= len(current_lines):
                                            current_line_content = current_lines[
                                                current_line_num - 1
                                            ].strip()
                                            current_normalized = (
                                                self.normalize_whitespace(
                                                    current_line_content
                                                )
                                            )
                                            if backup_normalized == current_normalized:
                                                found_match = True
                                                break

                                    if not found_match:
                                        return (
                                            False,
                                            f"Vulnerable line content from backup line {backup_line_num} not found: '{backup_line_content}'",
                                        )
                        else:
                            logger.warning(
                                f"Could not compare line content for {file_name} - backup or current file missing"
                            )

            return True, None
        except Exception as e:
            return False, f"Error during validation: {e}"

    def apply_mutation_sequential(self, mutation: "BaseMutation") -> bool:
        logger.info(f"Applying mutation: {mutation.name}")

        if not self.create_backup():
            return False

        try:
            if not mutation.apply(self.base_dir):
                raise Exception(f"Failed to apply {mutation.name}")

            # Get partial samples from the mutation if any
            partial_samples = mutation.get_partial_samples()
            if partial_samples:
                self.metadata_manager.add_partial_applications(
                    mutation.name, partial_samples
                )

            # Load both current and backup data
            with open(os.path.join(self.base_dir, "mizan.json"), "r") as f:
                updated_data = json.load(f)

            backup_mizan_path = os.path.join(self.backup_dir, "mizan.json")
            with open(backup_mizan_path, "r") as f:
                backup_data = json.load(f)

            # Validate and handle failures per sample
            failed_samples = []
            total_samples = sum(
                len(vuln["code_samples"]) for vuln in updated_data["vulnerabilities"]
            )
            validated_count = 0

            logger.info(f"Validating {total_samples} code samples...")

            for vuln_idx, vuln in enumerate(updated_data["vulnerabilities"]):
                for sample_idx, sample in enumerate(vuln["code_samples"]):
                    sample_path = sample["path_to_crate"]
                    backup_sample = backup_data["vulnerabilities"][vuln_idx][
                        "code_samples"
                    ][sample_idx]

                    validated_count += 1
                    logger.info(
                        f"Validating sample {validated_count}/{total_samples}: {sample_path}"
                    )

                    success, error_msg = self.validate_sample(
                        sample_path, vuln, sample, backup_sample
                    )

                    if not success:
                        failed_samples.append(
                            {
                                "vuln_id": vuln["id"],
                                "sample_path": sample_path,
                                "error": error_msg,
                            }
                        )

                        # Restore this specific sample
                        logger.warning(
                            f"Mutation {mutation.name} failed for {sample_path}: {error_msg}"
                        )
                        logger.info(f"Rolling back sample: {sample_path}")

                        # Restore the sample directory
                        sample_backup_path = os.path.join(
                            self.backup_dir, "samples", sample_path
                        )
                        sample_current_path = os.path.join(
                            self.base_dir, "samples", sample_path
                        )

                        if os.path.exists(sample_backup_path):
                            shutil.rmtree(sample_current_path)
                            shutil.copytree(sample_backup_path, sample_current_path)

                            # Restore the ground truth for this sample
                            updated_data["vulnerabilities"][vuln_idx]["code_samples"][
                                sample_idx
                            ] = copy.deepcopy(backup_sample)

            # Save the updated mizan.json with reverted samples
            with open(os.path.join(self.base_dir, "mizan.json"), "w") as f:
                json.dump(updated_data, f, indent=2)

            # Record the mutation outcome
            if failed_samples:
                self.metadata_manager.add_partial_mutation(
                    mutation.name, failed_samples
                )
                logger.warning(
                    f"Mutation {mutation.name} applied with {len(failed_samples)} failures"
                )
                logger.info(
                    f"Successfully validated {total_samples - len(failed_samples)}/{total_samples} samples"
                )
            else:
                self.metadata_manager.add_successful_mutation(mutation.name)
                logger.info(f"Successfully applied mutation: {mutation.name}")
                logger.info(f"All {total_samples} samples validated successfully")

            self.cleanup_backup()
            return len(failed_samples) == 0

        except Exception as e:
            logger.error(f"Error applying mutation {mutation.name}: {e}")
            self.restore_backup()
            self.cleanup_backup()
            self.metadata_manager.add_failed_mutation(mutation.name, str(e))
            return False

    def finalize(self):
        self.metadata_manager.save()
        self.cleanup_backup()


class BaseMutation(ABC):
    def __init__(self, name: str):
        self.name = name
        self.partial_samples: List[str] = []

    @abstractmethod
    def apply(self, base_dir: str) -> bool:
        pass

    def get_partial_samples(self) -> List[str]:
        """Return list of samples that were partially mutated."""
        return self.partial_samples
