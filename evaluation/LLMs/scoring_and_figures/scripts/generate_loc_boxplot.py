import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns
from pathlib import Path

plt.rcParams.update(
    {
        "font.size": 11,
        "font.family": "serif",
        "axes.linewidth": 0.8,
        "pdf.fonttype": 42,
        "ps.fonttype": 42,
    }
)


def main():
    current_dir = Path(__file__).parent.parent
    data_dir = current_dir / "data"
    figures_dir = current_dir / "figures"
    figures_dir.mkdir(exist_ok=True)

    df = pd.read_csv(data_dir / "dataset_samples_detailed.csv")

    # Create figure with two subplots side by side with equal width
    fig, (ax1, ax2) = plt.subplots(
        1, 2, figsize=(12, 2.76), gridspec_kw={"width_ratios": [1, 1]}
    )

    # Left plot: Boxplot
    granularity_order = ["function", "file", "crate"]
    granularity_labels = ["Function", "File", "Crate"]
    plot_data = df[df["granularity"].isin(granularity_order)].copy()

    sns.boxplot(
        data=plot_data,
        y="granularity",
        x="lines_of_code",
        order=granularity_order,
        ax=ax1,
    )

    ax1.set_ylabel("Level", fontsize=10)
    ax1.set_xlabel("Lines of Code", fontsize=10)
    ax1.set_title("Code Variant Size Distribution by Level", fontsize=11, pad=15)
    ax1.set_yticklabels(granularity_labels, fontsize=9)
    ax1.tick_params(axis="x", labelsize=9)
    ax1.grid(True, alpha=0.3, axis="x")

    # Right plot: Table with yearly stats
    yearly_stats = pd.DataFrame(
        {
            "Year": [2018, 2019, 2020, 2021, 2023, 2025],
            "# Vulnerabilities": [2, 4, 28, 1, 1, 6],
            "# Code Variants": [10, 16, 109, 6, 4, 28],
        }
    )

    # Hide axes for table
    ax2.axis("tight")
    ax2.axis("off")

    # Create table
    table = ax2.table(
        cellText=yearly_stats.values,
        colLabels=yearly_stats.columns,
        cellLoc="center",
        loc="center",
        colWidths=[0.25, 0.375, 0.375],
    )

    # Style the table
    table.auto_set_font_size(False)
    table.set_fontsize(10)
    table.scale(1.2, 1.8)

    # Style header row
    for i in range(len(yearly_stats.columns)):
        table[(0, i)].set_facecolor("#E8E8E8")
        table[(0, i)].set_text_props(weight="bold")

    # Add title for the table
    ax2.set_title("Vulnerability Distribution by Year", fontsize=11, pad=15)

    plt.tight_layout()

    for fmt in ["pdf", "png", "eps", "svg"]:
        plt.savefig(
            figures_dir / f"loc_and_yearly_distribution.{fmt}",
            format=fmt,
            dpi=300,
            bbox_inches="tight",
        )

    plt.close()


if __name__ == "__main__":
    main()
