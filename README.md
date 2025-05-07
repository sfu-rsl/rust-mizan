# RustMizan

RustMizan (**ميزانَ**- meaning "scale" in Arabic) is a benchmark designed to evaluate LLMs for Rust vulnerability detection. The goal is to build a structured dataset of real-world vulnerabilities that provides a framework to test the ability of LLMs to detect vulnerabilities in code samples at different granularities (function, file, module and crate levels).

## Setup Instructions

1. Clone the repository
2. Build the project

```sh
cargo build --workspace
```

## Evaluation Tasks

Each code sample can be used to evaluate one or more of the following:

1. Vulnerability Existence Detection
2. CWE Type Inference
3. Key Data Objects & Functions Identification:
4. Root Cause Location

All metadata is stored in `mizan.json`.

## Granularity Levels

For each vulnerability, we include up to 6 samples:

- 3 vulnerable and 3 fixed crates: each at the crate, module, file, and function level.
- This allows testing model performance across different code sizes and contexts.

## Evaluation

See the [`evaluation/`](./evaluation) folder for scripts and instructions on running benchmarks using LLMs.

> [!Note]
> Vulnerabilities 0030–0037 depend on the same crate that uses `links = "sqlite3"` in its manifest.  
> Cargo does not allow multiple crates with the same `links` value in a single workspace.  
> When evaluating these vulnerabilities, run each one in isolation, outside the workspace.

To add a new vulnerability to the dataset, please follow the instructions in [CONTRIBUTING.md](./CONTRIBUTING.md).
