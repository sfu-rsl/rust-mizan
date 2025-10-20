#!/usr/bin/env python3

import json
import csv
import matplotlib.pyplot as plt
import seaborn as sns
from pathlib import Path

INPUT_FILE = "/Users/tareknasser/Documents/workspace/sfu/mizan/rust-mizan/evaluation/LLMs/mizan-datasets/mizan-format-expanded.json"
DATASET_NAME = "mizan-format-expanded"
OUTPUT_CSV = f"evaluation/LLMs/scoring_and_figures/scripts/{DATASET_NAME}_prompt_token_distribution.csv"
OUTPUT_FIGURE = f"evaluation/LLMs/scoring_and_figures/scripts/{DATASET_NAME}_prompt_token_distribution.png"


def estimate_tokens(text: str) -> int:
    """Estimate tokens: 100 tokens ~= 75 words"""
    return int(len(text.split()) * 4 / 3)


def main():
    with open(INPUT_FILE, "r", encoding="utf-8") as f:
        data = json.load(f)

    results = [
        (example["metadata"]["id"], estimate_tokens(example["inputs"]["prompt"]))
        for example in data["examples"]
    ]

    Path(OUTPUT_CSV).parent.mkdir(parents=True, exist_ok=True)
    with open(OUTPUT_CSV, "w", newline="", encoding="utf-8") as f:
        writer = csv.writer(f)
        writer.writerow(["example_id", "token_count"])
        writer.writerows(results)

    tokens = [count for _, count in results]

    sns.set_style("whitegrid")
    fig, (ax1, ax2) = plt.subplots(2, 1, figsize=(12, 10))

    ax1.hist(tokens, bins=30, color="steelblue", edgecolor="black", alpha=0.7)
    ax1.set_xlabel("Token Count", fontsize=12)
    ax1.set_ylabel("Frequency", fontsize=12)
    ax1.set_title(
        f"Token Distribution - {DATASET_NAME}.json", fontsize=14, fontweight="bold"
    )
    ax1.grid(True, alpha=0.3)

    stats_text = f"Total Examples: {len(tokens)}\n"
    stats_text += f"Mean: {sum(tokens)/len(tokens):.1f}\n"
    stats_text += f"Median: {sorted(tokens)[len(tokens)//2]}\n"
    stats_text += f"Min: {min(tokens)}\n"
    stats_text += f"Max: {max(tokens)}"
    ax1.text(
        0.98,
        0.97,
        stats_text,
        transform=ax1.transAxes,
        verticalalignment="top",
        horizontalalignment="right",
        bbox=dict(boxstyle="round", facecolor="wheat", alpha=0.5),
        fontsize=10,
        fontfamily="monospace",
    )

    ax2.boxplot(tokens, vert=False, widths=0.5)
    ax2.set_xlabel("Token Count", fontsize=12)
    ax2.set_title("Token Distribution Box Plot", fontsize=14, fontweight="bold")
    ax2.grid(True, alpha=0.3, axis="x")

    plt.tight_layout()
    plt.savefig(OUTPUT_FIGURE, dpi=300, bbox_inches="tight")
    plt.close()


if __name__ == "__main__":
    main()
