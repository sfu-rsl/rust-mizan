"""Helper functions for evaluation metrics to reduce code duplication."""

from typing import Dict, Any, List, Set, Tuple, Callable
from sklearn.metrics import precision_score, recall_score, f1_score


def handle_errors(outputs: Dict[str, Any], metric_key: str) -> Dict[str, Any] | None:
    """Check for errors and return error response if found, None otherwise."""
    if outputs.get("errors") is not None:
        return {
            "key": metric_key,
            "score": 0,
            "comment": "Target function failed - no prediction available",
        }
    return None


def extract_sets_from_data(
    outputs: Dict[str, Any], reference_outputs: Dict[str, Any], field_name: str
) -> Tuple[Set, Set]:
    """Extract predicted and expected sets from the data based on field type."""
    if field_name == "cwe_type":
        predicted = set(outputs.get("parsed_response", {}).get(field_name, []))
        expected = set(reference_outputs.get(field_name, []))
    else:  # vulnerable_functions or vulnerable_lines
        predicted_dict = outputs.get("parsed_response", {}).get(field_name, {})
        expected_dict = reference_outputs.get(field_name, {})

        predicted = set()
        for file_path, items in predicted_dict.items():
            for item in items:
                predicted.add((file_path, item))

        expected = set()
        for file_path, items in expected_dict.items():
            for item in items:
                expected.add((file_path, item))

    return predicted, expected


def calculate_sklearn_metrics(
    predicted: Set, expected: Set, metrics: List[str] = None
) -> Dict[str, float]:
    """Calculate sklearn metrics for given predicted and expected sets.

    Args:
        predicted: Set of predicted items
        expected: Set of expected items
        metrics: List of metrics to calculate ['precision', 'recall', 'f1']

    Returns:
        Dict with calculated metrics
    """
    if metrics is None:
        metrics = ["precision", "recall", "f1"]

    # Handle empty case
    all_items = predicted | expected
    if not all_items:
        return {metric: 1.0 for metric in metrics}

    # Convert to binary vectors
    y_true = [1 if item in expected else 0 for item in all_items]
    y_pred = [1 if item in predicted else 0 for item in all_items]

    results = {}
    if "precision" in metrics:
        results["precision"] = precision_score(
            y_true, y_pred, average="binary", zero_division=1.0
        )
    if "recall" in metrics:
        results["recall"] = recall_score(
            y_true, y_pred, average="binary", zero_division=1.0
        )
    if "f1" in metrics:
        results["f1"] = f1_score(y_true, y_pred, average="binary", zero_division=0.0)

    return results


def create_evaluation_result(
    key: str,
    score: float,
    predicted: Set,
    expected: Set,
    metrics: Dict[str, float] = None,
    extra_metrics: List[str] = None,
) -> Dict[str, Any]:
    """Create standardized evaluation result format.

    Args:
        key: The metric key
        score: The primary score
        predicted: Set of predicted items
        expected: Set of expected items
        metrics: Dict of calculated metrics (precision, recall, f1)
        extra_metrics: List of additional metrics to include in comment

    Returns:
        Standardized evaluation result dict
    """
    # Calculate TP, FP, FN for comment
    tp = len(predicted & expected)
    fp = len(predicted - expected)
    fn = len(expected - predicted)

    # Build comment
    comment_parts = [
        f"Predicted: {list(predicted)}",
        f"Expected: {list(expected)}",
        f"TP: {tp}, FP: {fp}, FN: {fn}",
    ]

    if metrics:
        if "precision" in metrics:
            comment_parts.append(f"P: {metrics['precision']:.3f}")
        if "recall" in metrics:
            comment_parts.append(f"R: {metrics['recall']:.3f}")
        if "f1" in metrics:
            comment_parts.append(f"F1: {metrics['f1']:.3f}")

    return {
        "key": key,
        "score": score,
        "comment": ", ".join(comment_parts),
    }


def safe_evaluator_wrapper(
    outputs: Dict[str, Any],
    reference_outputs: Dict[str, Any],
    metric_key: str,
    evaluation_func: Callable,
) -> Dict[str, Any]:
    """Wrapper that provides error handling for evaluator functions.

    Args:
        outputs: The model outputs
        reference_outputs: The ground truth
        metric_key: The metric key for error responses
        evaluation_func: Function that performs the actual evaluation

    Returns:
        Evaluation result dict
    """
    # Check for errors first
    error_result = handle_errors(outputs, metric_key)
    if error_result:
        return error_result

    try:
        return evaluation_func(outputs, reference_outputs, metric_key)
    except Exception as e:
        return {
            "key": metric_key,
            "score": 0,
            "comment": f"Evaluator error: {str(e)}",
            "extra": {"error": True},
        }


def generic_multilabel_evaluator(
    outputs: Dict[str, Any],
    reference_outputs: Dict[str, Any],
    metric_key: str,
    field_name: str,
    metric_type: str,
) -> Dict[str, Any]:
    """Generic evaluator for multilabel classification metrics.

    Args:
        outputs: Model outputs
        reference_outputs: Ground truth
        metric_key: Key for the result
        field_name: Field name (cwe_type, vulnerable_functions, vulnerable_lines)
        metric_type: Type of metric (precision, recall, f1)

    Returns:
        Evaluation result
    """
    predicted, expected = extract_sets_from_data(outputs, reference_outputs, field_name)

    # Calculate metrics
    metrics = calculate_sklearn_metrics(predicted, expected, [metric_type])
    score = metrics[metric_type]

    return create_evaluation_result(
        key=metric_key,
        score=score,
        predicted=predicted,
        expected=expected,
        metrics=metrics,
    )


# Multilabel calculation helpers for summary evaluators
def calculate_multilabel_metrics(
    outputs: List[Dict[str, Any]],
    reference_outputs: List[Dict[str, Any]],
    field_name: str,
    metric_func: Callable,
    average: str = "micro",
) -> float:
    """Calculate multilabel classification metrics using sklearn."""
    if not outputs or not reference_outputs:
        return 0.0

    # Collect all unique items across all samples
    all_items = set()
    valid_pairs = []

    for output, reference in zip(outputs, reference_outputs):
        if output.get("errors") is not None:
            continue

        predicted, expected = extract_sets_from_data(output, reference, field_name)

        all_items.update(predicted)
        all_items.update(expected)
        valid_pairs.append((predicted, expected))

    if not valid_pairs or not all_items:
        return 1.0 if metric_func == precision_score else 0.0

    # Create binary vectors for all items across all samples
    y_true_all = []
    y_pred_all = []

    for predicted, expected in valid_pairs:
        for item in all_items:
            y_true_all.append(1 if item in expected else 0)
            y_pred_all.append(1 if item in predicted else 0)

    zero_division = 1.0 if metric_func == precision_score else 0.0
    return metric_func(
        y_true_all, y_pred_all, average=average, zero_division=zero_division
    )


def calculate_sample_level_metrics(
    outputs: List[Dict[str, Any]],
    reference_outputs: List[Dict[str, Any]],
    field_name: str,
    metric_func: Callable,
) -> float:
    """Calculate sample-level (macro) metrics."""
    if not outputs or not reference_outputs:
        return 0.0

    scores = []

    for output, reference in zip(outputs, reference_outputs):
        if output.get("errors") is not None:
            continue

        predicted, expected = extract_sets_from_data(output, reference, field_name)

        # Convert sets to binary vectors for sklearn
        all_items = predicted | expected
        if not all_items:
            # For empty sets (perfect match): precision=1.0, recall=1.0, f1=1.0
            scores.append(1.0)
        else:
            y_true = [1 if item in expected else 0 for item in all_items]
            y_pred = [1 if item in predicted else 0 for item in all_items]
            zero_division = 1.0 if metric_func == precision_score else 0.0
            score = metric_func(
                y_true, y_pred, average="binary", zero_division=zero_division
            )
            scores.append(score)

    return sum(scores) / len(scores) if scores else 0.0
