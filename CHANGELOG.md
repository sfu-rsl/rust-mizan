# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added - 2026-11-26

- `mizan evaluate run` runs the benchmark from the CLI. Choose the agent with `--agent`,
  override its settings with `--agent-arg KEY=VALUE`, and set models, samples, limits and
  `--reasoning-effort` (default `low`). See [The mizan CLI](docs/src/cli.md#evaluate-run).
- `mizan evaluate agents` lists the available agents, their supported models and default settings.
- Agent registry (`AGENTS` in `inspect_benchmark/agents.py`) with `react` (paper default),
  `claude_code` and `codex_cli`. All agents get the Mizan system prompt; other settings use the
  agent's own defaults. `codex_cli` defaults to `openai/gpt-6.1-sol`.
  See [Add an agent](docs/src/contributing/agents.md).
- CI checks now also run on pushes and PRs to `dev`.

### Removed - 2026-11-26

- `run_eval_claude_code.py`. Use `mizan evaluate run --agent claude_code` or `run_eval.py`.

### Fixed - 2026-11-26

- The task prompt asked for `result.json` in one place; it now says `results.json` throughout.
- `run_eval.py` failed with an `ImportError` on `from inspect_ai import get_model`.

### Changes - 2026-11-26

- Upgraded `inspect-ai` 0.3.205 to 0.3.291; this changes its built-in ReAct system prompt.
  The default assistant prompt now tells the model to prioritize parallel tool calls,
  so every model run with the default `react_agent()` gets a different system message.
  Results from before and after this upgrade are not directly comparable.
  See [inspect-ai 0.3.291 upgrade](docs/src/changelog/inspect-ai-0.3.291.md).
- `inspect-ai` 0.3.291 also renames the `bash()` tool argument from `cmd` to `command`
  and runs independent tool calls from one message concurrently.
- Upgraded `inspect-swe` 0.2.32 to 0.3.2 (provides the Claude Code and Codex CLI agents).
- Upgraded model-provider SDKs: `openai` 2.32.0 to 3.56.0, `anthropic` 0.80.0 to 2.11.0,
  `google-genai` 1.73.1 to 3.6.1.
- Upgraded `pydantic` 2.12.5 to 2.13.5.
- `mizan-cli` now requires Python `>=3.11,<3.14` (was `^3.10`).
