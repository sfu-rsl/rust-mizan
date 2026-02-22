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
