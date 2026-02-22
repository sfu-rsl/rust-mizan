from textwrap import dedent
from inspect_ai.agent import Agent, AgentState, agent, react
from inspect_ai.tool import bash
from inspect_ai.util import sample_limits


async def message_tracking_continue(state: AgentState) -> str | bool:
    """on_continue hook that injects message warnings at key thresholds."""
    # message_limit counts ALL messages (system, user, assistant, tool)
    message_count = len(state.messages)

    try:
        limits = sample_limits()
        msg_limit = limits.message.limit
    except Exception:
        msg_limit = None

    if msg_limit:
        remaining = msg_limit - message_count

        if remaining < 10:
            return f"[URGENT: You are almost out of messages. Focus on writing your findings to results.json immediately based on your current knowledge."

        if remaining < 15:
            return (
                f"[WARNING: ~{remaining} messages remaining] "
                "Plan strategically - you will need to write your findings to results.json soon."
            )

    # continue without injecting a message
    return True


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
        ],  # 5 minute timeout for bash commands
        on_continue=message_tracking_continue,
    )
