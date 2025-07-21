import os
import json
import random
import copy
import subprocess
import tempfile
from pathlib import Path
from typing import Dict, Any, List, Optional
from mizan_cli.utils.logging import get_logger
from ..orchestrator import BaseMutation
from ..utils.markers import MarkerHandler


logger = get_logger()


class InsertionMutation(BaseMutation):
    """Insert comments or code blocks around vulnerable lines."""

    def __init__(self, mutation_type: str, seed: int = 42):
        valid_types = [
            "benign-comments",
            "benign-blocks",
            "malignant-comments",
            "malignant-blocks",
        ]
        if mutation_type not in valid_types:
            raise ValueError(f"Invalid mutation type. Must be one of: {valid_types}")

        super().__init__(mutation_type, seed)
        self.mutation_type = mutation_type
        
        # Set random seed for reproducibility
        random.seed(seed)

        # Load the appropriate asset file
        asset_name = mutation_type.replace("-", "_") + ".json"
        asset_path = (
            Path(__file__).parent.parent.parent.parent
            / "assets"
            / "mutate"
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

                    self._insert_content_in_file(file_path, line_numbers, os.path.join(base_dir, "samples", path_to_crate))

    def _insert_content_in_file(
        self, file_path: str, vulnerable_lines: List[int], crate_path: str
    ) -> None:
        """Insert content around vulnerable lines with compile validation retry mechanism."""
        with open(file_path, "r") as f:
            original_lines = f.readlines()

        # Sort vulnerable lines in reverse order to maintain line numbers
        for line_num in sorted(vulnerable_lines, reverse=True):
            if line_num <= len(original_lines):
                # Get random content from pool
                content = random.choice(self.content_pool)
                insertion = content + "\n"
                
                # Try different insertion positions with compile validation
                success = self._try_insertion_with_validation(
                    file_path, original_lines, line_num, insertion, crate_path
                )
                
                if success:
                    # Update original_lines with successful insertion for next iteration
                    with open(file_path, "r") as f:
                        original_lines = f.readlines()
                else:
                    logger.warning(f"Failed to insert content at line {line_num} in {file_path} after all retry attempts")

    def _try_insertion_with_validation(
        self, file_path: str, lines: List[str], line_num: int, insertion: str, crate_path: str
    ) -> bool:
        """Try inserting content at different positions with compile validation."""
        position_offsets = [
            -1,   # line_num - 1
            -5,   # 5 lines before
            5,    # 5 lines after  
            -10,  # 10 lines before
            10,   # 10 lines after
        ]
        
        for offset in position_offsets:
            insert_position = line_num + offset
            
            # Ensure insert_position is within valid bounds
            if insert_position < 0 or insert_position >= len(lines):
                continue
                
            # Create a copy of lines for testing
            test_lines = lines.copy()
            test_lines.insert(insert_position, insertion)
            
            # Test compilation
            if self._test_compilation(file_path, test_lines, crate_path):
                # Compilation successful, apply the insertion
                with open(file_path, "w") as f:
                    f.writelines(test_lines)
                logger.debug(f"Successfully inserted content at position {insert_position} (offset {offset}) for line {line_num} in {file_path}")
                return True
            else:
                logger.debug(f"Compilation failed for insertion at position {insert_position} (offset {offset}) for line {line_num} in {file_path}")
        
        return False

    def _test_compilation(self, file_path: str, test_lines: List[str], crate_path: str) -> bool:
        """Test if the modified file compiles successfully."""
        # Only test compilation for block insertions, not comments
        is_block_insertion = "blocks" in self.mutation_type
        if not is_block_insertion:
            return True  # Skip compilation test for comments
            
        # Create a temporary file with the test content
        with tempfile.NamedTemporaryFile(mode='w', suffix='.rs', delete=False) as temp_file:
            temp_file.writelines(test_lines)
            temp_file_path = temp_file.name
        
        try:
            # Copy the original file to backup
            with open(file_path, "r") as f:
                original_content = f.read()
            
            # Replace original file with test content
            with open(file_path, "w") as f:
                f.writelines(test_lines)
            
            # Run cargo check on the crate
            result = subprocess.run(
                ["cargo", "check", "--quiet"],
                cwd=crate_path,
                capture_output=True,
                text=True,
            )
            
            # Restore original file
            with open(file_path, "w") as f:
                f.write(original_content)
            
            return result.returncode == 0
            
        except Exception as e:
            logger.debug(f"Compilation test failed with exception: {e}")
            try:
                with open(file_path, "w") as f:
                    f.write(original_content)
            except:
                pass
            return False
        finally:
            try:
                os.unlink(temp_file_path)
            except:
                pass


class BenignCommentsMutation(InsertionMutation):
    """Insert benign comments around vulnerable lines."""

    def __init__(self, seed: int = 42):
        super().__init__("benign-comments", seed)


class BenignBlocksMutation(InsertionMutation):
    """Insert benign code blocks around vulnerable lines."""

    def __init__(self, seed: int = 42):
        super().__init__("benign-blocks", seed)


class MalignantCommentsMutation(InsertionMutation):
    """Insert malignant comments around vulnerable lines."""

    def __init__(self, seed: int = 42):
        super().__init__("malignant-comments", seed)


class MalignantBlocksMutation(InsertionMutation):
    """Insert malignant code blocks around vulnerable lines."""

    def __init__(self, seed: int = 42):
        super().__init__("malignant-blocks", seed)
