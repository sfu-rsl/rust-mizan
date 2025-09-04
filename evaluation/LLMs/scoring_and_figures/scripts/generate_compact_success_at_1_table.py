import sys
import json
import pandas as pd
from pathlib import Path
from typing import Dict

sys.path.append(str(Path(__file__).parent.parent))

from common.data_utils import get_vanilla_experiment_ids, MODEL_ORDER
from common.metrics import load_experiment_data


def load_vulnerability_mapping() -> Dict[str, Dict[str, str]]:
    current_dir = Path(__file__).parent.parent
    mapping_path = current_dir / "data" / "vulnerability_mapping.json"

    if not mapping_path.exists():
        raise FileNotFoundError(f"Vulnerability mapping not found: {mapping_path}")

    with open(mapping_path, "r", encoding="utf-8") as f:
        return json.load(f)


def get_success_status(
    df: pd.DataFrame, vuln_id: str, granularity: str, model: str
) -> str:
    """
    Check if model achieved Success@1 for function localization on this vulnerability.

    Args:
        df: Combined dataframe with results from all models
        vuln_id: Vulnerability ID to check
        granularity: Code granularity level
        model: Model name

    Returns:
        LaTeX cell formatting for success status
    """
    filtered = df[
        (df["vuln_id"] == vuln_id)
        & (df["granularity"] == granularity)
        & (df["model"] == model)
        & (df["is_vulnerable_gt"] == True)
    ]

    if filtered.empty:
        return "\\cellna"

    has_success = (filtered["function_tp"] > 0).any()
    return "\\cellyes" if has_success else "\\cellno"


def generate_table_rows(
    experiment_data: Dict[str, pd.DataFrame], vulnerability_mapping: Dict
) -> str:
    models = list(experiment_data.keys())
    granularities = ["crate", "file", "function"]
    combined_df = pd.concat(experiment_data.values(), ignore_index=True)

    # Group vulnerabilities by CWE
    cwe_groups = {}
    for vuln_id in sorted(combined_df["vuln_id"].unique()):
        if vuln_id in vulnerability_mapping:
            cwe = vulnerability_mapping[vuln_id]["cwe"]
            cwe_groups.setdefault(cwe, []).append(vuln_id)

    table_rows = []
    sorted_cwes = sorted(cwe_groups.keys())

    for i, cwe in enumerate(sorted_cwes):
        vuln_ids = sorted(cwe_groups[cwe])

        for j, vuln_id in enumerate(vuln_ids):
            cve_id = vulnerability_mapping[vuln_id]["cve_id"]

            # Build row
            row = [f"% Row for {vuln_id}"]

            # CWE cell (multirow on first occurrence)
            if j == 0:
                row.append(
                    f"& \\multirow{{{len(vuln_ids)}}}{{*}}{{{cwe}}} & \\vulnId{{{cve_id}}} & "
                )
            else:
                row.append(f"& & \\vulnId{{{cve_id}}} & ")

            # Model results
            model_results = []
            for model in models:
                results = [
                    get_success_status(combined_df, vuln_id, g, model)
                    for g in granularities
                ]
                model_results.append(" & ".join(results))

            row.append("& " + " \n& ".join(model_results) + " \\\\")

            # Add separator between CWE groups
            if j == len(vuln_ids) - 1 and i < len(sorted_cwes) - 1:
                row.append("\\hline")

            table_rows.append("\n".join(row))

    return "\n".join(table_rows)


def generate_success_at_1_table() -> None:
    vanilla_experiments = get_vanilla_experiment_ids()
    experiment_ids = list(vanilla_experiments.values())
    model_names = list(vanilla_experiments.keys())

    experiment_data = load_experiment_data(experiment_ids, model_names)
    vulnerability_mapping = load_vulnerability_mapping()
    table_rows = generate_table_rows(experiment_data, vulnerability_mapping)

    current_dir = Path(__file__).parent.parent
    template_path = current_dir / "latex" / "TEMPLATE_success_at_1.tex"

    with open(template_path, "r", encoding="utf-8") as f:
        template_content = f.read()

    final_content = template_content.replace("{TABLE_ROWS}", table_rows)

    latex_dir = current_dir / "latex"
    latex_dir.mkdir(exist_ok=True)
    output_path = latex_dir / "success_at_1_compact_generated.tex"

    with open(output_path, "w", encoding="utf-8") as f:
        f.write(final_content)


if __name__ == "__main__":
    generate_success_at_1_table()
