#!/usr/bin/env python3
"""Validates mizan.json structure and referenced files."""

import json
import os
from jsonschema import validate, ValidationError

# Schema for mizan.json
schema = {
    "type": "object",
    "properties": {
        "general_information": {
            "type": "object",
            "properties": {
                "benchmark_name": {"type": "string"},
                "rust_version": {"type": "string"},
                "dataset_version": {"type": "string"},
            },
            "required": ["benchmark_name", "rust_version", "dataset_version"],
        },
        "vulnerabilities": {
            "type": "array",
            "items": {
                "type": "object",
                "properties": {
                    "id": {"type": "string"},
                    "author": {"type": "string"},
                    "source_link": {"type": "string"},
                    "crate_name": {"type": "string"},
                    "year": {"type": "integer"},
                    "code_samples": {
                        "type": "array",
                        "items": {
                            "type": "object",
                            "properties": {
                                "path_to_crate": {"type": "string"},
                                "is_vulnerability": {"type": "boolean"},
                                "cwe_type": {
                                    "type": "array",
                                    "items": {"type": "string"},
                                },
                                "vulnerable_functions": {"type": "object"},
                                "vulnerable_lines": {"type": "object"},
                                "deps": {"type": "array", "items": {"type": "string"}},
                            },
                            "required": [
                                "path_to_crate",
                                "is_vulnerability",
                                "cwe_type",
                                "vulnerable_functions",
                                "vulnerable_lines",
                                "deps",
                            ],
                        },
                    },
                },
                "required": ["id", "author", "source_link", "code_samples"],
            },
        },
    },
    "required": ["general_information", "vulnerabilities"],
}


def validate_mizan():
    """Main validation function."""
    # Load and validate JSON schema
    try:
        with open("mizan.json", "r") as f:
            data = json.load(f)
        validate(instance=data, schema=schema)
        print("✅ JSON schema validation successful!")
    except ValidationError as e:
        print(f"❌ JSON validation error: {e.message}")
        return False
    except json.JSONDecodeError as e:
        print(f"❌ Invalid JSON format: {e.msg}")
        return False
    except FileNotFoundError:
        print("❌ mizan.json not found!")
        return False

    errors = []

    # Validate each vulnerability
    for vuln in data["vulnerabilities"]:
        vuln_id = vuln["id"]

        for sample in vuln["code_samples"]:
            path_to_crate = sample["path_to_crate"]
            # Now path_to_crate already has the correct format, just prepend samples/
            actual_path = os.path.join("samples", path_to_crate)

            # Check if sample directory exists
            if not os.path.exists(actual_path):
                errors.append(f"Sample directory not found: {actual_path}")
                continue

            # Validate vulnerable vs fixed consistency
            is_vuln = sample["is_vulnerability"]
            has_vuln_funcs = len(sample["vulnerable_functions"]) > 0
            has_vuln_lines = len(sample["vulnerable_lines"]) > 0

            if is_vuln and not (has_vuln_funcs and has_vuln_lines):
                errors.append(
                    f"{path_to_crate}: marked as vulnerable but missing vulnerable functions/lines"
                )
            elif not is_vuln and (has_vuln_funcs or has_vuln_lines):
                errors.append(
                    f"{path_to_crate}: marked as fixed but has vulnerable functions/lines"
                )

            # Validate referenced files exist
            for file_path in sample["vulnerable_functions"]:
                full_path = os.path.join(actual_path, file_path)
                if not os.path.exists(full_path):
                    errors.append(f"Referenced file not found: {full_path}")

            for file_path in sample["vulnerable_lines"]:
                full_path = os.path.join(actual_path, file_path)
                if not os.path.exists(full_path):
                    errors.append(f"Referenced file not found: {full_path}")

            # Validate dependencies exist
            for dep in sample["deps"]:
                dep_path = os.path.join("samples", "deps", dep)
                if not os.path.exists(dep_path):
                    errors.append(f"Dependency not found: {dep_path}")

    # Report results
    if errors:
        print("\n❌ Validation errors found:")
        for error in errors:
            print(f"  - {error}")
        return False

    print("✅ All validations passed!")
    return True


if __name__ == "__main__":
    success = validate_mizan()
    exit(0 if success else 1)
