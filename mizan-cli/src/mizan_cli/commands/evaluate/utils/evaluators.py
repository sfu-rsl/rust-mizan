"""Simple evaluators for LLM vulnerability detection evaluation."""

from typing import Dict, Any, List


def json_validity_evaluator(
    outputs: Dict[str, Any], reference_outputs: Dict[str, Any]
) -> Dict[str, Any]:
    """Check if response contains valid JSON with expected fields."""
    if outputs.get("errors") is not None:
        return {"key": "json_validity", "score": 0, "comment": "Target function failed"}

    parsed_response = outputs.get("parsed_response", {})
    required_fields = [
        "is_vulnerable",
        "cwe_type",
        "vulnerable_functions",
        "vulnerable_lines",
    ]
    has_valid_json = all(field in parsed_response for field in required_fields)

    return {
        "key": "json_validity",
        "score": 1 if has_valid_json else 0,
        "comment": f"JSON parsing {'succeeded' if has_valid_json else 'failed'}",
    }


def get_all_evaluators() -> List:
    """Get all available evaluators."""
    return [
        json_validity_evaluator,
    ]
