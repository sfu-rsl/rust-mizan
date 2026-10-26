# Getting Started

Setup and a complete run, from building the dataset to viewing evaluation results.

## Requirements

- Rust 1.84.1 and nightly-2026-04-20 for `mizan-mut`.
- Python 3.10 to 3.13. 
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
# Only needed if your default python is 3.14
poetry env use python3.13
poetry install --only main
cd ..
```

The `mizan`, `python` and `inspect` commands on this page need the Poetry
environment. Either activate it once per terminal:

```bash
eval $(poetry -P mizan-cli env activate)
```

Or put `poetry -P <path-to-mizan-cli> run` in front of each command. The path
depends on where you run it from:

```bash
# From the repository root
poetry -P mizan-cli run mizan checkout --help

# From output/
poetry -P ../mizan-cli run python ../mizan-cli/run_eval.py
```

All `mizan` commands run from a directory that contains `mizan.json` (the dataset root).

## Set up API keys

The following API keys are supported. Export the key for each provider
you use:

| Provider   | Environment variable |
| ---------- | -------------------- |
| Anthropic  | `ANTHROPIC_API_KEY`  |
| OpenAI     | `OPENAI_API_KEY`     |
| Google     | `GOOGLE_API_KEY`     |
| OpenRouter | `OPENROUTER_API_KEY` |

The default `MODELS` in `mizan-cli/run_eval.py` is the paper's four models
and needs all four keys. Remove any model whose provider key you haven't set.


## End-to-end run
This example prepares a small unmodified subset. It assumes the Poetry
environment is active (see [Install the CLI](#install-the-cli)).

```bash
# 1. Select samples into an output directory
mizan checkout -v vuln-0001 -v vuln-0002 -l function -o output
cd output

# 2. Convert to a parquet dataset for evaluation
mizan evaluate prepare-dataset --tag vanilla -o ../datasets/mizan-vanilla.parquet

# 3. Run the evaluation 
python ../mizan-cli/run_eval.py

# 4. View results
inspect view
```

Each step is documented in detail:

- [The mizan CLI](cli.md) covers `checkout`, `mutate`, and `evaluate prepare-dataset`.
- [Mutations](mutations/index.md) lists every mutation and explains ground-truth tracking.
- [Evaluation](evaluation.md) describes the task, the metrics, and how to configure a run.

For the paper's four-split Docker recipe, see the repository README.
