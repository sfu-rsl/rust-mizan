import sys
import pandas as pd
from pathlib import Path

sys.path.append(str(Path(__file__).parent.parent))

from common.data_utils import get_vanilla_experiment_ids
from common.metrics import compute_experiment_metrics


def main():
    current_dir = Path(__file__).parent.parent
    tables_dir = current_dir / "tables"
    tables_dir.mkdir(exist_ok=True)

    vanilla_experiments = get_vanilla_experiment_ids()
    experiment_ids = list(vanilla_experiments.values())
    model_names = list(vanilla_experiments.keys())

    # Get common samples and compute metrics for all experiments
    experiment_data = compute_experiment_metrics(experiment_ids, model_names)

    # Filter to vulnerable samples and combine all models
    hit_data = []

    for model_name, df in experiment_data.items():
        vuln_df = df[df["is_vulnerable_gt"] == True]

        vuln_df_copy = vuln_df.copy()
        vuln_df_copy["model"] = model_name
        hit_data.append(vuln_df_copy)

    hit_df = pd.concat(hit_data, ignore_index=True)

    # Create detailed table
    table_rows = []

    for sample_id in hit_df["example_id"].unique():
        sample_data = hit_df[hit_df["example_id"] == sample_id]
        first_row = sample_data.iloc[0]

        row = {
            "Sample ID": sample_id,
            "Vuln ID": first_row["vuln_id"],
            "Granularity": first_row["granularity"],
        }

        for _, model_row in sample_data.iterrows():
            model = model_row["model"]
            hit_count = model_row["function_tp"]
            total_count = model_row["function_tp"] + model_row["function_fn"]
            short_name = model

            row[short_name] = f"{hit_count}/{total_count}"

        table_rows.append(row)

    table_df = pd.DataFrame(table_rows)
    table_df = table_df.sort_values("Sample ID")

    # Save detailed table
    detailed_path = tables_dir / "table_7_2_hit_at_1_detailed.csv"
    table_df.to_csv(detailed_path, index=False)


if __name__ == "__main__":
    main()
