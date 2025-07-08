# RustMizan

RustMizan (**ميزانَ**- meaning "scale" in Arabic) is a benchmark designed to evaluate LLMs for Rust vulnerability detection. The goal is to build a structured dataset of real-world vulnerabilities that provides a framework to test the ability of LLMs to detect vulnerabilities in code samples at different granularities (function, file, module and crate levels).

## Project Structure

All code samples are organized in the `samples/` directory, with each vulnerability having its own folder (e.g., `vuln-0001`, `vuln-0002`, etc.). Each vulnerability folder contains up to 6 code samples.

### Naming Convention

Samples follow the pattern: `sample-XNNNN-level` where:

- X: First digit indicates vulnerability status
  - `0`: Vulnerable code
  - `1`: Fixed code
- NNNN: Four-digit vulnerability ID (e.g., 0001, 0002)
- level: Granularity level (function, file, or crate)

Example:

- `sample-00001-function` - Vulnerable function from vulnerability 0001
- `sample-10001-function` - Fixed function from vulnerability 0001

To add a new vulnerability to the dataset, please follow the instructions in [CONTRIBUTING.md](./CONTRIBUTING.md).

## Setup Instructions

1. Clone the repository
2. Build the project

```sh
cargo +nightly build --workspace
```

> Using nightly toolchain because `mizan-mut` depends on rust-analyzer crates which require nightly features

## Granularity Levels

For each vulnerability, we include up to 6 samples:

- 3 vulnerable and 3 fixed crates: each at the crate, module, file, and function level.
- This allows testing model performance across different code sizes and contexts.

> By file code sample, we don't mean that the whole code sample is a single file. It is the file that contains the vulnerability
> along with all of its dependencies to keep the file itself unmodified. Same for function code sample.

## Tools

### mizan-mut

[`mizan-mut`](./mizan-mut) is a Rust code mutation tool that provides semantic-preserving transformations and symbol renaming. It offers two main commands:

1. mutate: Applies various AST-based mutations like converting for loops to while loops and reordering if-else branches
2. rename: Renames any symbol in Rust codebases using rust-analyzer

> Note: While not fully tested across all edge cases, we have manually verified the implementation by applying all mutations to large crates like `ripgrep`, `tokio`, `clap`, `hyper`, and `pulldown-cmark`, and making sure that their tests still pass.

### mizan-cli

[`mizan-cli`](./mizan-cli) is a Python CLI tool that provides convenient interaction with RustMizan dataset. It allows developers to checkout specific code samples, apply various mutations, and run experiments on subsets of the dataset.
