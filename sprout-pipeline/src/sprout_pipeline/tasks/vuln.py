from pathlib import Path
from typing import Dict, List, Tuple, Iterator, Any, Optional, Set
import json
from importlib.resources import files

from .base import TaskBase
from ..models import VulnerabilityReport


class VulnerabilityTask(TaskBase):
    """
    Task to detect memory-safety vulnerabilities in Rust code.
    
    This task analyzes Rust source code to detect vulnerabilities,
    identify their CWE types, vulnerable functions, and line numbers.
    """
    name = "vuln"
    schema = VulnerabilityReport
    prompt_path = files("sprout_pipeline.prompts").joinpath("detect_vulnerability.txt")

    @staticmethod
    def iterate_samples(mizan_path: Path) -> Iterator[Tuple[Path, Any, Optional[Dict]]]:
        """
        Yield samples from mizan.json for vulnerability detection.
        
        For each code sample in the dataset, extract the path to the code
        and the ground truth vulnerability information.
        
        Args:
            mizan_path: Path to mizan.json file
            
        Returns:
            Iterator of (code_dir, ground_truth, None) tuples
        """
        data = json.loads(mizan_path.read_text())
        base_dir = mizan_path.parent
        
        for vuln in data["vulnerabilities"]:
            for sample in vuln["code_samples"]:
                code_dir = base_dir / sample["path_to_crate"]
                
                # Extract ground truth
                ground_truth = {
                    k: sample[k]
                    for k in (
                        "is_vulnerability",
                        "cwe_type",
                        "vulnerable_functions",
                        "vulnerable_lines",
                    )
                }
                
                yield code_dir, ground_truth, None

    @staticmethod
    def score(pred: Dict, truth: Dict) -> Dict:
        """
        Score the vulnerability detection prediction against ground truth.
        
        Args:
            pred: Model prediction dictionary
            truth: Ground truth dictionary
            
        Returns:
            Dictionary with structured scoring metrics
        """
        # Check if vulnerability existence prediction is correct
        existence_ok = bool(pred["is_vulnerable"]) == bool(truth["is_vulnerability"])

        # Score CWE type predictions
        pred_cwe, true_cwe = set(pred["cwe_type"]), set(truth["cwe_type"])
        cwe_score = {
            "correct_predictions": sorted(pred_cwe & true_cwe),
            "missed_predictions": sorted(true_cwe - pred_cwe),
            "extra_predictions": sorted(pred_cwe - true_cwe),
        }

        # Helper function to flatten dictionaries for comparison
        def flatten(d: Dict[str, List]) -> Set[str]:
            """Convert {"file":[items]} to {"file:item"} for easier comparison"""
            return {f"{k}:{v}" for k, lst in d.items() for v in lst}

        # Score vulnerable function predictions
        pred_funcs = flatten(pred["vulnerable_functions"])
        true_funcs = flatten(truth["vulnerable_functions"])
        function_score = {
            "true_positive_keys": sorted(pred_funcs & true_funcs),
            "false_positive_keys": sorted(pred_funcs - true_funcs),
            "missed_keys": sorted(true_funcs - pred_funcs),
        }

        # Score vulnerable line predictions
        pred_lines = flatten(pred["vulnerable_lines"])
        true_lines = flatten(truth["vulnerable_lines"])
        line_score = {
            "true_positive_keys": sorted(pred_lines & true_lines),
            "false_positive_keys": sorted(pred_lines - true_lines),
            "missed_keys": sorted(true_lines - pred_lines),
        }

        return {
            "existence_detection": {"is_correct": existence_ok},
            "cwe_inference": cwe_score,
            "key_objects_identification": function_score,
            "root_cause_location": line_score,
        }

    @staticmethod
    def dataset_row(meta: Dict, score: Dict) -> Dict[str, Any]:
        """
        Create a CSV row from scoring results.
        
        Args:
            meta: Run metadata
            score: Scoring results
            
        Returns:
            Flattened dictionary for CSV export
        """
        return {
            **meta,
            "existence_correct": score["existence_detection"]["is_correct"],
            "cwe_tp": len(score["cwe_inference"]["correct_predictions"]),
            "cwe_fp": len(score["cwe_inference"]["extra_predictions"]),
            "cwe_fn": len(score["cwe_inference"]["missed_predictions"]),
            "func_tp": len(score["key_objects_identification"]["true_positive_keys"]),
            "func_fp": len(score["key_objects_identification"]["false_positive_keys"]),
            "func_fn": len(score["key_objects_identification"]["missed_keys"]),
            "line_tp": len(score["root_cause_location"]["true_positive_keys"]),
            "line_fp": len(score["root_cause_location"]["false_positive_keys"]),
            "line_fn": len(score["root_cause_location"]["missed_keys"]),
        }