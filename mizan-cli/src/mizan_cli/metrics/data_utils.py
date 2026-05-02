from pathlib import Path
from typing import List
import pandas as pd
import json

_experiment_cache = {}


def load_processed_experiment(experiment_id: str, data_file: str = None) -> pd.DataFrame:
    if experiment_id in _experiment_cache:
        return _experiment_cache[experiment_id]

    from src.leaderboard.read_evals import load_leaderboard_config

    config = load_leaderboard_config()
    current_file = Path(__file__).resolve()
    repo_root = current_file.parents[2]

    if data_file is None:
        for exp in config["experiments"]:
            exp_key = f"{exp['name']}_{exp.get('tag', 'unknown')}"
            if exp_key == experiment_id:
                data_file = repo_root / "data" / "experiments" / exp["data_file"]
                break

    if data_file is None:
        raise FileNotFoundError(f"Experiment not found: {experiment_id}")

    if isinstance(data_file, str):
        data_file = Path(data_file)

    with open(data_file) as f:
        experiment_data = json.load(f)

    df = pd.DataFrame(experiment_data["samples"])
    _experiment_cache[experiment_id] = df
    return df


def find_common_samples(experiment_ids: List[str], only_valid_json: bool = False) -> List[str]:
    if not experiment_ids:
        return []

    from src.leaderboard.read_evals import get_all_experiments_data
    all_experiments = get_all_experiments_data()

    exp_map = {exp["experiment_id"]: exp for exp in all_experiments}

    df = load_processed_experiment(
        experiment_ids[0],
        exp_map.get(experiment_ids[0], {}).get("data_file")
    )
    if only_valid_json:
        common_samples = set(df[df["is_valid_json"]]["example_id"])
    else:
        common_samples = set(df["example_id"])

    for exp_id in experiment_ids[1:]:
        df = load_processed_experiment(
            exp_id,
            exp_map.get(exp_id, {}).get("data_file")
        )
        if only_valid_json:
            valid_samples = set(df[df["is_valid_json"]]["example_id"])
            common_samples &= valid_samples
        else:
            all_samples = set(df["example_id"])
            common_samples &= all_samples

    return sorted(list(common_samples))
