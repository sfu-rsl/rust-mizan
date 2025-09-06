import sys
import pandas as pd
import matplotlib.pyplot as plt
import numpy as np
from pathlib import Path

sys.path.append(str(Path(__file__).parent.parent))

from common.data_utils import (
    get_vanilla_experiment_ids,
    get_short_model_name,
    get_ordered_models,
)
from common.metrics import load_experiment_data, compute_micro_averaged_f1

plt.style.use("default")
plt.rcParams.update(
    {
        "font.size": 12,
        "font.family": "serif",
        "font.serif": ["Times New Roman", "DejaVu Serif"],
        "axes.labelsize": 13,
        "axes.titlesize": 14,
        "xtick.labelsize": 12,
        "ytick.labelsize": 12,
        "legend.fontsize": 11,
        "figure.titlesize": 16,
        "axes.linewidth": 0.8,
        "grid.alpha": 0.3,
        "grid.linewidth": 0.5,
        "pdf.fonttype": 42,
        "ps.fonttype": 42,
        "svg.fonttype": "none",
    }
)


def main():
    current_dir = Path(__file__).parent.parent
    figures_dir = current_dir / "figures"
    figures_dir.mkdir(exist_ok=True)

    vanilla_experiments = get_vanilla_experiment_ids()
    experiment_ids = list(vanilla_experiments.values())
    model_names = list(vanilla_experiments.keys())

    experiment_data = load_experiment_data(experiment_ids, model_names)

    granularity_data = []
    for model_name, df in experiment_data.items():
        for granularity in ["crate", "file", "function"]:
            gran_df = df[df["granularity"] == granularity]
            if len(gran_df) > 0:
                aggregated_f1 = compute_micro_averaged_f1(gran_df, "function")
                granularity_data.append(
                    {
                        "Model": get_short_model_name(model_name),
                        "Granularity": granularity,
                        "Micro-averaged Vulnerable Function F1": aggregated_f1,
                    }
                )

    granularity_df = pd.DataFrame(granularity_data)

    fig, ax = plt.subplots(figsize=(12, 4))
    palette = plt.get_cmap("tab10")
    models = get_ordered_models(granularity_df["Model"].unique())
    granularities = ["crate", "file", "function"]
    hatches = ["///", "...", "xxx"]

    x = np.arange(len(models))
    width = 0.25

    for i, granularity in enumerate(granularities):
        values = []
        for model in models:
            model_data = granularity_df[
                (granularity_df["Model"] == model)
                & (granularity_df["Granularity"] == granularity)
            ]
            values.append(
                model_data["Micro-averaged Vulnerable Function F1"].iloc[0]
                if len(model_data) > 0
                else 0
            )

        bars = ax.bar(
            x + i * width,
            values,
            width,
            label=granularity.capitalize(),
            color=palette(i),
            alpha=0.7,
            edgecolor="black",
            linewidth=0.5,
            hatch=hatches[i],
        )

        for bar, value in zip(bars, values):
            if value > 0:
                ax.text(
                    bar.get_x() + bar.get_width() / 2.0,
                    bar.get_height() + 0.005,
                    f"{value*100:.1f}%",
                    ha="center",
                    va="bottom",
                    fontsize=11,
                    fontweight="bold",
                )

    ax.set_xlabel("Model", fontsize=13)
    ax.set_ylabel("Vuln. Func. Localization Micro F1 (%)", fontsize=13)
    ax.set_title(
        "Vulnerable Function Localization Across Code Levels",
        fontsize=14,
        pad=15,
    )
    ax.set_xticks(x + width)
    ax.set_xticklabels(models)
    ax.set_ylim(0.0, 0.4)
    ax.yaxis.set_major_formatter(plt.FuncFormatter(lambda y, _: f"{y*100:.0f}%"))

    legend = ax.legend(
        title="Code Level",
        loc="upper left",
        frameon=True,
        fancybox=True,
        shadow=False,
        bbox_to_anchor=(0.02, 0.98),
        fontsize=10,
        title_fontsize=10,
    )
    legend.get_frame().set_facecolor("white")
    legend.get_frame().set_alpha(0.9)
    legend.get_frame().set_edgecolor("lightgray")

    ax.grid(True, alpha=0.3, axis="y", linestyle="-", linewidth=0.5)
    ax.set_axisbelow(True)
    ax.spines["top"].set_visible(False)
    ax.spines["right"].set_visible(False)
    ax.spines["left"].set_linewidth(0.8)
    ax.spines["bottom"].set_linewidth(0.8)

    plt.tight_layout()
    base_name = "performance_by_level"

    plt.savefig(
        figures_dir / f"{base_name}.png",
        dpi=300,
        bbox_inches="tight",
        facecolor="white",
    )
    plt.savefig(
        figures_dir / f"{base_name}.pdf",
        format="pdf",
        bbox_inches="tight",
        facecolor="white",
    )
    plt.savefig(
        figures_dir / f"{base_name}.eps",
        format="eps",
        bbox_inches="tight",
        facecolor="white",
    )
    plt.savefig(
        figures_dir / f"{base_name}.svg",
        format="svg",
        bbox_inches="tight",
        facecolor="white",
    )

    plt.close()


if __name__ == "__main__":
    main()
