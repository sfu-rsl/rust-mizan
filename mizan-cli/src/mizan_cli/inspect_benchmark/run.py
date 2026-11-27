from pathlib import Path
from typing import Any, Sequence

from inspect_ai import eval
from inspect_ai.model import Model, get_model

from .agents import get_agent, get_spec, supports
from .rustmizan import rustmizan

# Bundled datasets at the repository root (mizan-cli is installed in editable mode).
DATASET_DIR = Path(__file__).resolve().parents[4] / "datasets"

# Dataset used when none is given, same as run_eval.py.
DEFAULT_DATASET_PATHS: list[Path] = [DATASET_DIR / "mizan-vanilla.parquet"]

# Models from the reported evaluations.
DEFAULT_MODELS: list[str] = [
    "anthropic/claude-sonnet-4-6",
    "openai/gpt-5.4",
    "google/gemini-3.1-pro-preview",
    "openrouter/qwen/qwen3.6-plus",
]

# Per-model args applied whenever that model is selected.
# Qwen runs with reasoning disabled, as in the reported evaluations.
DEFAULT_MODEL_ARGS: dict[str, dict[str, Any]] = {
    "openrouter/qwen/qwen3.6-plus": {"reasoning_enabled": False},
}


def resolve_models(names: Sequence[str]) -> list[str | Model]:
    """Turn model names into what `eval()` accepts, applying DEFAULT_MODEL_ARGS."""
    return [
        get_model(name, **DEFAULT_MODEL_ARGS[name]) if name in DEFAULT_MODEL_ARGS else name
        for name in names
    ]


def run_evaluation(
    dataset_paths: Sequence[Path | str] | None = None,
    agent: str = "react",
    agent_args: dict[str, Any] | None = None,
    models: Sequence[str] | None = None,
    sample_ids: list[str] | None = None,
    limit: int | None = None,
    message_limit: int | None = 100,
    time_limit: int | None = 3600,
    fail_on_error: bool = False,
    max_sandboxes: int = 10,
    reasoning_effort: str | None = "low",
):
    """Run the benchmark with the named agent on every model that agent supports."""
    spec = get_spec(agent)  # fails early on an unknown agent name
    if not models:
        models = spec.default_models or DEFAULT_MODELS
    if not dataset_paths:
        dataset_paths = DEFAULT_DATASET_PATHS
    missing = [str(p) for p in dataset_paths if not Path(p).is_file()]
    if missing:
        raise ValueError(f"Dataset not found: {', '.join(missing)}")
    selected = [m for m in models if supports(agent, m)]
    skipped = [m for m in models if not supports(agent, m)]
    if skipped:
        print(f"Skipping {agent} for models: {', '.join(skipped)}")
    if not selected:
        raise ValueError(f"None of the selected models are supported by agent {agent!r}")

    try:
        solver = get_agent(agent, **(agent_args or {}))
    except TypeError as e:
        # Usually an agent setting the agent's factory does not accept.
        raise ValueError(f"Invalid settings for agent {agent!r}: {e}") from e

    tasks = rustmizan(
        dataset_paths=list(dataset_paths),
        sample_ids=sample_ids,
        agent=solver,
    )

    return eval(
        tasks=tasks,
        model=resolve_models(selected),
        limit=limit,
        fail_on_error=fail_on_error,
        message_limit=message_limit,
        time_limit=time_limit,
        # Keeps concurrent compose networks under Docker Desktop's default
        # pool (~15 /16 slots). Also serves as the primary memory lever —
        # raise/lower to trade throughput for RAM.
        max_sandboxes=max_sandboxes,
        # Low effort, with Qwen reasoning disabled in DEFAULT_MODEL_ARGS.
        # See https://inspect.aisi.org.uk/reasoning.html#google-gemini
        reasoning_effort=reasoning_effort,
    )
