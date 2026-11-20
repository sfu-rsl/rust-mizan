# inspect-ai 0.3.291 upgrade: ReAct prompt change

Commit `a897612` upgraded `inspect-ai` from 0.3.205 to 0.3.291 (resolved in `mizan-cli/poetry.lock`).
Mizan's own prompt in `solver.py` is unchanged. However, `react(prompt=PROMPT)` appends the library's
built-in `DEFAULT_ASSISTANT_PROMPT` to it, and that default changed upstream in inspect-ai 0.3.230.
Every model run with `react_agent()` now gets a different system message.

## Old and new prompt

Both are copied from `inspect_ai/agent/_types.py` in the PyPI wheel for each version.

**0.3.205** (lines 13–15):

```python
DEFAULT_ASSISTANT_PROMPT = """
You are a helpful assistant attempting to submit the best possible answer. You have several tools available to help with finding the answer. You will see the result of tool calls right after sending the message. If you need to perform multiple actions, you can always send more messages with additional tool calls. Do some reasoning before your actions, describing what tool calls you are going to use and how they fit into your plan.
"""
```

**0.3.291** (lines 13–18):

```python
PARALLEL_TOOLS_PROMPT = "Prioritize parallel tool calls: when operations are independent, run them in one response — e.g. reading several files or running several searches at once — rather than one at a time. Only sequence calls when one depends on another's result."

DEFAULT_ASSISTANT_PROMPT = f"""
You are a helpful assistant attempting to submit the best possible answer. You have several tools available to help with finding the answer. You will see the result of tool calls right after sending the message. {PARALLEL_TOOLS_PROMPT} Do some reasoning before your actions, describing what tool calls you are going to use and how they fit into your plan.
"""
```

The submit prompt is unchanged in both versions.

## What changed

One sentence was swapped:

- **Removed:** "If you need to perform multiple actions, you can always send more messages with additional tool calls."
- **Added:** "Prioritize parallel tool calls: … Only sequence calls when one depends on another's result."

The same upgrade also changed the agent loop in two ways that affect behaviour:

- The `bash()` tool's argument is renamed from `cmd` to `command`.
- Tool calls in one message now run concurrently. In 0.3.205 they ran one at a time.

## Reproducibility

`^0.3.291` still allows later releases to change the prompt again. Either of these would fix it in place:

- Pin `inspect-ai` exactly.
- Pass a full `AgentPrompt(...)` with an explicit `assistant_prompt`.
