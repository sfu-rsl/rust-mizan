import json
from pathlib import Path
from typing import Dict, List, Any
import pandas as pd


def get_evaluation_results_path() -> Path:
    current_file = Path(__file__).resolve()
    repo_root = current_file.parents[4]
    return repo_root / "paper_evaluation_section" / "LLMs" / "evaluation_results"


def get_short_model_name(model_name: str) -> str:
    if "claude-3-7-sonnet" in model_name:
        return "Claude 3.7 Sonnet"
    elif "gpt-4.1" in model_name:
        return "GPT-4.1"
    elif "gemini-1.5-pro" in model_name:
        return "Gemini 1.5 Pro"
    elif "deepseek-chat" in model_name:
        return "DeepSeek-V3.1"
    else:
        return model_name


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
