from textwrap import dedent
from inspect_ai.agent import Agent, agent, react
from inspect_ai.tool import bash


@agent
def react_agent() -> Agent:

    PROMPT = dedent(
        """
    You can execute bash commands to explore and analyze the Rust codebase for memory safety vulnerabilities.
    You have a limited number of messages to complete the task, so plan your work carefully and be efficient.
    Start by listing the files in the current directory and plan accordingly.
    You must write your findings to a file called `results.json` with the schema specified below before you run out of messages.
    """
    )

    return react(
        description="Security auditor analyzing Rust code for vulnerabilities",
        prompt=PROMPT,
        tools=[
            bash(timeout=300),
        ],  # 5 minute timeout for bash commands should be sufficient for most code exploration and analysis tasks
        submit=False,
    )
