# This module is copied from rust-mizan main github repository with minor modifications.
# Ideally, this module should be somehow shared between the two repositories to avoid code duplication.
from typing import Dict, List, Any
import pandas as pd


def compute_f1_score(tp: int, fp: int, fn: int) -> float:
    """
    Compute F1 score from true positives, false positives, and false negatives.

    Handles edge cases:
    - Perfect empty prediction for empty ground truth: F1 = 1.0
    - No correct predictions: F1 = 0.0

    Args:
        tp: True positives (correct predictions)
        fp: False positives (incorrect predictions)
        fn: False negatives (missed ground truth items)

    Returns:
        F1 score between 0.0 and 1.0
    """
    if tp == 0 and fp == 0 and fn == 0:
        return 1.0  # Perfect empty prediction for empty ground truth

    if tp == 0:
        return 0.0  # No correct predictions

    precision = tp / (tp + fp)
    recall = tp / (tp + fn)
    return 2 * precision * recall / (precision + recall)


def compute_precision(tp: int, fp: int) -> float:
    """Compute precision from TP and FP."""
    return tp / (tp + fp) if (tp + fp) > 0 else 0.0


def compute_recall(tp: int, fn: int) -> float:
    """Compute recall from TP and FN."""
    return tp / (tp + fn) if (tp + fn) > 0 else 0.0


def load_experiment_data(experiment_ids: List[str], model_names: List[str] = None) -> Dict[str, pd.DataFrame]:
    from .data_utils import load_processed_experiment, find_common_samples
    from src.leaderboard.read_evals import get_all_experiments_data

    if model_names and len(model_names) != len(experiment_ids):
        raise ValueError("model_names list must match experiment_ids list length")

    common_samples = find_common_samples(experiment_ids)
    all_experiments = get_all_experiments_data()
    exp_map = {exp["experiment_id"]: exp for exp in all_experiments}

    results = {}
    for i, exp_id in enumerate(experiment_ids):
        data_file = exp_map.get(exp_id, {}).get("data_file")
        df = load_processed_experiment(exp_id, data_file)
        common_df = df[df["example_id"].isin(common_samples)]
        model_name = model_names[i] if model_names else f"model_{i}"
        results[model_name] = common_df

    return results


def compute_aggregate_metrics(
    experiment_data: Dict[str, pd.DataFrame], vulnerable_only: bool = False
) -> List[Dict[str, Any]]:
    """
    Compute aggregate metrics across all samples using micro-averaging.

    Micro-averaging sums TP/FP/FN across all samples before computing metrics,
    giving appropriate weight to complex vulnerabilities with more elements.

    Args:
        experiment_data: Dictionary mapping model names to their DataFrames
        vulnerable_only: If True, only include vulnerable samples in calculations

    Returns:
        List of dictionaries with aggregate metrics for each model
    """
    metrics_list = []

    for model_name, df in experiment_data.items():
        if vulnerable_only:
            df = df[df["is_vulnerable_gt"] == True]

        if len(df) == 0:
            # Handle empty datasets
            metrics_list.append(
                {
                    "Model": model_name,
                    "Samples": 0,
                    "Invalid JSON Count": 0,
                    "Invalid JSON Rate": 0.0,
                    "Binary Accuracy": 0.0,
                    "CWE Precision": 0.0,
                    "CWE Recall": 0.0,
                    "CWE F1": 0.0,
                    "Function Precision": 0.0,
                    "Function Recall": 0.0,
                    "Function F1": 0.0,
                    "Line Precision": 0.0,
                    "Line Recall": 0.0,
                    "Line F1": 0.0,
                    "Success@1-Function": 0.0,
                    "Success@1-Function Hits": 0,
                    "Success@1-Function Total": 0,
                    "Success@1-Line": 0.0,
                    "Success@1-Line Hits": 0,
                    "Success@1-Line Total": 0,
                }
            )
            continue

        # Calculate invalid JSON metrics
        total_samples = len(df)
        invalid_json_count = len(df[df["is_valid_json"] == False])
        invalid_json_rate = invalid_json_count / total_samples if total_samples > 0 else 0.0

        # Micro-averaging: Sum TP/FP/FN across all samples (invalid JSONs contribute zeros)
        cwe_tp_total = df["cwe_tp"].sum()
        cwe_fp_total = df["cwe_fp"].sum()
        cwe_fn_total = df["cwe_fn"].sum()

        func_tp_total = df["function_tp"].sum()
        func_fp_total = df["function_fp"].sum()
        func_fn_total = df["function_fn"].sum()

        line_tp_total = df["line_tp"].sum()
        line_fp_total = df["line_fp"].sum()
        line_fn_total = df["line_fn"].sum()

        # Success@1 metrics only for vulnerable samples
        vuln_samples = df[df["is_vulnerable_gt"] == True]
        success_at_1_func_rate = vuln_samples["success_at_1_function"].mean() if len(vuln_samples) > 0 else 0.0
        success_at_1_func_hits = int(vuln_samples["success_at_1_function"].sum()) if len(vuln_samples) > 0 else 0
        success_at_1_line_rate = vuln_samples["success_at_1_line"].mean() if len(vuln_samples) > 0 else 0.0
        success_at_1_line_hits = int(vuln_samples["success_at_1_line"].sum()) if len(vuln_samples) > 0 else 0
        success_at_1_total = len(vuln_samples)

        metrics_list.append(
            {
                "Model": model_name,
                "Samples": total_samples,
                # Invalid JSON metrics
                "Invalid JSON Count": invalid_json_count,
                "Invalid JSON Rate": invalid_json_rate,
                # Binary classification (simple averaging)
                "Binary Accuracy": df["binary_accuracy"].mean(),
                # CWE classification (micro-averaged)
                "CWE Precision": compute_precision(cwe_tp_total, cwe_fp_total),
                "CWE Recall": compute_recall(cwe_tp_total, cwe_fn_total),
                "CWE F1": compute_f1_score(cwe_tp_total, cwe_fp_total, cwe_fn_total),
                # Function localization (micro-averaged)
                "Function Precision": compute_precision(func_tp_total, func_fp_total),
                "Function Recall": compute_recall(func_tp_total, func_fn_total),
                "Function F1": compute_f1_score(func_tp_total, func_fp_total, func_fn_total),
                # Line localization (micro-averaged)
                "Line Precision": compute_precision(line_tp_total, line_fp_total),
                "Line Recall": compute_recall(line_tp_total, line_fn_total),
                "Line F1": compute_f1_score(line_tp_total, line_fp_total, line_fn_total),
                # Success@1 metrics (only for vulnerable samples)
                "Success@1-Function": success_at_1_func_rate,
                "Success@1-Function Hits": success_at_1_func_hits,
                "Success@1-Function Total": success_at_1_total,
                "Success@1-Line": success_at_1_line_rate,
                "Success@1-Line Hits": success_at_1_line_hits,
                "Success@1-Line Total": success_at_1_total,
            }
        )

    return metrics_list


def compute_micro_averaged_f1(df: pd.DataFrame, metric_type: str = "function") -> float:
    """
    Compute micro-averaged F1 score by summing TP/FP/FN across samples.

    This is the standard approach for aggregating F1 scores in multi-label
    classification and information retrieval tasks.

    Args:
        df: DataFrame containing TP/FP/FN columns
        metric_type: Type of metric ("function", "cwe", "line")

    Returns:
        Micro-averaged F1 score
    """
    if len(df) == 0:
        return 0.0

    tp_col = f"{metric_type}_tp"
    fp_col = f"{metric_type}_fp"
    fn_col = f"{metric_type}_fn"

    tp_total = df[tp_col].sum()
    fp_total = df[fp_col].sum()
    fn_total = df[fn_col].sum()

    return compute_f1_score(tp_total, fp_total, fn_total)


def compute_success_at_1_function_rate(df: pd.DataFrame) -> float:
    """
    Compute Success@1 rate for function localization.
    Only considers vulnerable samples.
    """
    if len(df) == 0:
        return 0.0

    vuln_df = df[df["is_vulnerable_gt"] == True]
    if len(vuln_df) == 0:
        return 0.0

    return vuln_df["success_at_1_function"].mean()


def compute_success_at_1_line_rate(df: pd.DataFrame) -> float:
    """
    Compute Success@1 rate for line localization.
    Only considers vulnerable samples.
    """
    if len(df) == 0:
        return 0.0

    vuln_df = df[df["is_vulnerable_gt"] == True]
    if len(vuln_df) == 0:
        return 0.0

    return vuln_df["success_at_1_line"].mean()
