from dataclasses import dataclass, field
import json
import jsonschema
import os
from jsonschema import validate

ignored_line_validation_leftover_warning_ids = {
    "vuln-0003" # removes a function from vuln-crate
}

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
                    "source_link": {"type": "string"},
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
                    "source_link",
                    "code_samples",
                ],
            },
        },
    },
    "required": ["general_information", "vulnerabilities"],
}

@dataclass
class LineNumberAndInfo:
    line_number_and_line_and_file_name: list[tuple[int, str, str]]

    def remove_line_if_present(self, line) -> bool:
        cleaned_line = line.strip()
        for pair in self.line_number_and_line_and_file_name:
            if pair[1] == cleaned_line:
                self.line_number_and_line_and_file_name.remove(pair)
                return True
        return False

    def __str__(self):
        return f"{self.line_number_and_line_and_file_name}"

    def __len__(self):
        return self.line_number_and_line_and_file_name.__len__()

class VulnCrateInfo:
    __line_number_and_info: LineNumberAndInfo

    def copy_line_info(self) -> LineNumberAndInfo:
        return LineNumberAndInfo(list(self.__line_number_and_info.line_number_and_line_and_file_name))

    def __init__(self, sample: any) -> None:
        crate_path = sample["path_to_crate"]
        line_number_and_info = list()
        for vuln_rust_file_location_key in sample["vulnerable_lines"]:
            vuln_rust_file_path = os.path.join(crate_path, vuln_rust_file_location_key)
            if not os.path.exists(vuln_rust_file_path):
                raise FileNotFoundError(f"❌ Path: {file_path} doesn't exist!")
            lines_array = set(sample["vulnerable_lines"][vuln_rust_file_location_key])
            with open(vuln_rust_file_path, "r", encoding="utf-8") as f:
                for line_idx, line in enumerate(f.readlines()):
                    if line_idx + 1 in lines_array:
                        clean = line.strip()
                        line_number_and_info.append((line_idx + 1, clean, str(vuln_rust_file_path)))
        self.__line_number_and_info = LineNumberAndInfo(line_number_and_info)

try:
    with open(json_file_path, "r", encoding="utf-8") as file:
        data = json.load(file)
    validate(instance=data, schema=schema)
    print("✅ JSON schema validation successful!")

    # validate file existence and line info
    any_error = False
    for vulnerability in data["vulnerabilities"]:
        vuln_crate_info: None | VulnCrateInfo = None

        for sample in vulnerability["code_samples"]:
            if sample["path_to_crate"].endswith("vuln-crate"):
                vuln_crate_info = VulnCrateInfo(sample)
                break
        if vuln_crate_info is None:
            id_name = vulnerability["id"]
            print(f"❌ {id_name}: vuln-crate doesn't exist!")
            any_error = True

        for sample in vulnerability["code_samples"]:
            path = sample["path_to_crate"]
            if not os.path.exists(path):
                print(f"❌ path: {path} doesn't exist!")
                exit(1)

            for file_name_key in sample["vulnerable_functions"]:
                file_path = os.path.join(path, file_name_key)
                if not os.path.exists(path):
                    print(f"❌ vulnerable_functions path: {file_path} doesn't exist!")
                    any_error = True

            if sample["is_vulnerability"]:
                if len(sample["vulnerable_lines"]) == 0:
                    print(f"❌ is_vulnerability is true but no vulnerable_lines are specified")
                    any_error = True
                if len(sample["vulnerable_functions"]) == 0:
                    print(f"❌ is_vulnerability is true but no vulnerable_functions are specified")
                    any_error = True
            else:
                if len(sample["vulnerable_lines"]) > 0:
                    print(f"❌ is_vulnerability is false but vulnerable_lines are specified")
                    any_error = True
                if len(sample["vulnerable_functions"]) > 0:
                    print(f"❌ is_vulnerability is false but vulnerable_functions are specified")
                    any_error = True

            if sample["is_vulnerability"] and vuln_crate_info is not None:
                remaining_vuln_crate_lines: LineNumberAndInfo = vuln_crate_info.copy_line_info()

                for file_name_key in sample["vulnerable_lines"]:
                    file_path = os.path.join(path, file_name_key)
                    if not os.path.exists(path):
                        print(f"❌ vulnerable_lines path: {file_path} doesn't exist!")
                        any_error = True
                        continue

                    vulnerable_lines = set(sample["vulnerable_lines"][file_name_key])

                    with open(file_path, "r", encoding="utf-8") as file:
                        for line_idx, line in enumerate(file.readlines()):
                            if line_idx + 1 not in vulnerable_lines:
                                continue

                            cleaned_line = line.strip()
                            if remaining_vuln_crate_lines.remove_line_if_present(cleaned_line):
                                pass
                            else:
                                print(f"❌ {file_path}: line {line_idx + 1} [{cleaned_line}] not matching any of "
                                      f"previous lines for file {file_name_key} ... "
                                      f"remaining lines from vuln-crate are {remaining_vuln_crate_lines} ")
                                any_error = True
                if (len(remaining_vuln_crate_lines) > 0
                        and vulnerability["id"] not in ignored_line_validation_leftover_warning_ids):
                    print(f"❓ warning: some lines from vuln-crate not matched to ones in {path}: "
                          f"{remaining_vuln_crate_lines}")
    if any_error:
        exit(1)
    print("✅ mizan.json file name keys and line number validation successful!")
except jsonschema.exceptions.ValidationError as e:
    print("❌ JSON validation error:", e.message)
    exit(1)
except json.JSONDecodeError as e:
    print("❌ Invalid JSON format:", e.msg)
    exit(1)
except Exception as e:
    import traceback
    print("❌ Unexpected error:", str(e))
    print(traceback.format_exc())
    exit(1)
