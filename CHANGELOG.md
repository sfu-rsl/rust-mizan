# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changes - 2026-11-26

- Upgraded `inspect-ai` 0.3.205 to 0.3.291; this changes its built-in ReAct system prompt.
  The default assistant prompt now tells the model to prioritize parallel tool calls,
  so every model run with the default `react_agent()` gets a different system message.
  Results from before and after this upgrade are not directly comparable.
  See [inspect-ai 0.3.291 upgrade](docs/src/changelog/inspect-ai-0.3.291.md).
- `inspect-ai` 0.3.291 also renames the `bash()` tool argument from `cmd` to `command`
  and runs independent tool calls from one message concurrently.
- Upgraded `inspect-swe` 0.2.32 to 0.3.2 (used by `run_eval_claude_code.py`).
- Upgraded model-provider SDKs: `openai` 2.32.0 to 3.56.0, `anthropic` 0.80.0 to 2.11.0,
  `google-genai` 1.73.1 to 3.6.1.
- Upgraded `pydantic` 2.12.5 to 2.13.5.
- `mizan-cli` now requires Python `>=3.11,<3.14` (was `^3.10`).
