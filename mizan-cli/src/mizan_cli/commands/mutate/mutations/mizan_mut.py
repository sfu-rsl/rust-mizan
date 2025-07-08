import os
import json
import subprocess
import copy
import shutil
from pathlib import Path
from typing import Dict, Any, List, Optional
from mizan_cli.utils.logging import get_logger
from ..orchestrator import BaseMutation


logger = get_logger()


class MizanMutMutation(BaseMutation):
    """Apply mutations using mizan-mut tool"""

    SUPPORTED_MUTATIONS = {
        "for-to-while": "Converts for loops to while loops",
        "while-to-loop": "Converts while loops to loop blocks with breaks",
        "if-else-reorder": "Reorders if-else branches by negating conditions",
        "derive-reorder": "Randomly reorders traits in derive attributes",
        "trait-bound-reorder": "Randomly reorders trait bounds in where clauses",
        "use-reorder": "Randomly reorders items in use statements",
        "arithmetic-identity": "Adds arithmetic identity operations (x + N - N)",
    }

    def __init__(self, mutation_name: str):
        if mutation_name not in self.SUPPORTED_MUTATIONS:
            raise ValueError(
                f"Invalid mutation: {mutation_name}. "
                f"Must be one of: {list(self.SUPPORTED_MUTATIONS.keys())}"
            )

        super().__init__(f"mizan-mut-{mutation_name}")
        self.mutation_name = mutation_name

    def apply(self, base_dir: str) -> bool:
        """Apply the mizan-mut mutation to all code samples."""
        try:
            logger.info(f"Applying {self.name} mutation...")

            # Track samples that were partially mutated
            self.partial_samples = []

            # Load ground truth
            mizan_path = Path(base_dir) / "mizan.json"
            data = json.loads(mizan_path.read_text())

            # Store the original vulnerable line content before mutation
            original_line_content = self._capture_vulnerable_lines(data, base_dir)

            # Apply mutation to all samples initially
            samples_dir = Path(base_dir) / "samples"
            for vuln in data["vulnerabilities"]:
                for sample in vuln["code_samples"]:
                    sample_path = samples_dir / sample["path_to_crate"]
                    self._apply_mutation_to_sample(str(sample_path))

            # Update ground truth
            updated_data = self._update_ground_truth(
                data, original_line_content, base_dir
            )

            # Save updated ground truth
            mizan_path.write_text(json.dumps(updated_data, indent=2))

            # Log if there were partial applications
            if self.partial_samples:
                logger.warning(
                    f"{self.name}: {len(self.partial_samples)} samples were partially mutated"
                )

            logger.info(f"Successfully applied {self.name} mutation")
            return True

        except Exception as e:
            logger.error(f"Failed to apply {self.name} mutation: {e}")
            return False

    def _capture_vulnerable_lines(
        self, data: Dict[str, Any], base_dir: str
    ) -> Dict[str, str]:
        """Capture the content of vulnerable lines before mutation."""
        line_content = {}
        samples_dir = Path(base_dir) / "samples"

        for vuln in data["vulnerabilities"]:
            for sample_idx, sample in enumerate(vuln["code_samples"]):
                if not sample.get("is_vulnerability", True):
                    continue

                sample_key = f"{vuln['id']}_{sample_idx}"
                path_to_crate = sample["path_to_crate"]

                for file_name, line_numbers in sample.get(
                    "vulnerable_lines", {}
                ).items():
                    file_path = samples_dir / path_to_crate / file_name

                    if file_path.exists():
                        lines = file_path.read_text().splitlines(keepends=True)

                        for line_num in line_numbers:
                            if 0 < line_num <= len(lines):
                                content = lines[line_num - 1].strip()
                                if content:  # Only store non-empty lines
                                    key = f"{sample_key}:{file_name}:{line_num}"
                                    line_content[key] = content

        return line_content

    def _apply_mutation_to_sample(
        self, sample_path: str, ignore_files: Optional[List[str]] = None
    ) -> bool:
        """Apply mizan-mut to a single sample directory."""
        try:
            cmd = ["mizan-mut", "mutate", "-r", sample_path, "-m", self.mutation_name]

            if ignore_files:
                for ignore_file in ignore_files:
                    cmd.extend(["-i", ignore_file])

            result = subprocess.run(cmd, capture_output=True, text=True)
            return result.returncode == 0

        except Exception as e:
            logger.error(f"Error applying mutation to {sample_path}: {e}")
            return False

    def _update_ground_truth(
        self, data: Dict[str, Any], original_line_content: Dict[str, str], base_dir: str
    ) -> Dict[str, Any]:
        """Update ground truth by finding where vulnerable lines moved to."""
        updated_data = copy.deepcopy(data)
        backup_dir = Path(base_dir) / ".mizan_backup"

        for vuln_idx, vuln in enumerate(updated_data["vulnerabilities"]):
            for sample_idx, sample in enumerate(vuln["code_samples"]):
                if not sample.get("is_vulnerability", True):
                    continue

                sample_key = f"{vuln['id']}_{sample_idx}"
                path_to_crate = sample["path_to_crate"]
                new_vulnerable_lines = {}
                files_to_exclude = []

                # Try to find each vulnerable line in the mutated code
                for file_name, original_line_numbers in sample.get(
                    "vulnerable_lines", {}
                ).items():
                    file_path = Path(base_dir) / "samples" / path_to_crate / file_name

                    if not file_path.exists():
                        continue

                    current_lines = file_path.read_text().splitlines()
                    found_lines = []

                    for original_line_num in original_line_numbers:
                        key = f"{sample_key}:{file_name}:{original_line_num}"
                        original_content = original_line_content.get(key, "")

                        if not original_content:
                            continue

                        # Find all occurrences of this line
                        matches = [
                            idx + 1
                            for idx, line in enumerate(current_lines)
                            if line.strip() == original_content
                        ]

                        if len(matches) == 1:
                            # Unique match - we can track it
                            found_lines.append(matches[0])
                        elif len(matches) > 1:
                            # Multiple matches - can't track reliably
                            logger.warning(
                                f"Line '{original_content[:50]}...' appears {len(matches)} times "
                                f"in {file_name}. Cannot track reliably."
                            )
                            files_to_exclude.append(file_name)
                            break
                        else:
                            # No match - line was changed by mutation
                            logger.warning(
                                f"Line '{original_content[:50]}...' not found after mutation "
                                f"in {file_name}."
                            )
                            files_to_exclude.append(file_name)
                            break

                    if file_name not in files_to_exclude and found_lines:
                        new_vulnerable_lines[file_name] = sorted(found_lines)

                # If we couldn't track lines in some files, re-apply mutation excluding those files
                if files_to_exclude:
                    self.partial_samples.append(path_to_crate)

                    sample_path = Path(base_dir) / "samples" / path_to_crate
                    backup_sample_path = backup_dir / "samples" / path_to_crate

                    if backup_sample_path.exists():
                        logger.info(
                            f"Re-applying mutation to {path_to_crate} "
                            f"excluding files: {files_to_exclude}"
                        )

                        # Restore from backup
                        shutil.rmtree(sample_path)
                        shutil.copytree(backup_sample_path, sample_path)

                        # Re-apply mutation excluding problematic files
                        if self._apply_mutation_to_sample(
                            str(sample_path), files_to_exclude
                        ):
                            logger.info(
                                f"Successfully re-applied mutation to {path_to_crate}"
                            )
                            # Keep original vulnerable_lines for excluded files
                            for file_name in files_to_exclude:
                                if file_name in sample["vulnerable_lines"]:
                                    new_vulnerable_lines[file_name] = sample[
                                        "vulnerable_lines"
                                    ][file_name]
                        else:
                            logger.error(
                                f"Failed to re-apply mutation to {path_to_crate}"
                            )
                            # Restore from backup without mutation
                            shutil.rmtree(sample_path)
                            shutil.copytree(backup_sample_path, sample_path)
                            # Keep all original vulnerable_lines
                            new_vulnerable_lines = sample["vulnerable_lines"]

                # Update vulnerable_lines in ground truth
                updated_data["vulnerabilities"][vuln_idx]["code_samples"][sample_idx][
                    "vulnerable_lines"
                ] = new_vulnerable_lines

        return updated_data


# Create convenience classes for each mutation type
class ForToWhileMutation(MizanMutMutation):
    def __init__(self):
        super().__init__("for-to-while")


class WhileToLoopMutation(MizanMutMutation):
    def __init__(self):
        super().__init__("while-to-loop")


class IfElseReorderMutation(MizanMutMutation):
    def __init__(self):
        super().__init__("if-else-reorder")


class DeriveReorderMutation(MizanMutMutation):
    def __init__(self):
        super().__init__("derive-reorder")


class TraitBoundReorderMutation(MizanMutMutation):
    def __init__(self):
        super().__init__("trait-bound-reorder")


class UseReorderMutation(MizanMutMutation):
    def __init__(self):
        super().__init__("use-reorder")


class ArithmeticIdentityMutation(MizanMutMutation):
    def __init__(self):
        super().__init__("arithmetic-identity")


class MizanMutAllMutation(MizanMutMutation):
    """Apply all mizan-mut mutations at once"""

    def __init__(self):
        # Call BaseMutation.__init__ directly to avoid the validation
        BaseMutation.__init__(self, "mizan-mut-all")
        self.mutation_name = "all"  # Use "all" for the mizan-mut command
