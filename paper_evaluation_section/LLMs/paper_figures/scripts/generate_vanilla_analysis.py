import sys
from pathlib import Path

sys.path.append(str(Path(__file__).parent.parent))

from common.data_utils import get_vanilla_experiment_ids, get_ordered_models
from common.metrics import load_experiment_data, compute_aggregate_metrics


def generate_table_rows(metrics_data):
    metrics_order = [
        ("Binary Accuracy", "Binary Accuracy"),
        ("CWE F1", "CWE F1"),
        ("Function F1", "Func. Localization F1"),
        ("Line F1", "Line Localization F1"),
        ("Success@1-Function", "Success@1-Func"),
        ("Success@1-Line", "Success@1-Line"),
    ]

    table_rows = []
    for metric_key, metric_display in metrics_order:
        row_parts = [metric_display]

        for model_data in metrics_data:
            value = model_data[metric_key]
            row_parts.append(value)

        table_rows.append(" & ".join(row_parts) + " \\\\")

    return "\n".join(table_rows)


def main():
    current_dir = Path(__file__).parent.parent
    latex_dir = current_dir / "latex"
    latex_dir.mkdir(exist_ok=True)

    vanilla_experiments = get_vanilla_experiment_ids()
    experiment_ids = list(vanilla_experiments.values())
    model_names = list(vanilla_experiments.keys())

    experiment_data = load_experiment_data(experiment_ids, model_names)
    all_metrics = compute_aggregate_metrics(experiment_data, vulnerable_only=False)
    vuln_metrics = compute_aggregate_metrics(experiment_data, vulnerable_only=True)

    metrics_list = []
    for i, all_metric in enumerate(all_metrics):
        vuln_metric = vuln_metrics[i]
        metrics_list.append(
            {
                "Model": all_metric["Model"],
                "Binary Accuracy": f"{all_metric['Binary Accuracy']:.3f}",
                "CWE F1": f"{all_metric['CWE F1']:.3f}",
                "Function F1": f"{all_metric['Function F1']:.3f}",
                "Line F1": f"{all_metric['Line F1']:.3f}",
                "Success@1-Function": f"{vuln_metric['Success@1-Function']*100:.1f}\\% ({vuln_metric['Success@1-Function Hits']}/{vuln_metric['Success@1-Function Total']})",
                "Success@1-Line": f"{vuln_metric['Success@1-Line']*100:.1f}\\% ({vuln_metric['Success@1-Line Hits']}/{vuln_metric['Success@1-Line Total']})",
            }
        )

    ordered_metrics = []
    available_models = [m["Model"] for m in metrics_list]
    for model in get_ordered_models(available_models):
        for metric in metrics_list:
            if metric["Model"] == model:
                ordered_metrics.append(metric)
                break

    template_path = current_dir / "latex" / "TEMPLATE_vanilla_performance.tex"
    with open(template_path, "r", encoding="utf-8") as f:
        template_content = f.read()

    table_rows = generate_table_rows(ordered_metrics)
    final_content = template_content.replace("{TABLE_ROWS}", table_rows)

    output_path = latex_dir / "vanilla_performance_generated.tex"
    with open(output_path, "w", encoding="utf-8") as f:
        f.write(final_content)


if __name__ == "__main__":
    main()
