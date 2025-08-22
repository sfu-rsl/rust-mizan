import sys
import pandas as pd
from pathlib import Path

sys.path.append(str(Path(__file__).parent.parent))

from common.data_utils import get_vanilla_experiment_ids, get_ordered_models
from common.metrics import compute_experiment_metrics, compute_aggregate_metrics


def generate_latex_table(metrics_data):
    latex_content = [
        "\\begin{table}[htbp]",
        "\\centering", 
        "\\caption{Overall LLM performance on vanilla RustMizan dataset}",
        "\\label{tab:vanilla_performance}",
        "\\begin{tabular}{lccccc}",
        "\\toprule",
        "\\textbf{Model} & \\textbf{Binary Acc.} & \\textbf{CWE F1} & \\textbf{Function F1} & \\textbf{Line F1} & \\textbf{Hit@1-Func} \\\\",
        "\\midrule"
    ]
    
    for metric in metrics_data:
        row = f"{metric['Model']} & {metric['Binary Accuracy']} & {metric['CWE F1']} & {metric['Function F1']} & {metric['Line F1']} & {metric['Hit@1-Function']} \\\\"
        latex_content.append(row)
    
    latex_content.extend([
        "\\bottomrule",
        "\\end{tabular}",
        "\\end{table}"
    ])
    
    return "\n".join(latex_content)


def main():
    current_dir = Path(__file__).parent.parent
    latex_dir = current_dir / "latex"
    latex_dir.mkdir(exist_ok=True)

    vanilla_experiments = get_vanilla_experiment_ids()
    experiment_ids = list(vanilla_experiments.values())
    model_names = list(vanilla_experiments.keys())

    experiment_data = compute_experiment_metrics(experiment_ids, model_names)
    all_metrics = compute_aggregate_metrics(experiment_data, vulnerable_only=False)
    vuln_metrics = compute_aggregate_metrics(experiment_data, vulnerable_only=True)

    metrics_list = []
    for i, all_metric in enumerate(all_metrics):
        vuln_metric = vuln_metrics[i]
        metrics_list.append({
            "Model": all_metric["Model"],
            "Binary Accuracy": f"{all_metric['Binary Accuracy']:.3f}",
            "CWE F1": f"{all_metric['CWE F1']:.3f}",
            "Function F1": f"{all_metric['Function F1']:.3f}",
            "Line F1": f"{all_metric['Line F1']:.3f}",
            "Hit@1-Function": f"{vuln_metric['Hit@1-Function']:.3f} ({vuln_metric['Hit@1-Function Hits']}/{vuln_metric['Hit@1-Function Total']})"
        })

    ordered_metrics = []
    available_models = [m["Model"] for m in metrics_list]
    for model in get_ordered_models(available_models):
        for metric in metrics_list:
            if metric["Model"] == model:
                ordered_metrics.append(metric)
                break

    latex_content = generate_latex_table(ordered_metrics)
    output_path = latex_dir / "vanilla_performance_generated.tex"
    with open(output_path, "w") as f:
        f.write(latex_content)


if __name__ == "__main__":
    main()
