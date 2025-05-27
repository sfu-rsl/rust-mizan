#!/usr/bin/env python3

import argparse
import json
import os
import re
import subprocess
import copy
from typing import Dict, List, Any


class GroundTruthUpdater:
    def __init__(self, base_dir: str):
        self.base_dir = base_dir

    def add_line_markers(
        self, vuln_id: str, sample_idx: int, file_path: str, line_numbers: List[int]
    ):
        full_path = os.path.join(self.base_dir, file_path)

        if not os.path.exists(full_path):
            print(f"ERROR: File not found: {full_path}")
            return

        with open(full_path, "r") as f:
            lines = f.readlines()

        for line_num in line_numbers:
            if line_num <= len(lines):
                line_idx = line_num - 1
                marker = (
                    f" // MIZAN_MARKER_{vuln_id}_SAMPLE{sample_idx}_LINE{line_num}\n"
                )
                lines[line_idx] = lines[line_idx].rstrip() + marker

        with open(full_path, "w") as f:
            f.writelines(lines)

    def add_rustfmt_skip_to_functions(self, file_path: str, functions: List[str]):
        full_path = os.path.join(self.base_dir, file_path)

        if not os.path.exists(full_path):
            print(f"ERROR: File not found: {full_path}")
            return

        with open(full_path, "r") as f:
            lines = f.readlines()

        # Process lines from bottom to top to maintain line numbers
        i = len(lines) - 1
        while i >= 0:
            line = lines[i]

            for func in functions:
                if func in line:
                    indent = re.match(r"^(\s*)", line).group(1)
                    lines.insert(i, f"{indent}#[rustfmt::skip]\n")
                    break

            i -= 1

        with open(full_path, "w") as f:
            f.writelines(lines)

    def run_rustfmt_workspace(self):
        try:
            result = subprocess.run(
                ["cargo", "+nightly", "fmt", "--all"],
                cwd=self.base_dir,
                capture_output=True,
                text=True,
            )

            if result.returncode != 0:
                print(f"Warning: rustfmt failed: {result.stderr}")
        except Exception as e:
            print(f"Error running rustfmt: {e}")

    def remove_rustfmt_skips(self, file_path: str):
        full_path = os.path.join(self.base_dir, file_path)

        if not os.path.exists(full_path):
            return

        with open(full_path, "r") as f:
            lines = f.readlines()

        cleaned_lines = []
        for line in lines:
            if line.strip() != "#[rustfmt::skip]":
                cleaned_lines.append(line)

        with open(full_path, "w") as f:
            f.writelines(cleaned_lines)

    def extract_new_line_numbers(
        self, file_path: str, vuln_id: str, sample_idx: int
    ) -> List[int]:
        full_path = os.path.join(self.base_dir, file_path)

        if not os.path.exists(full_path):
            return []

        with open(full_path, "r") as f:
            lines = f.readlines()

        new_line_numbers = []
        marker_pattern = f"MIZAN_MARKER_{vuln_id}_SAMPLE{sample_idx}_LINE(\\d+)"

        for i, line in enumerate(lines):
            match = re.search(marker_pattern, line)
            if match:
                new_line_numbers.append(i + 1)

        return new_line_numbers

    def remove_markers(self, file_path: str):
        full_path = os.path.join(self.base_dir, file_path)

        if not os.path.exists(full_path):
            return

        with open(full_path, "r") as f:
            lines = f.readlines()

        cleaned_lines = []
        for line in lines:
            cleaned_line = re.sub(r"\s*// MIZAN_MARKER_[^\n]+", "", line)
            cleaned_lines.append(cleaned_line)

        with open(full_path, "w") as f:
            f.writelines(cleaned_lines)

    def process(self):
        print("Loading mizan.json...")
        with open(os.path.join(self.base_dir, "mizan.json"), "r") as f:
            data = json.load(f)
        updated_data = copy.deepcopy(data)

        processed_crates = set()
        files_with_markers = set()
        files_with_skips = set()

        print("\nStep 1: Adding markers and rustfmt::skip annotations...")
        for vuln in data["vulnerabilities"]:
            vuln_id = vuln["id"]
            print(f"\nProcessing {vuln_id}...")

            for sample_idx, sample in enumerate(vuln["code_samples"]):
                if not sample["is_vulnerability"]:
                    continue

                path_to_crate = sample["path_to_crate"]
                processed_crates.add(path_to_crate)

                for file_name, line_numbers in sample["vulnerable_lines"].items():
                    file_path = os.path.join(path_to_crate, file_name)
                    print(f"  Adding markers to {file_path}")
                    self.add_line_markers(vuln_id, sample_idx, file_path, line_numbers)
                    files_with_markers.add(file_path)

                for file_name, functions in sample["vulnerable_functions"].items():
                    file_path = os.path.join(path_to_crate, file_name)
                    print(f"  Adding rustfmt::skip to functions in {file_path}")
                    self.add_rustfmt_skip_to_functions(file_path, functions)
                    files_with_skips.add(file_path)

        print("\nStep 2: Running rustfmt on the workspace...")
        self.run_rustfmt_workspace()

        print("\nStep 3: Removing rustfmt::skip annotations...")
        for file_path in files_with_skips:
            print(f"  Removing skips from {file_path}")
            self.remove_rustfmt_skips(file_path)

        print("\nStep 4: Extracting new line numbers...")
        for vuln_idx, vuln in enumerate(updated_data["vulnerabilities"]):
            vuln_id = vuln["id"]

            for sample_idx, sample in enumerate(vuln["code_samples"]):
                if not sample["is_vulnerability"]:
                    continue

                path_to_crate = sample["path_to_crate"]
                new_vulnerable_lines = {}

                for file_name in sample["vulnerable_lines"].keys():
                    file_path = os.path.join(path_to_crate, file_name)
                    new_lines = self.extract_new_line_numbers(
                        file_path, vuln_id, sample_idx
                    )
                    if new_lines:
                        new_vulnerable_lines[file_name] = new_lines

                updated_data["vulnerabilities"][vuln_idx]["code_samples"][sample_idx][
                    "vulnerable_lines"
                ] = new_vulnerable_lines

        print("\nStep 5: Saving updated mizan.json...")
        with open(os.path.join(self.base_dir, "mizan.json"), "w") as f:
            json.dump(updated_data, f, indent=2)

        print("\nStep 6: Cleaning up markers...")
        for file_path in files_with_markers:
            print(f"  Removing markers from {file_path}")
            self.remove_markers(file_path)

        print("\nDone! Ground truth has been updated.")


def main():
    parser = argparse.ArgumentParser(
        description="Update ground truth line numbers after rustfmt formatting"
    )
    parser.add_argument(
        "base_dir",
        nargs="?",
        default=".",
        help="Base directory containing Cargo.toml (default: current directory)",
    )
    
    args = parser.parse_args()
    
    base_dir = os.path.abspath(args.base_dir)
    
    if not os.path.exists(os.path.join(base_dir, "Cargo.toml")):
        print(f"ERROR: No Cargo.toml found in {base_dir}")
        return 1
    
    print(f"Using base directory: {base_dir}")
    updater = GroundTruthUpdater(base_dir)
    updater.process()
    return 0


if __name__ == "__main__":
    exit(main())