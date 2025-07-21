import os
import re
import json
import subprocess
import random
import string
from pathlib import Path
from typing import Dict, Any, List, Tuple, Optional
from mizan_cli.utils.logging import get_logger
from ..orchestrator import BaseMutation


logger = get_logger()


class RenameMutation(BaseMutation):
    """Base class for rename mutations using mizan-mut rename tool."""

    def __init__(self, name: str, rename_type: str, target_type: str, seed: int = 42):
        super().__init__(name, seed)
        self.rename_type = rename_type  # "benign" or "malignant"
        self.target_type = target_type  # "function" or "variable"
        self.rename_counter = 0
        
        # Set random seed for reproducibility
        random.seed(seed)

        # Load malignant names from JSON file
        malignant_names_path = (
            Path(__file__).parent.parent.parent.parent
            / "assets"
            / "mutate"
            / "malignant_names.json"
        )
        with open(malignant_names_path, "r") as f:
            malignant_data = json.load(f)
        self.malignant_names = malignant_data["names"]

    def apply(self, base_dir: str) -> bool:
        """Apply rename mutations to each code sample independently."""
        try:
            logger.info(f"Applying {self.name} mutation...")

            # Load ground truth
            mizan_path = os.path.join(base_dir, "mizan.json")
            with open(mizan_path, "r") as f:
                data = json.load(f)

            # Apply renames to each sample independently
            for vuln in data["vulnerabilities"]:
                for sample in vuln["code_samples"]:
                    if not sample.get("is_vulnerability", True):
                        continue
                    
                    sample_path = sample["path_to_crate"]
                    logger.debug(f"Applying {self.name} to sample: {sample_path}")
                    
                    # Apply renames for this specific sample
                    self._apply_renames_to_sample(sample, base_dir)

            # Save updated ground truth
            with open(mizan_path, "w") as f:
                json.dump(data, f, indent=2)

            logger.debug(f"Successfully applied {self.name} mutation")
            return True

        except Exception as e:
            logger.error(f"Failed to apply {self.name} mutation: {e}")
            return False

    def _apply_renames_to_sample(self, sample: Dict[str, Any], base_dir: str) -> None:
        """Apply renames to a specific code sample and update its ground truth."""
        path_to_crate = sample["path_to_crate"]
        sample_path = os.path.join(base_dir, "samples", path_to_crate)
        
        # Track successful renames for this sample
        successful_renames = {}

        # Process each file with vulnerable lines
        for file_name, line_numbers in sample.get("vulnerable_lines", {}).items():
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
                if old_name in ["self", "main", "new", "default", "_", "pack", "index", "deserialize"]:
                    continue

                new_name = self._generate_unique_name(id_type)

                logger.debug(
                    f"{path_to_crate}: Attempting to rename {id_type} '{old_name}' "
                    f"at offset {byte_offset} in {file_name}"
                )
                
                # Use the sample directory as the crate root for mizan-mut
                cmd = [
                    "mizan-mut",
                    "rename",
                    "-c",
                    sample_path,
                    "-f",
                    file_name,
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
                    # Track successful rename
                    successful_renames[old_name] = new_name
                else:
                    stderr_msg = result.stderr.strip()
                    if stderr_msg and "No references found at position" in stderr_msg:
                        logger.debug(
                            f"{path_to_crate}: Could not rename '{old_name}' at offset {byte_offset} "
                            f"(no references found - might be in a macro or special context)"
                        )
                    else:
                        logger.warning(
                            f"{path_to_crate}: Failed to rename '{old_name}' at offset {byte_offset}: "
                            f"{stderr_msg}"
                        )
        
        # Update this sample's ground truth with the renames
        if successful_renames:
            self._update_sample_ground_truth(sample, successful_renames)

    def _update_sample_ground_truth(self, sample: Dict[str, Any], renames: Dict[str, str]) -> None:
        """Update the ground truth for a specific sample with successful renames."""
        sample_path = sample["path_to_crate"]
        
        # For variable renames, we should NOT update function signatures
        # Function signatures are used for identification and shouldn't change
        # when we rename variables/parameters inside them
        if self.target_type == "variable":
            logger.info(f"Variable renames applied to {sample_path}: {renames}")
            logger.info(f"Function signatures in ground truth remain unchanged for identification purposes")
            return
        
        # For function renames, update the function signatures in ground truth
        if self.target_type == "function":
            vulnerable_functions = sample.get("vulnerable_functions", {})
            for file_name, functions in vulnerable_functions.items():
                updated_functions = []
                for func in functions:
                    updated_func = func
                    for old_name, new_name in renames.items():
                        # Use word boundaries to avoid partial matches
                        pattern = r'\b' + re.escape(old_name) + r'\b'
                        updated_func = re.sub(pattern, new_name, updated_func)
                    
                    updated_functions.append(updated_func)
                    if updated_func != func:
                        logger.info(f"Ground truth for code sample {sample_path} was updated: function '{func}' -> '{updated_func}'")
                
                vulnerable_functions[file_name] = updated_functions
        
        logger.info(f"Ground truth for code sample {sample_path} was updated with renamed identifiers: {renames}")

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
                    "from", "into", "clone", "drop", "deref", "as_ref", "as_mut",
                    "fmt", "hash", "eq", "ne", "cmp", "partial_cmp", "pack", "index", "deserialize"
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

    def __init__(self, seed: int = 42):
        super().__init__("benign-rename-fn", "benign", "function", seed)


class BenignRenameVarMutation(RenameMutation):
    """Apply benign rename mutations to variables with neutral names."""

    def __init__(self, seed: int = 42):
        super().__init__("benign-rename-var", "benign", "variable", seed)


class MalignantRenameFnMutation(RenameMutation):
    """Apply malignant rename mutations to functions with names that suggest safety."""

    def __init__(self, seed: int = 42):
        super().__init__("malignant-rename-fn", "malignant", "function", seed)


class MalignantRenameVarMutation(RenameMutation):
    """Apply malignant rename mutations to variables with names that suggest safety."""

    def __init__(self, seed: int = 42):
        super().__init__("malignant-rename-var", "malignant", "variable", seed)
