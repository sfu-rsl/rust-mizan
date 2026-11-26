ASSISTANT_PROMPT = """ You can execute bash commands to explore and analyze the Rust codebase for memory safety vulnerabilities.
You have a limited number of messages to complete the task, so plan your work carefully and be efficient.
Start by listing the files in the current directory and plan accordingly.
You must write your findings to a file called `results.json` with the schema specified below before you run out of messages.
"""