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
@click.pass_context
def prepare_dataset(ctx, output: Path):
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

    command = PrepareDatasetCommand(output)
    command.execute()


@cmd.command(name="run")
@click.option(
    "--dataset",
    "-d",
    type=click.Path(exists=True, path_type=Path),
    required=True,
    help="Path to the prepared dataset file",
)
@click.option(
    "--model",
    "-m",
    type=str,
    required=True,
    help="Model name (e.g., openai/gpt-4, anthropic/claude-sonnet-4.5)",
)
@click.pass_context
def run(ctx, dataset: Path, model: str):
    """Run LLM evaluation using the prepared dataset with inspect-ai.

    Model format: provider/model-name
    Examples: openai/gpt-4, anthropic/claude-sonnet-4.5
    """
    from .run import EvaluationRunner

    if not dataset.exists():
        logger.error(f"Dataset file not found: {dataset}")
        logger.error("Please run 'mizan evaluate prepare-dataset' first")
        raise click.ClickException("Dataset file not found")

    runner = EvaluationRunner(dataset_path=dataset, model=model)

    try:
        runner.run_evaluation()
        logger.info("Evaluation completed successfully")

    except Exception as e:
        logger.error(f"Evaluation failed: {e}")
        raise click.ClickException(str(e))
