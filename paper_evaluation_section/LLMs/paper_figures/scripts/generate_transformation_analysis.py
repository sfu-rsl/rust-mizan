import sys
import pandas as pd
from pathlib import Path
import json

sys.path.append(str(Path(__file__).parent.parent))

from common.data_utils import load_processed_experiment, get_short_model_name
from common.metrics import compute_hit_at_1_function_from_dataframe


def load_experiments_mapping():
    experiments_path = (
        Path(__file__).parent.parent.parent / "evaluation_results" / "experiments.json"
    )
    with open(experiments_path, "r") as f:
        return json.load(f)


def get_transformation_names():
    return [
        "format-expanded",
        "format-compact",
        "mizan-mut-arithmetic-identity",
        "mizan-mut-derive-reorder",
        "mizan-mut-for-to-while",
        "mizan-mut-if-else-reorder",
        "mizan-mut-trait-bound-reorder",
        "mizan-mut-use-reorder",
        "mizan-mut-while-to-loop",
        "benign-blocks",
        "benign-comments",
        "benign-rename-fn",
        "benign-rename-var",
        "malignant-blocks",
        "malignant-comments",
        "malignant-rename-fn",
        "malignant-rename-var",
    ]


def calculate_sample_metrics(df):
    """Calculate metrics for a DataFrame - now using common function."""
    return {"hit_at_1_function": compute_hit_at_1_function_from_dataframe(df)}


def get_short_transformation_name(transformation):
    name_mapping = {
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
    return name_mapping.get(transformation, transformation)


def compute_transformation_deltas(experiments_map):
    transformations = get_transformation_names()
    models = list(experiments_map.keys())

    delta_data = []

    for model in models:
        model_experiments = experiments_map[model]

        vanilla_exp_id = model_experiments["vanilla"]
        vanilla_df = load_processed_experiment(vanilla_exp_id)
        vanilla_valid = vanilla_df[vanilla_df["is_valid_json"] == True]

        for transformation in transformations:
            if transformation not in model_experiments:
                continue

            trans_exp_id = model_experiments[transformation]

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

                vanilla_metrics = calculate_sample_metrics(vanilla_common)
                trans_metrics = calculate_sample_metrics(trans_common)

                # Only process hit@1-function metric
                metric = "hit_at_1_function"
                delta = trans_metrics[metric] - vanilla_metrics[metric]

                delta_data.append(
                    {
                        "model": get_short_model_name(model),
                        "transformation": get_short_transformation_name(transformation),
                        "transformation_raw": transformation,
                        "metric": metric,
                        "vanilla_score": vanilla_metrics[metric],
                        "transformation_score": trans_metrics[metric],
                        "delta": delta,
                        "common_samples": len(common_samples),
                    }
                )

            except Exception:
                continue

    return pd.DataFrame(delta_data)


def create_transformation_table(delta_df):
    from common.data_utils import get_vanilla_experiment_ids
    from common.metrics import compute_experiment_metrics, compute_aggregate_metrics

    # Get overall vanilla baseline scores using all common samples across all experiments
    experiments_map = load_experiments_mapping()
    all_experiment_ids = []
    for model, experiments in experiments_map.items():
        all_experiment_ids.extend(experiments.values())

    vanilla_experiments = get_vanilla_experiment_ids()
    experiment_ids = list(vanilla_experiments.values())
    model_names = list(vanilla_experiments.keys())

    # Use utility function to get vanilla baseline scores
    vanilla_data = compute_experiment_metrics(experiment_ids, model_names)
    vanilla_metrics = compute_aggregate_metrics(vanilla_data, vulnerable_only=True)

    vanilla_baseline_scores = {}
    for metric in vanilla_metrics:
        vanilla_baseline_scores[metric["Model"]] = metric["Hit@1-Function"]

    # Get transformation deltas and scores
    transformation_data = (
        delta_df.groupby(["transformation", "model"])
        .agg({"delta": "mean", "transformation_score": "mean"})
        .reset_index()
    )

    # Create vanilla baseline row
    vanilla_row = {"Transformation": "vanilla (baseline)"}
    for model, score in vanilla_baseline_scores.items():
        vanilla_row[model] = f"{score:.3f}"

    # Create transformation rows
    table_rows = [vanilla_row]

    for transformation in transformation_data["transformation"].unique():
        trans_data = transformation_data[
            transformation_data["transformation"] == transformation
        ]
        row = {"Transformation": transformation}

        for _, trans_row in trans_data.iterrows():
            model = trans_row["model"]
            actual_score = trans_row["transformation_score"]
            # Calculate delta relative to overall vanilla baseline
            baseline_score = vanilla_baseline_scores[model]
            delta = actual_score - baseline_score
            row[model] = f"{actual_score:.3f} ({delta:+.3f})"

        table_rows.append(row)

    table_df = pd.DataFrame(table_rows)
    model_order = ["Claude 3.7 Sonnet", "GPT-4.1", "Gemini 1.5 Pro", "DeepSeek-V3.1"]
    column_order = ["Transformation"] + [
        col for col in model_order if col in table_df.columns
    ]
    table_df = table_df.reindex(columns=column_order)

    return table_df


def main():
    current_dir = Path(__file__).parent.parent
    tables_dir = current_dir / "tables"
    tables_dir.mkdir(exist_ok=True)

    experiments_map = load_experiments_mapping()
    delta_df = compute_transformation_deltas(experiments_map)

    if len(delta_df) == 0:
        return

    table_df = create_transformation_table(delta_df)

    # Save transformation impact table
    table_path = tables_dir / "transformation_impact_table.csv"
    table_df.to_csv(table_path, index=False)


if __name__ == "__main__":
    main()
