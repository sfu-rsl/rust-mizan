from pathlib import Path
from typing import Union

from langchain_community.document_loaders import DirectoryLoader, TextLoader

__all__ = ["load_prompt", "load_directory"]


def load_prompt(prompt: Union[str, Path]) -> str:
    """
    Return the prompt text based on various input types.
    
    Args:
        prompt: Either a Path object, a string containing the prompt text,
               a string path to a prompt file, or a prompt name in the package.
    
    Returns:
        The prompt text content.
    """
    # Handle Path object
    if isinstance(prompt, Path):
        return prompt.read_text(encoding="utf-8")

    # Handle full prompt text (contains newlines)
    if "\n" in prompt:
        return prompt

    # Try as local file path
    path = Path(prompt)
    if path.is_file():
        return path.read_text(encoding="utf-8")

    # Try as packaged prompt template
    try:
        from importlib.resources import files
        return files("sprout_pipeline.prompts").joinpath(path.name).read_text(encoding="utf-8")
    except FileNotFoundError:
        pass

    # Default: return the input string as is
    return prompt


def load_directory(dir_path: Path) -> str:
    """
    Load all Rust source files and Cargo.toml from a directory.
    
    Args:
        dir_path: Path to the directory containing Rust code.
        
    Returns:
        Formatted string containing all file contents with line numbers.
    """
    loader = DirectoryLoader(
        str(dir_path),
        glob=["**/*.rs", "**/Cargo.toml"],
        loader_cls=TextLoader,
        recursive=True,
        silent_errors=True,
    )
    docs = loader.load()
    
    parts = []
    for doc in docs:
        rel_path = Path(doc.metadata["source"]).relative_to(dir_path)
        parts.append(f"// FILE: {rel_path}")
        
        for line_num, line in enumerate(doc.page_content.splitlines(), 1):
            parts.append(f"{line_num:04d}: {line}")
        
        parts.append("")
        
    return "\n".join(parts)