from pathlib import Path
from inspect_ai import eval
from mizan_cli.utils.logging import get_logger
from mizan_cli.inspect_benchmark import rustmizan

logger = get_logger()


class EvaluationRunner:
    def __init__(self, dataset_path: Path, model: str):
        self.dataset_path = dataset_path
        self.model = model

    def run_evaluation(self):
        task = rustmizan(dataset_path=self.dataset_path)
        result = eval(
            task,
            model=self.model,
        )
        return result
