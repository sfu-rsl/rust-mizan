import json
from inspect_ai.scorer import Score, Scorer, Target, accuracy, scorer
from inspect_ai.solver import TaskState
from inspect_ai.util import sandbox
from typing import Any


def validate_json_schema(parsed_response: dict[str, Any]) -> bool:
    try:
        if not isinstance(parsed_response, dict):
            return False

        required_fields = [
            "is_vulnerable",
            "cwe_type",
            "vulnerable_functions",
            "vulnerable_lines",
        ]

        if not all(field in parsed_response for field in required_fields):
            return False

        if not isinstance(parsed_response["is_vulnerable"], bool):
            return False

        if not isinstance(parsed_response["cwe_type"], list):
            return False
        if not all(isinstance(cwe, str) for cwe in parsed_response["cwe_type"]):
            return False

        if not isinstance(parsed_response["vulnerable_functions"], dict):
            return False
        if not all(
            isinstance(v, list)
            for v in parsed_response["vulnerable_functions"].values()
        ):
            return False

        if not isinstance(parsed_response["vulnerable_lines"], dict):
            return False
        if not all(
            isinstance(v, list) for v in parsed_response["vulnerable_lines"].values()
        ):
            return False
        if not all(
            all(isinstance(line, int) for line in v)
            for v in parsed_response["vulnerable_lines"].values()
        ):
            return False

        return True
    except:
        return False


def binary_accuracy(predicted: bool, actual: bool) -> float:
    return 1.0 if predicted == actual else 0.0


def compute_set_metrics(
    predicted_set: set[Any], actual_set: set[Any]
) -> tuple[int, int, int]:
    tp = len(predicted_set & actual_set)
    fp = len(predicted_set - actual_set)
    fn = len(actual_set - predicted_set)
    return tp, fp, fn


def success_at_1_function(func_tp: int, is_vulnerable: bool) -> float:
    return 1.0 if (is_vulnerable and func_tp > 0) else 0.0


def success_at_1_line(line_tp: int, is_vulnerable: bool) -> float:
    return 1.0 if (is_vulnerable and line_tp > 0) else 0.0


@scorer(metrics=[accuracy()])
def rustmizan_scorer() -> Scorer:
    async def score(state: TaskState, target: Target) -> Score:
        artifacts = {}

        try:
            content = await sandbox().read_file("results.json")
            artifacts["results.json"] = content
        except Exception as e:
            return Score(
                value="I",
                answer="no_file",
                explanation=f"results.json not found: {str(e)}",
                metadata={
                    "artifacts": artifacts,
                    "is_vulnerable_gt": state.metadata.get("is_vulnerable"),
                    "binary_accuracy": 0.0,
                    "cwe_tp": 0,
                    "cwe_fp": 0,
                    "cwe_fn": 0,
                    "function_tp": 0,
                    "function_fp": 0,
                    "function_fn": 0,
                    "line_tp": 0,
                    "line_fp": 0,
                    "line_fn": 0,
                    "success_at_1_function": 0.0,
                    "success_at_1_line": 0.0,
                },
            )

        try:
            parsed = json.loads(content)

            if not validate_json_schema(parsed):
                return Score(
                    value="I",
                    answer="invalid",
                    explanation="Invalid JSON schema",
                    metadata={
                        "artifacts": artifacts,
                        "is_vulnerable_gt": state.metadata.get("is_vulnerable"),
                        "binary_accuracy": 0.0,
                        "cwe_tp": 0,
                        "cwe_fp": 0,
                        "cwe_fn": 0,
                        "function_tp": 0,
                        "function_fp": 0,
                        "function_fn": 0,
                        "line_tp": 0,
                        "line_fp": 0,
                        "line_fn": 0,
                        "success_at_1_function": 0.0,
                        "success_at_1_line": 0.0,
                    },
                )

            actual = {
                "is_vulnerable": state.metadata["is_vulnerable"],
                "cwe_type": state.metadata["cwe_type"],
                "vulnerable_functions": state.metadata["vulnerable_functions"],
                "vulnerable_lines": state.metadata["vulnerable_lines"],
            }

            is_vulnerable = actual["is_vulnerable"]

            bin_accuracy = binary_accuracy(parsed["is_vulnerable"], is_vulnerable)

            cwe_pred_set = set(parsed["cwe_type"])
            cwe_actual_set = set(actual["cwe_type"])
            cwe_tp, cwe_fp, cwe_fn = compute_set_metrics(cwe_pred_set, cwe_actual_set)

            func_pred_set = {
                (file_path, func)
                for file_path, functions in parsed["vulnerable_functions"].items()
                for func in functions
            }
            func_actual_set = {
                (file_path, func)
                for file_path, functions in actual["vulnerable_functions"].items()
                for func in functions
            }
            func_tp, func_fp, func_fn = compute_set_metrics(
                func_pred_set, func_actual_set
            )

            line_pred_set = {
                (file_path, line)
                for file_path, lines in parsed["vulnerable_lines"].items()
                for line in lines
            }
            line_actual_set = {
                (file_path, line)
                for file_path, lines in actual["vulnerable_lines"].items()
                for line in lines
            }
            line_tp, line_fp, line_fn = compute_set_metrics(
                line_pred_set, line_actual_set
            )

            success_at_1_func = success_at_1_function(func_tp, is_vulnerable)
            success_at_1_line_score = success_at_1_line(line_tp, is_vulnerable)

            return Score(
                value="C",
                answer="valid",
                explanation="Valid response with all required fields",
                metadata={
                    "parsed_response": parsed,
                    "artifacts": artifacts,
                    "is_vulnerable_gt": is_vulnerable,
                    "binary_accuracy": bin_accuracy,
                    "cwe_tp": cwe_tp,
                    "cwe_fp": cwe_fp,
                    "cwe_fn": cwe_fn,
                    "function_tp": func_tp,
                    "function_fp": func_fp,
                    "function_fn": func_fn,
                    "line_tp": line_tp,
                    "line_fp": line_fp,
                    "line_fn": line_fn,
                    "success_at_1_function": success_at_1_func,
                    "success_at_1_line": success_at_1_line_score,
                },
            )

        except json.JSONDecodeError as e:
            return Score(
                value="I",
                answer="invalid",
                explanation=f"JSON parsing failed: {str(e)}",
                metadata={
                    "artifacts": artifacts,
                    "is_vulnerable_gt": state.metadata.get("is_vulnerable"),
                    "binary_accuracy": 0.0,
                    "cwe_tp": 0,
                    "cwe_fp": 0,
                    "cwe_fn": 0,
                    "function_tp": 0,
                    "function_fp": 0,
                    "function_fn": 0,
                    "line_tp": 0,
                    "line_fp": 0,
                    "line_fn": 0,
                    "success_at_1_function": 0.0,
                    "success_at_1_line": 0.0,
                },
            )
        except Exception as e:
            return Score(
                value="I",
                answer="error",
                explanation=f"Error computing metrics: {str(e)}",
                metadata={
                    "artifacts": artifacts,
                    "is_vulnerable_gt": state.metadata.get("is_vulnerable"),
                    "binary_accuracy": 0.0,
                    "cwe_tp": 0,
                    "cwe_fp": 0,
                    "cwe_fn": 0,
                    "function_tp": 0,
                    "function_fp": 0,
                    "function_fn": 0,
                    "line_tp": 0,
                    "line_fp": 0,
                    "line_fn": 0,
                    "success_at_1_function": 0.0,
                    "success_at_1_line": 0.0,
                },
            )

    return score
