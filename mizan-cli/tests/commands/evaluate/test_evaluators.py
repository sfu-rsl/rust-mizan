"""Tests for sample-level evaluators."""

import pytest
from mizan_cli.commands.evaluate.evaluators.evaluators import (
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
)


def create_mock_output(parsed_response=None, errors=None):
    """Helper to create mock LLM output."""
    output = {}
    if parsed_response is not None:
        output["parsed_response"] = parsed_response
    if errors is not None:
        output["errors"] = errors
    return output


def create_mock_reference(
    is_vulnerable=False, cwe_type=None, vulnerable_functions=None, vulnerable_lines=None
):
    """Helper to create mock reference output."""
    return {
        "is_vulnerable": is_vulnerable,
        "cwe_type": cwe_type or [],
        "vulnerable_functions": vulnerable_functions or {},
        "vulnerable_lines": vulnerable_lines or {},
    }


class TestIsVulnerableEvaluator:
    """Test binary classification evaluator."""

    def test_correct_positive_prediction(self):
        """Test correct positive prediction."""
        output = create_mock_output({"is_vulnerable": True})
        reference = create_mock_reference(is_vulnerable=True)

        result = is_vulnerable_evaluator(output, reference)

        assert result["key"] == "is_vulnerable_accuracy"
        assert result["score"] == 1
        assert "Predicted: True, Expected: True" in result["comment"]

    def test_correct_negative_prediction(self):
        """Test correct negative prediction."""
        output = create_mock_output({"is_vulnerable": False})
        reference = create_mock_reference(is_vulnerable=False)

        result = is_vulnerable_evaluator(output, reference)

        assert result["key"] == "is_vulnerable_accuracy"
        assert result["score"] == 1
        assert "Predicted: False, Expected: False" in result["comment"]

    def test_false_positive(self):
        """Test false positive prediction."""
        output = create_mock_output({"is_vulnerable": True})
        reference = create_mock_reference(is_vulnerable=False)

        result = is_vulnerable_evaluator(output, reference)

        assert result["key"] == "is_vulnerable_accuracy"
        assert result["score"] == 0
        assert "Predicted: True, Expected: False" in result["comment"]

    def test_false_negative(self):
        """Test false negative prediction."""
        output = create_mock_output({"is_vulnerable": False})
        reference = create_mock_reference(is_vulnerable=True)

        result = is_vulnerable_evaluator(output, reference)

        assert result["key"] == "is_vulnerable_accuracy"
        assert result["score"] == 0
        assert "Predicted: False, Expected: True" in result["comment"]

    def test_missing_parsed_response(self):
        """Test handling of missing parsed response."""
        output = create_mock_output()
        reference = create_mock_reference(is_vulnerable=True)

        result = is_vulnerable_evaluator(output, reference)

        assert result["key"] == "is_vulnerable_accuracy"
        assert result["score"] == 0
        assert "Predicted: False, Expected: True" in result["comment"]

    def test_target_function_error(self):
        """Test handling of target function errors."""
        output = create_mock_output(errors="API call failed")
        reference = create_mock_reference(is_vulnerable=True)

        result = is_vulnerable_evaluator(output, reference)

        assert result["key"] == "is_vulnerable_accuracy"
        assert result["score"] == 0
        assert "Target function failed" in result["comment"]


class TestCweTypeEvaluator:
    """Test CWE type detection evaluator."""

    def test_perfect_match(self):
        """Test perfect CWE type match."""
        output = create_mock_output({"cwe_type": ["CWE-125", "CWE-416"]})
        reference = create_mock_reference(cwe_type=["CWE-125", "CWE-416"])

        result = cwe_type_evaluator(output, reference)

        assert result["key"] == "cwe_type_f1"
        assert result["score"] == 1.0

    def test_partial_match(self):
        """Test partial CWE type match."""
        output = create_mock_output({"cwe_type": ["CWE-125", "CWE-787"]})
        reference = create_mock_reference(cwe_type=["CWE-125", "CWE-416"])

        result = cwe_type_evaluator(output, reference)

        # TP=1, FP=1, FN=1 -> P=0.5, R=0.5, F1=0.5
        assert result["key"] == "cwe_type_f1"
        assert result["score"] == 0.5

    def test_no_match(self):
        """Test no CWE type match."""
        output = create_mock_output({"cwe_type": ["CWE-787"]})
        reference = create_mock_reference(cwe_type=["CWE-125", "CWE-416"])

        result = cwe_type_evaluator(output, reference)

        # TP=0, FP=1, FN=2 -> P=0, R=0, F1=0
        assert result["key"] == "cwe_type_f1"
        assert result["score"] == 0.0

    def test_empty_prediction(self):
        """Test empty CWE prediction."""
        output = create_mock_output({"cwe_type": []})
        reference = create_mock_reference(cwe_type=["CWE-125"])

        result = cwe_type_evaluator(output, reference)

        # TP=0, FP=0, FN=1 -> P=1.0, R=0, F1=0
        assert result["key"] == "cwe_type_f1"
        assert result["score"] == 0.0

    def test_empty_reference(self):
        """Test empty CWE reference."""
        output = create_mock_output({"cwe_type": ["CWE-125"]})
        reference = create_mock_reference(cwe_type=[])

        result = cwe_type_evaluator(output, reference)

        # TP=0, FP=1, FN=0 -> P=0, R=1.0, F1=0
        assert result["key"] == "cwe_type_f1"
        assert result["score"] == 0.0

    def test_both_empty(self):
        """Test both prediction and reference empty."""
        output = create_mock_output({"cwe_type": []})
        reference = create_mock_reference(cwe_type=[])

        result = cwe_type_evaluator(output, reference)

        # TP=0, FP=0, FN=0 -> P=1.0, R=1.0, F1=1.0
        assert result["key"] == "cwe_type_f1"
        assert result["score"] == 1.0

    def test_target_function_error(self):
        """Test handling of target function errors."""
        output = create_mock_output(errors="API call failed")
        reference = create_mock_reference(cwe_type=["CWE-125"])

        result = cwe_type_evaluator(output, reference)

        assert result["key"] == "cwe_type_f1"
        assert result["score"] == 0
        assert "Target function failed" in result["comment"]


class TestCweTypePrecisionEvaluator:
    """Test CWE type precision evaluator."""

    def test_perfect_precision(self):
        """Test perfect precision."""
        output = create_mock_output({"cwe_type": ["CWE-125"]})
        reference = create_mock_reference(cwe_type=["CWE-125", "CWE-416"])

        result = cwe_type_precision_evaluator(output, reference)

        assert result["key"] == "cwe_type_precision"
        assert result["score"] == 1.0

    def test_zero_precision(self):
        """Test zero precision."""
        output = create_mock_output({"cwe_type": ["CWE-787"]})
        reference = create_mock_reference(cwe_type=["CWE-125", "CWE-416"])

        result = cwe_type_precision_evaluator(output, reference)

        assert result["key"] == "cwe_type_precision"
        assert result["score"] == 0.0

    def test_no_predictions(self):
        """Test no predictions (should default to 1.0 precision)."""
        output = create_mock_output({"cwe_type": []})
        reference = create_mock_reference(cwe_type=["CWE-125"])

        result = cwe_type_precision_evaluator(output, reference)

        assert result["key"] == "cwe_type_precision"
        assert result["score"] == 1.0


class TestCweTypeRecallEvaluator:
    """Test CWE type recall evaluator."""

    def test_perfect_recall(self):
        """Test perfect recall."""
        output = create_mock_output({"cwe_type": ["CWE-125", "CWE-416", "CWE-787"]})
        reference = create_mock_reference(cwe_type=["CWE-125", "CWE-416"])

        result = cwe_type_recall_evaluator(output, reference)

        assert result["key"] == "cwe_type_recall"
        assert result["score"] == 1.0

    def test_zero_recall(self):
        """Test zero recall."""
        output = create_mock_output({"cwe_type": ["CWE-787"]})
        reference = create_mock_reference(cwe_type=["CWE-125", "CWE-416"])

        result = cwe_type_recall_evaluator(output, reference)

        assert result["key"] == "cwe_type_recall"
        assert result["score"] == 0.0

    def test_no_ground_truth(self):
        """Test no ground truth (should default to 1.0 recall)."""
        output = create_mock_output({"cwe_type": ["CWE-125"]})
        reference = create_mock_reference(cwe_type=[])

        result = cwe_type_recall_evaluator(output, reference)

        assert result["key"] == "cwe_type_recall"
        assert result["score"] == 1.0


class TestVulnerableFunctionsEvaluator:
    """Test vulnerable function detection evaluator."""

    def test_perfect_match(self):
        """Test perfect function match."""
        predicted_funcs = {"src/lib.rs": ["vulnerable_fn", "another_fn"]}
        expected_funcs = {"src/lib.rs": ["vulnerable_fn", "another_fn"]}

        output = create_mock_output({"vulnerable_functions": predicted_funcs})
        reference = create_mock_reference(vulnerable_functions=expected_funcs)

        result = vulnerable_functions_evaluator(output, reference)

        assert result["key"] == "vulnerable_functions_f1"
        assert result["score"] == 1.0

    def test_partial_match(self):
        """Test partial function match."""
        predicted_funcs = {"src/lib.rs": ["vulnerable_fn", "wrong_fn"]}
        expected_funcs = {"src/lib.rs": ["vulnerable_fn", "correct_fn"]}

        output = create_mock_output({"vulnerable_functions": predicted_funcs})
        reference = create_mock_reference(vulnerable_functions=expected_funcs)

        result = vulnerable_functions_evaluator(output, reference)

        # TP=1, FP=1, FN=1 -> P=0.5, R=0.5, F1=0.5
        assert result["key"] == "vulnerable_functions_f1"
        assert result["score"] == 0.5

    def test_different_files(self):
        """Test functions in different files."""
        predicted_funcs = {"src/lib.rs": ["fn1"], "src/other.rs": ["fn2"]}
        expected_funcs = {"src/lib.rs": ["fn1"], "src/main.rs": ["fn3"]}

        output = create_mock_output({"vulnerable_functions": predicted_funcs})
        reference = create_mock_reference(vulnerable_functions=expected_funcs)

        result = vulnerable_functions_evaluator(output, reference)

        # TP=1, FP=1, FN=1 -> P=0.5, R=0.5, F1=0.5
        assert result["key"] == "vulnerable_functions_f1"
        assert result["score"] == 0.5

    def test_empty_prediction(self):
        """Test empty function prediction."""
        output = create_mock_output({"vulnerable_functions": {}})
        reference = create_mock_reference(vulnerable_functions={"src/lib.rs": ["fn1"]})

        result = vulnerable_functions_evaluator(output, reference)

        assert result["key"] == "vulnerable_functions_f1"
        assert result["score"] == 0.0

    def test_both_empty(self):
        """Test both prediction and reference empty."""
        output = create_mock_output({"vulnerable_functions": {}})
        reference = create_mock_reference(vulnerable_functions={})

        result = vulnerable_functions_evaluator(output, reference)

        assert result["key"] == "vulnerable_functions_f1"
        assert result["score"] == 1.0


class TestVulnerableFunctionsPrecisionEvaluator:
    """Test vulnerable function precision evaluator."""

    def test_perfect_precision(self):
        """Test perfect precision."""
        predicted_funcs = {"src/lib.rs": ["fn1"]}
        expected_funcs = {"src/lib.rs": ["fn1", "fn2"]}

        output = create_mock_output({"vulnerable_functions": predicted_funcs})
        reference = create_mock_reference(vulnerable_functions=expected_funcs)

        result = vulnerable_functions_precision_evaluator(output, reference)

        assert result["key"] == "vulnerable_functions_precision"
        assert result["score"] == 1.0

    def test_zero_precision(self):
        """Test zero precision."""
        predicted_funcs = {"src/lib.rs": ["wrong_fn"]}
        expected_funcs = {"src/lib.rs": ["fn1", "fn2"]}

        output = create_mock_output({"vulnerable_functions": predicted_funcs})
        reference = create_mock_reference(vulnerable_functions=expected_funcs)

        result = vulnerable_functions_precision_evaluator(output, reference)

        assert result["key"] == "vulnerable_functions_precision"
        assert result["score"] == 0.0

    def test_partial_precision(self):
        """Test partial precision."""
        predicted_funcs = {"src/lib.rs": ["fn1", "wrong_fn"]}
        expected_funcs = {"src/lib.rs": ["fn1", "fn2"]}

        output = create_mock_output({"vulnerable_functions": predicted_funcs})
        reference = create_mock_reference(vulnerable_functions=expected_funcs)

        result = vulnerable_functions_precision_evaluator(output, reference)

        # TP=1, FP=1 -> P=0.5
        assert result["key"] == "vulnerable_functions_precision"
        assert result["score"] == 0.5

    def test_no_predictions(self):
        """Test no predictions (should default to 1.0 precision)."""
        output = create_mock_output({"vulnerable_functions": {}})
        reference = create_mock_reference(vulnerable_functions={"src/lib.rs": ["fn1"]})

        result = vulnerable_functions_precision_evaluator(output, reference)

        assert result["key"] == "vulnerable_functions_precision"
        assert result["score"] == 1.0

    def test_target_function_error(self):
        """Test handling of target function errors."""
        output = create_mock_output(errors="API call failed")
        reference = create_mock_reference(vulnerable_functions={"src/lib.rs": ["fn1"]})

        result = vulnerable_functions_precision_evaluator(output, reference)

        assert result["key"] == "vulnerable_functions_precision"
        assert result["score"] == 0
        assert "Target function failed" in result["comment"]


class TestVulnerableFunctionsRecallEvaluator:
    """Test vulnerable function recall evaluator."""

    def test_perfect_recall(self):
        """Test perfect recall."""
        predicted_funcs = {"src/lib.rs": ["fn1", "fn2", "extra_fn"]}
        expected_funcs = {"src/lib.rs": ["fn1", "fn2"]}

        output = create_mock_output({"vulnerable_functions": predicted_funcs})
        reference = create_mock_reference(vulnerable_functions=expected_funcs)

        result = vulnerable_functions_recall_evaluator(output, reference)

        assert result["key"] == "vulnerable_functions_recall"
        assert result["score"] == 1.0

    def test_zero_recall(self):
        """Test zero recall."""
        predicted_funcs = {"src/lib.rs": ["wrong_fn"]}
        expected_funcs = {"src/lib.rs": ["fn1", "fn2"]}

        output = create_mock_output({"vulnerable_functions": predicted_funcs})
        reference = create_mock_reference(vulnerable_functions=expected_funcs)

        result = vulnerable_functions_recall_evaluator(output, reference)

        assert result["key"] == "vulnerable_functions_recall"
        assert result["score"] == 0.0

    def test_partial_recall(self):
        """Test partial recall."""
        predicted_funcs = {"src/lib.rs": ["fn1"]}
        expected_funcs = {"src/lib.rs": ["fn1", "fn2"]}

        output = create_mock_output({"vulnerable_functions": predicted_funcs})
        reference = create_mock_reference(vulnerable_functions=expected_funcs)

        result = vulnerable_functions_recall_evaluator(output, reference)

        # TP=1, FN=1 -> R=0.5
        assert result["key"] == "vulnerable_functions_recall"
        assert result["score"] == 0.5

    def test_no_ground_truth(self):
        """Test no ground truth (should default to 1.0 recall)."""
        predicted_funcs = {"src/lib.rs": ["fn1"]}
        output = create_mock_output({"vulnerable_functions": predicted_funcs})
        reference = create_mock_reference(vulnerable_functions={})

        result = vulnerable_functions_recall_evaluator(output, reference)

        assert result["key"] == "vulnerable_functions_recall"
        assert result["score"] == 1.0

    def test_target_function_error(self):
        """Test handling of target function errors."""
        output = create_mock_output(errors="API call failed")
        reference = create_mock_reference(vulnerable_functions={"src/lib.rs": ["fn1"]})

        result = vulnerable_functions_recall_evaluator(output, reference)

        assert result["key"] == "vulnerable_functions_recall"
        assert result["score"] == 0
        assert "Target function failed" in result["comment"]


class TestVulnerableLinesEvaluator:
    """Test vulnerable line detection evaluator."""

    def test_perfect_match(self):
        """Test perfect line match."""
        predicted_lines = {"src/lib.rs": [10, 15, 20]}
        expected_lines = {"src/lib.rs": [10, 15, 20]}

        output = create_mock_output({"vulnerable_lines": predicted_lines})
        reference = create_mock_reference(vulnerable_lines=expected_lines)

        result = vulnerable_lines_evaluator(output, reference)

        assert result["key"] == "vulnerable_lines_f1"
        assert result["score"] == 1.0

    def test_partial_match(self):
        """Test partial line match."""
        predicted_lines = {"src/lib.rs": [10, 25]}
        expected_lines = {"src/lib.rs": [10, 15, 20]}

        output = create_mock_output({"vulnerable_lines": predicted_lines})
        reference = create_mock_reference(vulnerable_lines=expected_lines)

        result = vulnerable_lines_evaluator(output, reference)

        # TP=1, FP=1, FN=2 -> P=0.5, R=0.333, F1=0.4
        assert result["key"] == "vulnerable_lines_f1"
        assert result["score"] == pytest.approx(0.4)

    def test_different_files(self):
        """Test lines in different files."""
        predicted_lines = {"src/lib.rs": [10], "src/other.rs": [5]}
        expected_lines = {"src/lib.rs": [10], "src/main.rs": [15]}

        output = create_mock_output({"vulnerable_lines": predicted_lines})
        reference = create_mock_reference(vulnerable_lines=expected_lines)

        result = vulnerable_lines_evaluator(output, reference)

        # TP=1, FP=1, FN=1 -> P=0.5, R=0.5, F1=0.5
        assert result["key"] == "vulnerable_lines_f1"
        assert result["score"] == 0.5


class TestVulnerableLinesPrecisionEvaluator:
    """Test vulnerable line precision evaluator."""

    def test_perfect_precision(self):
        """Test perfect precision."""
        predicted_lines = {"src/lib.rs": [10]}
        expected_lines = {"src/lib.rs": [10, 15]}

        output = create_mock_output({"vulnerable_lines": predicted_lines})
        reference = create_mock_reference(vulnerable_lines=expected_lines)

        result = vulnerable_lines_precision_evaluator(output, reference)

        assert result["key"] == "vulnerable_lines_precision"
        assert result["score"] == 1.0

    def test_zero_precision(self):
        """Test zero precision."""
        predicted_lines = {"src/lib.rs": [25]}
        expected_lines = {"src/lib.rs": [10, 15]}

        output = create_mock_output({"vulnerable_lines": predicted_lines})
        reference = create_mock_reference(vulnerable_lines=expected_lines)

        result = vulnerable_lines_precision_evaluator(output, reference)

        assert result["key"] == "vulnerable_lines_precision"
        assert result["score"] == 0.0

    def test_partial_precision(self):
        """Test partial precision."""
        predicted_lines = {"src/lib.rs": [10, 25]}
        expected_lines = {"src/lib.rs": [10, 15]}

        output = create_mock_output({"vulnerable_lines": predicted_lines})
        reference = create_mock_reference(vulnerable_lines=expected_lines)

        result = vulnerable_lines_precision_evaluator(output, reference)

        # TP=1, FP=1 -> P=0.5
        assert result["key"] == "vulnerable_lines_precision"
        assert result["score"] == 0.5

    def test_no_predictions(self):
        """Test no predictions (should default to 1.0 precision)."""
        output = create_mock_output({"vulnerable_lines": {}})
        reference = create_mock_reference(vulnerable_lines={"src/lib.rs": [10]})

        result = vulnerable_lines_precision_evaluator(output, reference)

        assert result["key"] == "vulnerable_lines_precision"
        assert result["score"] == 1.0

    def test_target_function_error(self):
        """Test handling of target function errors."""
        output = create_mock_output(errors="API call failed")
        reference = create_mock_reference(vulnerable_lines={"src/lib.rs": [10]})

        result = vulnerable_lines_precision_evaluator(output, reference)

        assert result["key"] == "vulnerable_lines_precision"
        assert result["score"] == 0
        assert "Target function failed" in result["comment"]


class TestVulnerableLinesRecallEvaluator:
    """Test vulnerable line recall evaluator."""

    def test_perfect_recall(self):
        """Test perfect recall."""
        predicted_lines = {"src/lib.rs": [10, 15, 25]}
        expected_lines = {"src/lib.rs": [10, 15]}

        output = create_mock_output({"vulnerable_lines": predicted_lines})
        reference = create_mock_reference(vulnerable_lines=expected_lines)

        result = vulnerable_lines_recall_evaluator(output, reference)

        assert result["key"] == "vulnerable_lines_recall"
        assert result["score"] == 1.0

    def test_zero_recall(self):
        """Test zero recall."""
        predicted_lines = {"src/lib.rs": [25]}
        expected_lines = {"src/lib.rs": [10, 15]}

        output = create_mock_output({"vulnerable_lines": predicted_lines})
        reference = create_mock_reference(vulnerable_lines=expected_lines)

        result = vulnerable_lines_recall_evaluator(output, reference)

        assert result["key"] == "vulnerable_lines_recall"
        assert result["score"] == 0.0

    def test_partial_recall(self):
        """Test partial recall."""
        predicted_lines = {"src/lib.rs": [10]}
        expected_lines = {"src/lib.rs": [10, 15]}

        output = create_mock_output({"vulnerable_lines": predicted_lines})
        reference = create_mock_reference(vulnerable_lines=expected_lines)

        result = vulnerable_lines_recall_evaluator(output, reference)

        # TP=1, FN=1 -> R=0.5
        assert result["key"] == "vulnerable_lines_recall"
        assert result["score"] == 0.5

    def test_no_ground_truth(self):
        """Test no ground truth (should default to 1.0 recall)."""
        predicted_lines = {"src/lib.rs": [10]}
        output = create_mock_output({"vulnerable_lines": predicted_lines})
        reference = create_mock_reference(vulnerable_lines={})

        result = vulnerable_lines_recall_evaluator(output, reference)

        assert result["key"] == "vulnerable_lines_recall"
        assert result["score"] == 1.0

    def test_target_function_error(self):
        """Test handling of target function errors."""
        output = create_mock_output(errors="API call failed")
        reference = create_mock_reference(vulnerable_lines={"src/lib.rs": [10]})

        result = vulnerable_lines_recall_evaluator(output, reference)

        assert result["key"] == "vulnerable_lines_recall"
        assert result["score"] == 0
        assert "Target function failed" in result["comment"]


class TestJsonValidityEvaluator:
    """Test JSON validity evaluator."""

    def test_valid_json(self):
        """Test valid JSON with all required fields."""
        output = create_mock_output(
            {
                "is_vulnerable": True,
                "cwe_type": ["CWE-125"],
                "vulnerable_functions": {"src/lib.rs": ["fn1"]},
                "vulnerable_lines": {"src/lib.rs": [10]},
            }
        )
        reference = create_mock_reference()

        result = json_validity_evaluator(output, reference)

        assert result["key"] == "json_validity"
        assert result["score"] == 1
        assert "succeeded" in result["comment"]

    def test_missing_field(self):
        """Test JSON missing required field."""
        output = create_mock_output(
            {
                "is_vulnerable": True,
                "cwe_type": ["CWE-125"],
                "vulnerable_functions": {"src/lib.rs": ["fn1"]},
                # Missing vulnerable_lines
            }
        )
        reference = create_mock_reference()

        result = json_validity_evaluator(output, reference)

        assert result["key"] == "json_validity"
        assert result["score"] == 0
        assert "failed" in result["comment"]

    def test_empty_parsed_response(self):
        """Test empty parsed response."""
        output = create_mock_output({})
        reference = create_mock_reference()

        result = json_validity_evaluator(output, reference)

        assert result["key"] == "json_validity"
        assert result["score"] == 0
        assert "failed" in result["comment"]

    def test_no_parsed_response(self):
        """Test no parsed response."""
        output = create_mock_output()
        reference = create_mock_reference()

        result = json_validity_evaluator(output, reference)

        assert result["key"] == "json_validity"
        assert result["score"] == 0
        assert "failed" in result["comment"]

    def test_target_function_error(self):
        """Test handling of target function errors."""
        output = create_mock_output(errors="API call failed")
        reference = create_mock_reference()

        result = json_validity_evaluator(output, reference)

        assert result["key"] == "json_validity"
        assert result["score"] == 0
        assert "Target function failed" in result["comment"]


class TestEdgeCasesAndErrors:
    """Test edge cases and error handling."""

    def test_evaluator_exception_handling(self):
        """Test evaluator handles internal exceptions gracefully."""
        # Create malformed output that could cause exceptions
        output = {
            "parsed_response": {"cwe_type": None}
        }  # None will cause set() to fail
        reference = create_mock_reference(cwe_type=["CWE-125"])

        result = cwe_type_evaluator(output, reference)

        assert result["key"] == "cwe_type_f1"
        assert result["score"] == 0
        assert "Evaluator error" in result["comment"]
        assert result.get("extra", {}).get("error") is True

    def test_vulnerable_functions_malformed_data(self):
        """Test vulnerable functions evaluator with malformed data."""
        # Create output that will cause TypeError when iterating
        output = {"parsed_response": {"vulnerable_functions": {"src/lib.rs": None}}}
        reference = create_mock_reference(vulnerable_functions={"src/lib.rs": ["fn1"]})

        result = vulnerable_functions_evaluator(output, reference)

        assert result["key"] == "vulnerable_functions_f1"
        assert result["score"] == 0
        assert "Evaluator error" in result["comment"]

    def test_vulnerable_lines_malformed_data(self):
        """Test vulnerable lines evaluator with malformed data."""
        # Create output that will cause TypeError when iterating
        output = {"parsed_response": {"vulnerable_lines": {"src/lib.rs": None}}}
        reference = create_mock_reference(vulnerable_lines={"src/lib.rs": [10]})

        result = vulnerable_lines_evaluator(output, reference)

        assert result["key"] == "vulnerable_lines_f1"
        assert result["score"] == 0
        assert "Evaluator error" in result["comment"]
