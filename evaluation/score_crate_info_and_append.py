#!/usr/bin/env python3
import json
import sys
import os
from datetime import datetime, timezone

MIZAN_JSON_PATH = "../mizan.json"
RESULTS_DIR = "results"
os.makedirs(RESULTS_DIR, exist_ok=True)


def load_ground_truth(code_sample):
    with open(MIZAN_JSON_PATH, "r") as f:
        mizan = json.load(f)

    for vuln in mizan.get("vulnerabilities", []):
        for sample in vuln.get("code_samples", []):
            if sample["path_to_crate"] == code_sample:
                # return both the parent vuln and the sample
                return vuln
    raise ValueError(f"Code sample {code_sample} not found in {MIZAN_JSON_PATH}")


def normalize_cves(cve_list):
    return sorted(set([cve.upper() for cve in cve_list]))


def score_crate_info(parsed, truth):
    # Crate name
    crate_pred = (parsed.get("crate_name") or "").lower()
    crate_true = (truth.get("crate_name") or "").lower()
    crate_name_match = crate_pred == crate_true

    # Year
    year_pred = str(parsed.get("likely_year") or "").strip()
    year_true = str(truth.get("year") or "").strip()
    year_match = year_pred == year_true

    # CVE
    pred_cves = normalize_cves(parsed.get("cve_list", []))
    true_cves = (
        normalize_cves([truth["source_link"].split("=")[-1]])
        if "source_link" in truth
        else []
    )
    cve_inference = {
        "correct_predictions": list(set(pred_cves) & set(true_cves)),
        "missed_predictions": list(set(true_cves) - set(pred_cves)),
        "extra_predictions": list(set(pred_cves) - set(true_cves)),
    }

    return {
        "crate_name_match": crate_name_match,
        "year_match": year_match,
        "cve_inference": cve_inference,
    }


def main():
    if len(sys.argv) != 5:
        print(
            f"Usage: {sys.argv[0]} <llm_name> <code_sample> <llm_output_json_file> <prompt_name>"
        )
        sys.exit(1)

    llm_name = sys.argv[1]
    code_sample = sys.argv[2]
    output_file = sys.argv[3]
    prompt_name = sys.argv[4]

    with open(output_file, "r") as f:
        parsed = json.load(f)

    ground_truth = load_ground_truth(code_sample)
    scoring = score_crate_info(parsed, ground_truth)

    result = {
        "llm_name": llm_name,
        "prompt_name": prompt_name,
        "code_sample": code_sample,
        "timestamp": datetime.now(timezone.utc).isoformat(),
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
