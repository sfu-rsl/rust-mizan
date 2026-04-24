from pathlib import Path
from inspect_ai import Task
from inspect_ai.agent import Agent

from .dataset import load_dataset
from .solver import react_agent
from .scorer import rustmizan_scorer


def rustmizan(
    dataset_paths: Path | str | list[Path | str],
    sample_ids: str | list[str] | None = None,
    agent: Agent | None = None,
) -> list[Task]:
    if isinstance(dataset_paths, (str, Path)):
        dataset_paths = [dataset_paths]

    tasks: list[Task] = []
    for path in dataset_paths:
        dataset_path = Path(path)
        dataset, dataset_metadata = load_dataset(dataset_path, sample_ids=sample_ids)
        tasks.append(
            Task(
                name=dataset_path.stem,
                dataset=dataset,
                solver=agent or react_agent(),
                scorer=rustmizan_scorer(),
                metadata=dataset_metadata,
            )
        )
    return tasks
