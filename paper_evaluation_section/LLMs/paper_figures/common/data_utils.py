import json
from pathlib import Path
from typing import Dict, List, Any
import pandas as pd


def get_evaluation_results_path() -> Path:
    current_file = Path(__file__).resolve()
    repo_root = current_file.parents[4]
    return repo_root / "paper_evaluation_section" / "LLMs" / "evaluation_results"


MODEL_NAMES = {
    "claude-3-7-sonnet": "Claude 3.7 Sonnet",
    "gpt-4.1": "GPT-4.1", 
    "gemini-1.5-pro": "Gemini 1.5 Pro",
    "deepseek-chat": "DeepSeek-V3.1"
}

MODEL_ORDER = ["GPT-4.1", "Gemini 1.5 Pro", "Claude 3.7 Sonnet", "DeepSeek-V3.1"]

def get_short_model_name(model_name: str) -> str:
    for key, value in MODEL_NAMES.items():
        if key in model_name:
            return value
    return model_name

def get_ordered_models(available_models: List[str]) -> List[str]:
    return [model for model in MODEL_ORDER if model in available_models]


def load_experiments_catalog() -> Dict[str, Dict[str, str]]:
    catalog_path = get_evaluation_results_path() / "experiments.json"

    if not catalog_path.exists():
        raise FileNotFoundError(f"Experiments catalog not found: {catalog_path}")

    with open(catalog_path, "r") as f:
        return json.load(f)


def load_experiment_results(experiment_id: str) -> List[Dict[str, Any]]:
    results_path = (
        get_evaluation_results_path() / f"experiment_{experiment_id}" / "results.json"
    )

    if not results_path.exists():
        raise FileNotFoundError(f"Results file not found: {results_path}")

    with open(results_path, "r") as f:
        return json.load(f)


def save_processed_results(
    experiment_id: str, df: pd.DataFrame, model_name: str
) -> None:
    df_copy = df.copy()
    df_copy["model"] = get_short_model_name(model_name)
    output_path = (
        get_evaluation_results_path()
        / f"experiment_{experiment_id}"
        / "processed_results.csv"
    )
    df_copy.to_csv(output_path, index=False)


def load_processed_experiment(experiment_id: str) -> pd.DataFrame:
    results_path = (
        get_evaluation_results_path()
        / f"experiment_{experiment_id}"
        / "processed_results.csv"
    )

    if not results_path.exists():
        raise FileNotFoundError(f"Processed results not found: {results_path}")

    return pd.read_csv(results_path)


def find_common_samples(experiment_ids: List[str]) -> List[str]:
    if not experiment_ids:
        return []

    df = load_processed_experiment(experiment_ids[0])
    common_samples = set(df[df["is_valid_json"]]["example_id"])

    for exp_id in experiment_ids[1:]:
        df = load_processed_experiment(exp_id)
        valid_samples = set(df[df["is_valid_json"]]["example_id"])
        common_samples &= valid_samples

    return sorted(list(common_samples))


def get_vanilla_experiment_ids() -> Dict[str, str]:
    catalog = load_experiments_catalog()
    vanilla_experiments = {}

    for model, experiments in catalog.items():
        if "vanilla" in experiments:
            vanilla_experiments[model] = experiments["vanilla"]

    return vanilla_experiments
