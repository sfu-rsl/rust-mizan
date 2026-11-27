from dataclasses import dataclass, field
from typing import Any, Callable 

from inspect_ai.agent import Agent 
from inspect_swe import claude_code, codex_cli

from .solver import react_agent 
from .prompts import ASSISTANT_PROMPT

AgentFactory = Callable[[], Agent]

@dataclass(frozen=True)
class AgentSpec: 
    description: str 
    factory: Callable[..., Agent]
    kwargs: dict[str, Any] = field(default_factory=dict)
    models: tuple[str, ...] | None = None 
    # Models to run when none are given. None means DEFAULT_MODELS in run.py.
    default_models: tuple[str, ...] | None = None

    def build(self, **overrides: Any) -> Agent: 
        """Build the agent. `overrides` replace or add to the default kwargs."""
        return self.factory(**{**self.kwargs, **overrides})

    def supports(self, model) -> bool: 
        return self.models is None or str(model).startswith(self.models)

AGENTS: dict[str, AgentSpec] = {
    "react": AgentSpec(
        description="inspect-ai ReAct loop agent. Paper default", factory=react_agent), 
    "claude_code": AgentSpec(
        description="Claude Code CLI", 
        factory=claude_code, 
        kwargs={
            "system_prompt": ASSISTANT_PROMPT, 
        },
        models=("anthropic/",),
    ),
    "codex_cli": AgentSpec(
        description="Codex CLI", 
        factory=codex_cli, 
        kwargs={
            "system_prompt": ASSISTANT_PROMPT, 
        },
        models=("openai/",),
        default_models=("openai/gpt-6.1-sol",),
    )
}


def get_spec(name: str) -> AgentSpec: 
    if name not in AGENTS: 
       raise ValueError(f"Unknown agent {name!r}. Choose from: {', '.join(AGENTS)}")
    return AGENTS[name]

def get_agent(name: str, **overrides: Any) -> Agent: 
    return get_spec(name).build(**overrides)

def supports(name: str, model) -> bool: 
    return get_spec(name).supports(model)