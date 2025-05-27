from __future__ import annotations
from pathlib import Path
from typing import Literal, Sequence, Type, Any, Dict, Union, Optional

from langchain_core.output_parsers import JsonOutputParser
from langchain_core.prompts import PromptTemplate
from langchain_openai import ChatOpenAI
from pydantic import BaseModel

try:
    from langchain_anthropic import ChatAnthropic
except ImportError:
    ChatAnthropic = None

from .loader import load_directory, load_prompt

Provider = Literal["openai", "anthropic"]


class LLMError:
    """Represents an error from the LLM invocation."""
    def __init__(self, error_type: str, error_message: str):
        self.error_type = error_type
        self.error_message = error_message
        self.is_error = True


def _select_llm(provider: Provider, model_name: str):
    """
    Select and initialize the appropriate LLM based on the provider.

    Args:
        provider: The LLM provider ('openai' or 'anthropic')
        model_name: The model name to use

    Returns:
        Initialized LLM instance

    Raises:
        ImportError: If langchain-anthropic is not installed for Anthropic models
        ValueError: If an unsupported provider is specified
    """
    if provider == "openai":
        return ChatOpenAI(model=model_name, temperature=0.0)
    if provider == "anthropic":
        if ChatAnthropic is None:
            raise ImportError("`langchain-anthropic` not installed.")
        return ChatAnthropic(model=model_name, temperature=0.0)
    raise ValueError(f"Unsupported provider: {provider}")


def run_pipeline(
    *,
    prompt: Union[Path, str],
    schema_model: Type[BaseModel],
    code_dir: Path = None,
    model_name: str,
    provider: Provider,
) -> Union[Dict[str, Any], Sequence[Dict[str, Any]], LLMError]:
    """
    Run the LLM pipeline with the given prompt and code directory.

    Args:
        prompt: The prompt text or path to prompt file
        schema_model: Pydantic model for parsing the LLM response
        code_dir: Directory containing Rust code (None for tasks not requiring code)
        model_name: Name of the LLM model to use
        provider: LLM provider to use ('openai' or 'anthropic')

    Returns:
        Parsed LLM response as a dictionary or sequence of dictionaries, or LLMError on failure
    """
    try:
        prompt_text = load_prompt(prompt)
        directory_text = "" if code_dir is None else load_directory(code_dir)

        parser = JsonOutputParser(pydantic_object=schema_model)
        prompt_template = PromptTemplate(
            template=(
                "{prompt}\n\n{format_instructions}\n\n"
                "### DIRECTORY_CONTENTS_START\n{directory}\n### DIRECTORY_CONTENTS_END"
            ),
            input_variables=["prompt", "directory"],
            partial_variables={"format_instructions": parser.get_format_instructions()},
        )

        llm = _select_llm(provider, model_name)
        response = (prompt_template | llm | parser).invoke(
            {"prompt": prompt_text, "directory": directory_text}
        )

        # Convert to dict format (handle both single items and lists)
        raw = getattr(response, "root", response)
        if isinstance(raw, list):
            return [r.model_dump() if isinstance(r, BaseModel) else r for r in raw]
        return raw.model_dump() if isinstance(raw, BaseModel) else raw
    
    except Exception as e:
        # Capture the error type and message
        error_type = type(e).__name__
        error_message = str(e)
        return LLMError(error_type, error_message)
