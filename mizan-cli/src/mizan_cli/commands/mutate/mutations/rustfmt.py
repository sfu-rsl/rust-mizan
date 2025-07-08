import os
import json
import shutil
import subprocess
from typing import Dict, List, Any
from mizan_cli.utils.logging import get_logger
from mizan_cli.assets import get_rustfmt_config
from ..orchestrator import BaseMutation
from ..utils import MarkerHandler, RustfmtSkipHandler


logger = get_logger()


class RustfmtMutation(BaseMutation):
    """Applies rustfmt with line tracking using modular utilities."""

    def __init__(self, name: str, style: str):
        super().__init__(name)
        self.style = style

    def apply(self, base_dir: str) -> bool:
        """Apply rustfmt mutation using modular marker and preprocessing handlers."""
        rustfmt_toml = None
        try:
            logger.info(f"Applying {self.name} mutation...")

            # Load ground truth
            with open(os.path.join(base_dir, "mizan.json"), "r") as f:
                data = json.load(f)

            # Initialize handlers
            marker_handler = MarkerHandler(base_dir)
            skip_handler = RustfmtSkipHandler(base_dir)

            # Step 1: Add markers to track vulnerable lines
            logger.debug("Adding line markers...")
            marker_handler.add_markers_to_vulnerable_lines(data)

            # Step 2: Add rustfmt::skip annotations
            logger.debug("Adding rustfmt::skip annotations...")
            skip_handler.add_skips_to_vulnerable_functions(data)

            # Step 3: Copy rustfmt config and run rustfmt
            rustfmt_toml = os.path.join(base_dir, "rustfmt.toml")
            shutil.copy2(get_rustfmt_config(self.style), rustfmt_toml)

            logger.debug("Running rustfmt...")
            result = subprocess.run(
                ["cargo", "+nightly", "fmt", "--all"],
                cwd=base_dir,
                capture_output=True,
                text=True,
            )

            if result.returncode != 0:
                logger.warning(f"rustfmt had issues: {result.stderr}")
                # Continue anyway, as rustfmt might have partially succeeded

            # Step 4: Remove rustfmt::skip annotations
            logger.debug("Removing rustfmt::skip annotations...")
            skip_handler.remove_all_skips()

            # Step 5: Extract new line numbers and update ground truth
            logger.debug("Extracting new line numbers...")
            updated_data = marker_handler.extract_new_line_numbers(data)

            # Step 6: Save updated ground truth
            logger.debug("Saving updated ground truth...")
            with open(os.path.join(base_dir, "mizan.json"), "w") as f:
                json.dump(updated_data, f, indent=2)

            # Step 7: Clean up markers and rustfmt.toml
            logger.debug("Cleaning up...")
            marker_handler.remove_all_markers()
            os.remove(rustfmt_toml)

            logger.info(f"Successfully applied {self.name}")
            return True

        except Exception as e:
            logger.error(f"Error applying {self.name}: {e}")
            # Try to clean up
            if rustfmt_toml and os.path.exists(rustfmt_toml):
                os.remove(rustfmt_toml)
            return False


class FormatCompactMutation(RustfmtMutation):
    def __init__(self):
        super().__init__("format-compact", "compact")


class FormatExpandedMutation(RustfmtMutation):
    def __init__(self):
        super().__init__("format-expanded", "expanded")
