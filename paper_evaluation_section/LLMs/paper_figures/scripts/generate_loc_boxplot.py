import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns
from pathlib import Path
import matplotlib.ticker as ticker

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

    df = pd.read_csv(data_dir / "dataset_samples_detailed.csv")

    fig, ax = plt.subplots(figsize=(10, 5))

    granularity_order = ["function", "file", "crate"]
    granularity_labels = ["Function", "File", "Crate"]
    plot_data = df[df["granularity"].isin(granularity_order)].copy()

    # Create box plot with single color
    palette = plt.get_cmap("tab10")
    box_plot = sns.boxplot(
        data=plot_data,
        x="granularity",
        y="lines_of_code",
        order=granularity_order,
        ax=ax,
        color=palette(0),
        linewidth=1.2,
    )

    ax.set_xlabel("Granularity", fontweight="bold", fontsize=16)
    ax.set_ylabel("Lines of Code", fontweight="bold", fontsize=16)
    ax.set_title(
        "Code Sample Size Distribution by Granularity",
        fontweight="bold",
        fontsize=18,
        pad=10,
    )
    ax.set_yscale("log")
    
    # Custom y-axis formatter to show 100, 1000, 10000 instead of 10^2, 10^3, 10^4
    ax.yaxis.set_major_formatter(ticker.FuncFormatter(lambda y, _: f'{int(y):,}'))
    
    ax.grid(True, alpha=0.3, axis="y", linestyle="-", linewidth=0.5)
    ax.set_xticklabels(granularity_labels)

    plt.tight_layout()

    for fmt in ["pdf", "png", "eps", "svg"]:
        plt.savefig(
            figures_dir / f"loc_distribution_boxplot.{fmt}",
            format=fmt,
            dpi=300,
            bbox_inches="tight",
            facecolor="white",
        )

    plt.close()


if __name__ == "__main__":
    main()
