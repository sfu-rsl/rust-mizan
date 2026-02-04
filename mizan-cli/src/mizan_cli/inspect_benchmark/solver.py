from inspect_ai.agent import Agent, agent, react
from inspect_ai.tool import bash


@agent
def react_agent() -> Agent:
    return react(
        description="Security auditor analyzing Rust code for vulnerabilities",
        prompt="You are a security auditor. Use the bash tool to explore and analyze the Rust codebase for memory safety vulnerabilities.",
        tools=[bash()],
        submit=False,
    )
