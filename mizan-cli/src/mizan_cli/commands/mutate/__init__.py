import click
import os
from typing import List
from mizan_cli.utils.logging import get_logger
from .orchestrator import MutationOrchestrator
from .mutations import MUTATION_REGISTRY


logger = get_logger()


@click.command(name="mutate")
@click.option(
    "--mutations",
    "-m",
    multiple=True,
    type=click.Choice(list(MUTATION_REGISTRY.keys()) + ["all"]),
    default=["all"],
    help="Mutations to apply (can be specified multiple times)",
)
@click.pass_context
def cmd(ctx, mutations: List[str]):
    """Apply semantic-preserving mutations to code samples"""

    for required_file in ["mizan.json", "Cargo.toml"]:
        if not os.path.exists(required_file):
            logger.error(f"No {required_file} found")
            ctx.exit(1)

    selected_mutations = (
        list(MUTATION_REGISTRY.keys()) if "all" in mutations else list(set(mutations))
    )
    logger.info(f"Selected mutations: {', '.join(selected_mutations)}")

    orchestrator = MutationOrchestrator(os.getcwd())
    mutation_instances = [MUTATION_REGISTRY[name]() for name in selected_mutations]

    try:
        for mutation in mutation_instances:
            orchestrator.apply_mutation_sequential(mutation)
        success = True
    finally:
        orchestrator.finalize()

    status = "successfully" if success else "with errors"
    logger.info(f"Mutation process completed {status}")
    logger.info("Check mizan_mutations.json for details about applied mutations")
