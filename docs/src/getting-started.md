# Getting Started

Setup and a complete run, from building the dataset to viewing evaluation results.

## Requirements

- Rust 1.84.1 and nightly-2026-04-20 for `mizan-mut`.
- [Poetry](https://python-poetry.org/) 2.3.4 for the locked Python environment.
- Docker, used by the evaluation harness to sandbox each sample.

## Get the code

Clone the [repository](https://github.com/sfu-rsl/rust-mizan); everything below runs from its root.

```bash
git clone https://github.com/sfu-rsl/rust-mizan.git
cd rust-mizan
```

## Build the mutation tool

Build the mutation tool and add it to your PATH:

```bash
rustup toolchain install 1.84.1 nightly-2026-04-20
cargo +nightly-2026-04-20 build --locked -p mizan-mut
export PATH="$(pwd)/target/debug:$PATH"
```

## Install the CLI

```bash
cd mizan-cli
poetry install --only main

# Run mizan through poetry
poetry run mizan checkout --help

# Or add it to your PATH
export PATH="$(poetry env info --path)/bin:$PATH"
cd ..
```

All `mizan` commands run from a directory that contains `mizan.json` (the dataset root).

## End-to-end run

This example prepares a small unmodified subset.

```bash
# 1. Select samples into an output directory
mizan checkout -v vuln-0001 -v vuln-0002 -l function -o output
cd output

# 2. Convert to a parquet dataset for evaluation
mizan evaluate prepare-dataset --tag vanilla -o mizan-vanilla.parquet

# 3. Set DATASET_PATHS in run_eval.py to this parquet, then run with provider API keys
python ../mizan-cli/run_eval.py

# 4. View results
inspect view
```

Each step is documented in detail:

- [The mizan CLI](cli.md) covers `checkout`, `mutate`, and `evaluate prepare-dataset`.
- [Mutations](mutations/index.md) lists every mutation and explains ground-truth tracking.
- [Evaluation](evaluation.md) describes the task, the metrics, and how to configure a run.

For the paper's four-split Docker recipe, see the repository README.
