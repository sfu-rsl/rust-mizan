import sys
import pandas as pd
from pathlib import Path

sys.path.append(str(Path(__file__).parent.parent))

from common.data_utils import get_vanilla_experiment_ids
from common.metrics import compute_experiment_metrics, compute_aggregate_metrics


def main():
    current_dir = Path(__file__).parent.parent
    tables_dir = current_dir / "tables"
    tables_dir.mkdir(exist_ok=True)

    vanilla_experiments = get_vanilla_experiment_ids()
    experiment_ids = list(vanilla_experiments.values())
    model_names = list(vanilla_experiments.keys())

    # Get common samples and compute metrics for all experiments
    experiment_data = compute_experiment_metrics(experiment_ids, model_names)

    # Compute aggregate metrics for all samples
    all_metrics = compute_aggregate_metrics(experiment_data, vulnerable_only=False)

    # Compute Hit@1-Function for vulnerable samples only
    vuln_metrics = compute_aggregate_metrics(experiment_data, vulnerable_only=True)

    # Combine metrics
    metrics_list = []
    for i, all_metric in enumerate(all_metrics):
        vuln_metric = vuln_metrics[i]
        metrics_list.append(
            {
                "Model": all_metric["Model"],
                "Samples": all_metric["Samples"],
                "Binary Accuracy": f"{all_metric['Binary Accuracy']:.3f}",
                "CWE F1": f"{all_metric['CWE F1']:.3f}",
                "Function F1": f"{all_metric['Function F1']:.3f}",
                "Line F1": f"{all_metric['Line F1']:.3f}",
                "Hit@1-Function": f"{vuln_metric['Hit@1-Function']:.3f} ({vuln_metric['Hit@1-Function Hits']}/{vuln_metric['Hit@1-Function Total']})",
            }
        )

    table_df = pd.DataFrame(metrics_list)
    table_df = table_df.sort_values(
        "Hit@1-Function",
        ascending=False,
        key=lambda x: x.str.extract("([0-9.]+)")[0].astype(float),
    )

    table_path = tables_dir / "table_7_1_vanilla_performance.csv"
    table_df.to_csv(table_path, index=False)


if __name__ == "__main__":
    main()
