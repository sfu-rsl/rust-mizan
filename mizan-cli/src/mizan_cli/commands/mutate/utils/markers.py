"""Marker handling utilities for tracking code locations through mutations."""

import os
import re
import json
import copy
from typing import Dict, List, Any, Set, Tuple
from mizan_cli.utils.logging import get_logger


logger = get_logger()


class MarkerHandler:
    """Handles adding, extracting, and removing line markers for mutations."""

    def __init__(self, base_dir: str):
        self.base_dir = base_dir
        self.processed_files: Set[str] = set()

    def add_markers_to_vulnerable_lines(self, data: Dict[str, Any]) -> None:
        for vuln in data["vulnerabilities"]:
            vuln_id = vuln["id"]

            for sample_idx, sample in enumerate(vuln["code_samples"]):
                if not sample.get("is_vulnerability", True):
                    continue

                path_to_crate = sample["path_to_crate"]

                # Add markers for vulnerable lines
                for file_name, line_numbers in sample.get(
                    "vulnerable_lines", {}
                ).items():
                    file_path = os.path.join(
                        self.base_dir, "samples", path_to_crate, file_name
                    )
                    self._add_line_markers(file_path, vuln_id, sample_idx, line_numbers)
                    self.processed_files.add(file_path)

    def _add_line_markers(
        self, file_path: str, vuln_id: str, sample_idx: int, line_numbers: List[int]
    ) -> None:
        if not os.path.exists(file_path):
            logger.warning(f"File not found: {file_path}")
            return

        with open(file_path, "r") as f:
            lines = f.readlines()

        for line_num in line_numbers:
            if line_num <= len(lines):
                line_idx = line_num - 1
                marker = (
                    f" // MIZAN_MARKER_{vuln_id}_SAMPLE{sample_idx}_LINE{line_num}\n"
                )
                lines[line_idx] = lines[line_idx].rstrip() + marker

        with open(file_path, "w") as f:
            f.writelines(lines)

    def extract_new_line_numbers(self, data: Dict[str, Any]) -> Dict[str, Any]:
        updated_data = copy.deepcopy(data)

        for vuln_idx, vuln in enumerate(updated_data["vulnerabilities"]):
            vuln_id = vuln["id"]

            for sample_idx, sample in enumerate(vuln["code_samples"]):
                if not sample.get("is_vulnerability", True):
                    continue

                path_to_crate = sample["path_to_crate"]
                new_vulnerable_lines = {}

                for file_name in sample.get("vulnerable_lines", {}).keys():
                    file_path = os.path.join(
                        self.base_dir, "samples", path_to_crate, file_name
                    )
                    new_lines = self._extract_markers_from_file(
                        file_path, vuln_id, sample_idx
                    )
                    if new_lines:
                        new_vulnerable_lines[file_name] = new_lines

                updated_data["vulnerabilities"][vuln_idx]["code_samples"][sample_idx][
                    "vulnerable_lines"
                ] = new_vulnerable_lines

        return updated_data

    def _extract_markers_from_file(
        self, file_path: str, vuln_id: str, sample_idx: int
    ) -> List[int]:
        if not os.path.exists(file_path):
            return []

        with open(file_path, "r") as f:
            lines = f.readlines()

        new_line_numbers = []
        marker_pattern = f"MIZAN_MARKER_{vuln_id}_SAMPLE{sample_idx}_LINE(\\d+)"

        for i, line in enumerate(lines):
            if re.search(marker_pattern, line):
                new_line_numbers.append(i + 1)

        return new_line_numbers

    def remove_all_markers(self) -> None:
        for file_path in self.processed_files:
            self._remove_markers_from_file(file_path)
        self.processed_files.clear()

    def _remove_markers_from_file(self, file_path: str) -> None:
        if not os.path.exists(file_path):
            return

        with open(file_path, "r") as f:
            lines = f.readlines()

        cleaned_lines = []
        for line in lines:
            cleaned_line = re.sub(r"\s*// MIZAN_MARKER_[^\n]+", "", line)
            cleaned_lines.append(cleaned_line)

        with open(file_path, "w") as f:
            f.writelines(cleaned_lines)
