#!/usr/bin/env python3
import json
import sys
import os
from datetime import datetime, timezone

MIZAN_JSON_PATH = "../mizan.json"
RESULTS_DIR = "results"
os.makedirs(RESULTS_DIR, exist_ok=True)


def load_vuln_entry(vuln_id):
    with open(MIZAN_JSON_PATH, "r") as f:
        mizan = json.load(f)
    for v in mizan.get("vulnerabilities", []):
        if v["id"] == vuln_id:
            return v
    raise ValueError(f"Vulnerability {vuln_id} not found in {MIZAN_JSON_PATH}")


def normalize_cves(cve_list):
    return sorted(set([cve.upper() for cve in cve_list]))


def score_crate_info(pred, truth):
    scoring = {}

    # Crate name
    crate_pred = (pred.get("crate_name") or "").lower()
    crate_true = (truth.get("crate_name") or "").lower()
    scoring["crate_name_match"] = crate_pred == crate_true

    # Year (allow str/int match)
    year_pred = str(pred.get("likely_year") or "").strip()
    year_true = str(truth.get("year") or "").strip()
    scoring["year_match"] = year_pred == year_true

    # CVEs
    pred_cves = normalize_cves(pred.get("cve_list", []))
    true_cves = (
        normalize_cves([truth["source_link"].split("=")[-1]])
        if "source_link" in truth
        else []
    )
    scoring["cve_inference"] = {
        "correct_predictions": list(set(pred_cves) & set(true_cves)),
        "missed_predictions": list(set(true_cves) - set(pred_cves)),
        "extra_predictions": list(set(pred_cves) - set(true_cves)),
    }

    return scoring


def main():
    if len(sys.argv) != 5:
        print(
            f"Usage: {sys.argv[0]} <llm_name> <vuln_id> <llm_output_json> <prompt_name>"
        )
        sys.exit(1)

    llm_name = sys.argv[1]
    vuln_id = sys.argv[2]
    output_file = sys.argv[3]
    prompt_name = sys.argv[4]

    with open(output_file, "r") as f:
        parsed = json.load(f)

    truth = load_vuln_entry(vuln_id)
    scoring = score_crate_info(parsed, truth)

    result = {
        "llm_name": llm_name,
        "prompt_name": prompt_name,
        "vulnerability_id": vuln_id,
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
