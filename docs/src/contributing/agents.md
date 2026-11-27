# Add an agent

Agents are registered in `AGENTS` in `mizan-cli/src/mizan_cli/inspect_benchmark/agents.py`. A registered agent works with `mizan evaluate run --agent <name>` and `run_eval.py`.

## Steps

1. Import the agent factory. inspect-swe provides `gemini_cli`, `opencode`, `mini_swe_agent`, `kimi_code` and others.
2. Add an `AgentSpec` to `AGENTS`:

   ```python
   from inspect_swe import gemini_cli

   AGENTS = {
       ...
       "gemini_cli": AgentSpec(
           description="Gemini CLI",
           factory=gemini_cli,
           kwargs={"system_prompt": ASSISTANT_PROMPT},
           models=("google/",),
           default_models=("google/gemini-3.1-pro-preview",),
       ),
   }
   ```

3. Check it is listed: `mizan evaluate agents`.

## `AgentSpec` fields

| Field | Description |
| --- | --- |
| `description` | Shown by `mizan evaluate agents` |
| `factory` | Function that returns an inspect-ai `Agent` |
| `kwargs` | Arguments passed to `factory`. Pass `system_prompt=ASSISTANT_PROMPT` so every agent gets the same task instructions. Leave anything else out to use the factory's default |
| `models` | Model prefixes the agent supports, e.g. `("google/",)`. Other models are skipped. `None` means any |
| `default_models` | Models to run when `--model` is not given. `None` means the four paper models |

Settings not in `kwargs` can still be set per run with `--agent-arg KEY=VALUE`.
