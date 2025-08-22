import sys
import pandas as pd
import matplotlib.pyplot as plt
from pathlib import Path

sys.path.append(str(Path(__file__).parent.parent))

from common.data_utils import get_vanilla_experiment_ids
from common.metrics import (
    compute_experiment_metrics,
    compute_aggregate_f1_from_dataframe,
)


def main():
    current_dir = Path(__file__).parent.parent
    figures_dir = current_dir / "figures"
    figures_dir.mkdir(exist_ok=True)

    vanilla_experiments = get_vanilla_experiment_ids()

    # Define model order mapping: GPT, Gemini, Claude, DeepSeek
    model_order_mapping = {
        "gpt-4.1-2025-04-14": "GPT-4.1",
        "gemini-1.5-pro": "Gemini 1.5 Pro",
        "claude-3-7-sonnet-20250219": "Claude 3.7 Sonnet",
        "deepseek-chat": "DeepSeek-V3.1",
    }

    # Reorder based on desired display order
    desired_display_order = [
        "GPT-4.1",
        "Gemini 1.5 Pro",
        "Claude 3.7 Sonnet",
        "DeepSeek-V3.1",
    ]

    # Find the original keys that map to our desired order
    experiment_ids = []
    model_names = []
    for display_name in desired_display_order:
        for orig_key, mapped_name in model_order_mapping.items():
            if mapped_name == display_name and orig_key in vanilla_experiments:
                model_names.append(orig_key)
                experiment_ids.append(vanilla_experiments[orig_key])
                break

    # Get common samples and compute metrics for all experiments
    experiment_data = compute_experiment_metrics(experiment_ids, model_names)

    # Generate Figure 7.1
    granularity_data = []

    for model_name, df in experiment_data.items():
        # Map to display name - need to handle the shortened names from compute_experiment_metrics
        if model_name == "Claude 3.7":
            display_name = "Claude 3.7 Sonnet"
        elif model_name == "DeepSeek":
            display_name = "DeepSeek-V3.1"
        else:
            display_name = model_name

        for granularity in ["crate", "file", "function"]:
            gran_df = df[df["granularity"] == granularity]
            if len(gran_df) > 0:
                # Use common function to compute aggregate F1 from filtered DataFrame
                aggregated_f1 = compute_aggregate_f1_from_dataframe(gran_df, "function")

                granularity_data.append(
                    {
                        "Model": display_name,
                        "Granularity": granularity,
                        "Macro Vulnerable Function F1": aggregated_f1,
                    }
                )

    granularity_df = pd.DataFrame(granularity_data)

    # Create bar plot
    fig, ax = plt.subplots(figsize=(10, 6))

    models = desired_display_order
    granularities = ["crate", "file", "function"]

    x = range(len(models))
    width = 0.25

    for i, granularity in enumerate(granularities):
        values = []
        for model in models:
            model_data = granularity_df[
                (granularity_df["Model"] == model)
                & (granularity_df["Granularity"] == granularity)
            ]
            if len(model_data) > 0:
                values.append(model_data["Macro Vulnerable Function F1"].iloc[0])
            else:
                values.append(0)

        ax.bar(
            [xi + i * width for xi in x], values, width, label=granularity.capitalize()
        )

    ax.set_xlabel("Model", fontweight="bold")
    ax.set_ylabel("Macro Vulnerable Function F1", fontweight="bold")
    ax.set_title("Performance by Code Granularity", fontsize=14, fontweight="bold")
    ax.set_xticks([xi + width for xi in x])
    ax.set_xticklabels(models)
    ax.set_ylim(0.0, 1.0)
    ax.legend(title="Granularity")
    ax.grid(True, alpha=0.3)

    plt.tight_layout()
    output_path = figures_dir / "figure_7_1_performance_by_granularity.png"
    plt.savefig(output_path, dpi=300, bbox_inches="tight")
    plt.close()


if __name__ == "__main__":
    main()
