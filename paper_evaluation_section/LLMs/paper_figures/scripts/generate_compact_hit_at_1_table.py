import sys
import json
import pandas as pd
from pathlib import Path
from typing import Dict

sys.path.append(str(Path(__file__).parent.parent))

from common.data_utils import get_vanilla_experiment_ids
from common.metrics import compute_experiment_metrics


def load_vulnerability_mapping() -> Dict[str, Dict[str, str]]:
    current_dir = Path(__file__).parent.parent
    mapping_path = current_dir / "data" / "vulnerability_mapping.json"

    if not mapping_path.exists():
        raise FileNotFoundError(f"Vulnerability mapping not found: {mapping_path}")

    with open(mapping_path, "r", encoding="utf-8") as f:
        return json.load(f)


def get_model_short_names() -> Dict[str, str]:
    return {
        "claude-3-7-sonnet": "Claude 3.7",
        "gpt-4.1": "GPT-4.1",
        "gemini-1.5-pro": "Gemini 1.5 Pro",
        "deepseek-chat": "DeepSeek",
    }


def format_cwe_for_latex(cwe: str, description: str) -> str:
    if cwe == "Unknown":
        return "Unknown"
    return cwe


def get_hit_status(df: pd.DataFrame, vuln_id: str, granularity: str, model: str) -> str:
    model_mapping = {
        "Claude 3.7 Sonnet": "Claude 3.7 Sonnet",
        "GPT-4.1": "GPT-4.1",
        "Gemini 1.5 Pro": "Gemini 1.5 Pro",
        "DeepSeek-V3.1": "DeepSeek-V3.1",
    }

    actual_model_name = model_mapping.get(model, model)

    filtered = df[
        (df["vuln_id"] == vuln_id)
        & (df["granularity"] == granularity)
        & (df["model"] == actual_model_name)
        & (df["is_vulnerable_gt"] == True)
    ]

    if filtered.empty:
        return "\\cellna"

    has_hit = (filtered["function_tp"] > 0).any()
    return "\\cellyes" if has_hit else "\\cellno"


def generate_table_rows(
    experiment_data: Dict[str, pd.DataFrame], vulnerability_mapping: Dict
) -> str:
    available_models = list(experiment_data.keys())
    granularities = ["crate", "file", "function"]
    combined_df = pd.concat([df for df in experiment_data.values()], ignore_index=True)
    available_vulns = sorted(combined_df["vuln_id"].unique())
    table_rows = []

    for vuln_id in available_vulns:
        if vuln_id not in vulnerability_mapping:
            continue

        vuln_info = vulnerability_mapping[vuln_id]
        cve_id = vuln_info["cve_id"]
        cwe_info = format_cwe_for_latex(vuln_info["cwe"], vuln_info["description"])

        row_parts = [f"% Row for {vuln_id}"]
        row_parts.append(f"& \\vulnId{{{cve_id}}} & {cwe_info} & ")

        model_results = []
        for model in available_models:
            granularity_results = []
            for granularity in granularities:
                status = get_hit_status(combined_df, vuln_id, granularity, model)
                granularity_results.append(status)
            model_results.append(" & ".join(granularity_results))

        row_parts.append("& " + " \n& ".join(model_results) + " \\\\")
        row_parts.append("\\hline")

        table_rows.append("\n".join(row_parts))

    return "\n".join(table_rows)


def generate_compact_table() -> None:
    vanilla_experiments = get_vanilla_experiment_ids()
    experiment_ids = list(vanilla_experiments.values())
    model_names = list(vanilla_experiments.keys())

    experiment_data = compute_experiment_metrics(experiment_ids, model_names)
    vulnerability_mapping = load_vulnerability_mapping()
    table_rows = generate_table_rows(experiment_data, vulnerability_mapping)

    current_dir = Path(__file__).parent.parent
    template_path = current_dir / "latex_formatters" / "TEMPLATE_hit_at_1.tex"

    with open(template_path, "r", encoding="utf-8") as f:
        template_content = f.read()

    final_content = template_content.replace("{TABLE_ROWS}", table_rows)

    latex_dir = current_dir / "latex"
    latex_dir.mkdir(exist_ok=True)
    output_path = latex_dir / "hit_at_1_compact_generated.tex"

    with open(output_path, "w", encoding="utf-8") as f:
        f.write(final_content)


if __name__ == "__main__":
    generate_compact_table()
