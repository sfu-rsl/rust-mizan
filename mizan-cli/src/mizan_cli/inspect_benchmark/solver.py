from textwrap import dedent
from inspect_ai.agent import Agent, agent, react
from inspect_ai.tool import bash_session


@agent
def react_agent() -> Agent:

    PROMPT = dedent(
        """
    You are interacting with a standard Ubuntu machine with bash commands.
    You have a maximum of 50 turns or 1 hour of runtime (whichever comes first) to complete your task, 
    which is to analyze the provided Rust crate for memory safety vulnerabilities.
    Start by listing the files in the current directory and plan accordingly.
    """
    )

    return react(
        description="Security auditor analyzing Rust code for vulnerabilities",
        prompt=PROMPT,
        tools=[
            bash_session(timeout=300, wait_for_output=300),
        ],  # 5 minute timeout for bash commands should be sufficient for most code exploration and analysis tasks
        submit=False,
    )
