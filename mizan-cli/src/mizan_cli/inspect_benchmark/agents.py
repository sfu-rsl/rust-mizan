# from inspect_swe import claude_code
# from .solver import react_agent 

# NO_WEB = ["WebSearch", "WebFetch"]

# AGENTS = {
#     "react": (lambda: react_agent(), None), 
#     "claude_code": (lambda: claude_code(disallowed_tools=NO_WEB), ("anthropic/"))
# }

# def get_agent(name): 
#     return AGENTS[name][0]() 

# def supports(name, model): 
#     prefixes = AGENTS[name][1]
#     return prefixes is None or str(model).startswith(prefixes)

from dataclasses import dataclass, field
from typing import Any, Callable 

from inspect_ai.agent import Agent 
from inspect_swe import claude_code

from .solver import react_agent 

AgentFactory = Callable[[], Agent]

CLAUDE_CODE_SYSTEM_PROMPT = """You are a security auditor analyzing Rust code for vulnerabilities.
Focus on memory safety issues and follow the task instructions carefully."""

@dataclass(frozen=True)
class AgentSpec: 
    description: str 
    factory: Callable[..., Agent]
    kwargs: dict[str, Any] = field(default_factory=dict)
    models: tuple[str, ...] | None = None 

    def build(self) -> Agent: 
        return self.factory(**self.kwargs)

    def supports(self, model) -> bool: 
        return self.models is None or str(model).startswith(self.models)

AGENTS: dict[str, AgentSpec] = {
    "react": AgentSpec(
        description="inspect-ai ReAct loop agent. Paper default", factory=react_agent), 
    "claude_code": AgentSpec(
        description="Claude Code CLI", 
        factory=claude_code, 
        kwargs={
            "system_prompt": (CLAUDE_CODE_SYSTEM_PROMPT), 
            "disallowed_tools": ["WebSearch", "WebFetch"],
        },
        models=("anthropic/",),
    )
}


def get_spec(name: str) -> AgentSpec: 
    if name not in AGENTS: 
       raise ValueError(f"Unknown agent {name!r}. Choose from: {', '.join(AGENTS)}")
    return AGENTS[name]

def get_agent(name: str) -> Agent: 
    return get_spec(name).build()

def supports(name: str, model) -> bool: 
    return get_spec(name).supports(model)