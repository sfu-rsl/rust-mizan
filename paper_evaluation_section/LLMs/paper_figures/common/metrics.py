from typing import Dict, List, Any, Set
import pandas as pd


def binary_accuracy(predicted: bool, actual: bool) -> float:
    return 1.0 if predicted == actual else 0.0


def f1_score_from_tp_fp_fn(tp: int, fp: int, fn: int) -> float:
    # Perfect case: no predicted, no actual
    if tp + fp == 0 and tp + fn == 0:
        return 1.0

    # No predictions made
    if tp + fp == 0:
        return 0.0

    # No actual positives exist (but we made predictions)
    if tp + fn == 0:
        return 0.0 if fp > 0 else 1.0

    precision = tp / (tp + fp)
    recall = tp / (tp + fn)

    if precision + recall == 0:
        return 0.0

    return 2 * precision * recall / (precision + recall)


def compute_tp_fp_fn_sets(
    predicted_set: Set[str], actual_set: Set[str]
) -> tuple[int, int, int]:
    tp = len(predicted_set & actual_set)
    fp = len(predicted_set - actual_set)
    fn = len(actual_set - predicted_set)
    return tp, fp, fn


def hit_at_1_function_from_tp(tp: int) -> float:
    return 1.0 if tp > 0 else 0.0


def compute_all_sample_metrics(sample: Dict[str, Any]) -> Dict[str, Any]:
    predicted = sample["outputs"]["parsed_response"]
    actual = sample["reference_outputs"]

    # CWE TP/FP/FN
    cwe_pred_set = set(predicted["cwe_type"])
    cwe_actual_set = set(actual["cwe_type"])
    cwe_tp, cwe_fp, cwe_fn = compute_tp_fp_fn_sets(cwe_pred_set, cwe_actual_set)

    # Function TP/FP/FN
    func_pred_set = set()
    for file_path, functions in predicted["vulnerable_functions"].items():
        for func in functions:
            func_pred_set.add((file_path, func))

    func_actual_set = set()
    for file_path, functions in actual["vulnerable_functions"].items():
        for func in functions:
            func_actual_set.add((file_path, func))

    func_tp, func_fp, func_fn = compute_tp_fp_fn_sets(func_pred_set, func_actual_set)

    # Line TP/FP/FN
    line_pred_set = set()
    for file_path, lines in predicted["vulnerable_lines"].items():
        for line in lines:
            line_pred_set.add((file_path, line))

    line_actual_set = set()
    for file_path, lines in actual["vulnerable_lines"].items():
        for line in lines:
            line_actual_set.add((file_path, line))

    line_tp, line_fp, line_fn = compute_tp_fp_fn_sets(line_pred_set, line_actual_set)

    # Hit@1-Function only applies to vulnerable samples
    is_vulnerable_sample = actual["is_vulnerable"]
    hit_at_1_score = hit_at_1_function_from_tp(func_tp) if is_vulnerable_sample else 0.0

    return {
        "binary_accuracy": binary_accuracy(
            predicted["is_vulnerable"], actual["is_vulnerable"]
        ),
        "cwe_f1": f1_score_from_tp_fp_fn(cwe_tp, cwe_fp, cwe_fn),
        "cwe_tp": cwe_tp,
        "cwe_fp": cwe_fp,
        "cwe_fn": cwe_fn,
        "function_f1": f1_score_from_tp_fp_fn(func_tp, func_fp, func_fn),
        "function_tp": func_tp,
        "function_fp": func_fp,
        "function_fn": func_fn,
        "line_f1": f1_score_from_tp_fp_fn(line_tp, line_fp, line_fn),
        "line_tp": line_tp,
        "line_fp": line_fp,
        "line_fn": line_fn,
        "hit_at_1_function": hit_at_1_score,
    }


def compute_experiment_metrics(
    experiment_ids: List[str], model_names: List[str] = None
) -> Dict[str, pd.DataFrame]:
    from .data_utils import (
        load_processed_experiment,
        find_common_samples,
        get_short_model_name,
    )

    if model_names and len(model_names) != len(experiment_ids):
        raise ValueError("model_names list must match experiment_ids list length")

    common_samples = find_common_samples(experiment_ids)

    results = {}
    for i, exp_id in enumerate(experiment_ids):
        df = load_processed_experiment(exp_id)
        common_df = df[df["example_id"].isin(common_samples)]
        valid_df = common_df[common_df["is_valid_json"] == True]

        model_name = model_names[i] if model_names else f"model_{i}"
        short_name = get_short_model_name(model_name) if model_names else model_name

        results[short_name] = valid_df

    return results


def compute_aggregate_metrics(
    experiment_data: Dict[str, pd.DataFrame], vulnerable_only: bool = False
) -> List[Dict[str, Any]]:
    metrics_list = []

    for model_name, df in experiment_data.items():
        if vulnerable_only:
            df = df[df["is_vulnerable_gt"] == True]

        if len(df) == 0:
            metrics_list.append(
                {
                    "Model": model_name,
                    "Samples": 0,
                    "Binary Accuracy": 0.0,
                    "CWE F1": 0.0,
                    "Function F1": 0.0,
                    "Line F1": 0.0,
                    "Hit@1-Function": 0.0,
                    "Hit@1-Function Hits": 0,
                    "Hit@1-Function Total": 0,
                }
            )
            continue

        # Aggregate TP/FP/FN across all samples
        cwe_tp_total = df["cwe_tp"].sum()
        cwe_fp_total = df["cwe_fp"].sum()
        cwe_fn_total = df["cwe_fn"].sum()

        func_tp_total = df["function_tp"].sum()
        func_fp_total = df["function_fp"].sum()
        func_fn_total = df["function_fn"].sum()

        line_tp_total = df["line_tp"].sum()
        line_fp_total = df["line_fp"].sum()
        line_fn_total = df["line_fn"].sum()

        # Hit@1-Function only for vulnerable samples
        vuln_samples = df[df["is_vulnerable_gt"] == True]
        hit_at_1_avg = (
            vuln_samples["hit_at_1_function"].mean() if len(vuln_samples) > 0 else 0.0
        )
        hit_at_1_hits = (
            int(vuln_samples["hit_at_1_function"].sum()) if len(vuln_samples) > 0 else 0
        )
        hit_at_1_total = len(vuln_samples)

        metrics_list.append(
            {
                "Model": model_name,
                "Samples": len(df),
                "Binary Accuracy": df["binary_accuracy"].mean(),
                "CWE F1": f1_score_from_tp_fp_fn(
                    cwe_tp_total, cwe_fp_total, cwe_fn_total
                ),
                "Function F1": f1_score_from_tp_fp_fn(
                    func_tp_total, func_fp_total, func_fn_total
                ),
                "Line F1": f1_score_from_tp_fp_fn(
                    line_tp_total, line_fp_total, line_fn_total
                ),
                "Hit@1-Function": hit_at_1_avg,
                "Hit@1-Function Hits": hit_at_1_hits,
                "Hit@1-Function Total": hit_at_1_total,
            }
        )

    return metrics_list


def compute_aggregate_f1_from_dataframe(
    df: pd.DataFrame, metric_type: str = "function"
) -> float:
    """
    Compute aggregate F1 score from a DataFrame by summing TP/FP/FN and then computing F1.
    This is the correct way to aggregate F1 scores across multiple samples.

    Args:
        df: DataFrame containing TP/FP/FN columns
        metric_type: Type of metric ("function", "cwe", "line")

    Returns:
        Aggregate F1 score
    """
    if len(df) == 0:
        return 0.0

    tp_col = f"{metric_type}_tp"
    fp_col = f"{metric_type}_fp"
    fn_col = f"{metric_type}_fn"

    tp_total = df[tp_col].sum()
    fp_total = df[fp_col].sum()
    fn_total = df[fn_col].sum()

    return f1_score_from_tp_fp_fn(tp_total, fp_total, fn_total)


def compute_hit_at_1_function_from_dataframe(df: pd.DataFrame) -> float:
    """
    Compute hit@1-function score from a DataFrame.
    Only considers vulnerable samples, returns mean hit@1-function score.

    Args:
        df: DataFrame containing hit_at_1_function and is_vulnerable_gt columns

    Returns:
        Hit@1-function score (0.0 if no vulnerable samples)
    """
    if len(df) == 0:
        return 0.0

    # Filter to vulnerable samples only
    vuln_df = df[df["is_vulnerable_gt"] == True]
    if len(vuln_df) == 0:
        return 0.0

    return vuln_df["hit_at_1_function"].mean()
