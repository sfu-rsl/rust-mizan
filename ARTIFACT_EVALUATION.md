# Artifact Evaluation

This document is the step-by-step reproduction guide for the paper _RustMizan: A Compilable, Contamination-Aware Benchmarking Framework for Rust Vulnerability Analysis_ (under review at NeurIPS 2026 Evaluations & Datasets Track).

The general project README is at [README.md](README.md). This file is reviewer-facing and lists the exact commands needed to verify each scientific claim.

## What this is

The repository contains:

- `mizan-cli/` — Python evaluation pipeline that produced the paper's RQ1–RQ4 results, built on the `inspect-ai` agentic harness.
- `mizan-mut/` — Rust mutation tool that applies semantic-preserving transformations to a Cargo workspace.
- `samples/` — 42 vuln-XXXX directories, each containing crate-, file-, and function-level reductions of one CVE (plus shared dependencies in `samples/deps/`).
- `mizan.json` — vulnerability metadata: CVE / CWE / file → vulnerable functions / file → vulnerable lines, peer-reviewed during dataset construction.
- `docker/Dockerfile.datasets` — recipe for materialising all four dataset splits (vanilla + benign + malignant + rust-specific).
- `program_analysis_tools/` — Kani / RAPx demonstrations referenced in Appendix I.
- `docs/mutations_specification.md` — full mutation operator catalogue.

The published vanilla dataset is on Hugging Face:

> https://huggingface.co/datasets/rustmizan-org/mizan-vanilla

The mutated splits used in the paper are not separately hosted; they are regenerated locally via `docker/Dockerfile.datasets` (see §2 below).

## Prerequisites

- Docker.
- Rust 1.84.1 (`rustup install 1.84.1`); also nightly for `mizan-mut` (`rustup install nightly`).
- Python 3.10+ with [Poetry](https://python-poetry.org/).
- `huggingface-cli` (no auth required for public download).
- API keys for at least one supported model provider (OpenAI / Anthropic / Google / OpenRouter), exported in your shell environment, if you intend to run the agentic eval against frontier models.

## 1. Reproduce the paper's main results (`run_eval.py` against vanilla)

This reproduces the RQ1–RQ2 numbers using the vanilla split.

### Download the vanilla parquet

```bash
mkdir -p mizan-cli/datasets
huggingface-cli download rustmizan-org/mizan-vanilla mizan-vanilla.parquet \
    --local-dir mizan-cli/datasets --repo-type dataset
```

### Install the eval CLI

```bash
cd mizan-cli
poetry install
```

### Run the eval

```bash
poetry run python run_eval.py
```

This drives `inspect-ai` against the four frontier models configured in `MODELS` at the top of `run_eval.py`, with each task running in a Docker sandbox built from `rust:1.84.1`. By default the eval runs against `./datasets/mizan-vanilla.parquet` only. Uncomment the additional entries in `DATASET_PATHS` to also run against the mutated splits (after generating them — see §2).

### View results

```bash
poetry run inspect view
```

## 2. Regenerate the four dataset splits

Reproducing RQ3 / RQ4 (the contamination and adversarial-robustness experiments) requires the mutated splits. Regenerate them all in one Docker build:

```bash
docker build -f docker/Dockerfile.datasets -t mizan-datasets .
docker run -v $(pwd)/mizan-cli/datasets:/app/datasets mizan-datasets
```

This image:

1. Installs Rust 1.84.1 stable + nightly, plus Poetry.
2. Builds `mizan-mut` (Rust mutation tool).
3. Installs `mizan-cli`.
4. Runs `mizan checkout --include-fixed` to materialise the working set from `samples/` and `mizan.json`.
5. Applies each mutation operator family (benign / malignant / rust-specific) via `mizan mutate -m <operator>`.
6. Writes four parquet files to `/app/datasets/`, mounted to `mizan-cli/datasets/` on the host:
   - `mizan-vanilla.parquet`
   - `mizan-benign.parquet`
   - `mizan-malignant.parquet`
   - `mizan-rust-specific.parquet`

Then re-run `run_eval.py` with all four entries in `DATASET_PATHS` uncommented.

## 3. Verify the mutation framework

### Read the source

`mizan-mut/src/main.rs` is the entry point. The mutation operators live in `mizan-cli/src/mizan_cli/commands/mutate/mutations/`. The framework's marker / content / rename tracking, which keeps the ground-truth annotations valid through every mutation, is in `mizan-cli/src/mizan_cli/commands/mutate/`.

### Run the framework on a single sample

```bash
cd mizan-cli
poetry run mizan checkout -v vuln-0001 -l function -o /tmp/out
cd /tmp/out
poetry run mizan mutate -m benign-rename-fn
```

The output shows the diff before / after the mutation and the per-variant log of which operators applied successfully.

## 4. Verify the program-analysis demos

See `program_analysis_tools/README.md` for Kani / RAPx run instructions.

## License

Apache-2.0. See [LICENSE](LICENSE).
