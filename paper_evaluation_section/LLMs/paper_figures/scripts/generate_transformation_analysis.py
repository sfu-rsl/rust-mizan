import sys
import pandas as pd
from pathlib import Path
import json

sys.path.append(str(Path(__file__).parent.parent))

from common.data_utils import (
    load_processed_experiment,
    get_short_model_name,
    get_vanilla_experiment_ids,
    get_ordered_models,
)
from common.metrics import (
    compute_success_at_1_function_rate,
    load_experiment_data,
    compute_aggregate_metrics,
)


TRANSFORMATIONS = {
    "format-expanded": "Format Expanded",
    "format-compact": "Format Compact",
    "mizan-mut-arithmetic-identity": "Arithmetic Identity",
    "mizan-mut-derive-reorder": "Derive Reorder",
    "mizan-mut-for-to-while": "For→While",
    "mizan-mut-if-else-reorder": "If-Else Reorder",
    "mizan-mut-trait-bound-reorder": "Trait Bound Reorder",
    "mizan-mut-use-reorder": "Use Reorder",
    "mizan-mut-while-to-loop": "While→Loop",
    "benign-blocks": "Benign Blocks",
    "benign-comments": "Benign Comments",
    "benign-rename-fn": "Benign Rename Fn",
    "benign-rename-var": "Benign Rename Var",
    "malignant-blocks": "Malignant Blocks",
    "malignant-comments": "Malignant Comments",
    "malignant-rename-fn": "Malignant Rename Fn",
    "malignant-rename-var": "Malignant Rename Var",
}

TRANSFORMATION_ORDER = [
    "vanilla (baseline)",
    "Format Compact",
    "Format Expanded",
    "For→While",
    "While→Loop",
    "If-Else Reorder",
    "Derive Reorder",
    "Trait Bound Reorder",
    "Use Reorder",
    "Arithmetic Identity",
    "Benign Comments",
    "Benign Blocks",
    "Benign Rename Fn",
    "Benign Rename Var",
    "Malignant Comments",
    "Malignant Blocks",
    "Malignant Rename Fn",
    "Malignant Rename Var",
]


def load_experiments_mapping():
    experiments_path = (
        Path(__file__).parent.parent.parent / "evaluation_results" / "experiments.json"
    )
    with open(experiments_path, "r") as f:
        return json.load(f)


def compute_transformation_deltas(experiments_map):
    models = list(experiments_map.keys())
    delta_data = []

    for model in models:
        model_experiments = experiments_map[model]
        vanilla_exp_id = model_experiments["vanilla"]
        vanilla_df = load_processed_experiment(vanilla_exp_id)
        vanilla_valid = vanilla_df[vanilla_df["is_valid_json"] == True]

        for transformation_raw, transformation_display in TRANSFORMATIONS.items():
            if transformation_raw not in model_experiments:
                continue

            trans_exp_id = model_experiments[transformation_raw]
            try:
                trans_df = load_processed_experiment(trans_exp_id)
                trans_valid = trans_df[trans_df["is_valid_json"] == True]

                vanilla_samples = set(vanilla_valid["example_id"].unique())
                trans_samples = set(trans_valid["example_id"].unique())
                common_samples = vanilla_samples.intersection(trans_samples)

                if len(common_samples) == 0:
                    continue

                vanilla_common = vanilla_valid[
                    vanilla_valid["example_id"].isin(common_samples)
                ]
                trans_common = trans_valid[
                    trans_valid["example_id"].isin(common_samples)
                ]

                vanilla_score = compute_success_at_1_function_rate(vanilla_common)
                trans_score = compute_success_at_1_function_rate(trans_common)
                delta = trans_score - vanilla_score

                delta_data.append(
                    {
                        "model": get_short_model_name(model),
                        "transformation": transformation_display,
                        "vanilla_score": vanilla_score,
                        "transformation_score": trans_score,
                        "delta": delta,
                    }
                )
            except Exception:
                continue

    return pd.DataFrame(delta_data)


def generate_table_rows(table_data):
    models = get_ordered_models(
        [col for col in table_data[0].keys() if col != "Transformation"]
    )

    def extract_delta(value_str):
        if "(" in str(value_str) and ")" in str(value_str):
            delta_part = str(value_str).split("(")[1].split(")")[0]
            return float(delta_part)
        return 0.0

    min_deltas = {}
    for model in models:
        deltas = [extract_delta(row[model]) for row in table_data[1:] if model in row]
        if deltas:
            min_deltas[model] = min(deltas)

    table_rows = []
    current_group = None

    for row in table_data:
        transform_name = row["Transformation"]

        if transform_name == "vanilla (baseline)":
            new_group = "vanilla"
        elif transform_name in ["Format Compact", "Format Expanded"]:
            new_group = "formatting"
        elif transform_name in [
            "For→While",
            "While→Loop",
            "If-Else Reorder",
            "Derive Reorder",
            "Trait Bound Reorder",
            "Use Reorder",
            "Arithmetic Identity",
        ]:
            new_group = "ast"
        elif transform_name in ["Benign Comments", "Benign Blocks"]:
            new_group = "benign_insert"
        elif transform_name in ["Benign Rename Fn", "Benign Rename Var"]:
            new_group = "benign_rename"
        elif transform_name in ["Malignant Comments", "Malignant Blocks"]:
            new_group = "malignant_insert"
        elif transform_name in ["Malignant Rename Fn", "Malignant Rename Var"]:
            new_group = "malignant_rename"
        else:
            new_group = "other"

        if current_group and current_group != new_group:
            if (
                (current_group == "vanilla" and new_group == "formatting")
                or (current_group == "ast" and new_group == "benign_insert")
                or (
                    current_group == "benign_rename" and new_group == "malignant_insert"
                )
            ):
                table_rows.append("\\addlinespace[4pt]")
            elif (
                (current_group == "formatting" and new_group == "ast")
                or (current_group == "benign_insert" and new_group == "benign_rename")
                or (
                    current_group == "malignant_insert"
                    and new_group == "malignant_rename"
                )
            ):
                table_rows.append("\\addlinespace[2pt]")

        current_group = new_group
        row_data = [transform_name]

        for model in models:
            value_str = str(row[model])
            if transform_name != "vanilla (baseline)":
                delta = extract_delta(value_str)
                if delta == min_deltas.get(model, 0) and delta < 0:
                    value_str = f"\\textcolor{{red}}{{{value_str}}}"
            row_data.append(value_str)

        table_rows.append(" & ".join(row_data) + " \\\\")

    return "\n".join(table_rows)


def main():
    current_dir = Path(__file__).parent.parent
    latex_dir = current_dir / "latex"
    latex_dir.mkdir(exist_ok=True)

    vanilla_experiments = get_vanilla_experiment_ids()
    experiment_ids = list(vanilla_experiments.values())
    model_names = list(vanilla_experiments.keys())
    vanilla_data = load_experiment_data(experiment_ids, model_names)
    vanilla_metrics = compute_aggregate_metrics(vanilla_data, vulnerable_only=True)

    vanilla_baseline_scores = {}
    for metric in vanilla_metrics:
        vanilla_baseline_scores[metric["Model"]] = metric["Success@1-Function"]

    experiments_map = load_experiments_mapping()
    delta_df = compute_transformation_deltas(experiments_map)

    if len(delta_df) == 0:
        return

    transformation_data = (
        delta_df.groupby(["transformation", "model"])
        .agg({"delta": "mean", "transformation_score": "mean"})
        .reset_index()
    )

    vanilla_row = {"Transformation": "vanilla (baseline)"}
    for model, score in vanilla_baseline_scores.items():
        vanilla_row[model] = f"{score:.3f}"

    table_rows = [vanilla_row]
    for transformation in TRANSFORMATION_ORDER[1:]:
        trans_data = transformation_data[
            transformation_data["transformation"] == transformation
        ]
        if trans_data.empty:
            continue

        row = {"Transformation": transformation}
        for _, trans_row in trans_data.iterrows():
            model = trans_row["model"]
            actual_score = trans_row["transformation_score"]
            baseline_score = vanilla_baseline_scores[model]
            delta = actual_score - baseline_score
            row[model] = f"{actual_score:.3f} ({delta:+.3f})"
        table_rows.append(row)

    template_path = current_dir / "latex" / "TEMPLATE_transformation_impact.tex"
    with open(template_path, "r", encoding="utf-8") as f:
        template_content = f.read()

    table_rows_content = generate_table_rows(table_rows)
    final_content = template_content.replace("{TABLE_ROWS}", table_rows_content)

    output_path = latex_dir / "transformation_impact_generated.tex"
    with open(output_path, "w", encoding="utf-8") as f:
        f.write(final_content)


if __name__ == "__main__":
    main()
