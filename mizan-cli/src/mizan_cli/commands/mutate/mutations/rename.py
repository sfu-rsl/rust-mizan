import os
import re
import json
import subprocess
import random
import string
from typing import Dict, Any, List, Tuple
from mizan_cli.utils.logging import get_logger
from ..orchestrator import BaseMutation
from ..utils.markers import MarkerHandler


logger = get_logger()


class RenameMutation(BaseMutation):
    """Base class for rename mutations using mizan-mut rename tool."""

    def __init__(self, name: str, rename_type: str, target_type: str):
        super().__init__(name)
        self.rename_type = rename_type  # "benign" or "malignant"
        self.target_type = target_type  # "function" or "variable"
        self.rename_counter = 0

        self.malignant_names = [
            "safe",
            "secure",
            "valid",
            "checked",
            "verified",
            "correct",
            "sanitized",
        ]

    def apply(self, base_dir: str) -> bool:
        """Apply rename mutations to all code samples."""
        try:
            logger.info(f"Applying {self.name} mutation...")

            # Load ground truth
            mizan_path = os.path.join(base_dir, "mizan.json")
            with open(mizan_path, "r") as f:
                data = json.load(f)

            # Create marker handler
            marker_handler = MarkerHandler(base_dir)

            # Add markers to track vulnerable lines
            marker_handler.add_markers_to_vulnerable_lines(data)

            # Apply renames to all samples
            self._apply_renames(data, base_dir)

            # Extract new line numbers
            updated_data = marker_handler.extract_new_line_numbers(data)

            # Remove markers
            marker_handler.remove_all_markers()

            # Save updated ground truth
            with open(mizan_path, "w") as f:
                json.dump(updated_data, f, indent=2)

            logger.debug(f"Successfully applied {self.name} mutation")
            return True

        except Exception as e:
            logger.error(f"Failed to apply {self.name} mutation: {e}")
            return False

    def _apply_renames(self, data: Dict[str, Any], base_dir: str) -> None:
        """Apply renames to vulnerable samples."""
        for vuln in data["vulnerabilities"]:
            for sample in vuln["code_samples"]:
                if not sample.get("is_vulnerability", True):
                    continue

                path_to_crate = sample["path_to_crate"]
                sample_path = os.path.join(base_dir, "samples", path_to_crate)

                # Process each file with vulnerable lines
                for file_name, line_numbers in sample.get(
                    "vulnerable_lines", {}
                ).items():
                    file_path = os.path.join(sample_path, file_name)

                    if not os.path.exists(file_path):
                        logger.warning(f"File not found: {file_path}")
                        continue

                    # Find and rename identifiers around vulnerable lines
                    identifiers = self._find_identifiers_in_region(
                        file_path, line_numbers, context_lines=10
                    )

                    for old_name, byte_offset, id_type in identifiers:
                        # Skip if this identifier type doesn't match our target
                        if id_type != self.target_type:
                            continue
                            
                        # Skip common names that might cause issues
                        if old_name in ["self", "main", "new", "default", "_"]:
                            continue

                        new_name = self._generate_unique_name(id_type)

                        relative_file_path = os.path.join(
                            "samples", path_to_crate, file_name
                        )
                        logger.debug(
                            f"{path_to_crate}: Attempting to rename {id_type} '{old_name}' "
                            f"at offset {byte_offset} in {file_name}"
                        )
                        cmd = [
                            "mizan-mut",
                            "rename",
                            "-c",
                            base_dir,
                            "-f",
                            relative_file_path,
                            "-o",
                            str(byte_offset),
                            "-n",
                            new_name,
                        ]
                        result = subprocess.run(cmd, capture_output=True, text=True)

                        if result.returncode == 0:
                            logger.info(
                                f"{path_to_crate}: Successfully renamed {id_type} '{old_name}' to '{new_name}' "
                                f"in {file_name} at offset {byte_offset}"
                            )
                        else:
                            stderr_msg = result.stderr.strip()
                            if (
                                stderr_msg
                                and "No references found at position" in stderr_msg
                            ):
                                logger.debug(
                                    f"{path_to_crate}: Could not rename '{old_name}' at offset {byte_offset} "
                                    f"(no references found - might be in a macro or special context)"
                                )
                            else:
                                logger.warning(
                                    f"{path_to_crate}: Failed to rename '{old_name}' at offset {byte_offset}: "
                                    f"{stderr_msg}"
                                )

    def _generate_unique_name(self, id_type: str) -> str:
        """Generate a unique variable/function name based on mutation type."""
        self.rename_counter += 1

        if self.rename_type == "benign":
            prefix = "fn" if id_type == "function" else "var"
            random_suffix = "".join(
                random.choices(string.ascii_lowercase + string.digits, k=6)
            )
            return f"{prefix}_{self.rename_counter}_{random_suffix}"
        else:
            base_name = random.choice(self.malignant_names)
            prefix = "fn" if id_type == "function" else "var"
            return f"{base_name}_{prefix}_{self.rename_counter}"

    def _find_identifiers_in_region(
        self, file_path: str, vulnerable_lines: List[int], context_lines: int = 10
    ) -> List[Tuple[str, int, str]]:
        """Find variables and functions in the region around vulnerable lines.

        Returns:
            List of tuples: (identifier_name, byte_offset, identifier_type)
        """
        identifiers = []

        with open(file_path, "rb") as f:
            content = f.read()

        text = content.decode("utf-8")
        lines = text.splitlines(keepends=True)

        # Determine the region of interest
        min_line = max(1, min(vulnerable_lines) - context_lines)
        max_line = min(len(lines), max(vulnerable_lines) + context_lines)

        # Track byte offset as we go through lines
        byte_offset = 0

        for line_num, line in enumerate(lines, 1):
            if line_num < min_line:
                byte_offset += len(line.encode("utf-8"))
                continue

            if line_num > max_line:
                break

            # Find let bindings (variables)
            let_pattern = r"\blet\s+(?:mut\s+)?([a-zA-Z_][a-zA-Z0-9_]*)\s*(?::|=)"
            for match in re.finditer(let_pattern, line):
                var_name = match.group(1)
                # Calculate byte offset to the start of the variable name
                var_start = match.start(1)
                var_byte_offset = byte_offset + len(line[:var_start].encode("utf-8"))
                identifiers.append((var_name, var_byte_offset, "variable"))

            # Find function declarations (but not methods or trait implementations)
            fn_pattern = r"\bfn\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*(?:<[^>]*>)?\s*\("
            for match in re.finditer(fn_pattern, line):
                func_name = match.group(1)
                # Skip if this is a method (has self parameter)
                if re.search(r"\(\s*(?:&\s*)?(?:mut\s+)?self", line[match.start() :]):
                    continue
                # Skip common trait method names
                if func_name in [
                    "from",
                    "into",
                    "clone",
                    "drop",
                    "deref",
                    "as_ref",
                    "as_mut",
                    "fmt",
                    "hash",
                    "eq",
                    "ne",
                    "cmp",
                    "partial_cmp",
                ]:
                    continue
                # Calculate byte offset to the start of the function name
                func_start = match.start(1)
                func_byte_offset = byte_offset + len(line[:func_start].encode("utf-8"))
                identifiers.append((func_name, func_byte_offset, "function"))

            byte_offset += len(line.encode("utf-8"))

        return identifiers


class BenignRenameFnMutation(RenameMutation):
    """Apply benign rename mutations to functions with neutral names."""

    def __init__(self):
        super().__init__("benign-rename-fn", "benign", "function")


class BenignRenameVarMutation(RenameMutation):
    """Apply benign rename mutations to variables with neutral names."""

    def __init__(self):
        super().__init__("benign-rename-var", "benign", "variable")


class MalignantRenameFnMutation(RenameMutation):
    """Apply malignant rename mutations to functions with names that suggest safety."""

    def __init__(self):
        super().__init__("malignant-rename-fn", "malignant", "function")


class MalignantRenameVarMutation(RenameMutation):
    """Apply malignant rename mutations to variables with names that suggest safety."""

    def __init__(self):
        super().__init__("malignant-rename-var", "malignant", "variable")
