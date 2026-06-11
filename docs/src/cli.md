# The mizan CLI

`mizan` is the Python CLI for working with the dataset. It selects samples, applies mutations, and prepares datasets for evaluation.

All commands run from a directory containing `mizan.json` (the dataset root).

## Installation

```bash
cd mizan-cli
poetry install
export PATH="$(poetry env info --path)/bin:$PATH"
```

## Configuration

Optional configuration lives at `~/.config/mizan/config.json`:

| Option | Description | Default |
| --- | --- | --- |
| `log_level` | `DEBUG`, `INFO`, `WARNING`, or `ERROR` | `INFO` |
| `log_file` | Path to a log file | none |

## `checkout`

Select and export samples from the dataset into an output directory.

```bash
mizan checkout [OPTIONS]
```

| Option | Short | Description | Default |
| --- | --- | --- | --- |
| `--output` | `-o` | Output directory | `./output` |
| `--level` | `-l` | `function`, `file`, `crate`, or `all` | `all` |
| `--vuln-ids` | `-v` | Specific vulnerability IDs (repeatable) | none |
| `--year` | `-y` | Filter by year | none |
| `--cwe-types` | `-c` | Filter by CWE type (repeatable) | none |
| `--include-fixed` | | Include fixed samples too | `false` |

```bash
# All function-level samples
mizan checkout --level function

# Two specific vulnerabilities
mizan checkout -v vuln-0001 -v vuln-0002

# Combine filters
mizan checkout --level function --year 2019 --cwe-types CWE-416 -o ./my-samples
```

`checkout` copies the selected samples and any dependencies they need, writes a workspace `Cargo.toml`, and emits a filtered `mizan.json` into the output directory.

## `mutate`

Apply semantic-preserving mutations to checked-out samples. Run it from inside the `checkout` output directory.

```bash
cd output
mizan mutate [OPTIONS]
```

| Option | Short | Description | Default |
| --- | --- | --- | --- |
| `--mutations` | `-m` | Mutations to apply (repeatable) | `all` |
| `--seed` | `-s` | Random seed for reproducibility | `42` |

```bash
# A single mutation
mizan mutate -m remove-comments

# Several, applied in order
mizan mutate -m format-compact -m benign-comments
```

The full list of mutations, their categories, and ordering caveats are on the [Mutations](mutations/index.md) page. `mutate` updates `mizan.json` with corrected line numbers and writes a `mizan_mutations.json` log.

## `evaluate prepare-dataset`

Convert checked-out samples into a parquet file for evaluation. Run it from the output directory.

```bash
mizan evaluate prepare-dataset [OPTIONS]
```

| Option | Short | Description | Default |
| --- | --- | --- | --- |
| `--output` | `-o` | Output parquet file | `dataset.parquet` |
| `--tag` | `-t` | Optional tag to identify the dataset | none |

The parquet bundles each sample's files and ground truth, plus dataset metadata (rust version, tag, applied mutations). It is the only artifact the evaluation harness consumes. See [Evaluation](evaluation.md).

## Running evaluations

Use the [`run_eval.py`](https://github.com/sfu-rsl/rust-mizan/blob/main/mizan-cli/run_eval.py) script for full control over models, limits, and the agent scaffold:

```bash
cd mizan-cli
# Edit run_eval.py: dataset path, models, message/time limits
python run_eval.py
```

The script exposes the full evaluation configuration, including the agent, which can be replaced with a custom implementation. See [Evaluation](evaluation.md).
