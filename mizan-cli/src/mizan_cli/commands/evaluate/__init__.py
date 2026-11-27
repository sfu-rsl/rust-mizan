import json
import click
from pathlib import Path
from mizan_cli.utils.logging import get_logger

logger = get_logger()


@click.group(name="evaluate")
@click.pass_context
def cmd(ctx):
    """Run LLM evaluation on code samples."""
    pass


@cmd.command(name="prepare-dataset")
@click.option(
    "--output",
    "-o",
    type=click.Path(path_type=Path),
    default=Path("dataset.parquet"),
    help="Output dataset file path",
)
@click.option(
    "--tag",
    "-t",
    type=str,
    default=None,
    help="Optional tag to identify this dataset",
)
@click.pass_context
def prepare_dataset(ctx, output: Path, tag: str | None):
    """Prepare dataset for evaluation.

    Reads mizan.json and code samples to create a parquet dataset for evaluation.
    """
    from .prepare_dataset import PrepareDatasetCommand

    if not Path("mizan.json").exists():
        logger.error("mizan.json not found in current directory")
        logger.error(
            "Please run this command from the output directory created by 'mizan checkout'"
        )
        raise click.ClickException("mizan.json not found")

    command = PrepareDatasetCommand(output, tag=tag)
    command.execute()


def _parse_agent_args(pairs: tuple[str, ...]) -> dict:
    """Parse KEY=VALUE pairs. Values are read as JSON when possible, else as strings."""
    args = {}
    for pair in pairs:
        key, sep, value = pair.partition("=")
        if not sep or not key:
            raise click.BadParameter(f"expected KEY=VALUE, got {pair!r}", param_hint="--agent-arg")
        try:
            args[key] = json.loads(value)
        except json.JSONDecodeError:
            args[key] = value
    return args


@cmd.command(name="agents")
def list_agents():
    """List the agents that `evaluate run` can use."""
    from mizan_cli.inspect_benchmark.agents import AGENTS

    for name, spec in AGENTS.items():
        models = ", ".join(spec.models) if spec.models else "any"
        click.echo(f"{name:<12} {spec.description} (models: {models})")
        if spec.default_models:
            click.echo(f"{'':<12}   default models: {', '.join(spec.default_models)}")
        for key, value in spec.kwargs.items():
            shown = repr(value) if len(repr(value)) <= 60 else repr(value)[:57] + "..."
            click.echo(f"{'':<12}   {key} = {shown}")


@cmd.command(name="run")
@click.option(
    "--dataset",
    "-d",
    "datasets",
    type=click.Path(exists=True, dir_okay=False, path_type=Path),
    multiple=True,
    help="Parquet dataset, e.g. from `prepare-dataset` (repeatable). "
    "Defaults to the bundled datasets/mizan-vanilla.parquet, like run_eval.py",
)
@click.option(
    "--agent",
    "-a",
    default="react",
    show_default=True,
    help="Agent to evaluate. See `mizan evaluate agents`",
)
@click.option(
    "--agent-arg",
    "agent_args",
    multiple=True,
    metavar="KEY=VALUE",
    help="Override an agent setting (repeatable). VALUE is parsed as JSON if it can be, "
    'e.g. --agent-arg disallowed_tools=\'["WebSearch"]\'',
)
@click.option(
    "--model",
    "-m",
    "models",
    multiple=True,
    help="Model to evaluate (repeatable). Defaults to the agent's default models, "
    "or the paper's four models; models the agent does not support are skipped",
)
@click.option("--sample-id", "-s", "sample_ids", multiple=True, help="Only run these sample IDs (repeatable)")
@click.option("--limit", type=int, default=None, help="Max samples per dataset")
@click.option("--message-limit", type=int, default=100, show_default=True, help="Max messages per sample, 0 for none")
@click.option("--time-limit", type=int, default=3600, show_default=True, help="Seconds per sample, 0 for none")
@click.option("--max-sandboxes", type=int, default=10, show_default=True, help="Max concurrent Docker sandboxes")
@click.option("--reasoning-effort", default="low", show_default=True, help="Reasoning effort passed to the model")
@click.option("--fail-on-error", is_flag=True, help="Stop the run on the first sample error")
def run(
    datasets: tuple[Path, ...],
    agent: str,
    agent_args: tuple[str, ...],
    models: tuple[str, ...],
    sample_ids: tuple[str, ...],
    limit: int | None,
    message_limit: int,
    time_limit: int,
    max_sandboxes: int,
    reasoning_effort: str,
    fail_on_error: bool,
):
    """Run the benchmark with a chosen agent.

    \b
    Examples:
      mizan evaluate run --agent claude_code -m anthropic/claude-sonnet-4-6
      mizan evaluate run -a codex_cli --agent-arg web_search=disabled --limit 5
    """
    from mizan_cli.inspect_benchmark.run import run_evaluation

    try:
        run_evaluation(
            dataset_paths=list(datasets) or None,
            agent=agent,
            agent_args=_parse_agent_args(agent_args),
            models=list(models) or None,
            sample_ids=list(sample_ids) or None,
            limit=limit,
            message_limit=message_limit or None,
            time_limit=time_limit or None,
            fail_on_error=fail_on_error,
            max_sandboxes=max_sandboxes,
            reasoning_effort=reasoning_effort,
        )
    except ValueError as e:
        raise click.ClickException(str(e))
