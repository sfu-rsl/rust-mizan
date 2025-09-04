import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns
from pathlib import Path

plt.rcParams.update({
    "font.size": 14,
    "font.family": "serif",
    "axes.linewidth": 0.8,
    "pdf.fonttype": 42,
    "ps.fonttype": 42,
})


def main():
    current_dir = Path(__file__).parent.parent
    data_dir = current_dir / "data"
    figures_dir = current_dir / "figures"
    figures_dir.mkdir(exist_ok=True)

    df = pd.read_csv(data_dir / "dataset_samples_detailed.csv")
    
    fig, ax = plt.subplots(figsize=(10, 4))

    granularity_order = ["function", "file", "crate"]
    granularity_labels = ["Function", "File", "Crate"]
    plot_data = df[df["granularity"].isin(granularity_order)].copy()

    sns.boxplot(
        data=plot_data,
        y="granularity",
        x="lines_of_code",
        order=granularity_order,
        ax=ax,
    )

    ax.set_ylabel("Granularity", fontweight="bold")
    ax.set_xlabel("Lines of Code", fontweight="bold")
    ax.set_title("Code Sample Size Distribution by Granularity", fontweight="bold", pad=20)
    ax.set_yticklabels(granularity_labels)
    ax.grid(True, alpha=0.3, axis="x")

    plt.tight_layout()

    for fmt in ["pdf", "png", "eps", "svg"]:
        plt.savefig(figures_dir / f"loc_distribution_boxplot.{fmt}", format=fmt, dpi=300, bbox_inches="tight")

    plt.close()


if __name__ == "__main__":
    main()
