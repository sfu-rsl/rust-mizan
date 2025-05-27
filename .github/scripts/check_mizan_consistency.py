#!/usr/bin/env python3
"""
Script to check consistency of mizan.json files across different dataset variants.
Checks that vulnerable_functions and vulnerable_lines match across all variants.
"""

import json
import sys
from pathlib import Path
from typing import Dict, List, Tuple


def load_mizan_json(directory: str) -> Dict:
    """Load mizan.json from the specified directory."""
    mizan_path = Path(directory) / "mizan.json"
    if not mizan_path.exists():
        raise FileNotFoundError(f"mizan.json not found in {directory}")

    with open(mizan_path, "r") as f:
        return json.load(f)


def extract_vulnerability_data(mizan_data: Dict) -> Dict[str, Dict]:
    """Extract vulnerability data organized by vulnerability ID."""
    vuln_data = {}

    for vuln in mizan_data.get("vulnerabilities", []):
        vuln_id = vuln["id"]
        vuln_data[vuln_id] = {
            "crate_name": vuln.get("crate_name", ""),
            "code_samples": [],
        }

        for sample in vuln.get("code_samples", []):
            if sample.get("is_vulnerability", False):
                vuln_data[vuln_id]["code_samples"].append(
                    {
                        "path_to_crate": sample["path_to_crate"],
                        "vulnerable_functions": sample.get("vulnerable_functions", {}),
                        "vulnerable_lines": sample.get("vulnerable_lines", {}),
                    }
                )

    return vuln_data


def read_file_lines(
    base_dir: str, path_to_crate: str, file_path: str, line_numbers: List[int]
) -> List[Tuple[int, str]]:
    """Read specific lines from a file."""
    full_path = Path(base_dir) / path_to_crate / file_path

    if not full_path.exists():
        return []

    lines = []
    with open(full_path, "r") as f:
        all_lines = f.readlines()
        for line_num in line_numbers:
            if 1 <= line_num <= len(all_lines):
                # Line numbers are 1-indexed
                lines.append((line_num, all_lines[line_num - 1].rstrip()))

    return lines


def check_consistency(directories: List[str]) -> Dict[str, List[str]]:
    """Check consistency across all directories."""
    errors = []

    # Load all mizan.json files
    all_data = {}
    for directory in directories:
        try:
            all_data[directory] = load_mizan_json(directory)
        except FileNotFoundError as e:
            errors.append(f"Error loading {directory}: {e}")
            continue

    if not all_data:
        return {"errors": errors}

    # Extract vulnerability data
    all_vuln_data = {}
    for directory, data in all_data.items():
        all_vuln_data[directory] = extract_vulnerability_data(data)

    # Find common vulnerability IDs across all variants
    all_vuln_ids = set()
    for vuln_data in all_vuln_data.values():
        all_vuln_ids.update(vuln_data.keys())

    # Check each vulnerability
    for vuln_id in sorted(all_vuln_ids):
        # Get directories that have this vulnerability
        dirs_with_vuln = [d for d in directories if vuln_id in all_vuln_data.get(d, {})]

        if len(dirs_with_vuln) < 2:
            # Only one variant has this vulnerability, skip consistency check
            continue

        # Use the first directory as reference
        ref_dir = dirs_with_vuln[0]
        ref_vuln = all_vuln_data[ref_dir][vuln_id]

        # Check consistency with other directories
        for compare_dir in dirs_with_vuln[1:]:
            compare_vuln = all_vuln_data[compare_dir][vuln_id]

            # Check crate name
            if ref_vuln["crate_name"] != compare_vuln["crate_name"]:
                errors.append(
                    f"{vuln_id}: Crate name mismatch between {ref_dir} ({ref_vuln['crate_name']}) and {compare_dir} ({compare_vuln['crate_name']})"
                )

            # Create mapping of path_to_crate to samples for easier comparison
            ref_samples_map = {s["path_to_crate"]: s for s in ref_vuln["code_samples"]}
            compare_samples_map = {
                s["path_to_crate"]: s for s in compare_vuln["code_samples"]
            }

            # Check each code sample
            for path_to_crate, ref_sample in ref_samples_map.items():
                if path_to_crate not in compare_samples_map:
                    errors.append(
                        f"{vuln_id}: Code sample {path_to_crate} missing in {compare_dir}"
                    )
                    continue

                compare_sample = compare_samples_map[path_to_crate]

                # Check vulnerable functions
                if (
                    ref_sample["vulnerable_functions"]
                    != compare_sample["vulnerable_functions"]
                ):
                    errors.append(
                        f"{vuln_id}/{path_to_crate}: Vulnerable functions mismatch between {ref_dir} and {compare_dir}"
                    )
                    errors.append(
                        f"  {ref_dir}: {json.dumps(ref_sample['vulnerable_functions'], indent=2)}"
                    )
                    errors.append(
                        f"  {compare_dir}: {json.dumps(compare_sample['vulnerable_functions'], indent=2)}"
                    )

                # Check vulnerable lines and actual file content
                for file_path, ref_lines in ref_sample["vulnerable_lines"].items():
                    if file_path not in compare_sample["vulnerable_lines"]:
                        errors.append(
                            f"{vuln_id}/{path_to_crate}: File {file_path} missing vulnerable lines in {compare_dir}"
                        )
                        continue

                    compare_lines = compare_sample["vulnerable_lines"][file_path]

                    # Read actual lines from both directories
                    ref_file_lines = read_file_lines(
                        ref_dir, path_to_crate, file_path, ref_lines
                    )
                    compare_file_lines = read_file_lines(
                        compare_dir, path_to_crate, file_path, compare_lines
                    )

                    # Compare actual content (whitespace-insensitive)
                    content_matches = True
                    if ref_file_lines and compare_file_lines:
                        if len(ref_file_lines) != len(compare_file_lines):
                            content_matches = False
                            errors.append(
                                f"{vuln_id}/{path_to_crate}/{file_path}: Different number of vulnerable lines"
                            )
                            errors.append(f"  {ref_dir}: {len(ref_file_lines)} lines")
                            errors.append(
                                f"  {compare_dir}: {len(compare_file_lines)} lines"
                            )
                        else:
                            # Always do whitespace-insensitive comparison
                            for (ref_num, ref_line), (comp_num, comp_line) in zip(
                                ref_file_lines, compare_file_lines
                            ):
                                # Remove all whitespace for comparison
                                ref_normalized = "".join(ref_line.split())
                                comp_normalized = "".join(comp_line.split())
                                if ref_normalized != comp_normalized:
                                    content_matches = False
                                    errors.append(
                                        f"{vuln_id}/{path_to_crate}/{file_path}: Line content differs"
                                    )
                                    errors.append(
                                        f"  {ref_dir} line {ref_num}: {ref_line}"
                                    )
                                    errors.append(
                                        f"  {compare_dir} line {comp_num}: {comp_line}"
                                    )

    return {"errors": errors}


def main():
    """Main function."""
    import argparse

    parser = argparse.ArgumentParser(
        description="Check mizan.json consistency across dataset variants"
    )
    parser.add_argument(
        "--verbose", "-v", action="store_true", help="Show detailed error messages"
    )
    parser.add_argument(
        "--directories",
        "-d",
        nargs="+",
        default=[".", "variants/18-vulns", "variants/fmt-unconventional"],
        help="Directories to check (default: . variants/18-vulns variants/fmt-unconventional)",
    )
    args = parser.parse_args()

    directories = args.directories

    print("Checking mizan.json consistency across variants...")
    print(f"Directories: {', '.join(directories)}")
    print()

    results = check_consistency(directories)

    if results["errors"]:
        # Group errors by type
        content_errors = [
            e
            for e in results["errors"]
            if "content" in e.lower() and not e.startswith("  ")
        ]
        other_errors = [
            e
            for e in results["errors"]
            if "content" not in e.lower() and not e.startswith("  ")
        ]

        print(f"Found {len(results['errors'])} total inconsistencies:")
        print(f"  - {len(content_errors)} content mismatches")
        print(f"  - {len(other_errors)} other issues")
        print()

        if args.verbose:
            for error in results["errors"]:
                print(error)
        else:
            # Show summary only
            print("Run with --verbose flag to see detailed errors")
            print()
            # Show first few errors as examples
            example_errors = [e for e in results["errors"] if not e.startswith("  ")][
                :5
            ]
            if example_errors:
                print("Example errors:")
                for error in example_errors:
                    print(f"  - {error}")
                if len([e for e in results["errors"] if not e.startswith("  ")]) > 5:
                    print(
                        f"  ... and {len([e for e in results['errors'] if not e.startswith('  ')]) - 5} more"
                    )

        sys.exit(1)
    else:
        print("✓ All mizan.json files are consistent across variants!")
        sys.exit(0)


if __name__ == "__main__":
    main()
