import os
import json
import random
import copy
from pathlib import Path
from typing import Dict, Any, List
from mizan_cli.utils.logging import get_logger
from ..orchestrator import BaseMutation
from ..utils.markers import MarkerHandler


logger = get_logger()


class InsertionMutation(BaseMutation):
    """Insert comments or code blocks around vulnerable lines."""

    def __init__(self, mutation_type: str):
        valid_types = [
            "benign-comments",
            "benign-blocks",
            "malignant-comments",
            "malignant-blocks",
        ]
        if mutation_type not in valid_types:
            raise ValueError(f"Invalid mutation type. Must be one of: {valid_types}")

        super().__init__(mutation_type)
        self.mutation_type = mutation_type

        # Load the appropriate asset file
        asset_name = mutation_type.replace("-", "_") + ".json"
        asset_path = (
            Path(__file__).parent.parent.parent.parent
            / "assets"
            / "insertions"
            / asset_name
        )

        with open(asset_path, "r") as f:
            asset_data = json.load(f)

        # Get the content key based on type
        content_key = "comments" if "comments" in mutation_type else "blocks"
        self.content_pool = asset_data[content_key]

    def apply(self, base_dir: str) -> bool:
        """Apply the insertion mutation to vulnerable code samples."""
        try:
            # Load ground truth
            mizan_path = os.path.join(base_dir, "mizan.json")
            with open(mizan_path, "r") as f:
                data = json.load(f)

            # Create marker handler
            marker_handler = MarkerHandler(base_dir)

            # Add markers to track vulnerable lines
            marker_handler.add_markers_to_vulnerable_lines(data)

            # Apply insertions
            self._apply_insertions(data, base_dir)

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

    def _apply_insertions(self, data: Dict[str, Any], base_dir: str) -> None:
        """Apply insertions to vulnerable lines."""
        for vuln in data["vulnerabilities"]:
            for sample in vuln["code_samples"]:
                if not sample.get("is_vulnerability", True):
                    continue

                path_to_crate = sample["path_to_crate"]

                # Process each file with vulnerable lines
                for file_name, line_numbers in sample.get(
                    "vulnerable_lines", {}
                ).items():
                    file_path = os.path.join(
                        base_dir, "samples", path_to_crate, file_name
                    )

                    if not os.path.exists(file_path):
                        logger.warning(f"File not found: {file_path}")
                        continue

                    self._insert_content_in_file(file_path, line_numbers)

    def _insert_content_in_file(
        self, file_path: str, vulnerable_lines: List[int]
    ) -> None:
        """Insert content one line before vulnerable lines in a file."""
        with open(file_path, "r") as f:
            lines = f.readlines()

        # Sort vulnerable lines in reverse order to maintain line numbers
        for line_num in sorted(vulnerable_lines, reverse=True):
            if line_num <= len(lines):
                # Insert one line before the vulnerable line
                insert_position = (
                    line_num - 1
                )  # line_num is 1-based, so subtract 1 to insert before

                # Get random content from pool
                content = random.choice(self.content_pool)

                insertion = content + "\n"

                # Insert the content
                lines.insert(insert_position, insertion)

        # Write back to file
        with open(file_path, "w") as f:
            f.writelines(lines)


class BenignCommentsMutation(InsertionMutation):
    """Insert benign comments around vulnerable lines."""

    def __init__(self):
        super().__init__("benign-comments")


class BenignBlocksMutation(InsertionMutation):
    """Insert benign code blocks around vulnerable lines."""

    def __init__(self):
        super().__init__("benign-blocks")


class MalignantCommentsMutation(InsertionMutation):
    """Insert malignant comments around vulnerable lines."""

    def __init__(self):
        super().__init__("malignant-comments")


class MalignantBlocksMutation(InsertionMutation):
    """Insert malignant code blocks around vulnerable lines."""

    def __init__(self):
        super().__init__("malignant-blocks")
