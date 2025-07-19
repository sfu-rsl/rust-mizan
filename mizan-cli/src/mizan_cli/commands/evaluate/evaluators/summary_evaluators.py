"""Summary evaluators for experiment-level metrics."""

from typing import Dict, Any, List
from sklearn.metrics import accuracy_score, precision_score, recall_score, f1_score
from mizan_cli.utils.logging import get_logger
from .evaluation_helpers import (
    calculate_multilabel_metrics,
    calculate_sample_level_metrics,
)

logger = get_logger()


def is_vulnerable_summary_evaluator(
    outputs: List[Dict[str, Any]], reference_outputs: List[Dict[str, Any]]
) -> Dict[str, Any]:
    """Calculate overall vulnerability detection metrics across all examples."""
    try:
        metrics = calculate_is_vulnerable_metrics(outputs, reference_outputs)
        return {"key": "is_vulnerable_metrics", "score": metrics["f1"]}
    except Exception as e:
        logger.error(f"Error in is_vulnerable summary evaluator: {e}")
        return {"key": "is_vulnerable_metrics", "score": 0.0}


def cwe_type_micro_summary_evaluator(
    outputs: List[Dict[str, Any]], reference_outputs: List[Dict[str, Any]]
) -> Dict[str, Any]:
    """Calculate micro-averaged F1 score for CWE type detection."""
    try:
        micro_f1 = calculate_cwe_type_micro_f1(outputs, reference_outputs)
        return {"key": "cwe_type_micro_f1", "score": micro_f1}
    except Exception as e:
        logger.error(f"Error in CWE type micro summary evaluator: {e}")
        return {"key": "cwe_type_micro_f1", "score": 0.0}


def cwe_type_macro_summary_evaluator(
    outputs: List[Dict[str, Any]], reference_outputs: List[Dict[str, Any]]
) -> Dict[str, Any]:
    """Calculate macro-averaged F1 score for CWE type detection."""
    try:
        macro_f1 = calculate_cwe_type_macro_f1(outputs, reference_outputs)
        return {"key": "cwe_type_macro_f1", "score": macro_f1}
    except Exception as e:
        logger.error(f"Error in CWE type macro summary evaluator: {e}")
        return {"key": "cwe_type_macro_f1", "score": 0.0}


def vulnerable_functions_micro_summary_evaluator(
    outputs: List[Dict[str, Any]], reference_outputs: List[Dict[str, Any]]
) -> Dict[str, Any]:
    """Calculate micro-averaged F1 score for vulnerable function detection."""
    try:
        micro_f1 = calculate_vulnerable_functions_micro_f1(outputs, reference_outputs)
        return {"key": "vulnerable_functions_micro_f1", "score": micro_f1}
    except Exception as e:
        logger.error(f"Error in vulnerable functions micro summary evaluator: {e}")
        return {"key": "vulnerable_functions_micro_f1", "score": 0.0}


def vulnerable_functions_macro_summary_evaluator(
    outputs: List[Dict[str, Any]], reference_outputs: List[Dict[str, Any]]
) -> Dict[str, Any]:
    """Calculate macro-averaged F1 score for vulnerable function detection."""
    try:
        macro_f1 = calculate_vulnerable_functions_macro_f1(outputs, reference_outputs)
        return {"key": "vulnerable_functions_macro_f1", "score": macro_f1}
    except Exception as e:
        logger.error(f"Error in vulnerable functions macro summary evaluator: {e}")
        return {"key": "vulnerable_functions_macro_f1", "score": 0.0}


def vulnerable_lines_micro_summary_evaluator(
    outputs: List[Dict[str, Any]], reference_outputs: List[Dict[str, Any]]
) -> Dict[str, Any]:
    """Calculate micro-averaged F1 score for vulnerable line detection."""
    try:
        micro_f1 = calculate_vulnerable_lines_micro_f1(outputs, reference_outputs)
        return {"key": "vulnerable_lines_micro_f1", "score": micro_f1}
    except Exception as e:
        logger.error(f"Error in vulnerable lines micro summary evaluator: {e}")
        return {"key": "vulnerable_lines_micro_f1", "score": 0.0}


def vulnerable_lines_macro_summary_evaluator(
    outputs: List[Dict[str, Any]], reference_outputs: List[Dict[str, Any]]
) -> Dict[str, Any]:
    """Calculate macro-averaged F1 score for vulnerable line detection."""
    try:
        macro_f1 = calculate_vulnerable_lines_macro_f1(outputs, reference_outputs)
        return {"key": "vulnerable_lines_macro_f1", "score": macro_f1}
    except Exception as e:
        logger.error(f"Error in vulnerable lines macro summary evaluator: {e}")
        return {"key": "vulnerable_lines_macro_f1", "score": 0.0}


def json_validity_summary_evaluator(
    outputs: List[Dict[str, Any]], reference_outputs: List[Dict[str, Any]]
) -> Dict[str, Any]:
    """Calculate JSON validity rate across all examples."""
    try:
        validity_rate = calculate_json_validity_rate(outputs, reference_outputs)
        return {"key": "json_validity_rate", "score": validity_rate}
    except Exception as e:
        logger.error(f"Error in JSON validity summary evaluator: {e}")
        return {"key": "json_validity_rate", "score": 0.0}


def calculate_is_vulnerable_metrics(
    outputs: List[Dict[str, Any]], reference_outputs: List[Dict[str, Any]]
) -> Dict[str, float]:
    """Calculate overall vulnerability detection metrics (accuracy, precision, recall, F1)."""
    if not outputs or not reference_outputs:
        return {"accuracy": 0.0, "precision": 0.0, "recall": 0.0, "f1": 0.0}

    y_true = []
    y_pred = []

    for output, reference in zip(outputs, reference_outputs):
        # Skip outputs that have errors
        if output.get("errors") is not None:
            continue

        predicted = output.get("parsed_response", {}).get("is_vulnerable", False)
        expected = reference.get("is_vulnerable", False)

        y_true.append(int(expected))
        y_pred.append(int(predicted))

    if not y_true:
        return {"accuracy": 0.0, "precision": 0.0, "recall": 0.0, "f1": 0.0}

    # Calculate metrics using sklearn
    accuracy = accuracy_score(y_true, y_pred)
    precision = precision_score(y_true, y_pred, zero_division=1.0)
    recall = recall_score(y_true, y_pred, zero_division=1.0)
    f1 = f1_score(y_true, y_pred, zero_division=0.0)

    return {
        "accuracy": accuracy,
        "precision": precision,
        "recall": recall,
        "f1": f1,
    }


# Simplified calculation functions using helpers
def calculate_cwe_type_micro_f1(
    outputs: List[Dict[str, Any]], reference_outputs: List[Dict[str, Any]]
) -> float:
    """Calculate micro-averaged F1 score for CWE type detection."""
    return calculate_multilabel_metrics(
        outputs, reference_outputs, "cwe_type", f1_score, "micro"
    )


def calculate_cwe_type_macro_f1(
    outputs: List[Dict[str, Any]], reference_outputs: List[Dict[str, Any]]
) -> float:
    """Calculate macro-averaged F1 score for CWE type detection."""
    return calculate_sample_level_metrics(
        outputs, reference_outputs, "cwe_type", f1_score
    )


def calculate_cwe_type_micro_precision(
    outputs: List[Dict[str, Any]], reference_outputs: List[Dict[str, Any]]
) -> float:
    """Calculate micro-averaged precision for CWE type detection."""
    return calculate_multilabel_metrics(
        outputs, reference_outputs, "cwe_type", precision_score, "micro"
    )


def calculate_cwe_type_macro_precision(
    outputs: List[Dict[str, Any]], reference_outputs: List[Dict[str, Any]]
) -> float:
    """Calculate macro-averaged precision for CWE type detection."""
    return calculate_sample_level_metrics(
        outputs, reference_outputs, "cwe_type", precision_score
    )


def calculate_cwe_type_micro_recall(
    outputs: List[Dict[str, Any]], reference_outputs: List[Dict[str, Any]]
) -> float:
    """Calculate micro-averaged recall for CWE type detection."""
    return calculate_multilabel_metrics(
        outputs, reference_outputs, "cwe_type", recall_score, "micro"
    )


def calculate_cwe_type_macro_recall(
    outputs: List[Dict[str, Any]], reference_outputs: List[Dict[str, Any]]
) -> float:
    """Calculate macro-averaged recall for CWE type detection."""
    return calculate_sample_level_metrics(
        outputs, reference_outputs, "cwe_type", recall_score
    )


def calculate_vulnerable_functions_micro_f1(
    outputs: List[Dict[str, Any]], reference_outputs: List[Dict[str, Any]]
) -> float:
    """Calculate micro-averaged F1 score for vulnerable function detection."""
    return calculate_multilabel_metrics(
        outputs, reference_outputs, "vulnerable_functions", f1_score, "micro"
    )


def calculate_vulnerable_functions_macro_f1(
    outputs: List[Dict[str, Any]], reference_outputs: List[Dict[str, Any]]
) -> float:
    """Calculate macro-averaged F1 score for vulnerable function detection."""
    return calculate_sample_level_metrics(
        outputs, reference_outputs, "vulnerable_functions", f1_score
    )


def calculate_vulnerable_functions_micro_precision(
    outputs: List[Dict[str, Any]], reference_outputs: List[Dict[str, Any]]
) -> float:
    """Calculate micro-averaged precision for vulnerable function detection."""
    return calculate_multilabel_metrics(
        outputs, reference_outputs, "vulnerable_functions", precision_score, "micro"
    )


def calculate_vulnerable_functions_macro_precision(
    outputs: List[Dict[str, Any]], reference_outputs: List[Dict[str, Any]]
) -> float:
    """Calculate macro-averaged precision for vulnerable function detection."""
    return calculate_sample_level_metrics(
        outputs, reference_outputs, "vulnerable_functions", precision_score
    )


def calculate_vulnerable_functions_micro_recall(
    outputs: List[Dict[str, Any]], reference_outputs: List[Dict[str, Any]]
) -> float:
    """Calculate micro-averaged recall for vulnerable function detection."""
    return calculate_multilabel_metrics(
        outputs, reference_outputs, "vulnerable_functions", recall_score, "micro"
    )


def calculate_vulnerable_functions_macro_recall(
    outputs: List[Dict[str, Any]], reference_outputs: List[Dict[str, Any]]
) -> float:
    """Calculate macro-averaged recall for vulnerable function detection."""
    return calculate_sample_level_metrics(
        outputs, reference_outputs, "vulnerable_functions", recall_score
    )


def calculate_vulnerable_lines_micro_f1(
    outputs: List[Dict[str, Any]], reference_outputs: List[Dict[str, Any]]
) -> float:
    """Calculate micro-averaged F1 score for vulnerable line detection."""
    return calculate_multilabel_metrics(
        outputs, reference_outputs, "vulnerable_lines", f1_score, "micro"
    )


def calculate_vulnerable_lines_macro_f1(
    outputs: List[Dict[str, Any]], reference_outputs: List[Dict[str, Any]]
) -> float:
    """Calculate macro-averaged F1 score for vulnerable line detection."""
    return calculate_sample_level_metrics(
        outputs, reference_outputs, "vulnerable_lines", f1_score
    )


def calculate_vulnerable_lines_micro_precision(
    outputs: List[Dict[str, Any]], reference_outputs: List[Dict[str, Any]]
) -> float:
    """Calculate micro-averaged precision for vulnerable line detection."""
    return calculate_multilabel_metrics(
        outputs, reference_outputs, "vulnerable_lines", precision_score, "micro"
    )


def calculate_vulnerable_lines_macro_precision(
    outputs: List[Dict[str, Any]], reference_outputs: List[Dict[str, Any]]
) -> float:
    """Calculate macro-averaged precision for vulnerable line detection."""
    return calculate_sample_level_metrics(
        outputs, reference_outputs, "vulnerable_lines", precision_score
    )


def calculate_vulnerable_lines_micro_recall(
    outputs: List[Dict[str, Any]], reference_outputs: List[Dict[str, Any]]
) -> float:
    """Calculate micro-averaged recall for vulnerable line detection."""
    return calculate_multilabel_metrics(
        outputs, reference_outputs, "vulnerable_lines", recall_score, "micro"
    )


def calculate_vulnerable_lines_macro_recall(
    outputs: List[Dict[str, Any]], reference_outputs: List[Dict[str, Any]]
) -> float:
    """Calculate macro-averaged recall for vulnerable line detection."""
    return calculate_sample_level_metrics(
        outputs, reference_outputs, "vulnerable_lines", recall_score
    )


def calculate_json_validity_rate(
    outputs: List[Dict[str, Any]], reference_outputs: List[Dict[str, Any]]
) -> float:
    """Calculate JSON validity rate."""
    if not outputs:
        return 0.0

    valid_count = 0
    total_count = len(outputs)

    for output in outputs:
        # Skip outputs that have errors
        if output.get("errors") is not None:
            continue

        # Count as valid if we have a parsed_response with expected fields
        parsed_response = output.get("parsed_response", {})
        if (
            parsed_response
            and "is_vulnerable" in parsed_response
            and "cwe_type" in parsed_response
            and "vulnerable_functions" in parsed_response
            and "vulnerable_lines" in parsed_response
        ):
            valid_count += 1

    return valid_count / total_count if total_count > 0 else 0.0


# Summary evaluator wrappers for precision/recall metrics
def cwe_type_micro_precision_summary_evaluator(
    outputs: List[Dict[str, Any]], reference_outputs: List[Dict[str, Any]]
) -> Dict[str, Any]:
    """Calculate micro-averaged precision for CWE type detection."""
    try:
        micro_precision = calculate_cwe_type_micro_precision(outputs, reference_outputs)
        return {"key": "cwe_type_micro_precision", "score": micro_precision}
    except Exception as e:
        logger.error(f"Error in CWE type micro precision summary evaluator: {e}")
        return {"key": "cwe_type_micro_precision", "score": 0.0}


def cwe_type_macro_precision_summary_evaluator(
    outputs: List[Dict[str, Any]], reference_outputs: List[Dict[str, Any]]
) -> Dict[str, Any]:
    """Calculate macro-averaged precision for CWE type detection."""
    try:
        macro_precision = calculate_cwe_type_macro_precision(outputs, reference_outputs)
        return {"key": "cwe_type_macro_precision", "score": macro_precision}
    except Exception as e:
        logger.error(f"Error in CWE type macro precision summary evaluator: {e}")
        return {"key": "cwe_type_macro_precision", "score": 0.0}


def cwe_type_micro_recall_summary_evaluator(
    outputs: List[Dict[str, Any]], reference_outputs: List[Dict[str, Any]]
) -> Dict[str, Any]:
    """Calculate micro-averaged recall for CWE type detection."""
    try:
        micro_recall = calculate_cwe_type_micro_recall(outputs, reference_outputs)
        return {"key": "cwe_type_micro_recall", "score": micro_recall}
    except Exception as e:
        logger.error(f"Error in CWE type micro recall summary evaluator: {e}")
        return {"key": "cwe_type_micro_recall", "score": 0.0}


def cwe_type_macro_recall_summary_evaluator(
    outputs: List[Dict[str, Any]], reference_outputs: List[Dict[str, Any]]
) -> Dict[str, Any]:
    """Calculate macro-averaged recall for CWE type detection."""
    try:
        macro_recall = calculate_cwe_type_macro_recall(outputs, reference_outputs)
        return {"key": "cwe_type_macro_recall", "score": macro_recall}
    except Exception as e:
        logger.error(f"Error in CWE type macro recall summary evaluator: {e}")
        return {"key": "cwe_type_macro_recall", "score": 0.0}


def vulnerable_functions_micro_precision_summary_evaluator(
    outputs: List[Dict[str, Any]], reference_outputs: List[Dict[str, Any]]
) -> Dict[str, Any]:
    """Calculate micro-averaged precision for vulnerable function detection."""
    try:
        micro_precision = calculate_vulnerable_functions_micro_precision(
            outputs, reference_outputs
        )
        return {"key": "vulnerable_functions_micro_precision", "score": micro_precision}
    except Exception as e:
        logger.error(
            f"Error in vulnerable functions micro precision summary evaluator: {e}"
        )
        return {"key": "vulnerable_functions_micro_precision", "score": 0.0}


def vulnerable_functions_macro_precision_summary_evaluator(
    outputs: List[Dict[str, Any]], reference_outputs: List[Dict[str, Any]]
) -> Dict[str, Any]:
    """Calculate macro-averaged precision for vulnerable function detection."""
    try:
        macro_precision = calculate_vulnerable_functions_macro_precision(
            outputs, reference_outputs
        )
        return {"key": "vulnerable_functions_macro_precision", "score": macro_precision}
    except Exception as e:
        logger.error(
            f"Error in vulnerable functions macro precision summary evaluator: {e}"
        )
        return {"key": "vulnerable_functions_macro_precision", "score": 0.0}


def vulnerable_functions_micro_recall_summary_evaluator(
    outputs: List[Dict[str, Any]], reference_outputs: List[Dict[str, Any]]
) -> Dict[str, Any]:
    """Calculate micro-averaged recall for vulnerable function detection."""
    try:
        micro_recall = calculate_vulnerable_functions_micro_recall(
            outputs, reference_outputs
        )
        return {"key": "vulnerable_functions_micro_recall", "score": micro_recall}
    except Exception as e:
        logger.error(
            f"Error in vulnerable functions micro recall summary evaluator: {e}"
        )
        return {"key": "vulnerable_functions_micro_recall", "score": 0.0}


def vulnerable_functions_macro_recall_summary_evaluator(
    outputs: List[Dict[str, Any]], reference_outputs: List[Dict[str, Any]]
) -> Dict[str, Any]:
    """Calculate macro-averaged recall for vulnerable function detection."""
    try:
        macro_recall = calculate_vulnerable_functions_macro_recall(
            outputs, reference_outputs
        )
        return {"key": "vulnerable_functions_macro_recall", "score": macro_recall}
    except Exception as e:
        logger.error(
            f"Error in vulnerable functions macro recall summary evaluator: {e}"
        )
        return {"key": "vulnerable_functions_macro_recall", "score": 0.0}


def vulnerable_lines_micro_precision_summary_evaluator(
    outputs: List[Dict[str, Any]], reference_outputs: List[Dict[str, Any]]
) -> Dict[str, Any]:
    """Calculate micro-averaged precision for vulnerable line detection."""
    try:
        micro_precision = calculate_vulnerable_lines_micro_precision(
            outputs, reference_outputs
        )
        return {"key": "vulnerable_lines_micro_precision", "score": micro_precision}
    except Exception as e:
        logger.error(
            f"Error in vulnerable lines micro precision summary evaluator: {e}"
        )
        return {"key": "vulnerable_lines_micro_precision", "score": 0.0}


def vulnerable_lines_macro_precision_summary_evaluator(
    outputs: List[Dict[str, Any]], reference_outputs: List[Dict[str, Any]]
) -> Dict[str, Any]:
    """Calculate macro-averaged precision for vulnerable line detection."""
    try:
        macro_precision = calculate_vulnerable_lines_macro_precision(
            outputs, reference_outputs
        )
        return {"key": "vulnerable_lines_macro_precision", "score": macro_precision}
    except Exception as e:
        logger.error(
            f"Error in vulnerable lines macro precision summary evaluator: {e}"
        )
        return {"key": "vulnerable_lines_macro_precision", "score": 0.0}


def vulnerable_lines_micro_recall_summary_evaluator(
    outputs: List[Dict[str, Any]], reference_outputs: List[Dict[str, Any]]
) -> Dict[str, Any]:
    """Calculate micro-averaged recall for vulnerable line detection."""
    try:
        micro_recall = calculate_vulnerable_lines_micro_recall(
            outputs, reference_outputs
        )
        return {"key": "vulnerable_lines_micro_recall", "score": micro_recall}
    except Exception as e:
        logger.error(f"Error in vulnerable lines micro recall summary evaluator: {e}")
        return {"key": "vulnerable_lines_micro_recall", "score": 0.0}


def vulnerable_lines_macro_recall_summary_evaluator(
    outputs: List[Dict[str, Any]], reference_outputs: List[Dict[str, Any]]
) -> Dict[str, Any]:
    """Calculate macro-averaged recall for vulnerable line detection."""
    try:
        macro_recall = calculate_vulnerable_lines_macro_recall(
            outputs, reference_outputs
        )
        return {"key": "vulnerable_lines_macro_recall", "score": macro_recall}
    except Exception as e:
        logger.error(f"Error in vulnerable lines macro recall summary evaluator: {e}")
        return {"key": "vulnerable_lines_macro_recall", "score": 0.0}


def get_all_summary_evaluators():
    """Get all available summary evaluators."""
    return [
        is_vulnerable_summary_evaluator,
        cwe_type_micro_summary_evaluator,
        cwe_type_macro_summary_evaluator,
        cwe_type_micro_precision_summary_evaluator,
        cwe_type_macro_precision_summary_evaluator,
        cwe_type_micro_recall_summary_evaluator,
        cwe_type_macro_recall_summary_evaluator,
        vulnerable_functions_micro_summary_evaluator,
        vulnerable_functions_macro_summary_evaluator,
        vulnerable_functions_micro_precision_summary_evaluator,
        vulnerable_functions_macro_precision_summary_evaluator,
        vulnerable_functions_micro_recall_summary_evaluator,
        vulnerable_functions_macro_recall_summary_evaluator,
        vulnerable_lines_micro_summary_evaluator,
        vulnerable_lines_macro_summary_evaluator,
        vulnerable_lines_micro_precision_summary_evaluator,
        vulnerable_lines_macro_precision_summary_evaluator,
        vulnerable_lines_micro_recall_summary_evaluator,
        vulnerable_lines_macro_recall_summary_evaluator,
        json_validity_summary_evaluator,
    ]


def calculate_all_summary_evaluations(
    outputs: List[Dict[str, Any]], reference_outputs: List[Dict[str, Any]]
) -> Dict[str, Any]:
    """Calculate all summary evaluations manually."""
    # Calculate is_vulnerable metrics
    is_vulnerable_metrics = calculate_is_vulnerable_metrics(outputs, reference_outputs)

    return {
        "is_vulnerable_accuracy": is_vulnerable_metrics["accuracy"],
        "is_vulnerable_precision": is_vulnerable_metrics["precision"],
        "is_vulnerable_recall": is_vulnerable_metrics["recall"],
        "is_vulnerable_f1": is_vulnerable_metrics["f1"],
        "cwe_type_micro_f1": calculate_cwe_type_micro_f1(outputs, reference_outputs),
        "cwe_type_macro_f1": calculate_cwe_type_macro_f1(outputs, reference_outputs),
        "cwe_type_micro_precision": calculate_cwe_type_micro_precision(
            outputs, reference_outputs
        ),
        "cwe_type_macro_precision": calculate_cwe_type_macro_precision(
            outputs, reference_outputs
        ),
        "cwe_type_micro_recall": calculate_cwe_type_micro_recall(
            outputs, reference_outputs
        ),
        "cwe_type_macro_recall": calculate_cwe_type_macro_recall(
            outputs, reference_outputs
        ),
        "vulnerable_functions_micro_f1": calculate_vulnerable_functions_micro_f1(
            outputs, reference_outputs
        ),
        "vulnerable_functions_macro_f1": calculate_vulnerable_functions_macro_f1(
            outputs, reference_outputs
        ),
        "vulnerable_functions_micro_precision": calculate_vulnerable_functions_micro_precision(
            outputs, reference_outputs
        ),
        "vulnerable_functions_macro_precision": calculate_vulnerable_functions_macro_precision(
            outputs, reference_outputs
        ),
        "vulnerable_functions_micro_recall": calculate_vulnerable_functions_micro_recall(
            outputs, reference_outputs
        ),
        "vulnerable_functions_macro_recall": calculate_vulnerable_functions_macro_recall(
            outputs, reference_outputs
        ),
        "vulnerable_lines_micro_f1": calculate_vulnerable_lines_micro_f1(
            outputs, reference_outputs
        ),
        "vulnerable_lines_macro_f1": calculate_vulnerable_lines_macro_f1(
            outputs, reference_outputs
        ),
        "vulnerable_lines_micro_precision": calculate_vulnerable_lines_micro_precision(
            outputs, reference_outputs
        ),
        "vulnerable_lines_macro_precision": calculate_vulnerable_lines_macro_precision(
            outputs, reference_outputs
        ),
        "vulnerable_lines_micro_recall": calculate_vulnerable_lines_micro_recall(
            outputs, reference_outputs
        ),
        "vulnerable_lines_macro_recall": calculate_vulnerable_lines_macro_recall(
            outputs, reference_outputs
        ),
        "json_validity_rate": calculate_json_validity_rate(outputs, reference_outputs),
    }
