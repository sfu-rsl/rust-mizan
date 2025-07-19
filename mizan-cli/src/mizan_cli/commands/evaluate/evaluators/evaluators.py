"""Sample level evaluators for LLM evaluation."""

from typing import Dict, Any
from .evaluation_helpers import (
    safe_evaluator_wrapper,
    generic_multilabel_evaluator,
    extract_sets_from_data,
    calculate_sklearn_metrics,
    create_evaluation_result,
)


def _is_vulnerable_evaluation_logic(
    outputs: Dict[str, Any], reference_outputs: Dict[str, Any], metric_key: str
) -> Dict[str, Any]:
    """Internal logic for is_vulnerable evaluation."""
    predicted = outputs.get("parsed_response", {}).get("is_vulnerable", False)
    expected = reference_outputs.get("is_vulnerable", False)

    accuracy = 1 if predicted == expected else 0

    return {
        "key": metric_key,
        "score": accuracy,
        "comment": f"Predicted: {predicted}, Expected: {expected}",
    }


def is_vulnerable_evaluator(
    outputs: Dict[str, Any], reference_outputs: Dict[str, Any]
) -> Dict[str, Any]:
    """Evaluate binary classification accuracy for is_vulnerable."""
    return safe_evaluator_wrapper(
        outputs,
        reference_outputs,
        "is_vulnerable_accuracy",
        _is_vulnerable_evaluation_logic,
    )


def cwe_type_evaluator(
    outputs: Dict[str, Any], reference_outputs: Dict[str, Any]
) -> Dict[str, Any]:
    """Evaluate CWE type detection using precision, recall, and F1 score."""

    def _cwe_evaluation_logic(outputs, reference_outputs, metric_key):
        predicted, expected = extract_sets_from_data(
            outputs, reference_outputs, "cwe_type"
        )
        metrics = calculate_sklearn_metrics(
            predicted, expected, ["precision", "recall", "f1"]
        )
        return create_evaluation_result(
            key=metric_key,
            score=metrics["f1"],
            predicted=predicted,
            expected=expected,
            metrics=metrics,
        )

    return safe_evaluator_wrapper(
        outputs, reference_outputs, "cwe_type_f1", _cwe_evaluation_logic
    )


def vulnerable_functions_evaluator(
    outputs: Dict[str, Any], reference_outputs: Dict[str, Any]
) -> Dict[str, Any]:
    """Evaluate vulnerable function detection using precision, recall, and F1 score."""

    def _functions_evaluation_logic(outputs, reference_outputs, metric_key):
        predicted, expected = extract_sets_from_data(
            outputs, reference_outputs, "vulnerable_functions"
        )
        metrics = calculate_sklearn_metrics(
            predicted, expected, ["precision", "recall", "f1"]
        )
        return create_evaluation_result(
            key=metric_key,
            score=metrics["f1"],
            predicted=predicted,
            expected=expected,
            metrics=metrics,
        )

    return safe_evaluator_wrapper(
        outputs,
        reference_outputs,
        "vulnerable_functions_f1",
        _functions_evaluation_logic,
    )


def vulnerable_lines_evaluator(
    outputs: Dict[str, Any], reference_outputs: Dict[str, Any]
) -> Dict[str, Any]:
    """Evaluate vulnerable line detection using precision, recall, and F1 score."""

    def _lines_evaluation_logic(outputs, reference_outputs, metric_key):
        predicted, expected = extract_sets_from_data(
            outputs, reference_outputs, "vulnerable_lines"
        )
        metrics = calculate_sklearn_metrics(
            predicted, expected, ["precision", "recall", "f1"]
        )
        return create_evaluation_result(
            key=metric_key,
            score=metrics["f1"],
            predicted=predicted,
            expected=expected,
            metrics=metrics,
        )

    return safe_evaluator_wrapper(
        outputs, reference_outputs, "vulnerable_lines_f1", _lines_evaluation_logic
    )


def _json_validity_evaluation_logic(
    outputs: Dict[str, Any], reference_outputs: Dict[str, Any], metric_key: str
) -> Dict[str, Any]:
    """Internal logic for JSON validity evaluation."""
    parsed_response = outputs.get("parsed_response", {})
    # Check if we have a valid parsed response with expected fields
    has_valid_json = (
        parsed_response
        and "is_vulnerable" in parsed_response
        and "cwe_type" in parsed_response
        and "vulnerable_functions" in parsed_response
        and "vulnerable_lines" in parsed_response
    )

    return {
        "key": metric_key,
        "score": 1 if has_valid_json else 0,
        "comment": f"JSON parsing {'succeeded' if has_valid_json else 'failed'}",
    }


def json_validity_evaluator(
    outputs: Dict[str, Any], reference_outputs: Dict[str, Any]
) -> Dict[str, Any]:
    """Evaluate JSON validity of the response."""
    return safe_evaluator_wrapper(
        outputs, reference_outputs, "json_validity", _json_validity_evaluation_logic
    )


def cwe_type_precision_evaluator(
    outputs: Dict[str, Any], reference_outputs: Dict[str, Any]
) -> Dict[str, Any]:
    """Evaluate CWE type detection precision."""
    return safe_evaluator_wrapper(
        outputs,
        reference_outputs,
        "cwe_type_precision",
        lambda o, r, k: generic_multilabel_evaluator(o, r, k, "cwe_type", "precision"),
    )


def cwe_type_recall_evaluator(
    outputs: Dict[str, Any], reference_outputs: Dict[str, Any]
) -> Dict[str, Any]:
    """Evaluate CWE type detection recall."""
    return safe_evaluator_wrapper(
        outputs,
        reference_outputs,
        "cwe_type_recall",
        lambda o, r, k: generic_multilabel_evaluator(o, r, k, "cwe_type", "recall"),
    )


def vulnerable_functions_precision_evaluator(
    outputs: Dict[str, Any], reference_outputs: Dict[str, Any]
) -> Dict[str, Any]:
    """Evaluate vulnerable function detection precision."""
    return safe_evaluator_wrapper(
        outputs,
        reference_outputs,
        "vulnerable_functions_precision",
        lambda o, r, k: generic_multilabel_evaluator(
            o, r, k, "vulnerable_functions", "precision"
        ),
    )


def vulnerable_functions_recall_evaluator(
    outputs: Dict[str, Any], reference_outputs: Dict[str, Any]
) -> Dict[str, Any]:
    """Evaluate vulnerable function detection recall."""
    return safe_evaluator_wrapper(
        outputs,
        reference_outputs,
        "vulnerable_functions_recall",
        lambda o, r, k: generic_multilabel_evaluator(
            o, r, k, "vulnerable_functions", "recall"
        ),
    )


def vulnerable_lines_precision_evaluator(
    outputs: Dict[str, Any], reference_outputs: Dict[str, Any]
) -> Dict[str, Any]:
    """Evaluate vulnerable line detection precision."""
    return safe_evaluator_wrapper(
        outputs,
        reference_outputs,
        "vulnerable_lines_precision",
        lambda o, r, k: generic_multilabel_evaluator(
            o, r, k, "vulnerable_lines", "precision"
        ),
    )


def vulnerable_lines_recall_evaluator(
    outputs: Dict[str, Any], reference_outputs: Dict[str, Any]
) -> Dict[str, Any]:
    """Evaluate vulnerable line detection recall."""
    return safe_evaluator_wrapper(
        outputs,
        reference_outputs,
        "vulnerable_lines_recall",
        lambda o, r, k: generic_multilabel_evaluator(
            o, r, k, "vulnerable_lines", "recall"
        ),
    )


def get_all_evaluators():
    """Get all available evaluators."""
    return [
        is_vulnerable_evaluator,
        cwe_type_evaluator,
        cwe_type_precision_evaluator,
        cwe_type_recall_evaluator,
        vulnerable_functions_evaluator,
        vulnerable_functions_precision_evaluator,
        vulnerable_functions_recall_evaluator,
        vulnerable_lines_evaluator,
        vulnerable_lines_precision_evaluator,
        vulnerable_lines_recall_evaluator,
        json_validity_evaluator,
    ]
