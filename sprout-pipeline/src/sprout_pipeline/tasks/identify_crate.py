from pathlib import Path
from typing import Dict, List, Tuple, Iterator, Any, Optional
import json
from importlib.resources import files

from .base import TaskBase
from ..models import CrateIDReport


class IdentifyCrateTask(TaskBase):
    """
    Task to identify crate name, year, and CVEs from source code.
    
    This task analyzes Rust source code to determine which crate it belongs to,
    when it was likely published, and whether it has associated CVEs.
    """
    name = "identify_crate"
    schema = CrateIDReport
    prompt_path = files("sprout_pipeline.prompts").joinpath("identify_crate.txt")

    @staticmethod
    def iterate_samples(mizan: Path) -> Iterator[Tuple[Path, Any, Dict]]:
        """
        Yield samples from mizan.json for crate identification.
        
        For each code sample in the dataset, extract crate name, year, and CVE information.
        
        Args:
            mizan: Path to mizan.json file
            
        Returns:
            Iterator of (code_dir, ground_truth, context) tuples
        """
        data = json.loads(mizan.read_text())
        root = mizan.parent
        
        for vuln in data["vulnerabilities"]:
            # Common ground truth for all samples of this vulnerability
            gt_common = {
                "crate_name": vuln["crate_name"],
                "likely_year": vuln["year"],
                "has_cve": bool(vuln.get("source_link")),
                "cve_list": (
                    [vuln["source_link"].split("=")[-1]]
                    if vuln.get("source_link")
                    else []
                ),
            }
            
            # Process each code sample for this vulnerability
            for sample in vuln["code_samples"]:
                yield (
                    root / sample["path_to_crate"],  # code_dir
                    gt_common,  # ground truth
                    gt_common,  # context for prompt
                )

    @staticmethod
    def score(pred: Dict, truth: Dict) -> Dict:
        """
        Score the crate identification prediction against ground truth.
        
        Args:
            pred: Model prediction dictionary
            truth: Ground truth dictionary
            
        Returns:
            Dictionary with scoring metrics
        """
        return {
            "crate_name_correct": (pred["crate_name"] or "").lower() == truth["crate_name"].lower(),
            "year_correct": pred["likely_year"] == truth["likely_year"],
            "has_cve_correct": pred["has_cve"] == truth["has_cve"],
            "cve_tp": sorted(set(pred["cve_list"]) & set(truth["cve_list"])),
            "cve_fn": sorted(set(truth["cve_list"]) - set(pred["cve_list"])),
            "cve_fp": sorted(set(pred["cve_list"]) - set(truth["cve_list"])),
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
        return {**meta, **score}

    @staticmethod
    def build_prompt(base_prompt_path: Path, ctx: Dict) -> str:
        """
        Customize the prompt with crate name and year for testing.
        
        Note: In a real-world scenario, this would defeat the purpose of the task,
        but it's useful for validation testing.
        
        Args:
            base_prompt_path: Path to the base prompt template
            ctx: Context with crate_name and likely_year
            
        Returns:
            Complete prompt string with crate context
        """
        base = base_prompt_path.read_text(encoding="utf-8")
        header = (
            f"crate_name = {ctx['crate_name']}\n"
            f"year = {ctx['likely_year']}\n\n"
        )
        return header + base