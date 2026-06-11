# Add a mutation

A mutation must be **semantically preserving**: it changes syntax without changing behavior. The framework handles backup, compilation checks, ground-truth tracking, and rollback, so a new mutation only has to perform the transformation. See the [Mutations](../mutations/index.md) overview for how the pipeline works.

There are two ways to add one, depending on what the transformation needs.

## Option 1: a Python mutation

Most mutations (formatting, comment and block insertion, renames) are implemented in the CLI. They subclass `BaseMutation` and implement a single `apply` method.

The interface, from `mizan-cli/src/mizan_cli/commands/mutate/orchestrator/base.py`:

```python
class BaseMutation(ABC):
    def __init__(self, name: str, seed: int = 42):
        self.name = name
        self.seed = seed
        self.partial_samples: List[str] = []

    @abstractmethod
    def apply(self, base_dir: str) -> bool:
        ...
```

Steps:

1. Add a class under `mizan-cli/src/mizan_cli/commands/mutate/mutations/` that subclasses `BaseMutation` and implements `apply(self, base_dir) -> bool`. Return `True` on success. `base_dir` is the checkout directory (it contains `samples/` and `mizan.json`).
2. Register it in `MUTATION_REGISTRY` in `mutations/__init__.py`, keyed by the identifier users pass to `mizan mutate -m`.
3. If your mutation removes comments or otherwise breaks the line markers, follow the content-based tracking approach used by the AST mutations (see the [Mutations](../mutations/index.md) overview). For most insertions, the default marker tracking is sufficient.

The orchestrator validates that each mutated sample still compiles and that the ground truth is preserved, rolling back any sample that fails. You do not need to handle backup or validation yourself.

## Option 2: an AST mutation in mizan-mut

Structural transformations that need real Rust AST manipulation belong in [`mizan-mut`](../mutations/mizan-mut.md).

Steps:

1. Add the mutation under `mizan-mut/src/mutations/` using `syn` and `quote`, and wire it into the mutation dispatch in `mizan-mut/src/mutate.rs`.
2. Add it to the `MUTATIONS` array in `docker/Dockerfile.mutations-test` so it is covered by the test suite (see [Testing](#testing) below).
3. To expose it through the CLI, add a thin `MizanMutMutation` subclass in `mizan_mut.py` and register it in `MUTATION_REGISTRY` with a `mizan-mut-` prefix, exactly like the existing AST mutations.

## Testing

A mutation should preserve program behavior. The `mizan-mut` repository ships a Docker-based test suite that applies each mutation to real-world crates and checks that their test suites still pass. Use it to test and iterate:

```bash
docker build -f docker/Dockerfile.mutations-test -t mizan-mut-test .
docker run mizan-mut-test
```

Add your mutation to the `MUTATIONS` array in `docker/Dockerfile.mutations-test` so it is included in the run, then iterate until the report is clean. For CLI mutations, the orchestrator also compiles each mutated sample and verifies the ground truth before saving, rolling back anything that fails.
