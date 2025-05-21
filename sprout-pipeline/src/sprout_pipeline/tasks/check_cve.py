from pathlib import Path
from typing import Dict, List, Tuple, Iterator, Any, Optional
import json
from importlib.resources import files

from .base import TaskBase
from ..models import CVECheckReport


class CheckCVETask(TaskBase):
    """
    Task to predict whether CVEs exist for a given crate and year without access to code.

    This task uses only the crate name and year as input, without any code samples.
    """

    name = "check_cve"
    schema = CVECheckReport
    prompt_path = files("sprout_pipeline.prompts").joinpath("check_cve.md")

    @staticmethod
    def iterate_samples(mizan: Path) -> Iterator[Tuple[Optional[Path], Any, Dict]]:
        """
        Yield samples from mizan.json for CVE checking.

        For each vulnerability in the dataset, extract crate name, year, and
        CVE information. No code is needed for this task.

        Args:
            mizan: Path to mizan.json file

        Returns:
            Iterator of (None, ground_truth, context) tuples
        """
        data = json.loads(mizan.read_text())

        for vuln in data["vulnerabilities"]:
            # Extract ground truth
            truth = {
                "crate_name": vuln["crate_name"],
                "year": vuln["year"],
                "has_cve": bool(vuln.get("source_link")),
                "cve_list": (
                    [vuln["source_link"].split("=")[-1]]
                    if vuln.get("source_link")
                    else []
                ),
            }
            # Context for prompt customization
            ctx = {k: truth[k] for k in ("crate_name", "year")}

            # No code_dir needed for this task
            yield None, truth, ctx

    @staticmethod
    def score(pred: Dict, truth: Dict) -> Dict:
        """
        Score the CVE prediction against ground truth.

        Args:
            pred: Model prediction dictionary
            truth: Ground truth dictionary

        Returns:
            Dictionary with scoring metrics
        """
        return {
            "crate_name": truth["crate_name"],
            "year": truth["year"],
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
        Customize the prompt with crate name and year.

        Args:
            base_prompt_path: Path to the base prompt template
            ctx: Context with crate_name and year

        Returns:
            Complete prompt string with crate context
        """
        base = base_prompt_path.read_text(encoding="utf-8")
        trailer = f"\ncrate = \"{ctx['crate_name']}\"\nyear = {ctx['year']}\n"

        return base + trailer
