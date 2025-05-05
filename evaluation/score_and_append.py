#!/usr/bin/env python3
import json
import sys
import os
from datetime import datetime, timezone

MIZAN_JSON_PATH = "../mizan.json"
RESULTS_DIR = "results"
os.makedirs(RESULTS_DIR, exist_ok=True)


def load_ground_truth(benchmark_id):
    with open(MIZAN_JSON_PATH, "r") as f:
        mizan = json.load(f)

    for vuln in mizan["vulnerabilities"]:
        for sample in vuln["code_samples"]:
            if sample["path_to_crate"] == benchmark_id:
                return sample
    raise ValueError(f"Benchmark ID {benchmark_id} not found.")


def build_keys(functions_or_lines):
    """Turns {'file.rs': [items]} into set of 'file.rs::item' strings."""
    keys = set()
    for filename, items in functions_or_lines.items():
        for item in items:
            keys.add(f"{filename}::{item}")
    return keys


def score(parsed, ground_truth, is_fixed_version):
    # Existence detection
    is_vulnerable_pred = parsed.get("is_vulnerable", False)
    is_vulnerable_gt = ground_truth.get("is_vulnerability", False)
    existence_detection = {"is_correct": is_vulnerable_pred == is_vulnerable_gt}

    # CWE inference
    parsed_cwes = set(parsed.get("cwe_type", []))
    gt_cwes = set(ground_truth.get("cwe_type", []))
    cwe_inference = {
        "correct_predictions": list(parsed_cwes & gt_cwes),
        "missed_predictions": list(gt_cwes - parsed_cwes),
        "extra_predictions": list(parsed_cwes - gt_cwes),
    }

    # Key object identification
    parsed_func_keys = build_keys(parsed.get("vulnerable_functions", {}))
    gt_func_keys = build_keys(ground_truth.get("vulnerable_functions", {}))
    key_objects_identification = {
        "true_positive_keys": list(parsed_func_keys & gt_func_keys),
        "false_positive_keys": list(parsed_func_keys - gt_func_keys),
        "missed_keys": list(gt_func_keys - parsed_func_keys),
    }

    # Root cause location
    parsed_line_keys = build_keys(parsed.get("vulnerable_lines", {}))
    gt_line_keys = build_keys(ground_truth.get("vulnerable_lines", {}))
    root_cause_location = {
        "true_positive_keys": list(parsed_line_keys & gt_line_keys),
        "false_positive_keys": list(parsed_line_keys - gt_line_keys),
        "missed_keys": list(gt_line_keys - parsed_line_keys),
    }

    return {
        "existence_detection": existence_detection,
        "cwe_inference": cwe_inference,
        "key_objects_identification": key_objects_identification,
        "root_cause_location": root_cause_location,
    }


def main():
    if len(sys.argv) != 5:
        print(
            f"Usage: {sys.argv[0]} <llm_name> <benchmark_id> <llm_output_json_file> <prompt_name>"
        )
        sys.exit(1)

    llm_name = sys.argv[1]
    benchmark_id = sys.argv[2]
    llm_output_file = sys.argv[3]
    prompt_name = sys.argv[4]

    with open(llm_output_file, "r") as f:
        parsed = json.load(f)

    ground_truth = load_ground_truth(benchmark_id)
    is_fixed_version = not ground_truth["is_vulnerability"]

    scoring = score(parsed, ground_truth, is_fixed_version)

    result = {
        "llm_name": llm_name,
        "prompt_name": prompt_name,
        "code_sample": benchmark_id,
        "timestamp": datetime.now(timezone.utc).isoformat(),
        "is_fixed_version": is_fixed_version,
        "raw_answer": json.dumps(parsed),
        "parsed_answer": parsed,
        "scoring": scoring,
    }

    results_file = os.path.join(RESULTS_DIR, f"{llm_name}--{prompt_name}.json")

    if os.path.exists(results_file):
        with open(results_file, "r") as f:
            results = json.load(f)
    else:
        results = []

    results.append(result)

    with open(results_file, "w") as f:
        json.dump(results, f, indent=2)

    print(f"Scored and appended to {results_file}")


if __name__ == "__main__":
    main()
