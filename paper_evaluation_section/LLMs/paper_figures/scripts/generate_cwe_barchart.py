import pandas as pd
import matplotlib.pyplot as plt
from pathlib import Path

plt.style.use("default")
plt.rcParams.update(
    {
        "font.size": 14,
        "font.family": "serif",
        "font.serif": ["Times New Roman", "DejaVu Serif"],
        "axes.labelsize": 16,
        "axes.titlesize": 18,
        "xtick.labelsize": 14,
        "ytick.labelsize": 14,
        "legend.fontsize": 14,
        "figure.titlesize": 20,
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
    data_dir = current_dir / "data"
    figures_dir = current_dir / "figures"
    figures_dir.mkdir(exist_ok=True)

    df = pd.read_csv(data_dir / "dataset_statistics_comprehensive.csv")
    stats = dict(zip(df["metric"], df["value"]))

    cwe_ids = []
    counts = []
    for i in range(1, 6):
        cwe_ids.append(stats[f"most_common_cwe_{i}"])
        counts.append(int(stats[f"most_common_cwe_{i}_count"]))

    fig, ax = plt.subplots(figsize=(10, 7))

    palette = plt.get_cmap("tab10")
    bars = ax.bar(
        range(len(cwe_ids)),
        counts,
        color=[palette(i) for i in range(len(cwe_ids))],
        alpha=0.7,
        edgecolor="black",
        linewidth=0.5,
    )

    ax.set_xlabel("CWE Types", fontweight="bold", fontsize=16)
    ax.set_ylabel("Number of Code Samples", fontweight="bold", fontsize=16)
    ax.set_title("Top 5 Most Common CWE Types", fontweight="bold", fontsize=18, pad=20)
    ax.set_xticks(range(len(cwe_ids)))
    ax.set_xticklabels(cwe_ids)
    ax.grid(True, alpha=0.3, axis="y", linestyle="-", linewidth=0.5)

    # Fix text positioning to avoid overflow
    max_count = max(counts)
    for bar, count in zip(bars, counts):
        height = bar.get_height()
        ax.text(
            bar.get_x() + bar.get_width() / 2.0,
            height + max_count * 0.02,
            str(count),
            ha="center",
            va="bottom",
            fontsize=12,
        )

    # Add some padding to y-axis to accommodate text
    ax.set_ylim(0, max_count * 1.15)

    plt.tight_layout()

    for fmt in ["pdf", "png", "eps", "svg"]:
        plt.savefig(
            figures_dir / f"top_cwe_distribution.{fmt}",
            format=fmt,
            dpi=300,
            bbox_inches="tight",
            facecolor="white",
        )

    plt.close()


if __name__ == "__main__":
    main()
