import sys
import argparse
from pathlib import Path

sys.path.append(str(Path(__file__).parent.parent))

from common.data_utils import (
    load_processed_experiment,
    load_experiments_catalog,
    find_common_samples,
    get_short_model_name,
)
from common.metrics import compute_success_at_1_line_rate

LATEX_TEMPLATE = r"""\begin{tabular}{lc}
\toprule
\textbf{Transformation} & \textbf{MODEL_NAME} \\
\midrule
{TABLE_ROWS}
\bottomrule
\end{tabular}
"""

TRANSFORMATIONS = [
    ("vanilla", "vanilla (baseline)"),
    ("benign", "Benign (all)"),
    ("malignant", "Malignant (all)"),
]


def main():
    parser = argparse.ArgumentParser(description="Generate transformation impact table for a specific model")
    parser.add_argument("--model", default="Qwen2.5-7B-Instruct", help="Model name from experiments.json (default: Qwen2.5-7B-Instruct)")
    args = parser.parse_args()

    experiments_catalog = load_experiments_catalog()
    model_experiments = experiments_catalog[args.model]

    # Collect experiment IDs
    experiment_ids = [
        model_experiments[key] for key, _ in TRANSFORMATIONS if model_experiments[key] != "na"
    ]

    # Find common samples
    common_samples = find_common_samples(experiment_ids)

    # Build table rows
    table_rows = []
    for trans_key, trans_display in TRANSFORMATIONS:
        if trans_key == "benign":
            table_rows.append("\\midrule")

        exp_id = model_experiments[trans_key]
        df = load_processed_experiment(exp_id)
        df_valid = df[df["is_valid_json"] == True]
        df_common = df_valid[df_valid["example_id"].isin(common_samples)]
        df_vuln = df_common[df_common["is_vulnerable_gt"] == True]

        score = compute_success_at_1_line_rate(df_common)
        hits = int(df_vuln["success_at_1_line"].sum())
        total = len(df_vuln)

        table_rows.append(f"{trans_display} & {score*100:.1f}\\% ({hits}/{total}) \\\\")

    # Generate and save LaTeX
    short_name = get_short_model_name(args.model)
    final_content = LATEX_TEMPLATE.replace("MODEL_NAME", short_name).replace("{TABLE_ROWS}", "\n".join(table_rows))

    output_filename = f"{args.model.lower().replace('.', '_').replace('-', '_')}_transformation_impact.tex"
    output_path = Path(__file__).parent.parent / "latex" / output_filename
    output_path.parent.mkdir(exist_ok=True)
    output_path.write_text(final_content, encoding="utf-8")


if __name__ == "__main__":
    main()
