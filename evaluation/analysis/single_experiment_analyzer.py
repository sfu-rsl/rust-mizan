#!/usr/bin/env python3
"""
RustMizan Single Experiment Analysis Script

This script analyzes the performance of LLMs on a single RustMizan experiment for Rust vulnerability detection.
It generates a comprehensive markdown report and multiple figures saved to an analysis directory.

Usage: python single_experiment_analyzer.py <experiment_id>
"""

import json
import pandas as pd
import numpy as np
import matplotlib.pyplot as plt
from collections import Counter
from typing import Dict, List, Tuple, Any
import warnings
import argparse
from pathlib import Path

warnings.filterwarnings("ignore")

plt.style.use("default")
colors_professional = [
    "#E41A1C",
    "#377EB8",
    "#4DAF4A",
    "#984EA3",
    "#FF7F00",
    "#FFFF33",
    "#A65628",
    "#F781BF",
]
plt.rcParams["axes.prop_cycle"] = plt.cycler(color=colors_professional)


class RustMizanSingleExperimentAnalyzer:
    def __init__(self, experiment_id: str):
        self.experiment_id = experiment_id
        self.experiment_path = (
            f"../evaluation_results/experiment_{experiment_id}/results.json"
        )
        self.metadata_path = (
            f"../evaluation_results/experiment_{experiment_id}/metadata.json"
        )
        self.results = None
        self.metadata = None
        self.df = None
        self.output_dir = None
        self.markdown_content = []

    def load_experiment_results(self, path: str) -> List[Dict]:
        """Load experiment results from JSON file."""
        with open(path, "r") as f:
            data = json.load(f)
        return data

    def load_metadata(self) -> Dict:
        """Load experiment metadata from JSON file."""
        try:
            with open(self.metadata_path, "r") as f:
                data = json.load(f)
            return data
        except FileNotFoundError:
            print(f"Warning: Metadata file not found at {self.metadata_path}")
            return {}

    def setup_output_directory(self):
        """Set up output directory for analysis results."""
        base_dir = Path(f"../evaluation_results/experiment_{self.experiment_id}")
        self.output_dir = base_dir / "analysis"
        self.output_dir.mkdir(exist_ok=True)

        # Create subdirectory for figures
        (self.output_dir / "figures").mkdir(exist_ok=True)

    def calculate_tp_fp_fn(
        self, predicted_set: set, reference_set: set
    ) -> Tuple[int, int, int]:
        """Calculate True Positives, False Positives, and False Negatives."""
        tp = len(predicted_set.intersection(reference_set))
        fp = len(predicted_set - reference_set)
        fn = len(reference_set - predicted_set)
        return tp, fp, fn

    def calculate_precision_recall_f1(
        self, tp: int, fp: int, fn: int
    ) -> Tuple[float, float, float]:
        """Calculate precision, recall, and F1 score."""
        precision = tp / (tp + fp) if (tp + fp) > 0 else 0.0
        recall = tp / (tp + fn) if (tp + fn) > 0 else 0.0
        f1 = (
            2 * (precision * recall) / (precision + recall)
            if (precision + recall) > 0
            else 0.0
        )
        return precision, recall, f1

    def extract_functions_from_dict(self, func_dict: Dict[str, List[str]]) -> set:
        """Extract all function signatures from the vulnerable_functions dictionary."""
        functions = set()
        for file_path, func_list in func_dict.items():
            for func in func_list:
                functions.add(f"{file_path}::{func}")
        return functions

    def extract_lines_from_dict(self, lines_dict: Dict[str, List[int]]) -> set:
        """Extract all line numbers from the vulnerable_lines dictionary."""
        lines = set()
        for file_path, line_list in lines_dict.items():
            for line in line_list:
                lines.add(f"{file_path}::{line}")
        return lines

    def calculate_sample_metrics(self, sample: Dict) -> Dict[str, Any]:
        """Calculate all metrics for a single sample."""
        metrics = {}

        # Basic sample info
        metrics["example_id"] = sample["example_id"]
        metrics["vuln_id"] = sample["vuln_id"]
        metrics["granularity"] = sample["granularity"]
        metrics["crate_name"] = sample["crate_name"]
        metrics["year"] = sample["year"]
        metrics["reference_is_vulnerable"] = sample["is_vulnerable"]
        metrics["reference_cwe_types"] = sample["cwe_types"]

        # JSON validity
        metrics["json_validity"] = sample["scores"].get("json_validity", 0)

        # Check if we have a valid parsed response
        parsed_response = sample["outputs"].get("parsed_response")
        reference = sample["reference_outputs"]

        if parsed_response is None or metrics["json_validity"] == 0:
            # Invalid JSON response - all metrics are 0
            metrics.update(
                {
                    "is_vulnerable_correct": 0,
                    "cwe_tp": 0,
                    "cwe_fp": 0,
                    "cwe_fn": 0,
                    "cwe_precision": 0.0,
                    "cwe_recall": 0.0,
                    "cwe_f1": 0.0,
                    "functions_tp": 0,
                    "functions_fp": 0,
                    "functions_fn": 0,
                    "functions_precision": 0.0,
                    "functions_recall": 0.0,
                    "functions_f1": 0.0,
                    "lines_tp": 0,
                    "lines_fp": 0,
                    "lines_fn": 0,
                    "lines_precision": 0.0,
                    "lines_recall": 0.0,
                    "lines_f1": 0.0,
                    "at_least_one_correct_cwe": 0,
                    "at_least_one_correct_function": 0,
                    "at_least_one_correct_line": 0,
                }
            )
            return metrics

        # is_vulnerable accuracy
        metrics["is_vulnerable_correct"] = int(
            parsed_response["is_vulnerable"] == reference["is_vulnerable"]
        )

        # CWE type metrics
        predicted_cwes = set(parsed_response.get("cwe_type", []))
        reference_cwes = set(reference.get("cwe_type", []))
        cwe_tp, cwe_fp, cwe_fn = self.calculate_tp_fp_fn(predicted_cwes, reference_cwes)
        cwe_precision, cwe_recall, cwe_f1 = self.calculate_precision_recall_f1(
            cwe_tp, cwe_fp, cwe_fn
        )

        metrics.update(
            {
                "cwe_tp": cwe_tp,
                "cwe_fp": cwe_fp,
                "cwe_fn": cwe_fn,
                "cwe_precision": cwe_precision,
                "cwe_recall": cwe_recall,
                "cwe_f1": cwe_f1,
                "at_least_one_correct_cwe": int(cwe_tp > 0),
            }
        )

        # Vulnerable functions metrics
        predicted_functions = self.extract_functions_from_dict(
            parsed_response.get("vulnerable_functions", {})
        )
        reference_functions = self.extract_functions_from_dict(
            reference.get("vulnerable_functions", {})
        )
        functions_tp, functions_fp, functions_fn = self.calculate_tp_fp_fn(
            predicted_functions, reference_functions
        )
        functions_precision, functions_recall, functions_f1 = (
            self.calculate_precision_recall_f1(functions_tp, functions_fp, functions_fn)
        )

        metrics.update(
            {
                "functions_tp": functions_tp,
                "functions_fp": functions_fp,
                "functions_fn": functions_fn,
                "functions_precision": functions_precision,
                "functions_recall": functions_recall,
                "functions_f1": functions_f1,
                "at_least_one_correct_function": int(functions_tp > 0),
            }
        )

        # Vulnerable lines metrics
        predicted_lines = self.extract_lines_from_dict(
            parsed_response.get("vulnerable_lines", {})
        )
        reference_lines = self.extract_lines_from_dict(
            reference.get("vulnerable_lines", {})
        )
        lines_tp, lines_fp, lines_fn = self.calculate_tp_fp_fn(
            predicted_lines, reference_lines
        )
        lines_precision, lines_recall, lines_f1 = self.calculate_precision_recall_f1(
            lines_tp, lines_fp, lines_fn
        )

        metrics.update(
            {
                "lines_tp": lines_tp,
                "lines_fp": lines_fp,
                "lines_fn": lines_fn,
                "lines_precision": lines_precision,
                "lines_recall": lines_recall,
                "lines_f1": lines_f1,
                "at_least_one_correct_line": int(lines_tp > 0),
            }
        )

        return metrics

    def load_and_process_data(self):
        """Load and process the experiment data."""
        print(f"Loading experiment results from {self.experiment_path}")
        self.results = self.load_experiment_results(self.experiment_path)

        print(f"Loading metadata from {self.metadata_path}")
        self.metadata = self.load_metadata()

        print("Calculating metrics for all samples...")
        all_metrics = [self.calculate_sample_metrics(sample) for sample in self.results]
        self.df = pd.DataFrame(all_metrics)

        print(
            f"Created DataFrame with {len(self.df)} samples and {len(self.df.columns)} metrics"
        )

    def add_metric_definitions(self):
        """Add metric definitions to the beginning of the report."""
        model_name = self.metadata.get("model", "Unknown Model")
        mutations_applied = self.metadata.get("mutations_metadata", {}).get(
            "mutations_applied", []
        )
        mutations_text = (
            ", ".join(mutations_applied)
            if mutations_applied
            else "None (vanilla dataset)"
        )

        self.markdown_content.append(f"# RustMizan Benchmark Analysis Report\n")
        self.markdown_content.append(f"- **Model:** {model_name}\n")
        self.markdown_content.append(f"**Mutations Applied:** {mutations_text}\n")
        self.markdown_content.append("## Metric Definitions\n")
        self.markdown_content.append(
            "- **JSON Validity Rate:** Percentage of samples with valid JSON responses that could be parsed"
        )
        self.markdown_content.append(
            "- **Vulnerability Detection Accuracy:** Percentage of samples where the model correctly identified whether code is vulnerable or not"
        )
        self.markdown_content.append(
            "- **At Least One Correct CWE:** Percentage of samples where the model identified at least one correct CWE type"
        )
        self.markdown_content.append(
            "- **At Least One Correct Function:** Percentage of samples where the model identified at least one correct vulnerable function"
        )
        self.markdown_content.append(
            "- **At Least One Correct Line:** Percentage of samples where the model identified at least one correct vulnerable line"
        )
        self.markdown_content.append(
            "- **Macro F1 Score:** Average F1 score across all samples for the given task (CWE/Functions/Lines)"
        )
        self.markdown_content.append(
            "- **Granularity Levels:** Function (single function), File (entire file), Crate (entire crate/package)\n"
        )

    def analyze_overall_performance(self):
        """Analyze overall performance metrics."""
        total_samples = len(self.df)
        json_valid_samples = self.df["json_validity"].sum()
        json_validity_rate = json_valid_samples / total_samples

        # Simple metrics
        vulnerability_accuracy = self.df["is_vulnerable_correct"].mean()

        # Fixed calculations for at_least_one metrics
        at_least_one_function_count = (
            self.df["at_least_one_correct_function"] == 1
        ).sum()
        at_least_one_function_rate = at_least_one_function_count / total_samples

        at_least_one_line_count = (self.df["at_least_one_correct_line"] == 1).sum()
        at_least_one_line_rate = at_least_one_line_count / total_samples

        at_least_one_cwe_count = (self.df["at_least_one_correct_cwe"] == 1).sum()
        at_least_one_cwe_rate = at_least_one_cwe_count / total_samples

        # Detailed metrics
        valid_df = self.df[self.df["json_validity"] == 1]

        if len(valid_df) > 0:
            cwe_macro_f1 = valid_df["cwe_f1"].mean()
            functions_macro_f1 = valid_df["functions_f1"].mean()
            lines_macro_f1 = valid_df["lines_f1"].mean()
        else:
            cwe_macro_f1 = functions_macro_f1 = lines_macro_f1 = 0.0

        # Store for later use
        self.overall_metrics = {
            "total_samples": total_samples,
            "json_valid_samples": json_valid_samples,
            "json_validity_rate": json_validity_rate,
            "vulnerability_accuracy": vulnerability_accuracy,
            "at_least_one_cwe_count": at_least_one_cwe_count,
            "at_least_one_cwe_rate": at_least_one_cwe_rate,
            "at_least_one_function_count": at_least_one_function_count,
            "at_least_one_function_rate": at_least_one_function_rate,
            "at_least_one_line_count": at_least_one_line_count,
            "at_least_one_line_rate": at_least_one_line_rate,
            "cwe_macro_f1": cwe_macro_f1,
            "functions_macro_f1": functions_macro_f1,
            "lines_macro_f1": lines_macro_f1,
            "valid_df": valid_df,
        }

        # Add to markdown
        self.markdown_content.append("## Overall Performance Summary\n")
        self.markdown_content.append("| Metric | Value |")
        self.markdown_content.append("|--------|-------|")
        self.markdown_content.append(f"| Total Samples | {total_samples} |")
        self.markdown_content.append(
            f"| JSON Validity Rate | {json_validity_rate:.1%} |"
        )
        self.markdown_content.append(
            f"| Vulnerability Detection Accuracy | {vulnerability_accuracy:.1%} |"
        )
        self.markdown_content.append(
            f"| Samples with At Least One Correct CWE Identified | {at_least_one_cwe_rate:.1%} ({at_least_one_cwe_count} out of {total_samples}) |"
        )
        self.markdown_content.append(
            f"| Samples with At Least One Correct Vulnerable Function Identified | {at_least_one_function_rate:.1%} ({at_least_one_function_count} out of {total_samples}) |"
        )
        self.markdown_content.append(
            f"| Samples with At Least One Correct Vulnerable Line Identified | {at_least_one_line_rate:.1%} ({at_least_one_line_count} out of {total_samples}) |"
        )

        if len(valid_df) > 0:
            self.markdown_content.append(f"| CWE Type Macro F1 | {cwe_macro_f1:.3f} |")
            self.markdown_content.append(
                f"| Vulnerable Functions Macro F1 | {functions_macro_f1:.3f} |"
            )
            self.markdown_content.append(
                f"| Vulnerable Lines Macro F1 | {lines_macro_f1:.3f} |"
            )

        self.markdown_content.append("")

        print("✓ Overall performance analysis completed")

    def analyze_vulnerable_samples_only(self):
        """Analyze performance for vulnerable samples only."""
        # Filter to only vulnerable samples
        vuln_df = self.df[self.df["reference_is_vulnerable"] == True]

        if len(vuln_df) == 0:
            self.markdown_content.append("## Analysis of Vulnerable Samples\n")
            self.markdown_content.append(
                "No vulnerable samples found in the dataset.\n"
            )
            self.vuln_only_stats = None
            return

        # Calculate statistics for vulnerable samples only
        total_vuln_samples = len(vuln_df)
        json_valid_vuln = vuln_df["json_validity"].sum()
        json_validity_rate_vuln = json_valid_vuln / total_vuln_samples
        vulnerability_accuracy_vuln = vuln_df["is_vulnerable_correct"].mean()

        # At least one correct metrics for vulnerable samples
        at_least_one_cwe_vuln_count = (vuln_df["at_least_one_correct_cwe"] == 1).sum()
        at_least_one_cwe_vuln = at_least_one_cwe_vuln_count / total_vuln_samples
        at_least_one_function_vuln_count = (
            vuln_df["at_least_one_correct_function"] == 1
        ).sum()
        at_least_one_function_vuln = (
            at_least_one_function_vuln_count / total_vuln_samples
        )
        at_least_one_line_vuln_count = (vuln_df["at_least_one_correct_line"] == 1).sum()
        at_least_one_line_vuln = at_least_one_line_vuln_count / total_vuln_samples

        # F1 scores for vulnerable samples only
        valid_vuln_df = vuln_df[vuln_df["json_validity"] == 1]
        if len(valid_vuln_df) > 0:
            cwe_macro_f1_vuln = valid_vuln_df["cwe_f1"].mean()
            functions_macro_f1_vuln = valid_vuln_df["functions_f1"].mean()
            lines_macro_f1_vuln = valid_vuln_df["lines_f1"].mean()
        else:
            cwe_macro_f1_vuln = functions_macro_f1_vuln = lines_macro_f1_vuln = 0.0

        # Store for visualization
        self.vuln_only_stats = {
            "total_samples": total_vuln_samples,
            "json_validity_rate": json_validity_rate_vuln,
            "vulnerability_accuracy": vulnerability_accuracy_vuln,
            "at_least_one_cwe": at_least_one_cwe_vuln,
            "at_least_one_function": at_least_one_function_vuln,
            "at_least_one_line": at_least_one_line_vuln,
            "cwe_macro_f1": cwe_macro_f1_vuln,
            "functions_macro_f1": functions_macro_f1_vuln,
            "lines_macro_f1": lines_macro_f1_vuln,
        }

        # Add to markdown
        self.markdown_content.append("## Analysis of Vulnerable Samples\n")
        self.markdown_content.append(
            "*Analysis focused only on samples that contain vulnerabilities*\n"
        )
        self.markdown_content.append("| Metric | Value |")
        self.markdown_content.append("|--------|-------|")
        self.markdown_content.append(
            f"| Total Vulnerable Samples | {total_vuln_samples} |"
        )
        self.markdown_content.append(
            f"| JSON Validity Rate | {json_validity_rate_vuln:.1%} |"
        )
        self.markdown_content.append(
            f"| Vulnerability Detection Accuracy | {vulnerability_accuracy_vuln:.1%} |"
        )
        self.markdown_content.append(
            f"| At Least One Correct CWE Identified | {at_least_one_cwe_vuln:.1%} ({at_least_one_cwe_vuln_count} out of {total_vuln_samples}) |"
        )
        self.markdown_content.append(
            f"| At Least One Correct Vulnerable Function Identified | {at_least_one_function_vuln:.1%} ({at_least_one_function_vuln_count} out of {total_vuln_samples}) |"
        )
        self.markdown_content.append(
            f"| At Least One Correct Vulnerable Line Identified | {at_least_one_line_vuln:.1%} ({at_least_one_line_vuln_count} out of {total_vuln_samples}) |"
        )

        self.markdown_content.append(
            "\n### Detailed F1 Scores (Vulnerable Samples Only)\n"
        )
        self.markdown_content.append("| Metric | F1 Score |")
        self.markdown_content.append("|--------|----------|")
        self.markdown_content.append(f"| CWE Type Macro F1 | {cwe_macro_f1_vuln:.3f} |")
        self.markdown_content.append(
            f"| Vulnerable Functions Macro F1 | {functions_macro_f1_vuln:.3f} |"
        )
        self.markdown_content.append(
            f"| Vulnerable Lines Macro F1 | {lines_macro_f1_vuln:.3f} |"
        )
        self.markdown_content.append("")

        print("✓ Vulnerable samples analysis completed")

    def analyze_by_granularity(self):
        """Analyze performance by granularity."""
        # Calculate percentages correctly for at_least_one metrics
        granularity_stats = []

        for granularity in self.df["granularity"].unique():
            gran_df = self.df[self.df["granularity"] == granularity]
            total_samples = len(gran_df)

            # Calculate counts
            at_least_one_cwe_count = (gran_df["at_least_one_correct_cwe"] == 1).sum()
            at_least_one_function_count = (
                gran_df["at_least_one_correct_function"] == 1
            ).sum()
            at_least_one_line_count = (gran_df["at_least_one_correct_line"] == 1).sum()

            stats = {
                "granularity": granularity,
                "total_samples": total_samples,
                "valid_json_samples": gran_df["json_validity"].sum(),
                "json_validity_rate": gran_df["json_validity"].mean(),
                "vulnerability_accuracy": gran_df["is_vulnerable_correct"].mean(),
                "at_least_one_cwe": at_least_one_cwe_count / total_samples,
                "at_least_one_cwe_count": at_least_one_cwe_count,
                "at_least_one_function": at_least_one_function_count / total_samples,
                "at_least_one_function_count": at_least_one_function_count,
                "at_least_one_line": at_least_one_line_count / total_samples,
                "at_least_one_line_count": at_least_one_line_count,
                "cwe_macro_f1": gran_df["cwe_f1"].mean(),
                "functions_macro_f1": gran_df["functions_f1"].mean(),
                "lines_macro_f1": gran_df["lines_f1"].mean(),
            }
            granularity_stats.append(stats)

        self.granularity_stats = pd.DataFrame(granularity_stats).set_index(
            "granularity"
        )

        # Add to markdown
        self.markdown_content.append("## Analysis by Granularity\n")
        self.markdown_content.append(
            "| Granularity | Samples | JSON Validity | Vulnerability Accuracy | At Least One CWE | At Least One Function | At Least One Line |"
        )
        self.markdown_content.append(
            "|-------------|---------|---------------|------------------------|------------------|----------------------|-------------------|"
        )

        for granularity, row in self.granularity_stats.iterrows():
            self.markdown_content.append(
                f"| {granularity.title()} | {int(row['total_samples'])} | {row['json_validity_rate']:.1%} | {row['vulnerability_accuracy']:.1%} | {row['at_least_one_cwe']:.1%} ({int(row['at_least_one_cwe_count'])} out of {int(row['total_samples'])}) | {row['at_least_one_function']:.1%} ({int(row['at_least_one_function_count'])} out of {int(row['total_samples'])}) | {row['at_least_one_line']:.1%} ({int(row['at_least_one_line_count'])} out of {int(row['total_samples'])}) |"
            )

        self.markdown_content.append("\n### Detailed F1 Scores by Granularity\n")
        self.markdown_content.append(
            "| Granularity | CWE Macro F1 | Functions Macro F1 | Lines Macro F1 |"
        )
        self.markdown_content.append(
            "|-------------|--------------|-------------------|----------------|"
        )

        for granularity, row in self.granularity_stats.iterrows():
            self.markdown_content.append(
                f"| {granularity.title()} | {row['cwe_macro_f1']:.3f} | {row['functions_macro_f1']:.3f} | {row['lines_macro_f1']:.3f} |"
            )

        self.markdown_content.append("")

        print("✓ Granularity analysis completed")

    def analyze_complete_granularity(self):
        """Analyze vulnerabilities with all three granularities and valid JSON."""
        # Group by vuln_id and check which ones have all 3 granularities
        vuln_granularity_check = (
            self.df.groupby("vuln_id")
            .agg(
                {
                    "granularity": lambda x: set(x),
                    "json_validity": "sum",
                    "example_id": "count",
                }
            )
            .rename(columns={"example_id": "total_samples"})
        )

        # Find vulnerabilities with all 3 granularities
        complete_vulns = vuln_granularity_check[
            vuln_granularity_check["granularity"].apply(
                lambda x: {"function", "file", "crate"}.issubset(x)
            )
        ]

        # Now filter for those where all samples have valid JSON
        complete_valid_vulns = []
        for vuln_id in complete_vulns.index:
            vuln_samples = self.df[self.df["vuln_id"] == vuln_id]
            if vuln_samples["json_validity"].sum() == len(vuln_samples):
                complete_valid_vulns.append(vuln_id)

        self.complete_valid_vulns = complete_valid_vulns

        # Add to markdown
        self.markdown_content.append("## Complete Granularity Analysis\n")
        self.markdown_content.append(
            "*Analysis of vulnerabilities with all three granularities (function, file, crate) and valid JSON responses*\n"
        )

        if len(complete_valid_vulns) > 0:
            # Filter dataframe to only include these complete vulnerabilities
            complete_df = self.df[self.df["vuln_id"].isin(complete_valid_vulns)]

            # Calculate statistics similar to granularity analysis
            complete_granularity_stats = []

            for granularity in complete_df["granularity"].unique():
                gran_df = complete_df[complete_df["granularity"] == granularity]
                total_samples = len(gran_df)

                # Calculate counts
                at_least_one_cwe_count = (
                    gran_df["at_least_one_correct_cwe"] == 1
                ).sum()
                at_least_one_function_count = (
                    gran_df["at_least_one_correct_function"] == 1
                ).sum()
                at_least_one_line_count = (
                    gran_df["at_least_one_correct_line"] == 1
                ).sum()

                stats = {
                    "granularity": granularity,
                    "total_samples": total_samples,
                    "json_validity_rate": gran_df["json_validity"].mean(),
                    "vulnerability_accuracy": gran_df["is_vulnerable_correct"].mean(),
                    "at_least_one_cwe": at_least_one_cwe_count / total_samples,
                    "at_least_one_cwe_count": at_least_one_cwe_count,
                    "at_least_one_function": at_least_one_function_count
                    / total_samples,
                    "at_least_one_function_count": at_least_one_function_count,
                    "at_least_one_line": at_least_one_line_count / total_samples,
                    "at_least_one_line_count": at_least_one_line_count,
                    "cwe_macro_f1": gran_df["cwe_f1"].mean(),
                    "functions_macro_f1": gran_df["functions_f1"].mean(),
                    "lines_macro_f1": gran_df["lines_f1"].mean(),
                }
                complete_granularity_stats.append(stats)

            self.complete_granularity_stats = pd.DataFrame(
                complete_granularity_stats
            ).set_index("granularity")

            self.markdown_content.append(
                f"Found {len(complete_valid_vulns)} vulnerabilities with all three granularities and valid JSON responses.\n"
            )
            self.markdown_content.append(
                f"Total samples in complete analysis: {len(complete_df)} ({len(complete_df) // 3} samples per granularity)\n"
            )

            self.markdown_content.append(
                "| Granularity | Samples | Vulnerability Accuracy | At Least One CWE | At Least One Function | At Least One Line |"
            )
            self.markdown_content.append(
                "|-------------|---------|------------------------|------------------|----------------------|-------------------|"
            )

            for granularity, row in self.complete_granularity_stats.iterrows():
                self.markdown_content.append(
                    f"| {granularity.title()} | {int(row['total_samples'])} | {row['vulnerability_accuracy']:.1%} | {row['at_least_one_cwe']:.1%} ({int(row['at_least_one_cwe_count'])} out of {int(row['total_samples'])}) | {row['at_least_one_function']:.1%} ({int(row['at_least_one_function_count'])} out of {int(row['total_samples'])}) | {row['at_least_one_line']:.1%} ({int(row['at_least_one_line_count'])} out of {int(row['total_samples'])}) |"
                )

            self.markdown_content.append(
                "\n### Detailed F1 Scores (Complete Granularity)\n"
            )
            self.markdown_content.append(
                "| Granularity | CWE Macro F1 | Functions Macro F1 | Lines Macro F1 |"
            )
            self.markdown_content.append(
                "|-------------|--------------|-------------------|----------------|"
            )

            for granularity, row in self.complete_granularity_stats.iterrows():
                self.markdown_content.append(
                    f"| {granularity.title()} | {row['cwe_macro_f1']:.3f} | {row['functions_macro_f1']:.3f} | {row['lines_macro_f1']:.3f} |"
                )
        else:
            self.markdown_content.append(
                "No vulnerabilities found with all three granularities and valid JSON responses."
            )
            self.complete_granularity_stats = None

        self.markdown_content.append("")

        print("✓ Complete granularity analysis completed")

    def analyze_by_cwe_type(self):
        """Analyze performance by CWE type."""
        # Extract all CWE types from reference
        all_cwes = []
        for cwe_list in self.df["reference_cwe_types"]:
            all_cwes.extend(cwe_list)

        cwe_counts = Counter(all_cwes)

        # Performance for top 10 most common CWEs
        top_cwes = [cwe for cwe, _ in cwe_counts.most_common(10)]

        cwe_performance = []
        for cwe in top_cwes:
            # Find samples that have this CWE in reference
            cwe_samples = self.df[
                self.df["reference_cwe_types"].apply(lambda x: cwe in x)
            ]

            if len(cwe_samples) > 0:
                total_cwe_samples = len(cwe_samples)
                avg_accuracy = cwe_samples["is_vulnerable_correct"].mean()
                avg_cwe_f1 = cwe_samples["cwe_f1"].mean()
                avg_func_f1 = cwe_samples["functions_f1"].mean()
                avg_lines_f1 = cwe_samples["lines_f1"].mean()
                json_validity = cwe_samples["json_validity"].mean()

                # Calculate counts
                at_least_one_cwe_count = (
                    cwe_samples["at_least_one_correct_cwe"] == 1
                ).sum()
                at_least_one_function_count = (
                    cwe_samples["at_least_one_correct_function"] == 1
                ).sum()
                at_least_one_line_count = (
                    cwe_samples["at_least_one_correct_line"] == 1
                ).sum()

                at_least_one_cwe = at_least_one_cwe_count / total_cwe_samples
                at_least_one_function = at_least_one_function_count / total_cwe_samples
                at_least_one_line = at_least_one_line_count / total_cwe_samples

                cwe_performance.append(
                    {
                        "CWE": cwe,
                        "samples": total_cwe_samples,
                        "json_validity": json_validity,
                        "vulnerability_accuracy": avg_accuracy,
                        "at_least_one_cwe": at_least_one_cwe,
                        "at_least_one_cwe_count": at_least_one_cwe_count,
                        "at_least_one_function": at_least_one_function,
                        "at_least_one_function_count": at_least_one_function_count,
                        "at_least_one_line": at_least_one_line,
                        "at_least_one_line_count": at_least_one_line_count,
                        "cwe_f1": avg_cwe_f1,
                        "functions_f1": avg_func_f1,
                        "lines_f1": avg_lines_f1,
                    }
                )

        self.cwe_perf_df = pd.DataFrame(cwe_performance)

        # Add to markdown
        self.markdown_content.append("## Performance by CWE Type\n")
        self.markdown_content.append(f"Total unique CWE types: {len(cwe_counts)}\n")

        if len(self.cwe_perf_df) > 0:
            self.markdown_content.append(
                "| CWE | Samples | JSON Validity | Vulnerability Accuracy | At Least One CWE | At Least One Function | At Least One Line |"
            )
            self.markdown_content.append(
                "|-----|---------|---------------|------------------------|------------------|----------------------|-------------------|"
            )

            for _, row in self.cwe_perf_df.head(5).iterrows():
                self.markdown_content.append(
                    f"| {row['CWE']} | {int(row['samples'])} | {row['json_validity']:.1%} | {row['vulnerability_accuracy']:.1%} | {row['at_least_one_cwe']:.1%} ({int(row['at_least_one_cwe_count'])} out of {int(row['samples'])}) | {row['at_least_one_function']:.1%} ({int(row['at_least_one_function_count'])} out of {int(row['samples'])}) | {row['at_least_one_line']:.1%} ({int(row['at_least_one_line_count'])} out of {int(row['samples'])}) |"
                )

            self.markdown_content.append("\n### Detailed F1 Scores by CWE Type\n")
            self.markdown_content.append("| CWE | CWE F1 | Functions F1 | Lines F1 |")
            self.markdown_content.append("|-----|--------|--------------|----------|")

            for _, row in self.cwe_perf_df.head(5).iterrows():
                self.markdown_content.append(
                    f"| {row['CWE']} | {row['cwe_f1']:.3f} | {row['functions_f1']:.3f} | {row['lines_f1']:.3f} |"
                )

        self.markdown_content.append("")

        print("✓ CWE type analysis completed")

    def create_visualizations(self):
        """Create all visualization figures."""
        print("Creating visualizations...")

        # Figure 1: Overall Performance - Simple Metrics
        fig, ax = plt.subplots(1, 1, figsize=(12, 8))

        metrics = [
            "JSON\nValidity",
            "Vulnerability\nAccuracy",
            "At Least One\nCorrect CWE",
            "At Least One\nCorrect Function",
            "At Least One\nCorrect Line",
        ]
        values = [
            self.overall_metrics["json_validity_rate"],
            self.overall_metrics["vulnerability_accuracy"],
            self.overall_metrics["at_least_one_cwe_rate"],
            self.overall_metrics["at_least_one_function_rate"],
            self.overall_metrics["at_least_one_line_rate"],
        ]

        bars = ax.bar(metrics, values, alpha=0.8)
        ax.set_title(
            "Overall Performance: Simple Metrics", fontsize=14, fontweight="bold"
        )
        ax.set_ylabel("Rate", fontsize=12)
        ax.set_ylim(0, 1)

        # Add value labels on bars
        for bar, value in zip(bars, values):
            ax.text(
                bar.get_x() + bar.get_width() / 2,
                bar.get_height() + 0.01,
                f"{value:.1%}",
                ha="center",
                va="bottom",
                fontweight="bold",
            )

        plt.tight_layout()
        plt.savefig(
            self.output_dir / "figures" / "figure1_overall_simple.png",
            dpi=300,
            bbox_inches="tight",
        )
        plt.close()

        # Figure 2: Overall Performance - Detailed Metrics
        fig, ax = plt.subplots(1, 1, figsize=(10, 6))

        if len(self.overall_metrics["valid_df"]) > 0:
            f1_metrics = [
                "Macro F1 Score\nfor CWE Type",
                "Macro F1 Score for\nVulnerable Functions",
                "Macro F1 Score for\nVulnerable Lines",
            ]
            f1_values = [
                self.overall_metrics["cwe_macro_f1"],
                self.overall_metrics["functions_macro_f1"],
                self.overall_metrics["lines_macro_f1"],
            ]

            bars = ax.bar(f1_metrics, f1_values, alpha=0.8)
            ax.set_title(
                "Overall Performance: Detailed Metrics", fontsize=14, fontweight="bold"
            )
            ax.set_ylabel("F1 Score", fontsize=12)
            ax.set_ylim(0, 1)

            # Add value labels on bars
            for bar, value in zip(bars, f1_values):
                ax.text(
                    bar.get_x() + bar.get_width() / 2,
                    bar.get_height() + 0.005,
                    f"{value:.3f}",
                    ha="center",
                    va="bottom",
                    fontweight="bold",
                )
        else:
            ax.text(
                0.5,
                0.5,
                "No valid JSON responses\nfor F1 calculation",
                ha="center",
                va="center",
                transform=ax.transAxes,
                fontsize=12,
            )
            ax.set_title(
                "Overall Performance: Detailed Metrics", fontsize=14, fontweight="bold"
            )

        plt.tight_layout()
        plt.savefig(
            self.output_dir / "figures" / "figure2_overall_detailed.png",
            dpi=300,
            bbox_inches="tight",
        )
        plt.close()

        # Figure 3: Vulnerable Samples Only - Simple Metrics
        if self.vuln_only_stats is not None:
            fig, ax = plt.subplots(1, 1, figsize=(12, 8))

            metrics = [
                "JSON\nValidity",
                "Vulnerability\nAccuracy",
                "At Least One\nCWE",
                "At Least One\nFunction",
                "At Least One\nLine",
            ]
            values = [
                self.vuln_only_stats["json_validity_rate"],
                self.vuln_only_stats["vulnerability_accuracy"],
                self.vuln_only_stats["at_least_one_cwe"],
                self.vuln_only_stats["at_least_one_function"],
                self.vuln_only_stats["at_least_one_line"],
            ]

            bars = ax.bar(metrics, values, alpha=0.8)
            ax.set_title(
                "Performance on Vulnerable Samples Only: Simple Metrics",
                fontsize=14,
                fontweight="bold",
            )
            ax.set_ylabel("Rate", fontsize=12)
            ax.set_ylim(0, 1)

            # Add value labels on bars
            for bar, value in zip(bars, values):
                ax.text(
                    bar.get_x() + bar.get_width() / 2,
                    bar.get_height() + 0.01,
                    f"{value:.1%}",
                    ha="center",
                    va="bottom",
                    fontweight="bold",
                )

            plt.tight_layout()
            plt.savefig(
                self.output_dir / "figures" / "figure3_vulnerable_samples_simple.png",
                dpi=300,
                bbox_inches="tight",
            )
            plt.close()

            # Figure 4: Vulnerable Samples Only - Detailed Metrics
            fig, ax = plt.subplots(1, 1, figsize=(10, 6))

            f1_metrics = [
                "Macro F1 Score\nfor CWE Type",
                "Macro F1 Score for\nVulnerable Functions",
                "Macro F1 Score for\nVulnerable Lines",
            ]
            f1_values = [
                self.vuln_only_stats["cwe_macro_f1"],
                self.vuln_only_stats["functions_macro_f1"],
                self.vuln_only_stats["lines_macro_f1"],
            ]

            bars = ax.bar(f1_metrics, f1_values, alpha=0.8)
            ax.set_title(
                "Performance on Vulnerable Samples Only: Detailed Metrics",
                fontsize=14,
                fontweight="bold",
            )
            ax.set_ylabel("F1 Score", fontsize=12)
            ax.set_ylim(0, 1)

            # Add value labels on bars
            for bar, value in zip(bars, f1_values):
                ax.text(
                    bar.get_x() + bar.get_width() / 2,
                    bar.get_height() + 0.005,
                    f"{value:.3f}",
                    ha="center",
                    va="bottom",
                    fontweight="bold",
                )

            plt.tight_layout()
            plt.savefig(
                self.output_dir / "figures" / "figure4_vulnerable_samples_detailed.png",
                dpi=300,
                bbox_inches="tight",
            )
            plt.close()

        # Figure 5: Performance by Granularity - Simple Metrics
        fig, ax = plt.subplots(1, 1, figsize=(12, 8))

        granularity_order = ["function", "file", "crate"]
        metrics = [
            "JSON\nValidity",
            "Vulnerability\nAccuracy",
            "At Least One\nCWE",
            "At Least One\nFunction",
            "At Least One\nLine",
        ]

        x = np.arange(len(metrics))
        width = 0.25

        for i, granularity in enumerate(granularity_order):
            if granularity in self.granularity_stats.index:
                row = self.granularity_stats.loc[granularity]
                values = [
                    row["json_validity_rate"],
                    row["vulnerability_accuracy"],
                    row["at_least_one_cwe"],
                    row["at_least_one_function"],
                    row["at_least_one_line"],
                ]

                bars = ax.bar(
                    x + i * width,
                    values,
                    width,
                    label=f'{granularity.title()} ({int(row["total_samples"])} samples)',
                    alpha=0.8,
                )

        ax.set_title(
            "Performance by Granularity: Simple Metrics", fontsize=14, fontweight="bold"
        )
        ax.set_ylabel("Rate", fontsize=12)
        ax.set_xticks(x + width)
        ax.set_xticklabels(metrics)
        ax.legend()
        ax.set_ylim(0, 1)

        plt.tight_layout()
        plt.savefig(
            self.output_dir / "figures" / "figure5_granularity_simple.png",
            dpi=300,
            bbox_inches="tight",
        )
        plt.close()

        # Figure 6: Performance by Granularity - Detailed Metrics
        fig, ax = plt.subplots(1, 1, figsize=(10, 6))

        f1_metrics = [
            "Macro F1 Score\nfor CWE Type",
            "Macro F1 Score for\nVulnerable Functions",
            "Macro F1 Score for\nVulnerable Lines",
        ]

        x = np.arange(len(f1_metrics))
        width = 0.25

        for i, granularity in enumerate(granularity_order):
            if granularity in self.granularity_stats.index:
                row = self.granularity_stats.loc[granularity]
                f1_values = [
                    row["cwe_macro_f1"],
                    row["functions_macro_f1"],
                    row["lines_macro_f1"],
                ]

                bars = ax.bar(
                    x + i * width,
                    f1_values,
                    width,
                    label=f"{granularity.title()}",
                    alpha=0.8,
                )

        ax.set_title(
            "Performance by Granularity: Detailed Metrics",
            fontsize=14,
            fontweight="bold",
        )
        ax.set_ylabel("F1 Score", fontsize=12)
        ax.set_xticks(x + width)
        ax.set_xticklabels(f1_metrics)
        ax.legend()

        # Set appropriate y-axis limit
        max_f1 = (
            self.granularity_stats[
                ["cwe_macro_f1", "functions_macro_f1", "lines_macro_f1"]
            ]
            .max()
            .max()
        )
        ax.set_ylim(0, 1)

        plt.tight_layout()
        plt.savefig(
            self.output_dir / "figures" / "figure6_granularity_detailed.png",
            dpi=300,
            bbox_inches="tight",
        )
        plt.close()

        # Figure 7 & 8: Complete Granularity Analysis (if data exists)
        if (
            len(self.complete_valid_vulns) > 0
            and self.complete_granularity_stats is not None
        ):
            # Figure 7: Complete Granularity Analysis - Simple Metrics
            fig, ax = plt.subplots(1, 1, figsize=(12, 8))

            metrics = [
                "Vulnerability\nAccuracy",
                "At Least One\nCWE",
                "At Least One\nFunction",
                "At Least One\nLine",
            ]

            x = np.arange(len(metrics))
            width = 0.25

            for i, granularity in enumerate(granularity_order):
                if granularity in self.complete_granularity_stats.index:
                    row = self.complete_granularity_stats.loc[granularity]
                    values = [
                        row["vulnerability_accuracy"],
                        row["at_least_one_cwe"],
                        row["at_least_one_function"],
                        row["at_least_one_line"],
                    ]

                    bars = ax.bar(
                        x + i * width,
                        values,
                        width,
                        label=f'{granularity.title()} ({int(row["total_samples"])} samples)',
                        alpha=0.8,
                    )

            ax.set_title(
                "Complete Granularity Analysis: Simple Metrics\n(Vulnerabilities with all granularities and valid JSON)",
                fontsize=14,
                fontweight="bold",
            )
            ax.set_ylabel("Rate", fontsize=12)
            ax.set_xticks(x + width)
            ax.set_xticklabels(metrics)
            ax.legend()
            ax.set_ylim(0, 1)

            plt.tight_layout()
            plt.savefig(
                self.output_dir / "figures" / "figure7_complete_granularity_simple.png",
                dpi=300,
                bbox_inches="tight",
            )
            plt.close()

            # Figure 8: Complete Granularity Analysis - Detailed Metrics
            fig, ax = plt.subplots(1, 1, figsize=(10, 6))

            f1_metrics = [
                "Macro F1 Score\nfor CWE Type",
                "Macro F1 Score for\nVulnerable Functions",
                "Macro F1 Score for\nVulnerable Lines",
            ]

            x = np.arange(len(f1_metrics))
            width = 0.25

            for i, granularity in enumerate(granularity_order):
                if granularity in self.complete_granularity_stats.index:
                    row = self.complete_granularity_stats.loc[granularity]
                    f1_values = [
                        row["cwe_macro_f1"],
                        row["functions_macro_f1"],
                        row["lines_macro_f1"],
                    ]

                    bars = ax.bar(
                        x + i * width,
                        f1_values,
                        width,
                        label=f"{granularity.title()}",
                        alpha=0.8,
                    )

            ax.set_title(
                "Complete Granularity Analysis: Detailed Metrics\n(Vulnerabilities with all granularities and valid JSON)",
                fontsize=14,
                fontweight="bold",
            )
            ax.set_ylabel("F1 Score", fontsize=12)
            ax.set_xticks(x + width)
            ax.set_xticklabels(f1_metrics)
            ax.legend()

            # Set appropriate y-axis limit
            max_f1 = (
                self.complete_granularity_stats[
                    ["cwe_macro_f1", "functions_macro_f1", "lines_macro_f1"]
                ]
                .max()
                .max()
            )
            ax.set_ylim(0, 1)

            plt.tight_layout()
            plt.savefig(
                self.output_dir
                / "figures"
                / "figure8_complete_granularity_detailed.png",
                dpi=300,
                bbox_inches="tight",
            )
            plt.close()

        print("✓ All visualizations created and saved")

    def save_markdown_report(self):
        """Save the markdown report to file."""
        # Add figures section to markdown
        self.markdown_content.append("## Figures\n")
        self.markdown_content.append(
            "The following figures have been generated and saved to the `figures/` directory:\n"
        )
        self.markdown_content.append(
            "1. **figure1_overall_simple.png** - Overall Performance: Simple Metrics"
        )
        self.markdown_content.append(
            "2. **figure2_overall_detailed.png** - Overall Performance: Detailed Metrics"
        )

        if self.vuln_only_stats is not None:
            self.markdown_content.append(
                "3. **figure3_vulnerable_samples_simple.png** - Performance on Vulnerable Samples Only: Simple Metrics"
            )
            self.markdown_content.append(
                "4. **figure4_vulnerable_samples_detailed.png** - Performance on Vulnerable Samples Only: Detailed Metrics"
            )

        self.markdown_content.append(
            "5. **figure5_granularity_simple.png** - Performance by Granularity: Simple Metrics"
        )
        self.markdown_content.append(
            "6. **figure6_granularity_detailed.png** - Performance by Granularity: Detailed Metrics"
        )

        if len(self.complete_valid_vulns) > 0:
            self.markdown_content.append(
                "7. **figure7_complete_granularity_simple.png** - Complete Granularity Analysis: Simple Metrics"
            )
            self.markdown_content.append(
                "8. **figure8_complete_granularity_detailed.png** - Complete Granularity Analysis: Detailed Metrics"
            )

        self.markdown_content.append("")

        # Save markdown report
        report_path = self.output_dir / "analysis_report.md"
        with open(report_path, "w") as f:
            f.write("\n".join(self.markdown_content))

        print(f"✓ Markdown report saved to: {report_path}")

    def run_analysis(self):
        """Run the complete analysis pipeline."""
        print(
            f"Starting RustMizan single experiment analysis for {self.experiment_id}..."
        )

        # Setup
        self.setup_output_directory()
        self.load_and_process_data()

        # Analysis sections
        self.add_metric_definitions()
        self.analyze_overall_performance()
        self.analyze_vulnerable_samples_only()
        self.analyze_by_granularity()
        self.analyze_complete_granularity()
        self.analyze_by_cwe_type()

        # Generate outputs
        self.create_visualizations()
        self.save_markdown_report()

        print(f"\nAnalysis complete. Results saved to: {self.output_dir}")
        print(f"Figures saved to: {self.output_dir}/figures/")
        print(f"Report saved to: {self.output_dir}/analysis_report.md")


def main():
    """Main function to run the analysis."""
    parser = argparse.ArgumentParser(
        description="Analyze RustMizan single experiment results"
    )
    parser.add_argument("experiment_id", help="Experiment ID")

    args = parser.parse_args()
    experiment_id = args.experiment_id

    # Check if the experiment directory exists
    experiment_dir = Path(f"../evaluation_results/experiment_{experiment_id}")
    if not experiment_dir.exists():
        print(f"Error: Experiment directory not found at {experiment_dir}")
        print("Please check the experiment ID and try again.")
        return

    # Check if results file exists
    results_file = experiment_dir / "results.json"
    if not results_file.exists():
        print(f"Error: Results file not found at {results_file}")
        print("Please check the experiment ID and try again.")
        return

    # Run analysis
    analyzer = RustMizanSingleExperimentAnalyzer(experiment_id)
    analyzer.run_analysis()


if __name__ == "__main__":
    main()
