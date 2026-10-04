from pathlib import Path
from inspect_ai import Task
from inspect_ai.agent import Agent

from .dataset import load_dataset
from .solver import react_agent
from .scorer import rustmizan_scorer


def rustmizan(
    dataset_paths: Path | str | list[Path | str] | None = None,
    sample_ids: str | list[str] | None = None,
    agent: Agent | None = None,
    *,
    dataset_path: Path | str | None = None,
) -> list[Task]:
    """Create one named task per dataset, as in the reported evaluations."""
    # Keep the earlier single-dataset keyword available to existing callers.
    if dataset_path is not None:
        if dataset_paths is not None:
            raise ValueError("Specify dataset_paths or dataset_path, not both")
        dataset_paths = dataset_path
    if dataset_paths is None:
        raise ValueError("At least one dataset path is required")
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
