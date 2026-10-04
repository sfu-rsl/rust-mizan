import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt
import matplotlib.ticker as mticker
import numpy as np

from common import MODEL_ORDER, NAMES, OUT, load_logs, micro

OUT_DIR = str(OUT)
logs = load_logs()
LINE_LOCALIZATION_F1 = {
    NAMES[model]: [micro([r for r in logs[("vanilla", model)].values()
                        if r["granularity"] == level], "line")[2]
                   for level in ("crate", "file", "function")]
    for model in MODEL_ORDER
}
import os

CONTEXT_LEVELS = ["Crate", "File", "Function"]

FIGURES = [
    {
        "data": LINE_LOCALIZATION_F1,
        "ylabel": "Line Localization F1 (%)",
        "output": os.path.join(OUT_DIR, "line_localization_by_level.pdf"),
    },
]

# ╔══════════════════════════════════════════════════════════════════╗
# ║                       STYLE CONFIGURATION                       ║
# ╚══════════════════════════════════════════════════════════════════╝

LEVEL_COLORS = ["#A8B6CC", "#5B8DBE", "#1B4F72"]  # light → dark
FIG_WIDTH = 10
FIG_HEIGHT = 3.8
BAR_WIDTH = 0.22
Y_MAX = 58
LABEL_FONT = 11


# ════════════════════════════════════════════════════════════════════


def make_figure(data, ylabel, output_path):
    fig, ax = plt.subplots(figsize=(FIG_WIDTH, FIG_HEIGHT))

    models = list(data.keys())
    x = np.arange(len(models))
    offsets = [-BAR_WIDTH, 0, BAR_WIDTH]

    crate_vals = [data[m][0] for m in models]
    file_vals = [data[m][1] for m in models]
    func_vals = [data[m][2] for m in models]

    for i, (level, vals) in enumerate(
        zip(CONTEXT_LEVELS, [crate_vals, file_vals, func_vals])
    ):
        bars = ax.bar(
            x + offsets[i],
            vals,
            BAR_WIDTH,
            label=level,
            color=LEVEL_COLORS[i],
            edgecolor="white",
            linewidth=0.6,
        )
        for bar, v in zip(bars, vals):
            ax.text(
                bar.get_x() + bar.get_width() / 2,
                bar.get_height() + 0.8,
                f"{v:.1f}",
                ha="center",
                va="bottom",
                fontsize=LABEL_FONT,
                color="#333333",
            )

    ax.set_xticks(x)
    ax.set_xticklabels(models, fontsize=13)
    ax.set_ylabel(ylabel, fontsize=14)
    ax.set_ylim(0, Y_MAX)

    ax.legend(
        fontsize=12,
        loc="lower center",
        bbox_to_anchor=(0.5, 1.02),
        ncol=3,
        framealpha=0.9,
        edgecolor="#cccccc",
    )
    ax.spines["top"].set_visible(False)
    ax.spines["right"].set_visible(False)
    ax.yaxis.set_major_locator(mticker.MultipleLocator(10))
    ax.grid(axis="y", alpha=0.3, linestyle="--")
    ax.tick_params(axis="both", labelsize=12)

    fig.tight_layout()
    fig.savefig(output_path, dpi=300, bbox_inches="tight")
    plt.close()
    print(f"Saved → {output_path}")


def main():
    for spec in FIGURES:
        make_figure(spec["data"], spec["ylabel"], spec["output"])


if __name__ == "__main__":
    main()
