#!/usr/bin/env python3
"""
RustMizan Multi-Experiment Comparison Analysis Script

This script compares performance across multiple RustMizan experiments for Rust vulnerability detection.
It generates a markdown report and comparison figures.

Usage: python multi_experiment_analyzer.py <output_dir> <experiment_id1> <experiment_id2> [experiment_id3] ...
"""

import json
import pandas as pd
import numpy as np
import matplotlib.pyplot as plt
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
    "#999999",
    "#66C2A5",
    "#FC8D62",
    "#8DA0CB",
    "#E78AC3",
    "#A6D854",
    "#FFD92F",
    "#E5C494",
    "#B3B3B3",
    "#1B9E77",
]
plt.rcParams["axes.prop_cycle"] = plt.cycler(color=colors_professional)


class RustMizanMultiExperimentAnalyzer:
    def __init__(self, experiment_specs: List[Tuple[str, str]], output_dir: str):
        self.experiment_specs = experiment_specs  # List of (name, id) tuples
        self.output_dir_name = output_dir
        self.experiments_data = {}
        self.comparison_df = None
        self.valid_json_comparison_df = None
        self.output_dir = None
        self.markdown_content = []

    def load_experiment_data(
        self, experiment_name: str, experiment_id: str
    ) -> Dict[str, Any]:
        """Load data for a single experiment."""
        experiment_path = (
            f"../evaluation_results/experiment_{experiment_id}/results.json"
        )
        metadata_path = (
            f"../evaluation_results/experiment_{experiment_id}/metadata.json"
        )

        print(f"Loading experiment {experiment_name} ({experiment_id})...")

        # Load results
        with open(experiment_path, "r") as f:
            results = json.load(f)

        # Load metadata
        try:
            with open(metadata_path, "r") as f:
                metadata = json.load(f)
        except FileNotFoundError:
            print(f"Warning: Metadata file not found for {experiment_id}")
            metadata = {}

        return {
            "experiment_name": experiment_name,
            "experiment_id": experiment_id,
            "results": results,
            "metadata": metadata,
        }

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

    def extract_lines_from_dict(self, lines_dict: Dict[str, List[str]]) -> set:
        """Extract all line references from the vulnerable_lines dictionary."""
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
        metrics["reference_is_vulnerable"] = sample["is_vulnerable"]

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

    def calculate_experiment_summary(
        self, experiment_data: Dict[str, Any]
    ) -> Dict[str, Any]:
        """Calculate summary statistics for a single experiment."""
        results = experiment_data["results"]
        metadata = experiment_data["metadata"]

        # Calculate metrics for all samples
        all_metrics = [self.calculate_sample_metrics(sample) for sample in results]
        df = pd.DataFrame(all_metrics)

        # Filter to vulnerable samples only
        vuln_df = df[df["reference_is_vulnerable"] == True]

        # Overall statistics
        total_samples = len(df)
        vulnerable_samples = len(vuln_df)
        json_validity_rate = df["json_validity"].mean()
        vulnerability_accuracy = df["is_vulnerable_correct"].mean()

        # At least one correct metrics (overall)
        at_least_one_cwe_count = (df["at_least_one_correct_cwe"] == 1).sum()
        at_least_one_cwe_rate = at_least_one_cwe_count / total_samples
        at_least_one_function_count = (df["at_least_one_correct_function"] == 1).sum()
        at_least_one_function_rate = at_least_one_function_count / total_samples
        at_least_one_line_count = (df["at_least_one_correct_line"] == 1).sum()
        at_least_one_line_rate = at_least_one_line_count / total_samples

        # F1 scores (from valid JSON samples only)
        valid_df = df[df["json_validity"] == 1]
        if len(valid_df) > 0:
            cwe_macro_f1 = valid_df["cwe_f1"].mean()
            functions_macro_f1 = valid_df["functions_f1"].mean()
            lines_macro_f1 = valid_df["lines_f1"].mean()
        else:
            cwe_macro_f1 = functions_macro_f1 = lines_macro_f1 = 0.0

        # Vulnerable samples only statistics
        if len(vuln_df) > 0:
            vuln_json_validity = vuln_df["json_validity"].mean()
            vuln_accuracy = vuln_df["is_vulnerable_correct"].mean()
            vuln_at_least_one_cwe_count = (
                vuln_df["at_least_one_correct_cwe"] == 1
            ).sum()
            vuln_at_least_one_cwe = vuln_at_least_one_cwe_count / len(vuln_df)
            vuln_at_least_one_function_count = (
                vuln_df["at_least_one_correct_function"] == 1
            ).sum()
            vuln_at_least_one_function = vuln_at_least_one_function_count / len(vuln_df)
            vuln_at_least_one_line_count = (
                vuln_df["at_least_one_correct_line"] == 1
            ).sum()
            vuln_at_least_one_line = vuln_at_least_one_line_count / len(vuln_df)

            valid_vuln_df = vuln_df[vuln_df["json_validity"] == 1]
            if len(valid_vuln_df) > 0:
                vuln_cwe_f1 = valid_vuln_df["cwe_f1"].mean()
                vuln_functions_f1 = valid_vuln_df["functions_f1"].mean()
                vuln_lines_f1 = valid_vuln_df["lines_f1"].mean()
            else:
                vuln_cwe_f1 = vuln_functions_f1 = vuln_lines_f1 = 0.0
        else:
            vuln_json_validity = vuln_accuracy = 0.0
            vuln_at_least_one_cwe_count = vuln_at_least_one_function_count = (
                vuln_at_least_one_line_count
            ) = 0
            vuln_at_least_one_cwe = vuln_at_least_one_function = (
                vuln_at_least_one_line
            ) = 0.0
            vuln_cwe_f1 = vuln_functions_f1 = vuln_lines_f1 = 0.0

        # Get model name and mutations
        model_name = metadata.get("model", "Unknown")
        mutations_applied = metadata.get("mutations_metadata", {}).get(
            "mutations_applied", []
        )
        mutations_text = (
            ", ".join(mutations_applied) if mutations_applied else "None (vanilla)"
        )

        return {
            "experiment_name": experiment_data["experiment_name"],
            "experiment_id": experiment_data["experiment_id"],
            "model_name": model_name,
            "mutations": mutations_text,
            "total_samples": total_samples,
            "vulnerable_samples": vulnerable_samples,
            # Overall metrics
            "json_validity_rate": json_validity_rate,
            "vulnerability_accuracy": vulnerability_accuracy,
            "at_least_one_cwe_rate": at_least_one_cwe_rate,
            "at_least_one_cwe_count": at_least_one_cwe_count,
            "at_least_one_function_rate": at_least_one_function_rate,
            "at_least_one_function_count": at_least_one_function_count,
            "at_least_one_line_rate": at_least_one_line_rate,
            "at_least_one_line_count": at_least_one_line_count,
            "cwe_macro_f1": cwe_macro_f1,
            "functions_macro_f1": functions_macro_f1,
            "lines_macro_f1": lines_macro_f1,
            # Vulnerable samples only metrics
            "vuln_json_validity": vuln_json_validity,
            "vuln_accuracy": vuln_accuracy,
            "vuln_at_least_one_cwe": vuln_at_least_one_cwe,
            "vuln_at_least_one_cwe_count": vuln_at_least_one_cwe_count,
            "vuln_at_least_one_function": vuln_at_least_one_function,
            "vuln_at_least_one_function_count": vuln_at_least_one_function_count,
            "vuln_at_least_one_line": vuln_at_least_one_line,
            "vuln_at_least_one_line_count": vuln_at_least_one_line_count,
            "vuln_cwe_f1": vuln_cwe_f1,
            "vuln_functions_f1": vuln_functions_f1,
            "vuln_lines_f1": vuln_lines_f1,
            # Store the full dataframe for cross-experiment analysis
            "dataframe": df,
        }

    def setup_output_directory(self):
        """Set up output directory for analysis results."""
        self.output_dir = Path(f"../evaluation_results/{self.output_dir_name}")
        self.output_dir.mkdir(exist_ok=True)

        # Create subdirectory for figures
        (self.output_dir / "figures").mkdir(exist_ok=True)

    def load_all_experiments(self):
        """Load data for all experiments."""
        print(f"Loading {len(self.experiment_specs)} experiments...")

        for experiment_name, experiment_id in self.experiment_specs:
            # Check if experiment exists
            experiment_dir = Path(f"../evaluation_results/experiment_{experiment_id}")
            if not experiment_dir.exists():
                print(f"Error: Experiment directory not found at {experiment_dir}")
                continue

            results_file = experiment_dir / "results.json"
            if not results_file.exists():
                print(f"Error: Results file not found at {results_file}")
                continue

            self.experiments_data[experiment_name] = self.load_experiment_data(
                experiment_name, experiment_id
            )

        print(f"Successfully loaded {len(self.experiments_data)} experiments")

    def create_comparison_dataframe(self):
        """Create a comparison dataframe with summary statistics for all experiments."""
        print("Creating comparison dataframe...")

        comparison_data = []
        for experiment_name, experiment_data in self.experiments_data.items():
            summary = self.calculate_experiment_summary(experiment_data)
            comparison_data.append(summary)
            # Store the dataframe in experiments_data for valid JSON analysis
            self.experiments_data[experiment_name]["dataframe"] = summary["dataframe"]

        self.comparison_df = pd.DataFrame(comparison_data)
        print(
            f"Created comparison dataframe with {len(self.comparison_df)} experiments"
        )

    def create_valid_json_analysis(self):
        """Create analysis for samples where all experiments have valid JSON responses."""
        print("Creating valid JSON analysis...")

        # Find common example_ids across all experiments that have valid JSON in all
        all_dataframes = {}
        for exp_name, exp_data in self.experiments_data.items():
            df = exp_data["dataframe"]
            valid_df = df[df["json_validity"] == 1]
            all_dataframes[exp_name] = valid_df.set_index("example_id")

        # Find intersection of valid example_ids across all experiments
        valid_example_ids = None
        for exp_name, df in all_dataframes.items():
            current_ids = set(df.index)
            if valid_example_ids is None:
                valid_example_ids = current_ids
            else:
                valid_example_ids = valid_example_ids.intersection(current_ids)

        print(
            f"Found {len(valid_example_ids)} samples with valid JSON across all experiments"
        )

        if len(valid_example_ids) == 0:
            print("No samples found with valid JSON across all experiments")
            self.valid_json_comparison_df = None
            return

        # Calculate metrics for this subset
        valid_json_data = []
        for exp_name, exp_data in self.experiments_data.items():
            df = exp_data["dataframe"]
            # Filter to only the common valid examples
            common_valid_df = df[
                df["example_id"].isin(valid_example_ids) & (df["json_validity"] == 1)
            ]

            if len(common_valid_df) == 0:
                continue

            # Calculate metrics for this subset
            total_samples = len(common_valid_df)
            vulnerability_accuracy = common_valid_df["is_vulnerable_correct"].mean()

            at_least_one_cwe_count = (
                common_valid_df["at_least_one_correct_cwe"] == 1
            ).sum()
            at_least_one_cwe_rate = at_least_one_cwe_count / total_samples
            at_least_one_function_count = (
                common_valid_df["at_least_one_correct_function"] == 1
            ).sum()
            at_least_one_function_rate = at_least_one_function_count / total_samples
            at_least_one_line_count = (
                common_valid_df["at_least_one_correct_line"] == 1
            ).sum()
            at_least_one_line_rate = at_least_one_line_count / total_samples

            cwe_macro_f1 = common_valid_df["cwe_f1"].mean()
            functions_macro_f1 = common_valid_df["functions_f1"].mean()
            lines_macro_f1 = common_valid_df["lines_f1"].mean()

            # Vulnerable samples only
            vuln_df = common_valid_df[
                common_valid_df["reference_is_vulnerable"] == True
            ]
            vuln_samples_count = len(vuln_df)
            if len(vuln_df) > 0:
                vuln_accuracy = vuln_df["is_vulnerable_correct"].mean()
                vuln_at_least_one_cwe_count = (
                    vuln_df["at_least_one_correct_cwe"] == 1
                ).sum()
                vuln_at_least_one_cwe = vuln_at_least_one_cwe_count / len(vuln_df)
                vuln_at_least_one_function_count = (
                    vuln_df["at_least_one_correct_function"] == 1
                ).sum()
                vuln_at_least_one_function = vuln_at_least_one_function_count / len(
                    vuln_df
                )
                vuln_at_least_one_line_count = (
                    vuln_df["at_least_one_correct_line"] == 1
                ).sum()
                vuln_at_least_one_line = vuln_at_least_one_line_count / len(vuln_df)
                vuln_cwe_f1 = vuln_df["cwe_f1"].mean()
                vuln_functions_f1 = vuln_df["functions_f1"].mean()
                vuln_lines_f1 = vuln_df["lines_f1"].mean()
            else:
                vuln_accuracy = 0.0
                vuln_at_least_one_cwe_count = vuln_at_least_one_function_count = (
                    vuln_at_least_one_line_count
                ) = 0
                vuln_at_least_one_cwe = vuln_at_least_one_function = (
                    vuln_at_least_one_line
                ) = 0.0
                vuln_cwe_f1 = vuln_functions_f1 = vuln_lines_f1 = 0.0
                vuln_samples_count = 0

            valid_json_data.append(
                {
                    "experiment_name": exp_name,
                    "total_valid_samples": total_samples,
                    "vulnerability_accuracy": vulnerability_accuracy,
                    "at_least_one_cwe_rate": at_least_one_cwe_rate,
                    "at_least_one_cwe_count": at_least_one_cwe_count,
                    "at_least_one_function_rate": at_least_one_function_rate,
                    "at_least_one_function_count": at_least_one_function_count,
                    "at_least_one_line_rate": at_least_one_line_rate,
                    "at_least_one_line_count": at_least_one_line_count,
                    "cwe_macro_f1": cwe_macro_f1,
                    "functions_macro_f1": functions_macro_f1,
                    "lines_macro_f1": lines_macro_f1,
                    "vuln_accuracy": vuln_accuracy,
                    "vuln_at_least_one_cwe": vuln_at_least_one_cwe,
                    "vuln_at_least_one_cwe_count": vuln_at_least_one_cwe_count,
                    "vuln_at_least_one_function": vuln_at_least_one_function,
                    "vuln_at_least_one_function_count": vuln_at_least_one_function_count,
                    "vuln_at_least_one_line": vuln_at_least_one_line,
                    "vuln_at_least_one_line_count": vuln_at_least_one_line_count,
                    "vuln_samples_count": vuln_samples_count,
                    "vuln_cwe_f1": vuln_cwe_f1,
                    "vuln_functions_f1": vuln_functions_f1,
                    "vuln_lines_f1": vuln_lines_f1,
                }
            )

        self.valid_json_comparison_df = pd.DataFrame(valid_json_data)
        print(
            f"Created valid JSON analysis with {len(self.valid_json_comparison_df)} experiments"
        )

    def add_report_header(self):
        """Add header and experiment overview to the report."""
        self.markdown_content.append("# RustMizan Multi-Experiment Comparison Report\n")

        self.markdown_content.append("## Experiment Overview\n")
        self.markdown_content.append(
            "| Experiment Name | Experiment ID | Model | Mutations Applied |"
        )
        self.markdown_content.append(
            "|-----------------|---------------|-------|-------------------|"
        )

        for _, row in self.comparison_df.iterrows():
            mutations_short = (
                row["mutations"][:80] + "..."
                if len(row["mutations"]) > 80
                else row["mutations"]
            )
            self.markdown_content.append(
                f"| {row['experiment_name']} | {row['experiment_id']} | {row['model_name']} | {mutations_short} |"
            )

        self.markdown_content.append("")

    def add_metric_definitions(self):
        """Add metric definitions to the report."""
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
            "- **Macro F1 Score:** Average F1 score across all samples for the given task (CWE/Functions/Lines)\n"
        )

    def create_comparison_tables(self):
        """Create comparison tables for all experiments."""
        print("Creating comparison tables...")

        # Overall Performance Comparison
        self.markdown_content.append("## Overall Performance Comparison\n")
        self.markdown_content.append(
            "| Experiment | JSON Validity | Vulnerability Accuracy | At Least One Correct CWE Identified | At Least One Correct Vulnerable Function Identified | At Least One Correct Vulnerable Line Identified |"
        )
        self.markdown_content.append(
            "|------------|---------------|------------------------|--------------------------------------|-----------------------------------------------------|--------------------------------------------------|"
        )

        for _, row in self.comparison_df.iterrows():
            self.markdown_content.append(
                f"| {row['experiment_name']} | {row['json_validity_rate']:.1%} | {row['vulnerability_accuracy']:.1%} | "
                f"{row['at_least_one_cwe_rate']:.1%} ({int(row['at_least_one_cwe_count'])} out of {int(row['total_samples'])}) | "
                f"{row['at_least_one_function_rate']:.1%} ({int(row['at_least_one_function_count'])} out of {int(row['total_samples'])}) | "
                f"{row['at_least_one_line_rate']:.1%} ({int(row['at_least_one_line_count'])} out of {int(row['total_samples'])}) |"
            )

        # F1 Scores Comparison
        self.markdown_content.append("\n### F1 Score Comparison\n")
        self.markdown_content.append(
            "| Experiment | CWE Macro F1 | Functions Macro F1 | Lines Macro F1 |"
        )
        self.markdown_content.append(
            "|------------|--------------|-------------------|----------------|"
        )

        for _, row in self.comparison_df.iterrows():
            self.markdown_content.append(
                f"| {row['experiment_name']} | {row['cwe_macro_f1']:.3f} | {row['functions_macro_f1']:.3f} | {row['lines_macro_f1']:.3f} |"
            )

        # Vulnerable Samples Only Comparison
        self.markdown_content.append("\n## Vulnerable Samples Only Comparison\n")
        self.markdown_content.append(
            "*Performance metrics calculated only on samples that contain vulnerabilities*\n"
        )
        self.markdown_content.append(
            "| Experiment | JSON Validity | Vulnerability Accuracy | At Least One Correct CWE Identified | At Least One Correct Vulnerable Function Identified | At Least One Correct Vulnerable Line Identified |"
        )
        self.markdown_content.append(
            "|------------|---------------|------------------------|--------------------------------------|-----------------------------------------------------|--------------------------------------------------|"
        )

        for _, row in self.comparison_df.iterrows():
            self.markdown_content.append(
                f"| {row['experiment_name']} | {row['vuln_json_validity']:.1%} | {row['vuln_accuracy']:.1%} | "
                f"{row['vuln_at_least_one_cwe']:.1%} ({int(row['vuln_at_least_one_cwe_count'])} out of {int(row['vulnerable_samples'])}) | "
                f"{row['vuln_at_least_one_function']:.1%} ({int(row['vuln_at_least_one_function_count'])} out of {int(row['vulnerable_samples'])}) | "
                f"{row['vuln_at_least_one_line']:.1%} ({int(row['vuln_at_least_one_line_count'])} out of {int(row['vulnerable_samples'])}) |"
            )

        # Vulnerable Samples F1 Scores
        self.markdown_content.append(
            "\n### F1 Score Comparison (Vulnerable Samples Only)\n"
        )
        self.markdown_content.append(
            "| Experiment | CWE Macro F1 | Functions Macro F1 | Lines Macro F1 |"
        )
        self.markdown_content.append(
            "|------------|--------------|-------------------|----------------|"
        )

        for _, row in self.comparison_df.iterrows():
            self.markdown_content.append(
                f"| {row['experiment_name']} | {row['vuln_cwe_f1']:.3f} | {row['vuln_functions_f1']:.3f} | {row['vuln_lines_f1']:.3f} |"
            )

        # Add Valid JSON Analysis section
        if (
            self.valid_json_comparison_df is not None
            and len(self.valid_json_comparison_df) > 0
        ):
            self.markdown_content.append(
                "\n## Analysis of Samples with Valid JSON Across All Experiments\n"
            )
            self.markdown_content.append(
                f"*Analysis focused only on the {self.valid_json_comparison_df.iloc[0]['total_valid_samples']} samples where all experiments produced valid JSON responses*\n"
            )

            self.markdown_content.append(
                "| Experiment | Vulnerability Accuracy | At Least One Correct CWE Identified | At Least One Correct Vulnerable Function Identified | At Least One Correct Vulnerable Line Identified |"
            )
            self.markdown_content.append(
                "|------------|------------------------|--------------------------------------|-----------------------------------------------------|--------------------------------------------------|"
            )

            for _, row in self.valid_json_comparison_df.iterrows():
                self.markdown_content.append(
                    f"| {row['experiment_name']} | {row['vulnerability_accuracy']:.1%} | "
                    f"{row['at_least_one_cwe_rate']:.1%} ({int(row['at_least_one_cwe_count'])} out of {int(row['total_valid_samples'])}) | "
                    f"{row['at_least_one_function_rate']:.1%} ({int(row['at_least_one_function_count'])} out of {int(row['total_valid_samples'])}) | "
                    f"{row['at_least_one_line_rate']:.1%} ({int(row['at_least_one_line_count'])} out of {int(row['total_valid_samples'])}) |"
                )

            self.markdown_content.append(
                "\n### F1 Score Comparison (Valid JSON Samples)\n"
            )
            self.markdown_content.append(
                "| Experiment | CWE Macro F1 | Functions Macro F1 | Lines Macro F1 |"
            )
            self.markdown_content.append(
                "|------------|--------------|-------------------|----------------|"
            )

            for _, row in self.valid_json_comparison_df.iterrows():
                self.markdown_content.append(
                    f"| {row['experiment_name']} | {row['cwe_macro_f1']:.3f} | {row['functions_macro_f1']:.3f} | {row['lines_macro_f1']:.3f} |"
                )

            self.markdown_content.append(
                "\n### Vulnerable Samples Performance (Valid JSON Samples)\n"
            )
            self.markdown_content.append(
                "| Experiment | Vulnerability Accuracy | At Least One Correct CWE Identified | At Least One Correct Vulnerable Function Identified | At Least One Correct Vulnerable Line Identified |"
            )
            self.markdown_content.append(
                "|------------|------------------------|--------------------------------------|-----------------------------------------------------|--------------------------------------------------|"
            )

            for _, row in self.valid_json_comparison_df.iterrows():
                self.markdown_content.append(
                    f"| {row['experiment_name']} | {row['vuln_accuracy']:.1%} | "
                    f"{row['vuln_at_least_one_cwe']:.1%} ({int(row['vuln_at_least_one_cwe_count'])} out of {int(row['vuln_samples_count'])}) | "
                    f"{row['vuln_at_least_one_function']:.1%} ({int(row['vuln_at_least_one_function_count'])} out of {int(row['vuln_samples_count'])}) | "
                    f"{row['vuln_at_least_one_line']:.1%} ({int(row['vuln_at_least_one_line_count'])} out of {int(row['vuln_samples_count'])}) |"
                )

        self.markdown_content.append("")

    def create_visualizations(self):
        """Create comparison visualizations."""
        print("Creating visualizations...")

        # Figure 1: Overall Performance Comparison - Simple Metrics
        fig, ax = plt.subplots(1, 1, figsize=(14, 8))

        metrics = [
            "JSON\nValidity",
            "Vulnerability\nAccuracy",
            "At Least One\nCWE",
            "At Least One\nFunction",
            "At Least One\nLine",
        ]
        x = np.arange(len(metrics))

        # Limit number of experiments shown to avoid overcrowding
        n_experiments = len(self.comparison_df)
        if n_experiments > 8:
            print(
                f"Warning: {n_experiments} experiments is a lot for visualization. Consider using fewer experiments for clearer plots."
            )

        width = 0.8 / n_experiments

        for i, (_, row) in enumerate(self.comparison_df.iterrows()):
            values = [
                row["json_validity_rate"],
                row["vulnerability_accuracy"],
                row["at_least_one_cwe_rate"],
                row["at_least_one_function_rate"],
                row["at_least_one_line_rate"],
            ]

            ax.bar(
                x + i * width, values, width, label=row["experiment_name"], alpha=0.8
            )

        ax.set_title(
            "Overall Performance Comparison: Simple Metrics",
            fontsize=14,
            fontweight="bold",
        )
        ax.set_ylabel("Rate", fontsize=12)
        ax.set_xticks(x + width * (n_experiments - 1) / 2)
        ax.set_xticklabels(metrics)
        ax.legend(bbox_to_anchor=(1.05, 1), loc="upper left")
        ax.set_ylim(0, 1)

        plt.tight_layout()
        plt.savefig(
            self.output_dir / "figures" / "figure1_overall_comparison.png",
            dpi=300,
            bbox_inches="tight",
        )
        plt.close()

        # Figure 2: F1 Score Comparison
        fig, ax = plt.subplots(1, 1, figsize=(12, 8))

        f1_metrics = ["CWE\nMacro F1", "Functions\nMacro F1", "Lines\nMacro F1"]
        x = np.arange(len(f1_metrics))

        for i, (_, row) in enumerate(self.comparison_df.iterrows()):
            f1_values = [
                row["cwe_macro_f1"],
                row["functions_macro_f1"],
                row["lines_macro_f1"],
            ]
            ax.bar(
                x + i * width, f1_values, width, label=row["experiment_name"], alpha=0.8
            )

        ax.set_title("F1 Score Comparison", fontsize=14, fontweight="bold")
        ax.set_ylabel("F1 Score", fontsize=12)
        ax.set_xticks(x + width * (n_experiments - 1) / 2)
        ax.set_xticklabels(f1_metrics)
        ax.legend(bbox_to_anchor=(1.05, 1), loc="upper left")

        # Set y-axis to 0-1 for F1 scores
        ax.set_ylim(0, 1)

        plt.tight_layout()
        plt.savefig(
            self.output_dir / "figures" / "figure2_f1_comparison.png",
            dpi=300,
            bbox_inches="tight",
        )
        plt.close()

        # Figure 3: Vulnerable Samples Only Comparison
        fig, ax = plt.subplots(1, 1, figsize=(14, 8))

        vuln_metrics = [
            "JSON\nValidity",
            "Vulnerability\nAccuracy",
            "At Least One\nCWE",
            "At Least One\nFunction",
            "At Least One\nLine",
        ]
        x = np.arange(len(vuln_metrics))

        for i, (_, row) in enumerate(self.comparison_df.iterrows()):
            vuln_values = [
                row["vuln_json_validity"],
                row["vuln_accuracy"],
                row["vuln_at_least_one_cwe"],
                row["vuln_at_least_one_function"],
                row["vuln_at_least_one_line"],
            ]

            ax.bar(
                x + i * width,
                vuln_values,
                width,
                label=row["experiment_name"],
                alpha=0.8,
            )

        ax.set_title(
            "Performance on Vulnerable Samples Only", fontsize=14, fontweight="bold"
        )
        ax.set_ylabel("Rate", fontsize=12)
        ax.set_xticks(x + width * (n_experiments - 1) / 2)
        ax.set_xticklabels(vuln_metrics)
        ax.legend(bbox_to_anchor=(1.05, 1), loc="upper left")
        ax.set_ylim(0, 1)

        plt.tight_layout()
        plt.savefig(
            self.output_dir / "figures" / "figure3_vulnerable_comparison.png",
            dpi=300,
            bbox_inches="tight",
        )
        plt.close()

        # Figure 4: Vulnerable Samples F1 Comparison
        fig, ax = plt.subplots(1, 1, figsize=(12, 8))

        x = np.arange(len(f1_metrics))  # Reset x for this figure

        for i, (_, row) in enumerate(self.comparison_df.iterrows()):
            vuln_f1_values = [
                row["vuln_cwe_f1"],
                row["vuln_functions_f1"],
                row["vuln_lines_f1"],
            ]
            ax.bar(
                x + i * width,
                vuln_f1_values,
                width,
                label=row["experiment_name"],
                alpha=0.8,
            )

        ax.set_title(
            "F1 Score Comparison (Vulnerable Samples Only)",
            fontsize=14,
            fontweight="bold",
        )
        ax.set_ylabel("F1 Score", fontsize=12)
        ax.set_xticks(x + width * (n_experiments - 1) / 2)
        ax.set_xticklabels(f1_metrics)
        ax.legend(bbox_to_anchor=(1.05, 1), loc="upper left")

        # Set y-axis to 0-1 for F1 scores
        ax.set_ylim(0, 1)

        plt.tight_layout()
        plt.savefig(
            self.output_dir / "figures" / "figure4_vulnerable_f1_comparison.png",
            dpi=300,
            bbox_inches="tight",
        )
        plt.close()

        # Figure 5 & 6: Valid JSON Analysis (if data exists)
        if (
            self.valid_json_comparison_df is not None
            and len(self.valid_json_comparison_df) > 0
        ):
            # Figure 5: Valid JSON Analysis - Simple Metrics
            fig, ax = plt.subplots(1, 1, figsize=(14, 8))

            valid_metrics = [
                "Vulnerability\nAccuracy",
                "At Least One\nCWE",
                "At Least One\nFunction",
                "At Least One\nLine",
            ]
            x = np.arange(len(valid_metrics))

            for i, (_, row) in enumerate(self.valid_json_comparison_df.iterrows()):
                valid_values = [
                    row["vulnerability_accuracy"],
                    row["at_least_one_cwe_rate"],
                    row["at_least_one_function_rate"],
                    row["at_least_one_line_rate"],
                ]

                ax.bar(
                    x + i * width,
                    valid_values,
                    width,
                    label=row["experiment_name"],
                    alpha=0.8,
                )

            ax.set_title(
                f"Performance on Samples with Valid JSON Across All Experiments\n({self.valid_json_comparison_df.iloc[0]['total_valid_samples']} samples)",
                fontsize=14,
                fontweight="bold",
            )
            ax.set_ylabel("Rate", fontsize=12)
            ax.set_xticks(x + width * (n_experiments - 1) / 2)
            ax.set_xticklabels(valid_metrics)
            ax.legend(bbox_to_anchor=(1.05, 1), loc="upper left")
            ax.set_ylim(0, 1)

            plt.tight_layout()
            plt.savefig(
                self.output_dir / "figures" / "figure5_valid_json_comparison.png",
                dpi=300,
                bbox_inches="tight",
            )
            plt.close()

            # Figure 6: Valid JSON Analysis - F1 Scores
            fig, ax = plt.subplots(1, 1, figsize=(12, 8))

            x = np.arange(len(f1_metrics))

            for i, (_, row) in enumerate(self.valid_json_comparison_df.iterrows()):
                valid_f1_values = [
                    row["cwe_macro_f1"],
                    row["functions_macro_f1"],
                    row["lines_macro_f1"],
                ]
                ax.bar(
                    x + i * width,
                    valid_f1_values,
                    width,
                    label=row["experiment_name"],
                    alpha=0.8,
                )

            ax.set_title(
                f"F1 Score Comparison on Samples with Valid JSON Across All Experiments\n({self.valid_json_comparison_df.iloc[0]['total_valid_samples']} samples)",
                fontsize=14,
                fontweight="bold",
            )
            ax.set_ylabel("F1 Score", fontsize=12)
            ax.set_xticks(x + width * (n_experiments - 1) / 2)
            ax.set_xticklabels(f1_metrics)
            ax.legend(bbox_to_anchor=(1.05, 1), loc="upper left")

            # Set y-axis to 0-1 for F1 scores
            ax.set_ylim(0, 1)

            plt.tight_layout()
            plt.savefig(
                self.output_dir / "figures" / "figure6_valid_json_f1_comparison.png",
                dpi=300,
                bbox_inches="tight",
            )
            plt.close()

        print("✓ All visualizations created and saved")

    def save_markdown_report(self):
        """Save the markdown report to file."""
        # Add figures section
        self.markdown_content.append("## Figures\n")
        self.markdown_content.append(
            "The following figures have been generated and saved to the `figures/` directory:\n"
        )
        self.markdown_content.append(
            "1. **figure1_overall_comparison.png** - Overall Performance Comparison: Simple Metrics"
        )
        self.markdown_content.append(
            "2. **figure2_f1_comparison.png** - F1 Score Comparison"
        )
        self.markdown_content.append(
            "3. **figure3_vulnerable_comparison.png** - Performance on Vulnerable Samples Only"
        )
        self.markdown_content.append(
            "4. **figure4_vulnerable_f1_comparison.png** - F1 Score Comparison (Vulnerable Samples Only)"
        )

        if (
            self.valid_json_comparison_df is not None
            and len(self.valid_json_comparison_df) > 0
        ):
            self.markdown_content.append(
                "5. **figure5_valid_json_comparison.png** - Performance on Samples with Valid JSON Across All Experiments"
            )
            self.markdown_content.append(
                "6. **figure6_valid_json_f1_comparison.png** - F1 Score Comparison on Samples with Valid JSON Across All Experiments"
            )

        self.markdown_content.append("")

        # Save markdown report
        report_path = self.output_dir / "comparison_report.md"
        with open(report_path, "w") as f:
            f.write("\n".join(self.markdown_content))

        print(f"✓ Markdown report saved to: {report_path}")

    def run_analysis(self):
        """Run the complete multi-experiment comparison analysis."""
        print(f"Starting RustMizan multi-experiment comparison analysis...")
        experiment_names = [name for name, _ in self.experiment_specs]
        print(
            f"Comparing {len(self.experiment_specs)} experiments: {', '.join(experiment_names)}"
        )

        # Setup and data loading
        self.setup_output_directory()
        self.load_all_experiments()
        self.create_comparison_dataframe()

        # Generate report content
        self.add_report_header()
        self.add_metric_definitions()
        self.create_valid_json_analysis()
        self.create_comparison_tables()

        # Generate outputs
        self.create_visualizations()
        self.save_markdown_report()

        print(f"\n✅ Multi-experiment analysis complete!")
        print(f"📂 Results saved to: {self.output_dir}")
        print(f"📊 Figures saved to: {self.output_dir}/figures/")
        print(f"📝 Report saved to: {self.output_dir}/comparison_report.md")


def main():
    """Main function to run the multi-experiment comparison analysis."""
    parser = argparse.ArgumentParser(
        description="Compare multiple RustMizan experiment results",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  python multi_experiment_analyzer.py model_comparison gpt4:exp1 claude:exp2 llama:exp3
        """,
    )
    parser.add_argument("output_dir", help="Name of the output directory to create")
    parser.add_argument(
        "experiments",
        nargs="+",
        help="Experiments in format 'name:id' (e.g., 'vanilla:5edf96b1')",
    )

    args = parser.parse_args()

    if len(args.experiments) < 2:
        print("Error: At least 2 experiments are required for comparison.")
        return

    # Parse experiment specifications
    experiment_specs = []
    for exp_spec in args.experiments:
        if ":" not in exp_spec:
            print(
                f"Error: Invalid experiment format '{exp_spec}'. Expected format: 'name:id'"
            )
            return

        name, exp_id = exp_spec.split(":", 1)
        experiment_specs.append((name, exp_id))

    # Check if all experiment directories exist
    missing_experiments = []
    for name, exp_id in experiment_specs:
        experiment_dir = Path(f"../evaluation_results/experiment_{exp_id}")
        if not experiment_dir.exists():
            missing_experiments.append((name, exp_id))

    if missing_experiments:
        print(f"Error: The following experiment directories were not found:")
        for name, exp_id in missing_experiments:
            print(f"  - {name} ({exp_id}): ../evaluation_results/experiment_{exp_id}")
        print("Please check the experiment IDs and try again.")
        return

    # Run analysis
    analyzer = RustMizanMultiExperimentAnalyzer(experiment_specs, args.output_dir)
    analyzer.run_analysis()


if __name__ == "__main__":
    main()
