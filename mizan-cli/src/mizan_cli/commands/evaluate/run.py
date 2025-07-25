import json
import os
from pathlib import Path
from typing import List, Dict, Any
from datetime import datetime
from uuid import uuid4

from langsmith import Client, evaluate
from langsmith.schemas import Example

from mizan_cli.utils.logging import get_logger
from mizan_cli.commands.evaluate.utils.evaluators import get_all_evaluators
from mizan_cli.commands.evaluate.target_function import create_target_function

logger = get_logger()


class EvaluationRunner:
    def __init__(self, dataset_path: Path, provider: str, model: str):
        self.dataset_path = dataset_path
        self.provider = provider
        self.model = model
        self.temperature = 0.0  # Fixed temperature for all evaluations
        self.client = Client()

        self.results_dir = Path.cwd() / "evaluation_results"
        self.results_dir.mkdir(exist_ok=True)

        self.experiment_id = str(uuid4())[:8]
        self.experiment_dir = self.results_dir / f"experiment_{self.experiment_id}"
        self.experiment_dir.mkdir(exist_ok=True)

    def run_evaluation(self) -> Dict[str, Any]:
        logger.info(f"Starting evaluation with {self.provider} {self.model}")
        logger.info(f"Dataset: {self.dataset_path}")
        logger.info(f"Results will be saved to: {self.experiment_dir}")

        examples, mutations_metadata = self._load_dataset()
        logger.info(f"Loaded {len(examples)} examples")

        target_function = create_target_function(
            self.provider, self.model, self.temperature, self.experiment_dir
        )

        evaluators = get_all_evaluators()

        experiment_name = f"mizan-{self.provider}-{self.model}-{datetime.now().strftime('%Y%m%d_%H%M%S')}"

        try:
            results = evaluate(
                target_function,
                data=examples,
                evaluators=evaluators,
                experiment_prefix=experiment_name,
                upload_results=False,  # Don't upload to LangSmith for local datasets
                max_concurrency=1,  # Avoid rate limits. Should we make this configurable?
                metadata={
                    "provider": self.provider,
                    "model": self.model,
                    "temperature": self.temperature,
                    "dataset_path": str(self.dataset_path),
                    "mutations_applied": mutations_metadata.get(
                        "mutations_applied", []
                    ),
                    "mutations_metadata": mutations_metadata,
                    "experiment_id": self.experiment_id,
                },
            )

            processed_results = self._process_results(results)
            experiment_metadata = {
                "experiment_id": self.experiment_id,
                "experiment_name": experiment_name,
                "provider": self.provider,
                "model": self.model,
                "temperature": self.temperature,
                "created_at": datetime.now().isoformat(),
                "total_examples": len(examples),
                "mutations_metadata": mutations_metadata,
            }
            self._save_results(processed_results, experiment_metadata)

            logger.info(f"Evaluation completed successfully")
            logger.info(f"Results saved to: {self.experiment_dir}")

            return {
                "experiment_id": self.experiment_id,
                "experiment_dir": str(self.experiment_dir),
                "total_examples": len(examples),
                "results": processed_results,
            }

        except Exception as e:
            logger.error(f"Evaluation failed: {e}")
            raise

    def _load_dataset(self) -> tuple[List[Example], Dict[str, Any]]:
        try:
            with open(self.dataset_path, "r") as f:
                data = json.load(f)

            examples = []
            for item in data.get("examples", []):
                try:
                    example = Example(
                        id=uuid4(),
                        dataset_id=uuid4(),
                        inputs=item.get("inputs", {}),
                        outputs=item.get("outputs", {}),
                        metadata=item.get("metadata", {}),
                        created_at=datetime.now(),
                    )
                    examples.append(example)
                except Exception as e:
                    logger.warning(f"Failed to parse example: {e}")
                    continue

            if not examples:
                raise ValueError("No valid examples found in dataset")

            mutations_metadata = data.get("mutations_metadata", {})
            return examples, mutations_metadata

        except json.JSONDecodeError as e:
            logger.error(f"JSON parsing failed: {e}")
            raise
        except FileNotFoundError:
            logger.error(f"Dataset file not found: {self.dataset_path}")
            raise

    def _process_results(self, results) -> List[Dict[str, Any]]:
        """Process evaluation results into a structured format."""
        processed_results = []

        for result in results:
            try:
                # LangSmith results are always dicts with 'example', 'run', and 'evaluation_results'
                example = result["example"]
                run = result["run"]
                evaluation_results = result["evaluation_results"]["results"]

                scores = {}
                errors = []

                for eval_result in evaluation_results:
                    key = eval_result.key
                    scores[key] = eval_result.score

                    if eval_result.extra and eval_result.extra.get("error"):
                        errors.append(f"{key}: {eval_result.comment}")

                # Check for target function errors in the outputs and add to errors field
                if run.outputs and "errors" in run.outputs and run.outputs["errors"]:
                    errors.append(f"target_function: {run.outputs['errors']}")

                processed_result = {
                    "example_id": example.metadata.get("id"),
                    "vuln_id": example.metadata.get("vuln_id"),
                    "granularity": example.metadata.get("granularity"),
                    "crate_name": example.metadata.get("crate_name"),
                    "is_vulnerable": example.metadata.get("is_vulnerable"),
                    "year": example.metadata.get("year"),
                    "cwe_types": example.metadata.get("cwe_types"),
                    "inputs": example.inputs,
                    "outputs": run.outputs,
                    "reference_outputs": example.outputs,
                    "scores": scores,
                    "errors": errors,
                    "timestamp": datetime.now().isoformat(),
                }

                processed_results.append(processed_result)

            except Exception as e:
                logger.warning(f"Failed to process result: {e}")
                continue

        return processed_results

    def _save_results(self, results: List[Dict[str, Any]], metadata: Dict[str, Any]):
        """Save results and metadata to files."""
        # Save detailed results
        results_file = self.experiment_dir / "results.json"
        with open(results_file, "w") as f:
            json.dump(results, f, indent=2)

        # Save experiment metadata
        metadata_file = self.experiment_dir / "metadata.json"
        with open(metadata_file, "w") as f:
            json.dump(metadata, f, indent=2)

        logger.info(f"Saved results to {results_file}")
        logger.info(f"Saved metadata to {metadata_file}")
