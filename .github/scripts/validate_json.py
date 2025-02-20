import json
import jsonschema
from jsonschema import validate

json_file_path = "mizan.json"

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
                    "title": {"type": "string"},
                    "vulnerability_type": {"type": "string"},
                    "vulnerability_subtypes": {
                        "type": "array",
                        "items": {"type": "string"},
                    },
                    "severity": {"type": "string"},
                    "source_link": {"type": "string"},
                    "description": {"type": "string"},
                    "code_samples": {
                        "type": "array",
                        "items": {
                            "type": "object",
                            "properties": {
                                "path_to_crate": {"type": "string"},
                                "is_vulnerability": {"type": "boolean"},
                                "cwe_type": {"type": ["string", "null"]},
                                "vulnerable_functions": {"type": "object"},
                                "vulnerable_lines": {"type": "object"},
                            },
                            "required": [
                                "path_to_crate",
                                "is_vulnerability",
                                "cwe_type",
                                "vulnerable_functions",
                                "vulnerable_lines",
                            ],
                        },
                    },
                },
                "required": [
                    "id",
                    "author",
                    "title",
                    "vulnerability_type",
                    "vulnerability_subtypes",
                    "severity",
                    "source_link",
                    "description",
                    "code_samples",
                ],
            },
        },
    },
    "required": ["general_information", "vulnerabilities"],
}

try:
    with open(json_file_path, "r", encoding="utf-8") as file:
        data = json.load(file)
    validate(instance=data, schema=schema)
    print("✅ JSON validation successful!")
except jsonschema.exceptions.ValidationError as e:
    print("❌ JSON validation error:", e.message)
    exit(1)
except json.JSONDecodeError as e:
    print("❌ Invalid JSON format:", e.msg)
    exit(1)
except Exception as e:
    print("❌ Unexpected error:", str(e))
    exit(1)
