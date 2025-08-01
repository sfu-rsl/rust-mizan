#!/usr/bin/env python3
import json
import os
import re
from collections import Counter, defaultdict
from pathlib import Path
from typing import Dict, Any

import matplotlib.pyplot as plt
import numpy as np
import pandas as pd


class RustMizanDatasetAnalyzer:
    """Analyzes the RustMizan dataset and generates comprehensive statistics."""

    def __init__(self, dataset_path: str):
        """Initialize analyzer with dataset path."""
        self.dataset_path = dataset_path
        self.output_dir = Path("dataset_analysis")
        self.figures_dir = self.output_dir / "figures"
        self.dataset = None
        self.markdown_content = []

        # Create output directories
        self.output_dir.mkdir(exist_ok=True)
        self.figures_dir.mkdir(exist_ok=True)

        # Set up matplotlib style
        plt.style.use("default")
        # Use simple, consistent colors
        self.colors = [
            "#1f77b4",
            "#ff7f0e",
            "#2ca02c",
            "#d62728",
            "#9467bd",
            "#8c564b",
            "#e377c2",
            "#7f7f7f",
        ]

    def load_dataset(self):
        """Load the mizan.json dataset."""
        print(f"Loading dataset from {self.dataset_path}...")
        with open(self.dataset_path, "r") as f:
            self.dataset = json.load(f)
        print(
            f"✓ Dataset loaded: {len(self.dataset['vulnerabilities'])} vulnerabilities"
        )

    def count_rust_lines(self, file_path: str) -> int:
        """Count non-empty, non-comment lines in a Rust file."""
        try:
            with open(file_path, "r", encoding="utf-8") as f:
                content = f.read()

            # Remove multi-line comments
            content = re.sub(r"/\*.*?\*/", "", content, flags=re.DOTALL)

            lines = content.split("\n")
            rust_lines = 0

            for line in lines:
                line = line.strip()
                # Skip empty lines and single-line comments
                if line and not line.startswith("//"):
                    rust_lines += 1

            return rust_lines
        except Exception as e:
            print(f"Warning: Could not read {file_path}: {e}")
            return 0

    def analyze_code_sample_metrics(self, sample_path: str) -> Dict[str, Any]:
        """Analyze metrics for a single code sample."""
        base_path = Path(self.dataset_path).parent / "samples" / sample_path

        if not base_path.exists():
            return {
                "total_lines": 0,
                "total_files": 0,
                "rust_files": 0,
                "cargo_toml_exists": False,
            }

        metrics = {
            "total_lines": 0,
            "total_files": 0,
            "rust_files": 0,
            "cargo_toml_exists": (base_path / "Cargo.toml").exists(),
        }

        # Count Rust files and lines
        for rust_file in base_path.rglob("*.rs"):
            if rust_file.is_file():
                metrics["rust_files"] += 1
                metrics["total_files"] += 1
                metrics["total_lines"] += self.count_rust_lines(str(rust_file))

        # Count other files
        for other_file in base_path.rglob("*"):
            if other_file.is_file() and not other_file.name.endswith(".rs"):
                metrics["total_files"] += 1

        return metrics

    def extract_granularity(self, sample_path: str) -> str:
        """Extract granularity level from sample path."""
        if "-function" in sample_path:
            return "function"
        elif "-file" in sample_path:
            return "file"
        elif "-crate" in sample_path:
            return "crate"
        else:
            return "unknown"

    def analyze_basic_statistics(self):
        """Analyze basic dataset statistics."""
        print("Analyzing basic dataset statistics...")

        vulnerabilities = self.dataset["vulnerabilities"]

        # Basic counts
        total_vulnerabilities = len(vulnerabilities)
        total_samples = sum(len(vuln["code_samples"]) for vuln in vulnerabilities)
        vulnerable_samples = sum(
            1
            for vuln in vulnerabilities
            for sample in vuln["code_samples"]
            if sample["is_vulnerability"]
        )
        fixed_samples = total_samples - vulnerable_samples

        # Year analysis
        years = [vuln["year"] for vuln in vulnerabilities]
        year_range = f"{min(years)}-{max(years)}"

        # Author analysis
        authors = [vuln["author"] for vuln in vulnerabilities]
        unique_authors = len(set(authors))

        # CWE analysis
        all_cwes = []
        for vuln in vulnerabilities:
            for sample in vuln["code_samples"]:
                all_cwes.extend(sample["cwe_type"])
        unique_cwes = len(set(all_cwes))

        # Store basic statistics
        self.basic_stats = {
            "total_vulnerabilities": total_vulnerabilities,
            "total_samples": total_samples,
            "vulnerable_samples": vulnerable_samples,
            "fixed_samples": fixed_samples,
            "year_range": year_range,
            "unique_authors": unique_authors,
            "unique_cwes": unique_cwes,
            "years": years,
            "authors": authors,
            "all_cwes": all_cwes,
        }

        print(
            f"✓ Basic statistics completed: {total_vulnerabilities} vulnerabilities, {total_samples} samples"
        )

    def analyze_vulnerabilities(self):
        """Analyze vulnerability-specific statistics."""
        print("Analyzing vulnerability patterns...")

        vulnerabilities = self.dataset["vulnerabilities"]

        # CWE distribution
        cwe_counter = Counter(self.basic_stats["all_cwes"])

        # Year distribution
        year_counter = Counter(self.basic_stats["years"])

        # Source link analysis
        source_patterns = defaultdict(int)
        for vuln in vulnerabilities:
            source = vuln.get("source_link", "")
            if "cve.mitre.org" in source:
                source_patterns["CVE"] += 1
            elif "github.com" in source:
                source_patterns["GitHub"] += 1
            elif "rustsec.org" in source:
                source_patterns["RustSec"] += 1
            else:
                source_patterns["Other"] += 1

        # Crate analysis
        crate_names = [vuln["crate_name"] for vuln in vulnerabilities]
        crate_counter = Counter(crate_names)

        self.vuln_stats = {
            "cwe_distribution": dict(cwe_counter),
            "year_distribution": dict(year_counter),
            "source_patterns": dict(source_patterns),
            "crate_distribution": dict(crate_counter),
        }

        print(f"✓ Vulnerability analysis completed: {len(cwe_counter)} unique CWEs")

    def analyze_code_samples(self):
        """Analyze code sample characteristics."""
        print("Analyzing code samples...")

        sample_data = []
        granularity_metrics = defaultdict(list)

        for vuln in self.dataset["vulnerabilities"]:
            for sample in vuln["code_samples"]:
                granularity = self.extract_granularity(sample["path_to_crate"])

                # Get code metrics
                metrics = self.analyze_code_sample_metrics(sample["path_to_crate"])

                sample_info = {
                    "vuln_id": vuln["id"],
                    "sample_path": sample["path_to_crate"],
                    "is_vulnerable": sample["is_vulnerability"],
                    "granularity": granularity,
                    "cwe_count": len(sample["cwe_type"]),
                    "function_count": sum(
                        len(funcs) for funcs in sample["vulnerable_functions"].values()
                    ),
                    "line_count": sum(
                        len(lines) for lines in sample["vulnerable_lines"].values()
                    ),
                    "deps_count": len(sample.get("deps", [])),
                    **metrics,
                }

                sample_data.append(sample_info)
                granularity_metrics[granularity].append(metrics)

        # Create DataFrame for analysis
        self.samples_df = pd.DataFrame(sample_data)

        # Granularity statistics
        granularity_stats = {}
        for granularity, metrics_list in granularity_metrics.items():
            if metrics_list:
                stats = {
                    "count": len(metrics_list),
                    "avg_lines": np.mean([m["total_lines"] for m in metrics_list]),
                    "avg_files": np.mean([m["total_files"] for m in metrics_list]),
                    "avg_rust_files": np.mean([m["rust_files"] for m in metrics_list]),
                }
                granularity_stats[granularity] = stats

        self.granularity_stats = granularity_stats

        print(f"✓ Code sample analysis completed: {len(sample_data)} samples analyzed")

    def create_visualizations(self):
        """Create all visualization figures."""
        print("Creating visualizations...")

        # Figure 1: Vulnerable vs Fixed samples
        fig, ax = plt.subplots(figsize=(8, 6))
        sizes = [
            self.basic_stats["vulnerable_samples"],
            self.basic_stats["fixed_samples"],
        ]
        labels = ["Vulnerable", "Fixed"]
        colors_pie = [self.colors[0], self.colors[1]]

        wedges, texts, autotexts = ax.pie(
            sizes, labels=labels, autopct="%1.1f%%", colors=colors_pie, startangle=90
        )
        ax.set_title(
            "Distribution of Vulnerable vs Fixed Code Samples",
            fontsize=14,
            fontweight="bold",
        )

        for autotext in autotexts:
            autotext.set_color("white")
            autotext.set_fontweight("bold")

        plt.tight_layout()
        plt.savefig(
            self.figures_dir / "figure1_sample_distribution.png",
            dpi=300,
            bbox_inches="tight",
        )
        plt.close()

        # Figure 2: CWE type distribution (top 10)
        cwe_items = sorted(
            self.vuln_stats["cwe_distribution"].items(),
            key=lambda x: x[1],
            reverse=True,
        )[:10]

        if cwe_items:
            fig, ax = plt.subplots(figsize=(10, 6))
            cwes, counts = zip(*cwe_items)

            bars = ax.barh(range(len(cwes)), counts, color=self.colors[2])
            ax.set_yticks(range(len(cwes)))
            ax.set_yticklabels(cwes)
            ax.set_xlabel("Number of Samples")
            ax.set_title("Top 10 CWE Types in Dataset", fontsize=14, fontweight="bold")
            ax.invert_yaxis()

            # Add value labels on bars
            for i, bar in enumerate(bars):
                width = bar.get_width()
                ax.text(
                    width + 0.1,
                    bar.get_y() + bar.get_height() / 2,
                    f"{int(width)}",
                    ha="left",
                    va="center",
                )

            plt.tight_layout()
            plt.savefig(
                self.figures_dir / "figure2_cwe_distribution.png",
                dpi=300,
                bbox_inches="tight",
            )
            plt.close()

        # Figure 3: Vulnerabilities by year
        year_items = sorted(self.vuln_stats["year_distribution"].items())

        if year_items:
            fig, ax = plt.subplots(figsize=(10, 6))
            years, counts = zip(*year_items)

            bars = ax.bar(years, counts, color=self.colors[3], alpha=0.8)
            ax.set_xlabel("Year")
            ax.set_ylabel("Number of Vulnerabilities")
            ax.set_title(
                "Distribution of Vulnerabilities by Year",
                fontsize=14,
                fontweight="bold",
            )
            ax.grid(True, alpha=0.3)

            # Add value labels on bars
            for bar in bars:
                height = bar.get_height()
                ax.text(
                    bar.get_x() + bar.get_width() / 2.0,
                    height + 0.05,
                    f"{int(height)}",
                    ha="center",
                    va="bottom",
                )

            plt.tight_layout()
            plt.savefig(
                self.figures_dir / "figure3_vulnerabilities_by_year.png",
                dpi=300,
                bbox_inches="tight",
            )
            plt.close()

        # Figure 4: Granularity distribution (ordered: crate, file, function)
        granularity_counts = self.samples_df["granularity"].value_counts()

        # Define the desired order
        granularity_order = ["crate", "file", "function"]
        ordered_counts = [granularity_counts.get(g, 0) for g in granularity_order]

        fig, ax = plt.subplots(figsize=(8, 6))
        bars = ax.bar(granularity_order, ordered_counts, color=self.colors[4])
        ax.set_xlabel("Granularity Level")
        ax.set_ylabel("Number of Code Samples")
        ax.set_title(
            "Distribution of Code Samples by Granularity Level",
            fontsize=14,
            fontweight="bold",
        )

        # Add value labels on bars
        for bar in bars:
            height = bar.get_height()
            ax.text(
                bar.get_x() + bar.get_width() / 2.0,
                height + 0.5,
                f"{int(height)}",
                ha="center",
                va="bottom",
            )

        plt.tight_layout()
        plt.savefig(
            self.figures_dir / "figure4_granularity_distribution.png",
            dpi=300,
            bbox_inches="tight",
        )
        plt.close()

        # Figure 5: Lines of code by granularity (box plot)
        if not self.samples_df["total_lines"].isna().all():
            fig, ax = plt.subplots(figsize=(10, 6))

            # Filter out samples with 0 lines for better visualization
            plot_data = self.samples_df[self.samples_df["total_lines"] > 0]

            if not plot_data.empty:
                # Use the same order as granularity distribution
                granularity_order = ["crate", "file", "function"]
                available_granularities = [
                    g
                    for g in granularity_order
                    if g in plot_data["granularity"].unique()
                ]

                box_plot = ax.boxplot(
                    [
                        plot_data[plot_data["granularity"] == g]["total_lines"].values
                        for g in available_granularities
                    ],
                    labels=available_granularities,
                    patch_artist=True,
                )

                # Color the boxes with consistent colors
                for i, patch in enumerate(box_plot["boxes"]):
                    patch.set_facecolor(self.colors[5])
                    patch.set_alpha(0.7)

                ax.set_xlabel("Granularity Level")
                ax.set_ylabel("Lines of Code")
                ax.set_title(
                    "Lines of Code Distribution by Granularity Level",
                    fontsize=14,
                    fontweight="bold",
                )
                ax.grid(True, alpha=0.3)

                plt.tight_layout()
                plt.savefig(
                    self.figures_dir / "figure5_lines_by_granularity.png",
                    dpi=300,
                    bbox_inches="tight",
                )
                plt.close()

        print(f"✓ All visualizations created and saved to {self.figures_dir}")

    def generate_report(self):
        """Generate comprehensive markdown report."""
        print("Generating markdown report...")

        self.markdown_content = []

        # Header
        self.markdown_content.extend(
            [
                "# RustMizan Dataset Analysis Report",
                "",
                f"**Dataset Version:** {self.dataset['general_information']['dataset_version']}",
                f"**Rust Version:** {self.dataset['general_information']['rust_version']}",
                f"**Analysis Date:** {pd.Timestamp.now().strftime('%Y-%m-%d')}",
                "",
            ]
        )

        # Basic Statistics Table
        self.markdown_content.extend(
            [
                "## Dataset Statistics",
                "",
                "| Metric | Value |",
                "|--------|-------|",
                f"| Total Vulnerabilities | {self.basic_stats['total_vulnerabilities']} |",
                f"| Total Code Samples | {self.basic_stats['total_samples']} |",
                f"| Vulnerable Samples | {self.basic_stats['vulnerable_samples']} |",
                f"| Fixed Samples | {self.basic_stats['fixed_samples']} |",
                f"| Year Range | {self.basic_stats['year_range']} |",
                f"| Unique Authors | {self.basic_stats['unique_authors']} |",
                f"| Unique CWE Types | {self.basic_stats['unique_cwes']} |",
                "",
            ]
        )

        # Vulnerability Analysis
        self.markdown_content.extend(
            [
                "## Vulnerability Analysis",
                "",
                "### Top 10 CWE Types",
                "",
                "| CWE Type | Frequency | Percentage |",
                "|----------|-----------|------------|",
            ]
        )

        cwe_items = sorted(
            self.vuln_stats["cwe_distribution"].items(),
            key=lambda x: x[1],
            reverse=True,
        )[:10]
        total_cwe_instances = sum(self.vuln_stats["cwe_distribution"].values())

        for cwe, count in cwe_items:
            percentage = (count / total_cwe_instances) * 100
            self.markdown_content.append(f"| {cwe} | {count} | {percentage:.1f}% |")

        self.markdown_content.extend(
            [
                "",
                "### Vulnerabilities by Year",
                "",
                "| Year | Count |",
                "|------|-------|",
            ]
        )

        year_items = sorted(self.vuln_stats["year_distribution"].items())
        for year, count in year_items:
            self.markdown_content.append(f"| {year} | {count} |")

        # Code Sample Analysis
        self.markdown_content.extend(
            [
                "",
                "## Code Sample Analysis",
                "",
                "### Granularity Level Statistics",
                "",
                "| Granularity | Samples | Avg Lines | Avg Rust Files |",
                "|-------------|---------|-----------|----------------|",
            ]
        )

        for granularity, stats in self.granularity_stats.items():
            self.markdown_content.append(
                f"| {granularity.title()} | {stats['count']} | "
                f"{stats['avg_lines']:.1f} | {stats['avg_rust_files']:.1f} |"
            )

        # Figures section
        self.markdown_content.extend(
            [
                "",
                "## Figures",
                "",
                "The following figures have been generated and saved to the `figures/` directory:",
                "",
                "1. **figure1_sample_distribution.png** - Distribution of Vulnerable vs Fixed Code Samples",
                "2. **figure2_cwe_distribution.png** - Top 10 CWE Types in Dataset",
                "3. **figure3_vulnerabilities_by_year.png** - Distribution of Vulnerabilities by Year",
                "4. **figure4_granularity_distribution.png** - Distribution of Code Samples by Granularity Level",
                "5. **figure5_lines_by_granularity.png** - Lines of Code Distribution by Granularity Level",
                "",
            ]
        )

        # Save report
        report_path = self.output_dir / "dataset_analysis_report.md"
        with open(report_path, "w") as f:
            f.write("\n".join(self.markdown_content))

        print(f"✓ Report generated: {report_path}")

    def run_analysis(self):
        """Run complete dataset analysis."""
        print("Starting RustMizan dataset analysis...")

        self.load_dataset()
        self.analyze_basic_statistics()
        self.analyze_vulnerabilities()
        self.analyze_code_samples()
        self.create_visualizations()
        self.generate_report()

        print(f"\n✓ Analysis complete!")
        print(f"Results saved to: {self.output_dir}")
        print(f"Figures saved to: {self.figures_dir}")
        print(f"Report saved to: {self.output_dir / 'dataset_analysis_report.md'}")


def main():
    """Main function to run the dataset analysis."""
    # Path to mizan.json is in the parent directory of the parent directory
    dataset_path = "../../mizan.json"

    if not os.path.exists(dataset_path):
        print(f"Error: Dataset file not found at {dataset_path}")
        print("Please ensure mizan.json exists in the project root directory.")
        return

    analyzer = RustMizanDatasetAnalyzer(dataset_path)
    analyzer.run_analysis()


if __name__ == "__main__":
    main()
