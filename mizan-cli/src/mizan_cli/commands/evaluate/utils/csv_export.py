import csv
from pathlib import Path
from typing import List, Dict, Any

from mizan_cli.utils.logging import get_logger

logger = get_logger()


def generate_csv_export(results: List[Dict[str, Any]], output_path: Path):
    """Generate CSV export of evaluation results."""
    if not results:
        logger.warning("No results to export to CSV")
        return

    # Define CSV headers
    headers = [
        "example_id",
        "vuln_id",
        "granularity",
        "crate_name",
        "is_vulnerable",
        "year",
        "cwe_types",
        # Per-sample scores
        "is_vulnerable_accuracy",
        "cwe_type_f1",
        "cwe_type_precision",
        "cwe_type_recall",
        "vulnerable_functions_f1",
        "vulnerable_functions_precision",
        "vulnerable_functions_recall",
        "vulnerable_lines_f1",
        "vulnerable_lines_precision",
        "vulnerable_lines_recall",
        "json_validity",
        "has_target_error",
    ]

    try:
        with open(output_path, "w", newline="", encoding="utf-8") as f:
            writer = csv.DictWriter(f, fieldnames=headers)
            writer.writeheader()

            for result in results:
                # Extract scores
                scores = result.get("scores", {})

                # Check if there was a target function error
                has_target_error = bool(result.get("errors")) and any(
                    "target_function" in str(error)
                    for error in result.get("errors", [])
                )

                # Create row
                row = {
                    "example_id": result.get("example_id", ""),
                    "vuln_id": result.get("vuln_id", ""),
                    "granularity": result.get("granularity", ""),
                    "crate_name": result.get("crate_name", ""),
                    "is_vulnerable": result.get("is_vulnerable", ""),
                    "year": result.get("year", ""),
                    "cwe_types": _format_cwe_list(result.get("cwe_types", [])),
                    "is_vulnerable_accuracy": scores.get("is_vulnerable_accuracy", ""),
                    "cwe_type_f1": scores.get("cwe_type_f1", ""),
                    "cwe_type_precision": scores.get("cwe_type_precision", ""),
                    "cwe_type_recall": scores.get("cwe_type_recall", ""),
                    "vulnerable_functions_f1": scores.get(
                        "vulnerable_functions_f1", ""
                    ),
                    "vulnerable_functions_precision": scores.get(
                        "vulnerable_functions_precision", ""
                    ),
                    "vulnerable_functions_recall": scores.get(
                        "vulnerable_functions_recall", ""
                    ),
                    "vulnerable_lines_f1": scores.get("vulnerable_lines_f1", ""),
                    "vulnerable_lines_precision": scores.get(
                        "vulnerable_lines_precision", ""
                    ),
                    "vulnerable_lines_recall": scores.get(
                        "vulnerable_lines_recall", ""
                    ),
                    "json_validity": scores.get("json_validity", ""),
                    "has_target_error": has_target_error,
                }

                writer.writerow(row)

        logger.info(f"CSV export saved to: {output_path}")

    except Exception as e:
        logger.error(f"Failed to generate CSV export: {e}")


def _format_cwe_list(cwe_list: List[str]) -> str:
    """Format CWE list as a semicolon-separated string."""
    if not cwe_list:
        return ""
    return ";".join(cwe_list)
