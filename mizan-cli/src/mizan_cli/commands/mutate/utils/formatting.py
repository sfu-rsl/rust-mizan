"""Formatting utilities for mutations - handles rustfmt skip annotations."""

import os
import re
from typing import Dict, List, Any, Set
from mizan_cli.utils.logging import get_logger


logger = get_logger()


class RustfmtSkipHandler:
    """Handles adding and removing rustfmt::skip annotations."""

    def __init__(self, base_dir: str):
        self.base_dir = base_dir
        self.files_with_skips: Set[str] = set()

    def add_skips_to_vulnerable_functions(self, data: Dict[str, Any]) -> None:
        """Add rustfmt::skip annotations to all vulnerable functions."""
        for vuln in data["vulnerabilities"]:
            for sample in vuln["code_samples"]:
                if not sample.get("is_vulnerability", True):
                    continue

                path_to_crate = sample["path_to_crate"]

                # Add rustfmt::skip for vulnerable functions
                for file_name, functions in sample.get(
                    "vulnerable_functions", {}
                ).items():
                    file_path = os.path.join(
                        self.base_dir, "samples", path_to_crate, file_name
                    )
                    self._add_rustfmt_skip_to_functions(file_path, functions)

    def _add_rustfmt_skip_to_functions(
        self, file_path: str, functions: List[str]
    ) -> None:
        """Add #[rustfmt::skip] annotations to vulnerable functions."""
        if not os.path.exists(file_path):
            return

        with open(file_path, "r") as f:
            lines = f.readlines()

        # Process from bottom to top to maintain line numbers
        i = len(lines) - 1
        while i >= 0:
            line = lines[i]

            # Check if this line contains any of the vulnerable functions
            for func in functions:
                if func in line:
                    indent = re.match(r"^(\s*)", line).group(1)
                    lines.insert(i, f"{indent}#[rustfmt::skip]\n")
                    self.files_with_skips.add(file_path)
                    break

            i -= 1

        with open(file_path, "w") as f:
            f.writelines(lines)

    def remove_all_skips(self) -> None:
        """Remove all rustfmt::skip annotations from processed files."""
        for file_path in self.files_with_skips:
            self._remove_rustfmt_skips(file_path)
        self.files_with_skips.clear()

    def _remove_rustfmt_skips(self, file_path: str) -> None:
        """Remove #[rustfmt::skip] annotations from a file."""
        if not os.path.exists(file_path):
            return

        with open(file_path, "r") as f:
            lines = f.readlines()

        cleaned_lines = []
        for line in lines:
            if line.strip() != "#[rustfmt::skip]":
                cleaned_lines.append(line)

        with open(file_path, "w") as f:
            f.writelines(cleaned_lines)
