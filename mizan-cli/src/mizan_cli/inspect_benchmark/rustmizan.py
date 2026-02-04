from pathlib import Path
from inspect_ai import Task, task
from inspect_ai.agent import Agent

from .dataset import load_dataset
from .solver import react_agent
from .scorer import rustmizan_scorer


@task
def rustmizan(
    dataset_path: Path | str,
    sample_ids: str | list[str] | None = None,
    agent: Agent | None = None,
    max_turns: int = 30,
) -> Task:
    dataset_path = Path(dataset_path)
    dataset, dataset_metadata = load_dataset(dataset_path, sample_ids=sample_ids)

    return Task(
        dataset=dataset,
        solver=agent or react_agent(),
        scorer=rustmizan_scorer(),
        max_messages=max_turns,
        metadata=dataset_metadata,
    )
