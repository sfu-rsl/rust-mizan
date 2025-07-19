import pytest
from mizan_cli.commands.evaluate.evaluators.summary_evaluators import (
    calculate_is_vulnerable_metrics,
    calculate_cwe_type_micro_f1,
    calculate_cwe_type_macro_f1,
    calculate_cwe_type_micro_precision,
    calculate_cwe_type_macro_precision,
    calculate_cwe_type_micro_recall,
    calculate_cwe_type_macro_recall,
    calculate_vulnerable_functions_micro_f1,
    calculate_vulnerable_functions_macro_f1,
    calculate_vulnerable_lines_micro_f1,
    calculate_vulnerable_lines_macro_f1,
    calculate_json_validity_rate,
)


def create_mock_experiment_output(
    cwe_type=None,
    vulnerable_functions=None,
    vulnerable_lines=None,
    is_vulnerable=False,
    errors=None,
):
    """Helper to create mock experiment output."""
    output = {}
    if errors is not None:
        output["errors"] = errors
        return output

    parsed_response = {
        "is_vulnerable": is_vulnerable,
        "cwe_type": cwe_type or [],
        "vulnerable_functions": vulnerable_functions or {},
        "vulnerable_lines": vulnerable_lines or {},
    }
    output["parsed_response"] = parsed_response
    return output


def create_mock_reference_output(
    cwe_type=None, vulnerable_functions=None, vulnerable_lines=None, is_vulnerable=False
):
    """Helper to create mock reference output."""
    return {
        "is_vulnerable": is_vulnerable,
        "cwe_type": cwe_type or [],
        "vulnerable_functions": vulnerable_functions or {},
        "vulnerable_lines": vulnerable_lines or {},
    }


class TestIsVulnerableMetrics:
    """Test binary classification metrics calculation."""

    def test_perfect_classification(self):
        """Test perfect binary classification."""
        outputs = [
            create_mock_experiment_output(is_vulnerable=True),
            create_mock_experiment_output(is_vulnerable=False),
            create_mock_experiment_output(is_vulnerable=True),
        ]
        reference_outputs = [
            create_mock_reference_output(is_vulnerable=True),
            create_mock_reference_output(is_vulnerable=False),
            create_mock_reference_output(is_vulnerable=True),
        ]

        metrics = calculate_is_vulnerable_metrics(outputs, reference_outputs)

        assert metrics["accuracy"] == 1.0
        assert metrics["precision"] == 1.0
        assert metrics["recall"] == 1.0
        assert metrics["f1"] == 1.0

    def test_mixed_classification(self):
        """Test mixed classification results."""
        outputs = [
            create_mock_experiment_output(is_vulnerable=True),  # TP
            create_mock_experiment_output(is_vulnerable=True),  # FP
            create_mock_experiment_output(is_vulnerable=False),  # FN
            create_mock_experiment_output(is_vulnerable=False),  # TN
        ]
        reference_outputs = [
            create_mock_reference_output(is_vulnerable=True),  # TP
            create_mock_reference_output(is_vulnerable=False),  # FP
            create_mock_reference_output(is_vulnerable=True),  # FN
            create_mock_reference_output(is_vulnerable=False),  # TN
        ]

        metrics = calculate_is_vulnerable_metrics(outputs, reference_outputs)

        # TP=1, TN=1, FP=1, FN=1
        assert metrics["accuracy"] == 0.5  # (1+1)/(1+1+1+1)
        assert metrics["precision"] == 0.5  # 1/(1+1)
        assert metrics["recall"] == 0.5  # 1/(1+1)
        assert metrics["f1"] == 0.5  # 2*(0.5*0.5)/(0.5+0.5)

    def test_all_positive_predictions(self):
        """Test all positive predictions."""
        outputs = [
            create_mock_experiment_output(is_vulnerable=True),
            create_mock_experiment_output(is_vulnerable=True),
        ]
        reference_outputs = [
            create_mock_reference_output(is_vulnerable=True),
            create_mock_reference_output(is_vulnerable=False),
        ]

        metrics = calculate_is_vulnerable_metrics(outputs, reference_outputs)

        # TP=1, FP=1, FN=0, TN=0
        assert metrics["accuracy"] == 0.5
        assert metrics["precision"] == 0.5
        assert metrics["recall"] == 1.0
        assert metrics["f1"] == pytest.approx(0.6667, abs=1e-3)

    def test_no_positive_predictions(self):
        """Test no positive predictions."""
        outputs = [
            create_mock_experiment_output(is_vulnerable=False),
            create_mock_experiment_output(is_vulnerable=False),
        ]
        reference_outputs = [
            create_mock_reference_output(is_vulnerable=True),
            create_mock_reference_output(is_vulnerable=False),
        ]

        metrics = calculate_is_vulnerable_metrics(outputs, reference_outputs)

        # TP=0, FP=0, FN=1, TN=1
        assert metrics["accuracy"] == 0.5
        assert metrics["precision"] == 1.0  # No positive predictions
        assert metrics["recall"] == 0.0
        assert metrics["f1"] == 0.0

    def test_skip_errors(self):
        """Test skipping outputs with errors."""
        outputs = [
            create_mock_experiment_output(is_vulnerable=True),
            create_mock_experiment_output(errors="API failed"),
            create_mock_experiment_output(is_vulnerable=False),
        ]
        reference_outputs = [
            create_mock_reference_output(is_vulnerable=True),
            create_mock_reference_output(is_vulnerable=True),
            create_mock_reference_output(is_vulnerable=False),
        ]

        metrics = calculate_is_vulnerable_metrics(outputs, reference_outputs)

        # Only first and third samples counted: TP=1, TN=1
        assert metrics["accuracy"] == 1.0
        assert metrics["precision"] == 1.0
        assert metrics["recall"] == 1.0
        assert metrics["f1"] == 1.0

    def test_empty_inputs(self):
        """Test empty inputs."""
        metrics = calculate_is_vulnerable_metrics([], [])

        assert metrics["accuracy"] == 0.0
        assert metrics["precision"] == 0.0
        assert metrics["recall"] == 0.0
        assert metrics["f1"] == 0.0


class TestCweTypeMicroVsMacro:
    """Test micro vs macro averaging for CWE types."""

    def test_micro_vs_macro_f1(self):
        """
        Tests the difference between micro and macro F1 averaging
        using a carefully crafted dataset.
        """
        # Sample 1: Good recall, perfect precision (TP=1, FP=0, FN=1) -> F1 = 0.667
        # Sample 2: Bad precision, perfect recall (TP=1, FP=3, FN=0) -> F1 = 0.4
        outputs = [
            create_mock_experiment_output(["A"]),
            create_mock_experiment_output(["C", "D", "E", "F"]),
        ]
        reference_outputs = [
            {"cwe_type": ["A", "B"]},
            {"cwe_type": ["C"]},
        ]

        # --- MACRO F1 ---
        # F1_sample1 = 0.666...
        # F1_sample2 = 0.4
        # Macro F1 = (0.666... + 0.4) / 2 = 0.5333...
        macro_f1 = calculate_cwe_type_macro_f1(outputs, reference_outputs)
        assert macro_f1 == pytest.approx(0.53333, abs=1e-4)

        # --- MICRO F1 ---
        # Total TP = 1 (from sample1) + 1 (from sample2) = 2
        # Total FP = 0 (from sample1) + 3 (from sample2) = 3
        # Total FN = 1 (from sample1) + 0 (from sample2) = 1
        # Micro-P = 2 / (2+3) = 0.4
        # Micro-R = 2 / (2+1) = 0.666...
        micro_f1 = calculate_cwe_type_micro_f1(outputs, reference_outputs)
        assert micro_f1 == pytest.approx(0.6667, abs=1e-3)

    def test_micro_vs_macro_precision(self):
        """Test micro vs macro precision calculation."""
        outputs = [
            create_mock_experiment_output(["A", "B"]),  # TP=1, FP=1
            create_mock_experiment_output(["C"]),  # TP=1, FP=0
        ]
        reference_outputs = [
            {"cwe_type": ["A"]},
            {"cwe_type": ["C"]},
        ]

        # Macro precision = (0.5 + 1.0) / 2 = 0.75
        macro_precision = calculate_cwe_type_macro_precision(outputs, reference_outputs)
        assert macro_precision == 0.75

        micro_precision = calculate_cwe_type_micro_precision(outputs, reference_outputs)
        assert micro_precision == pytest.approx(0.8333, abs=1e-3)

    def test_micro_vs_macro_recall(self):
        """Test micro vs macro recall calculation."""
        outputs = [
            create_mock_experiment_output(["A"]),  # TP=1, FN=1
            create_mock_experiment_output(["C", "D"]),  # TP=1, FN=0
        ]
        reference_outputs = [
            {"cwe_type": ["A", "B"]},
            {"cwe_type": ["C"]},
        ]

        # Macro recall = (0.5 + 1.0) / 2 = 0.75
        macro_recall = calculate_cwe_type_macro_recall(outputs, reference_outputs)
        assert macro_recall == 0.75

        micro_recall = calculate_cwe_type_micro_recall(outputs, reference_outputs)
        assert micro_recall == pytest.approx(0.75, abs=1e-3)

    def test_perfect_micro_macro_match(self):
        """Test case where micro and macro metrics should be identical."""
        # When all samples have identical performance, micro = macro
        outputs = [
            create_mock_experiment_output(["A"]),
            create_mock_experiment_output(["B"]),
        ]
        reference_outputs = [
            {"cwe_type": ["A", "C"]},
            {"cwe_type": ["B", "D"]},
        ]

        # Each sample: TP=1, FP=0, FN=1 -> P=1.0, R=0.5, F1=0.667
        macro_f1 = calculate_cwe_type_macro_f1(outputs, reference_outputs)
        micro_f1 = calculate_cwe_type_micro_f1(outputs, reference_outputs)

        assert micro_f1 == pytest.approx(0.75, abs=1e-3)
        assert macro_f1 == pytest.approx(0.6667, abs=1e-3)


class TestVulnerableFunctionsSummary:
    """Test vulnerable functions summary metrics."""

    def test_micro_functions_f1(self):
        """Test micro F1 for vulnerable functions."""
        outputs = [
            create_mock_experiment_output(
                vulnerable_functions={"file1.rs": ["fn1", "fn2"]}
            ),
            create_mock_experiment_output(vulnerable_functions={"file1.rs": ["fn3"]}),
        ]
        reference_outputs = [
            {"vulnerable_functions": {"file1.rs": ["fn1", "fn3"]}},
            {"vulnerable_functions": {"file1.rs": ["fn3", "fn4"]}},
        ]

        # Sample 1: TP=1 (fn1), FP=1 (fn2), FN=1 (fn3)
        # Sample 2: TP=1 (fn3), FP=0, FN=1 (fn4)
        # Total: TP=2, FP=1, FN=2
        # Micro-P = 2/(2+1) = 0.667, Micro-R = 2/(2+2) = 0.5
        # Micro-F1 = 2 * (0.667 * 0.5) / (0.667 + 0.5) = 0.571

        micro_f1 = calculate_vulnerable_functions_micro_f1(outputs, reference_outputs)
        assert micro_f1 == pytest.approx(0.625, abs=1e-3)

    def test_macro_functions_f1(self):
        """Test macro F1 for vulnerable functions."""
        outputs = [
            create_mock_experiment_output(vulnerable_functions={"file1.rs": ["fn1"]}),
            create_mock_experiment_output(
                vulnerable_functions={"file1.rs": ["fn3", "fn4"]}
            ),
        ]
        reference_outputs = [
            {"vulnerable_functions": {"file1.rs": ["fn1", "fn2"]}},
            {"vulnerable_functions": {"file1.rs": ["fn3"]}},
        ]

        # Sample 1: TP=1, FP=0, FN=1 -> P=1.0, R=0.5, F1=0.667
        # Sample 2: TP=1, FP=1, FN=0 -> P=0.5, R=1.0, F1=0.667
        # Macro F1 = (0.667 + 0.667) / 2 = 0.667

        macro_f1 = calculate_vulnerable_functions_macro_f1(outputs, reference_outputs)
        assert macro_f1 == pytest.approx(0.6667, abs=1e-3)

    def test_empty_functions(self):
        """Test handling of empty function predictions."""
        outputs = [
            create_mock_experiment_output(vulnerable_functions={}),
            create_mock_experiment_output(vulnerable_functions={"file1.rs": ["fn1"]}),
        ]
        reference_outputs = [
            {"vulnerable_functions": {}},
            {"vulnerable_functions": {"file1.rs": ["fn1"]}},
        ]

        micro_f1 = calculate_vulnerable_functions_micro_f1(outputs, reference_outputs)
        macro_f1 = calculate_vulnerable_functions_macro_f1(outputs, reference_outputs)

        # First sample: perfect match (empty = empty) -> F1 = 1.0
        # Second sample: perfect match -> F1 = 1.0
        assert micro_f1 == 1.0  # Perfect match when samples have perfect matches
        assert (
            macro_f1 == 1.0
        )  # Average of [1.0, 1.0] - both samples are perfect matches


class TestVulnerableLinesSummary:
    """Test vulnerable lines summary metrics."""

    def test_micro_lines_f1(self):
        """Test micro F1 for vulnerable lines."""
        outputs = [
            create_mock_experiment_output(vulnerable_lines={"file1.rs": [10, 20]}),
            create_mock_experiment_output(vulnerable_lines={"file1.rs": [30]}),
        ]
        reference_outputs = [
            {"vulnerable_lines": {"file1.rs": [10, 15]}},
            {"vulnerable_lines": {"file1.rs": [30, 35]}},
        ]

        # Sample 1: TP=1 (line 10), FP=1 (line 20), FN=1 (line 15)
        # Sample 2: TP=1 (line 30), FP=0, FN=1 (line 35)
        # Total: TP=2, FP=1, FN=2
        # Micro-P = 2/(2+1) = 0.667, Micro-R = 2/(2+2) = 0.5
        # Micro-F1 = 2 * (0.667 * 0.5) / (0.667 + 0.5) = 0.571

        micro_f1 = calculate_vulnerable_lines_micro_f1(outputs, reference_outputs)
        assert micro_f1 == pytest.approx(0.7, abs=1e-3)

    def test_macro_lines_f1(self):
        """Test macro F1 for vulnerable lines."""
        outputs = [
            create_mock_experiment_output(vulnerable_lines={"file1.rs": [10]}),
            create_mock_experiment_output(vulnerable_lines={"file1.rs": [30, 35]}),
        ]
        reference_outputs = [
            {"vulnerable_lines": {"file1.rs": [10, 15]}},
            {"vulnerable_lines": {"file1.rs": [30]}},
        ]

        # Sample 1: TP=1, FP=0, FN=1 -> P=1.0, R=0.5, F1=0.667
        # Sample 2: TP=1, FP=1, FN=0 -> P=0.5, R=1.0, F1=0.667
        # Macro F1 = (0.667 + 0.667) / 2 = 0.667

        macro_f1 = calculate_vulnerable_lines_macro_f1(outputs, reference_outputs)
        assert macro_f1 == pytest.approx(0.6667, abs=1e-3)

    def test_different_files_lines(self):
        """Test lines in different files."""
        outputs = [
            create_mock_experiment_output(
                vulnerable_lines={"file1.rs": [10], "file2.rs": [20]}
            ),
            create_mock_experiment_output(vulnerable_lines={"file1.rs": [30]}),
        ]
        reference_outputs = [
            {"vulnerable_lines": {"file1.rs": [10], "file3.rs": [40]}},
            {"vulnerable_lines": {"file1.rs": [30]}},
        ]

        # Sample 1: TP=1 (file1:10), FP=1 (file2:20), FN=1 (file3:40)
        # Sample 2: TP=1 (file1:30), FP=0, FN=0
        # Total: TP=2, FP=1, FN=1

        micro_f1 = calculate_vulnerable_lines_micro_f1(outputs, reference_outputs)
        assert micro_f1 == pytest.approx(0.75, abs=1e-3)


class TestJsonValidityRate:
    """Test JSON validity rate calculation."""

    def test_all_valid(self):
        """Test all valid JSON responses."""
        outputs = [
            create_mock_experiment_output(["CWE-125"]),
            create_mock_experiment_output(["CWE-416"]),
        ]

        validity_rate = calculate_json_validity_rate(outputs, [])
        assert validity_rate == 1.0

    def test_mixed_validity(self):
        """Test mixed valid/invalid JSON responses."""
        outputs = [
            create_mock_experiment_output(["CWE-125"]),  # Valid
            create_mock_experiment_output(errors="API failed"),  # Invalid (error)
            {"parsed_response": {"is_vulnerable": True}},  # Invalid (missing fields)
            create_mock_experiment_output(["CWE-416"]),  # Valid
        ]

        validity_rate = calculate_json_validity_rate(outputs, [])
        assert validity_rate == 0.5  # 2 valid out of 4 total

    def test_all_invalid(self):
        """Test all invalid JSON responses."""
        outputs = [
            create_mock_experiment_output(errors="API failed"),
            {"parsed_response": {"is_vulnerable": True}},  # Missing required fields
            {"parsed_response": {}},  # Empty response
        ]

        validity_rate = calculate_json_validity_rate(outputs, [])
        assert validity_rate == 0.0

    def test_empty_outputs(self):
        """Test empty outputs list."""
        validity_rate = calculate_json_validity_rate([], [])
        assert validity_rate == 0.0


class TestEdgeCasesAndCornerCases:
    """Test edge cases and corner cases."""

    def test_all_samples_have_errors(self):
        """Test when all samples have errors."""
        outputs = [
            create_mock_experiment_output(errors="API failed"),
            create_mock_experiment_output(errors="Timeout"),
        ]
        reference_outputs = [
            {"cwe_type": ["CWE-125"]},
            {"cwe_type": ["CWE-416"]},
        ]

        # All samples should be skipped, so we have no data to process
        # When there are no valid samples, the functions return 0.0
        micro_f1 = calculate_cwe_type_micro_f1(outputs, reference_outputs)
        macro_f1 = calculate_cwe_type_macro_f1(outputs, reference_outputs)

        # With no valid samples to process, both should return 0.0
        assert micro_f1 == 0.0
        assert macro_f1 == 0.0

    def test_zero_division_protection(self):
        """Test protection against zero division."""
        # Case where all predictions and references are empty
        outputs = [
            create_mock_experiment_output([]),
            create_mock_experiment_output([]),
        ]
        reference_outputs = [
            {"cwe_type": []},
            {"cwe_type": []},
        ]

        micro_f1 = calculate_cwe_type_micro_f1(outputs, reference_outputs)
        macro_f1 = calculate_cwe_type_macro_f1(outputs, reference_outputs)

        assert micro_f1 == 0.0  # No predictions, no ground truth -> F1 = 0 in sklearn
        assert macro_f1 == 1.0  # Sample-level perfect matches

    def test_single_sample(self):
        """Test with single sample."""
        outputs = [create_mock_experiment_output(["CWE-125", "CWE-416"])]
        reference_outputs = [{"cwe_type": ["CWE-125"]}]

        # TP=1, FP=1, FN=0 -> P=0.5, R=1.0, F1=0.667
        micro_f1 = calculate_cwe_type_micro_f1(outputs, reference_outputs)
        macro_f1 = calculate_cwe_type_macro_f1(outputs, reference_outputs)

        assert micro_f1 == pytest.approx(0.5, abs=1e-3)
        assert macro_f1 == pytest.approx(0.6667, abs=1e-3)

    def test_mismatched_input_lengths(self):
        """Test mismatched input lengths (shouldn't happen in practice)."""
        outputs = [create_mock_experiment_output(["CWE-125"])]
        reference_outputs = [
            {"cwe_type": ["CWE-125"]},
            {"cwe_type": ["CWE-416"]},  # Extra reference
        ]

        # Should only process pairs that exist
        micro_f1 = calculate_cwe_type_micro_f1(outputs, reference_outputs)
        assert micro_f1 == 1.0  # Perfect match for the one pair
