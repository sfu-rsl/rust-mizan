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

> Note: no `-m` applies every mutation in the registry 

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

## `evaluate run`

Run the benchmark on a prepared dataset with a chosen agent. Requires Docker and the API keys for the selected models.

```bash
mizan evaluate run [OPTIONS]
```

| Option | Short | Description | Default |
| --- | --- | --- | --- |
| `--dataset` | `-d` | Parquet file, e.g. from `prepare-dataset` (repeatable) | `datasets/mizan-vanilla.parquet` |
| `--agent` | `-a` | Agent to evaluate: `react`, `claude_code`, or `codex_cli` | `react` |
| `--agent-arg` | | Override an agent setting as `KEY=VALUE` (repeatable) | none |
| `--model` | `-m` | Model to evaluate (repeatable) | the agent's default models, else the four paper models |
| `--sample-id` | `-s` | Only run these sample IDs (repeatable) | all |
| `--limit` | | Max samples per dataset | none |
| `--message-limit` | | Max messages per sample, `0` for none | `100` |
| `--time-limit` | | Seconds per sample, `0` for none | `3600` |
| `--max-sandboxes` | | Max concurrent Docker sandboxes | `10` |
| `--reasoning-effort` | | Reasoning effort passed to the model | `low` |
| `--fail-on-error` | | Stop on the first sample error | `false` |

| Agent | Supported models | Default models (no `--model`) | Settings Mizan sets |
| --- | --- | --- | --- |
| `react` | any | the four paper models | none (takes no `--agent-arg`) |
| `claude_code` | `anthropic/` | `anthropic/claude-sonnet-4-6` | `system_prompt` |
| `codex_cli` | `openai/` | `openai/gpt-6.1-sol` | `system_prompt` |

- Unsupported models are skipped with a message.
- Settings not listed above use the agent's inspect-swe defaults unless you pass them with `--agent-arg`.
- `--agent-arg` values are parsed as JSON when possible, so lists and numbers work.
- To change these defaults, edit the agent's entry in `AGENTS` (see [Add an agent](contributing/agents.md)).

> **Note:** `--agent-arg` keys are the agent factory's own arguments, and Mizan passes them through unchanged. For the available arguments and their value formats, see the [inspect-swe docs](https://meridianlabs-ai.github.io/inspect_swe/) for `claude_code` and `codex_cli`. The `react` agent takes no `--agent-arg` settings.

```bash
# Same as run_eval.py's defaults: ReAct agent, all four models, bundled vanilla dataset
mizan evaluate run

# A dataset you prepared
mizan evaluate run -d output/dataset.parquet

# Claude Code on two samples
mizan evaluate run -a claude_code -m anthropic/claude-sonnet-4-6 -s vuln-0001 -s vuln-0002

# Codex CLI with web search turned off
mizan evaluate run -a codex_cli --agent-arg web_search=disabled

# Claude Code without web tools, at medium reasoning effort
mizan evaluate run -a claude_code --agent-arg disallowed_tools='["WebSearch","WebFetch"]' --reasoning-effort medium
```

## `evaluate agents`

List the available agents, the models each supports, and their default settings (the values `--agent-arg` overrides).

```bash
mizan evaluate agents
```

## Running evaluations from a script

[`run_eval.py`](https://github.com/sfu-rsl/rust-mizan/blob/master/mizan-cli/run_eval.py) runs the same code as `evaluate run`, with its settings as constants at the top of the file:

```bash
cd mizan-cli
# Edit run_eval.py: AGENT, AGENT_ARGS, MODELS, DATASET_PATHS, limits
poetry run python run_eval.py
```

To add an agent, see [Add an agent](contributing/agents.md). See [Evaluation](evaluation.md).
