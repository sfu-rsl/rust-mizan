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
    default=Path("dataset.json"),
    help="Output dataset file path",
)
@click.pass_context
def prepare_dataset(ctx, output: Path):
    """Prepare dataset for evaluation.

    Reads mizan.json and code samples to create a dataset for evaluation. This includes
    preparing the prompts and expected outputs for the LLM evaluation.
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
    "--provider",
    "-p",
    type=click.Choice(["openai", "anthropic", "gemini", "deepseek", "local"]),
    required=True,
    help="LLM provider to use",
)
@click.option(
    "--model",
    "-m",
    type=str,
    required=True,
    help="Model name to use",
)
@click.pass_context
def run(ctx, dataset: Path, provider: str, model: str):
    """Run LLM evaluation using the prepared dataset.

    Executes vulnerability detection evaluation using the specified LLM provider and model.
    Results are stored locally in the `evaluation_results` directory.
    """
    from .run import EvaluationRunner

    if not dataset.exists():
        logger.error(f"Dataset file not found: {dataset}")
        logger.error("Please run 'mizan evaluate prepare-dataset' first")
        raise click.ClickException("Dataset file not found")

    runner = EvaluationRunner(dataset_path=dataset, provider=provider, model=model)

    try:
        result = runner.run_evaluation()

        logger.info("Evaluation completed successfully")
        logger.info(f"Experiment ID: {result['experiment_id']}")
        logger.info(f"Results directory: {result['experiment_dir']}")

    except Exception as e:
        logger.error(f"Evaluation failed: {e}")
        raise click.ClickException(str(e))
