from __future__ import annotations
from abc import ABC, abstractmethod
from pathlib import Path
from typing import Iterator, Tuple, Any, Dict, Type, Optional
from pydantic import BaseModel


class TaskBase(ABC):
    """
    Base class for all LLM-based analysis tasks.
    
    Each task subclass must implement:
    1. iterate_samples - to extract samples from input data
    2. score - to evaluate LLM responses against ground truth
    3. dataset_row - to format results for CSV output
    
    Tasks may optionally implement:
    - build_prompt - to customize prompts per sample
    """
    
    name: str
    schema: Type[BaseModel]
    prompt_path: Path

    @staticmethod
    @abstractmethod
    def iterate_samples(input_path: Path) -> Iterator[Tuple[Optional[Path], Any, Optional[Dict]]]:
        """
        Yield (code_dir, ground_truth, context) tuples parsed from input.
        
        Args:
            input_path: Path to the input data file (typically mizan.json)
            
        Returns:
            Iterator yielding tuples of:
            - code_dir: Path to the code directory (None if code not required)
            - ground_truth: The expected correct output
            - context: Optional context for prompt customization (None if not needed)
        """
        pass

    @staticmethod
    @abstractmethod
    def score(prediction: Any, ground_truth: Any) -> Dict:
        """
        Score the LLM prediction against ground truth.
        
        Args:
            prediction: The LLM's prediction
            ground_truth: The expected correct output
            
        Returns:
            Dictionary containing scoring metrics
        """
        pass

    @staticmethod
    @abstractmethod
    def dataset_row(meta: Dict, score: Dict) -> Dict[str, Any]:
        """
        Format results for CSV dataset output.
        
        Args:
            meta: Metadata about the run (model, timestamp, etc.)
            score: Scoring results from the score() method
            
        Returns:
            Flattened dictionary for CSV row
        """
        pass