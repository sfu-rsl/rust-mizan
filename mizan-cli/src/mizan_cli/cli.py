import click
from pathlib import Path
from typing import Optional
from mizan_cli import __version__
from mizan_cli.commands import checkout, mutate, evaluate
from mizan_cli.utils import init_logger, init_config, get_logger


@click.group()
@click.version_option(__version__, prog_name="mizan-cli")
@click.option(
    "--config",
    type=click.Path(exists=True, path_type=Path),
    help="Path to configuration file",
)
@click.option(
    "--log-level",
    type=click.Choice(["DEBUG", "INFO", "WARNING", "ERROR", "CRITICAL"]),
    default="INFO",
    help="Logging level",
)
@click.option("--log-file", type=click.Path(path_type=Path), help="Path to log file")
@click.pass_context
def main(ctx, config: Optional[Path], log_level: str, log_file: Optional[Path]):
    """Mizan CLI - A tool for interacting with RustMizan dataset"""
    ctx.ensure_object(dict)

    config_manager = init_config(config)
    ctx.obj["config"] = config_manager

    logger = init_logger(
        level=log_level, log_file=log_file or config_manager.config.log_file
    )
    ctx.obj["logger"] = logger


main.add_command(checkout.cmd)
main.add_command(mutate.cmd)
main.add_command(evaluate.cmd)


if __name__ == "__main__":
    main()
