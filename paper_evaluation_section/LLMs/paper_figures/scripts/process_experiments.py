import sys
import json
import pandas as pd
from pathlib import Path
from typing import Dict, Any
from tqdm import tqdm

sys.path.append(str(Path(__file__).parent.parent))

from common.validation import validate_json_schema
from common.metrics import compute_sample_metrics
from common.data_utils import (
    load_experiments_catalog,
    load_experiment_results,
    save_processed_results,
)


def process_single_experiment(
    experiment_id: str, model_name: str = "unknown"
) -> Dict[str, Any]:
    results = load_experiment_results(experiment_id)

    processed_data = []

    for sample in tqdm(results, desc=f"Processing {experiment_id}"):
        row_data = {
            "example_id": sample["example_id"],
            "vuln_id": sample["vuln_id"],
            "granularity": sample["granularity"],
            "crate_name": sample["crate_name"],
            "year": sample["year"],
            "is_vulnerable_gt": sample["reference_outputs"]["is_vulnerable"],
            "cwe_types_gt": json.dumps(sample["reference_outputs"]["cwe_type"]),
            "is_valid_json": False,
            "is_vulnerable_pred": None,
            "cwe_types_pred": None,
        }

        try:
            parsed_response = sample["outputs"]["parsed_response"]
            row_data["is_valid_json"] = validate_json_schema(parsed_response)

            if row_data["is_valid_json"]:
                row_data["is_vulnerable_pred"] = parsed_response["is_vulnerable"]
                row_data["cwe_types_pred"] = json.dumps(parsed_response["cwe_type"])

                metrics = compute_sample_metrics(sample)
                row_data.update(metrics)

        except Exception:
            pass

        if not row_data["is_valid_json"]:
            row_data.update(
                {
                    "binary_accuracy": 0.0,
                    "cwe_tp": 0,
                    "cwe_fp": 0,
                    "cwe_fn": 0,
                    "function_tp": 0,
                    "function_fp": 0,
                    "function_fn": 0,
                    "line_tp": 0,
                    "line_fp": 0,
                    "line_fn": 0,
                    "success_at_1_function": 0.0,
                    "success_at_1_line": 0.0,
                }
            )

        processed_data.append(row_data)

    df = pd.DataFrame(processed_data)
    save_processed_results(experiment_id, df, model_name)

    return {
        "experiment_id": experiment_id,
        "model_name": model_name,
        "total_samples": len(df),
        "valid_samples": sum(df["is_valid_json"]),
        "validity_rate": sum(df["is_valid_json"]) / len(df) if len(df) > 0 else 0,
    }


def main():
    if len(sys.argv) > 1:
        experiment_id = sys.argv[1]
        process_single_experiment(experiment_id)
        return

    catalog = load_experiments_catalog()

    for model, experiments in catalog.items():
        for experiment_id in experiments.values():
            process_single_experiment(experiment_id, model)


if __name__ == "__main__":
    main()
