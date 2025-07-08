import os
import json
import re
from typing import Dict, Any, List
from mizan_cli.utils.logging import get_logger
from ..orchestrator import BaseMutation
from ..utils.markers import MarkerHandler


logger = get_logger()


class CommentRemovalMutation(BaseMutation):
    """Remove Rust comments from code while preserving markers."""

    def __init__(self):
        super().__init__("remove-comments")

    def apply(self, base_dir: str) -> bool:
        """Apply the comment removal mutation to all code samples."""
        try:
            # Load ground truth
            mizan_path = os.path.join(base_dir, "mizan.json")
            with open(mizan_path, "r") as f:
                data = json.load(f)

            # Create marker handler
            marker_handler = MarkerHandler(base_dir)

            # Add markers to track vulnerable lines
            marker_handler.add_markers_to_vulnerable_lines(data)

            # Apply comment removal
            self._remove_comments_from_samples(data, base_dir)

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

    def _remove_comments_from_samples(
        self, data: Dict[str, Any], base_dir: str
    ) -> None:
        """Remove comments from all Rust files in samples."""
        for vuln in data["vulnerabilities"]:
            for sample in vuln["code_samples"]:
                path_to_crate = sample["path_to_crate"]
                sample_dir = os.path.join(base_dir, "samples", path_to_crate)

                # Process all Rust files in the sample
                for root, dirs, files in os.walk(sample_dir):
                    for file in files:
                        if file.endswith(".rs"):
                            file_path = os.path.join(root, file)
                            self._remove_comments_from_file(file_path)

    def _remove_comments_from_file(self, file_path: str) -> None:
        """Remove comments from a single Rust file."""
        try:
            with open(file_path, "r") as f:
                content = f.read()
            cleaned_content = self._remove_rust_comments(content)
            with open(file_path, "w") as f:
                f.write(cleaned_content)
        except Exception as e:
            logger.warning(f"Failed to process file {file_path}: {e}")

    def _remove_rust_comments(self, content: str) -> str:
        """Remove all Rust comments with proper handling of nested block comments."""
        lines = content.split("\n")
        cleaned_lines = []
        block_comment_depth = 0
        in_attribute = False
        paren_depth = 0

        i = 0
        while i < len(lines):
            line = lines[i]

            # Check if we're inside a multi-line attribute
            if in_attribute:
                # Count parentheses to track when attribute ends
                for char in line:
                    if char == "(":
                        paren_depth += 1
                    elif char == ")":
                        paren_depth -= 1

                # If we've closed all parentheses, attribute is done
                if paren_depth <= 0:
                    in_attribute = False
                    paren_depth = 0

                # Skip this line (it's part of the attribute)
                cleaned_lines.append("")
                i += 1
                continue

            # Check for attribute comments like #![...] or #[...]
            stripped_line = line.lstrip()
            if block_comment_depth == 0 and (
                stripped_line.startswith("#![") or stripped_line.startswith("#[")
            ):
                # Check if this is a doc-related deny/warn/forbid attribute we should remove
                doc_related_attrs = ["missing_docs", "missing_doc"]
                has_doc_attr = any(attr in stripped_line for attr in doc_related_attrs)
                has_lint_level = any(
                    level in stripped_line for level in ["deny", "warn", "forbid"]
                )

                if has_doc_attr and has_lint_level:
                    # Check if attribute completes on this line
                    paren_depth = 0
                    for char in stripped_line:
                        if char == "(":
                            paren_depth += 1
                        elif char == ")":
                            paren_depth -= 1

                    if paren_depth > 0:
                        # Multi-line attribute
                        in_attribute = True

                    # Remove this line
                    cleaned_lines.append("")
                    i += 1
                    continue
                else:
                    # Keep other attributes
                    cleaned_lines.append(line.rstrip())
                    i += 1
                    continue

            # Process the line for regular comments
            cleaned_line = ""
            j = 0
            in_string = False
            string_char = None

            while j < len(line):
                # Handle string literals to avoid removing comment-like content inside strings
                if (
                    not in_string
                    and block_comment_depth == 0
                    and (line[j] == '"' or line[j] == "'")
                ):
                    # Check if it's escaped
                    if j == 0 or line[j - 1] != "\\":
                        in_string = True
                        string_char = line[j]
                    cleaned_line += line[j]
                    j += 1
                elif in_string and line[j] == string_char:
                    # Check if it's escaped
                    if j == 0 or line[j - 1] != "\\":
                        in_string = False
                        string_char = None
                    cleaned_line += line[j]
                    j += 1
                elif in_string:
                    # Inside string, keep everything
                    cleaned_line += line[j]
                    j += 1
                # Check for start of block comment
                elif j < len(line) - 1 and line[j : j + 2] == "/*":
                    block_comment_depth += 1
                    j += 2
                # Check for end of block comment
                elif (
                    j < len(line) - 1
                    and line[j : j + 2] == "*/"
                    and block_comment_depth > 0
                ):
                    block_comment_depth -= 1
                    j += 2
                # Skip content inside block comments
                elif block_comment_depth > 0:
                    j += 1
                # Check for line comment
                elif j < len(line) - 1 and line[j : j + 2] == "//":
                    # Check if this is a MIZAN marker
                    if "MIZAN_MARKER_" in line[j:]:
                        # Keep the marker comment
                        cleaned_line += line[j:]
                        break
                    else:
                        # Remove everything from here to end of line
                        break
                else:
                    cleaned_line += line[j]
                    j += 1

            cleaned_lines.append(cleaned_line.rstrip())
            i += 1

        return "\n".join(cleaned_lines)
