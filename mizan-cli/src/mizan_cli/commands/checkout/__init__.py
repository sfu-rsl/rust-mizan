import click
from pathlib import Path
from typing import List, Optional
from mizan_cli.utils.logging import get_logger
from mizan_cli.commands.checkout.models import MizanDataset
from mizan_cli.commands.checkout.filters import DatasetFilter
from mizan_cli.commands.checkout.checkout import CheckoutManager


logger = get_logger()


@click.command(name="checkout")
@click.option(
    "--output",
    "-o",
    type=click.Path(path_type=Path),
    default=Path("./output"),
    help="Output directory for checked out samples",
)
@click.option(
    "--level",
    "-l",
    type=click.Choice(["function", "file", "crate", "all"]),
    default="all",
    help="Code granularity level to checkout",
)
@click.option(
    "--vuln-ids",
    "-v",
    multiple=True,
    help="Specific vulnerability IDs to checkout (can be specified multiple times)",
)
@click.option("--year", "-y", type=int, help="Filter vulnerabilities by year")
@click.option(
    "--cwe-types",
    "-c",
    multiple=True,
    help="Filter by CWE types (can be specified multiple times)",
)
@click.option(
    "--include-fixed/--only-vulnerable",
    default=False,
    help="Include fixed samples along with vulnerable ones",
)
@click.pass_context
def cmd(
    ctx,
    output: Path,
    level: str,
    vuln_ids: List[str],
    year: Optional[int],
    cwe_types: List[str],
    include_fixed: bool,
):
    """
    Select and export specific code samples from the dataset
    """
    dataset_path = Path.cwd() / "mizan.json"
    if not dataset_path.exists():
        logger.error("Could not find mizan.json in current directory")
        ctx.exit(1)

    try:
        dataset = MizanDataset.from_file(dataset_path)
        logger.info(f"Loaded {len(dataset.vulnerabilities)} vulnerabilities")

        filter_manager = DatasetFilter(dataset)
        filtered_dataset = filter_manager.apply_filters(
            level=level,
            vuln_ids=list(vuln_ids) if vuln_ids else None,
            year=year,
            cwe_types=list(cwe_types) if cwe_types else None,
            only_vulnerable=not include_fixed,
        )

        total_samples = sum(
            len(v.code_samples) for v in filtered_dataset.vulnerabilities
        )
        if total_samples == 0:
            logger.warning("No samples match the specified filters")
            return

        checkout_manager = CheckoutManager(dataset_path, output)
        checkout_manager.checkout(filtered_dataset)

    except Exception as e:
        logger.error(f"Checkout failed: {str(e)}")
        ctx.exit(1)
