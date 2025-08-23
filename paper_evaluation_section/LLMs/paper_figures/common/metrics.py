from typing import Dict, List, Any, Set, Tuple
import pandas as pd


def binary_accuracy(predicted: bool, actual: bool) -> float:
    """Compute binary classification accuracy for a single sample."""
    return 1.0 if predicted == actual else 0.0


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


def compute_set_metrics(
    predicted_set: Set[Any], actual_set: Set[Any]
) -> Tuple[int, int, int]:
    """
    Compute TP/FP/FN for set-based comparison using intersection.

    Args:
        predicted_set: Set of predicted items
        actual_set: Set of ground truth items

    Returns:
        Tuple of (tp, fp, fn)
    """
    tp = len(predicted_set & actual_set)  # Intersection
    fp = len(predicted_set - actual_set)  # Predicted but not actual
    fn = len(actual_set - predicted_set)  # Actual but not predicted
    return tp, fp, fn


def success_at_1_function(func_tp: int, is_vulnerable: bool) -> float:
    """
    Compute Success@1 for function localization.
    Only applies to vulnerable samples.

    Args:
        func_tp: Number of correctly identified functions
        is_vulnerable: Whether the sample is actually vulnerable

    Returns:
        1.0 if vulnerable sample with at least one correct function, 0.0 otherwise
    """
    return 1.0 if (is_vulnerable and func_tp > 0) else 0.0


def success_at_1_line(line_tp: int, is_vulnerable: bool) -> float:
    """
    Compute Success@1 for line localization.
    Only applies to vulnerable samples.

    Args:
        line_tp: Number of correctly identified lines
        is_vulnerable: Whether the sample is actually vulnerable

    Returns:
        1.0 if vulnerable sample with at least one correct line, 0.0 otherwise
    """
    return 1.0 if (is_vulnerable and line_tp > 0) else 0.0


def compute_sample_metrics(sample: Dict[str, Any]) -> Dict[str, Any]:
    """
    Compute all metrics for a single sample.

    Args:
        sample: Dictionary containing model outputs and reference outputs

    Returns:
        Dictionary with all computed metrics for the sample
    """
    predicted = sample["outputs"]["parsed_response"]
    actual = sample["reference_outputs"]
    is_vulnerable = actual["is_vulnerable"]

    # 1. Binary vulnerability detection
    bin_accuracy = binary_accuracy(predicted["is_vulnerable"], is_vulnerable)

    # 2. CWE type classification (multi-label)
    cwe_pred_set = set(predicted["cwe_type"])
    cwe_actual_set = set(actual["cwe_type"])
    cwe_tp, cwe_fp, cwe_fn = compute_set_metrics(cwe_pred_set, cwe_actual_set)

    # 3. Function localization
    func_pred_set = {
        (file_path, func)
        for file_path, functions in predicted["vulnerable_functions"].items()
        for func in functions
    }
    func_actual_set = {
        (file_path, func)
        for file_path, functions in actual["vulnerable_functions"].items()
        for func in functions
    }
    func_tp, func_fp, func_fn = compute_set_metrics(func_pred_set, func_actual_set)

    # 4. Line localization
    line_pred_set = {
        (file_path, line)
        for file_path, lines in predicted["vulnerable_lines"].items()
        for line in lines
    }
    line_actual_set = {
        (file_path, line)
        for file_path, lines in actual["vulnerable_lines"].items()
        for line in lines
    }
    line_tp, line_fp, line_fn = compute_set_metrics(line_pred_set, line_actual_set)

    # 5. Success@1 metrics (only for vulnerable samples)
    success_at_1_func = success_at_1_function(func_tp, is_vulnerable)
    success_at_1_line_score = success_at_1_line(line_tp, is_vulnerable)

    return {
        # Binary classification
        "binary_accuracy": bin_accuracy,
        # CWE classification
        "cwe_tp": cwe_tp,
        "cwe_fp": cwe_fp,
        "cwe_fn": cwe_fn,
        # Function localization
        "function_tp": func_tp,
        "function_fp": func_fp,
        "function_fn": func_fn,
        # Line localization
        "line_tp": line_tp,
        "line_fp": line_fp,
        "line_fn": line_fn,
        # Success@1 metrics
        "success_at_1_function": success_at_1_func,
        "success_at_1_line": success_at_1_line_score,
    }


def load_experiment_data(
    experiment_ids: List[str], model_names: List[str] = None
) -> Dict[str, pd.DataFrame]:
    """
    Load experiment data for multiple experiments, filtering to common valid samples.

    Args:
        experiment_ids: List of experiment IDs to load
        model_names: Optional list of model names (must match experiment_ids length)

    Returns:
        Dictionary mapping model names to their DataFrames
    """
    from .data_utils import (
        load_processed_experiment,
        find_common_samples,
        get_short_model_name,
    )

    if model_names and len(model_names) != len(experiment_ids):
        raise ValueError("model_names list must match experiment_ids list length")

    # Find samples that are valid across all experiments
    common_samples = find_common_samples(experiment_ids)

    results = {}
    for i, exp_id in enumerate(experiment_ids):
        df = load_processed_experiment(exp_id)
        # Filter to common samples with valid JSON
        common_df = df[df["example_id"].isin(common_samples)]
        valid_df = common_df[common_df["is_valid_json"] == True]

        model_name = model_names[i] if model_names else f"model_{i}"
        short_name = get_short_model_name(model_name) if model_names else model_name

        results[short_name] = valid_df

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

        # Micro-averaging: Sum TP/FP/FN across all samples
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
        success_at_1_func_rate = (
            vuln_samples["success_at_1_function"].mean()
            if len(vuln_samples) > 0
            else 0.0
        )
        success_at_1_func_hits = (
            int(vuln_samples["success_at_1_function"].sum())
            if len(vuln_samples) > 0
            else 0
        )
        success_at_1_line_rate = (
            vuln_samples["success_at_1_line"].mean() if len(vuln_samples) > 0 else 0.0
        )
        success_at_1_line_hits = (
            int(vuln_samples["success_at_1_line"].sum()) if len(vuln_samples) > 0 else 0
        )
        success_at_1_total = len(vuln_samples)

        metrics_list.append(
            {
                "Model": model_name,
                "Samples": len(df),
                # Binary classification (simple averaging)
                "Binary Accuracy": df["binary_accuracy"].mean(),
                # CWE classification (micro-averaged)
                "CWE Precision": compute_precision(cwe_tp_total, cwe_fp_total),
                "CWE Recall": compute_recall(cwe_tp_total, cwe_fn_total),
                "CWE F1": compute_f1_score(cwe_tp_total, cwe_fp_total, cwe_fn_total),
                # Function localization (micro-averaged)
                "Function Precision": compute_precision(func_tp_total, func_fp_total),
                "Function Recall": compute_recall(func_tp_total, func_fn_total),
                "Function F1": compute_f1_score(
                    func_tp_total, func_fp_total, func_fn_total
                ),
                # Line localization (micro-averaged)
                "Line Precision": compute_precision(line_tp_total, line_fp_total),
                "Line Recall": compute_recall(line_tp_total, line_fn_total),
                "Line F1": compute_f1_score(
                    line_tp_total, line_fp_total, line_fn_total
                ),
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
