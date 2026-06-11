# Getting Started

Setup and a complete run, from building the dataset to viewing evaluation results.

## Requirements

- A nightly Rust toolchain. `mizan-mut` depends on `rust-analyzer` crates that need nightly features.
- [Poetry](https://python-poetry.org/) for the Python CLI.
- Docker, used by the evaluation harness to sandbox each sample.

## Build the dataset

All variants are members of one Cargo workspace. Build them with:

```bash
cargo +nightly build --workspace
```

## Install the CLI

```bash
cd mizan-cli
poetry install

# Run mizan through poetry
poetry run mizan checkout --help

# Or add it to your PATH
export PATH="$(poetry env info --path)/bin:$PATH"
```

All `mizan` commands run from a directory that contains `mizan.json` (the dataset root).

## End-to-end run

```bash
# 1. Select samples into an output directory
mizan checkout -v vuln-0001 -v vuln-0002 -l function -o output
cd output

# 2. Apply semantic-preserving mutations (optional)
mizan mutate -m remove-comments

# 3. Convert to a parquet dataset for evaluation
mizan evaluate prepare-dataset --tag comments_removed -o mizan_comments_removed.parquet

# 4. Run the evaluation (edit mizan-cli/run_eval.py with your dataset path and config)
python ../mizan-cli/run_eval.py

# 5. View results
inspect view
```

Each step is documented in detail:

- [The mizan CLI](cli.md) covers `checkout`, `mutate`, and `evaluate prepare-dataset`.
- [Mutations](mutations/index.md) lists every mutation and explains ground-truth tracking.
- [Evaluation](evaluation.md) describes the task, the metrics, and how to configure a run.
