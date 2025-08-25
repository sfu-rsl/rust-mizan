import json
import pandas as pd
from pathlib import Path
from typing import Dict


def count_rust_lines_in_directory(directory_path: str) -> int:
    total_lines = 0
    directory = Path(directory_path)

    if not directory.exists():
        # panic and abort
        raise FileNotFoundError(f"Directory {directory_path} does not exist.")

    # Walk through all .rs files in the directory
    for rust_file in directory.rglob("*.rs"):
        try:
            with open(rust_file, "r", encoding="utf-8") as f:
                lines = len(f.readlines())
                total_lines += lines
        except (UnicodeDecodeError, IOError) as e:
            print(f"Warning: Could not read {rust_file}: {e}")
            continue

    return total_lines


def load_mizan_data(mizan_json_path: str) -> pd.DataFrame:
    with open(mizan_json_path, "r") as f:
        data = json.load(f)

    rows = []
    samples_base_dir = Path(mizan_json_path).parent / "samples"

    for vuln in data["vulnerabilities"]:
        vuln_id = vuln["id"]
        crate_name = vuln["crate_name"]
        year = vuln["year"]

        for sample in vuln["code_samples"]:
            # Determine granularity from path
            path_parts = sample["path_to_crate"].split("-")
            granularity = path_parts[-1]  # e.g., 'function', 'file', 'crate'

            # Build full path to the sample directory
            sample_dir = samples_base_dir / sample["path_to_crate"]

            # Count lines of code
            loc = count_rust_lines_in_directory(str(sample_dir))

            # Extract CWE types
            cwe_types = sample.get("cwe_type", [])

            rows.append(
                {
                    "vulnerability_id": vuln_id,
                    "crate_name": crate_name,
                    "year": year,
                    "path_to_crate": sample["path_to_crate"],
                    "is_vulnerability": sample["is_vulnerability"],
                    "granularity": granularity,
                    "cwe_types": cwe_types,
                    "lines_of_code": loc,
                    "sample_dir": str(sample_dir),
                }
            )

    return pd.DataFrame(rows)


def load_vulnerability_mapping(mapping_path: str) -> Dict:
    with open(mapping_path, "r") as f:
        return json.load(f)


def collect_comprehensive_statistics(
    df: pd.DataFrame, vuln_mapping: Dict
) -> pd.DataFrame:
    # Basic counts
    unique_vulns = df["vulnerability_id"].nunique()
    total_samples = len(df)
    vulnerable_samples = len(df[df["is_vulnerability"] == True])
    fixed_samples = len(df[df["is_vulnerability"] == False])

    # Vulnerability pairing analysis
    vuln_with_fix = 0
    for vuln_id in df["vulnerability_id"].unique():
        vuln_samples = df[df["vulnerability_id"] == vuln_id]
        has_vuln = any(vuln_samples["is_vulnerability"] == True)
        has_fix = any(vuln_samples["is_vulnerability"] == False)
        if has_vuln and has_fix:
            vuln_with_fix += 1

    # Granularity breakdown
    granularity_counts = df["granularity"].value_counts()

    # CWE analysis
    all_cwes = set()
    for cwe_list in df["cwe_types"]:
        all_cwes.update(cwe_list)
    unique_cwe_count = len(all_cwes)

    # Year range
    min_year = df["year"].min()
    max_year = df["year"].max()

    # Lines of code statistics - overall
    overall_loc_mean = round(df["lines_of_code"].mean())
    overall_loc_median = round(df["lines_of_code"].median())
    overall_loc_std = round(df["lines_of_code"].std())
    overall_loc_min = df["lines_of_code"].min()
    overall_loc_max = df["lines_of_code"].max()

    # LOC by granularity
    loc_stats_by_granularity = {}
    for gran in ["crate", "file", "function"]:
        gran_data = df[df["granularity"] == gran]["lines_of_code"]
        if len(gran_data) > 0:
            loc_stats_by_granularity[gran] = {
                "mean": round(gran_data.mean()),
                "median": round(gran_data.median()),
                "std": round(gran_data.std()),
                "min": gran_data.min(),
                "max": gran_data.max(),
                "count": len(gran_data),
            }

    # LOC by vulnerability status
    vuln_loc_data = df[df["is_vulnerability"] == True]["lines_of_code"]
    fixed_loc_data = df[df["is_vulnerability"] == False]["lines_of_code"]

    # CWE frequency analysis
    cwe_counts = {}
    for cwe_list in df["cwe_types"]:
        for cwe in cwe_list:
            cwe_counts[cwe] = cwe_counts.get(cwe, 0) + 1

    # Year distribution
    year_counts = df["year"].value_counts().sort_index()

    # Create comprehensive statistics
    stats_entries = [
        # Basic dataset composition
        ["total_vulnerabilities", unique_vulns],
        ["total_code_samples", total_samples],
        ["vulnerable_samples", vulnerable_samples],
        ["fixed_samples", fixed_samples],
        ["vulnerabilities_with_fix", vuln_with_fix],
        [
            "vulnerabilities_with_fix_percent",
            round((vuln_with_fix / unique_vulns) * 100, 1),
        ],
        # Granularity breakdown
        ["crate_level_samples", granularity_counts.get("crate", 0)],
        ["file_level_samples", granularity_counts.get("file", 0)],
        ["function_level_samples", granularity_counts.get("function", 0)],
        # Vulnerability characteristics
        ["unique_cwe_types", unique_cwe_count],
        ["vulnerability_years_range", f"{min_year}--{max_year}"],
        ["min_year", min_year],
        ["max_year", max_year],
        # Overall LOC statistics
        ["overall_loc_mean", overall_loc_mean],
        ["overall_loc_median", overall_loc_median],
        ["overall_loc_std", overall_loc_std],
        ["overall_loc_min", overall_loc_min],
        ["overall_loc_max", overall_loc_max],
        # LOC by vulnerability status
        [
            "vulnerable_loc_mean",
            round(vuln_loc_data.mean()) if len(vuln_loc_data) > 0 else 0,
        ],
        [
            "vulnerable_loc_median",
            round(vuln_loc_data.median()) if len(vuln_loc_data) > 0 else 0,
        ],
        [
            "fixed_loc_mean",
            round(fixed_loc_data.mean()) if len(fixed_loc_data) > 0 else 0,
        ],
        [
            "fixed_loc_median",
            round(fixed_loc_data.median()) if len(fixed_loc_data) > 0 else 0,
        ],
    ]

    # LOC statistics by granularity
    for gran in ["crate", "file", "function"]:
        if gran in loc_stats_by_granularity:
            stats = loc_stats_by_granularity[gran]
            stats_entries.extend(
                [
                    [f"{gran}_level_loc_mean", stats["mean"]],
                    [f"{gran}_level_loc_median", stats["median"]],
                    [f"{gran}_level_loc_std", stats["std"]],
                    [f"{gran}_level_loc_min", stats["min"]],
                    [f"{gran}_level_loc_max", stats["max"]],
                    [f"{gran}_level_loc_count", stats["count"]],
                ]
            )

    # most common CWEs
    most_common_cwes = sorted(cwe_counts.items(), key=lambda x: x[1], reverse=True)[:5]
    for i, (cwe, count) in enumerate(most_common_cwes):
        stats_entries.extend(
            [
                [f"most_common_cwe_{i+1}", cwe],
                [f"most_common_cwe_{i+1}_count", count],
                [f"most_common_cwe_{i+1}_percent", round((count / len(df)) * 100, 1)],
            ]
        )

    # year distribution
    for year, count in year_counts.items():
        stats_entries.append([f"year_{year}_samples", count])

    return pd.DataFrame(stats_entries, columns=["metric", "value"])


def main():
    """Main function to generate comprehensive dataset statistics CSV."""
    current_dir = Path(__file__).parent.parent
    data_dir = current_dir / "data"
    data_dir.mkdir(exist_ok=True)

    mizan_json_path = current_dir.parent.parent.parent / "mizan.json"
    vuln_mapping_path = current_dir / "data" / "vulnerability_mapping.json"
    output_path = data_dir / "dataset_statistics_comprehensive.csv"

    print("Loading mizan.json data...")
    df = load_mizan_data(str(mizan_json_path))
    print(
        f"Loaded {len(df)} code samples from {df['vulnerability_id'].nunique()} vulnerabilities"
    )

    print("Loading vulnerability mapping...")
    vuln_mapping = load_vulnerability_mapping(str(vuln_mapping_path))

    print("Collecting comprehensive dataset statistics...")
    stats_df = collect_comprehensive_statistics(df, vuln_mapping)

    print(f"Saving statistics to {output_path}...")
    stats_df.to_csv(output_path, index=False)

    # Also save detailed sample data for visualizations
    sample_data_path = data_dir / "dataset_samples_detailed.csv"
    print(f"Saving detailed sample data to {sample_data_path}...")
    df.to_csv(sample_data_path, index=False)

    print(f"Generated comprehensive statistics with {len(stats_df)} metrics")
    print(f"Generated detailed sample data with {len(df)} samples")
    print("Done!")


if __name__ == "__main__":
    main()
