import pandas as pd
from pathlib import Path


def load_statistics_csv(csv_path: str) -> dict:
    df = pd.read_csv(csv_path)
    return dict(zip(df["metric"], df["value"]))


def populate_latex_template(stats: dict, template_path: str, output_path: str):
    with open(template_path, "r") as f:
        template_content = f.read()

    # Replace all placeholder zeros with actual values
    replacements = {
        # Dataset Composition
        "Total vulnerabilities & 0": f'Total vulnerabilities & {int(stats["total_vulnerabilities"])}',
        "Total code samples & 0": f'Total code samples & {int(stats["total_code_samples"])}',
        "Vulnerabilities with paired fix & 0 (0\\%)": f'Vulnerabilities with paired fix & {int(stats["vulnerabilities_with_fix"])} ({stats["vulnerabilities_with_fix_percent"]}\\%)',
        "Vulnerable code samples & 0": f'Vulnerable code samples & {int(stats["vulnerable_samples"])}',
        "Fixed code samples & 0": f'Fixed code samples & {int(stats["fixed_samples"])}',
        # Multi-Granularity Coverage
        "Crate-level samples & 0": f'Crate-level samples & {int(stats["crate_level_samples"])}',
        "File-level samples & 0": f'File-level samples & {int(stats["file_level_samples"])}',
        "Function-level samples & 0": f'Function-level samples & {int(stats["function_level_samples"])}',
        # Vulnerability Profile
        "Unique CWE types & 0": f'Unique CWE types & {int(stats["unique_cwe_types"])}',
        "Vulnerability years range & 0--0": f'Vulnerability years range & {stats["vulnerability_years_range"]}',
        # Lines of Code
        "Overall (Mean/Median) & 0/0": f'Overall (Mean/Median) & {int(stats["overall_loc_mean"])} LoC / {int(stats["overall_loc_median"])} LoC',
        "Crate-level (Mean/Median) & 0/0": f'Crate-level (Mean/Median) & {int(stats["crate_level_loc_mean"])} LoC / {int(stats["crate_level_loc_median"])} LoC',
        "File-level (Mean/Median) & 0/0": f'File-level (Mean/Median) & {int(stats["file_level_loc_mean"])} LoC / {int(stats["file_level_loc_median"])} LoC',
        "Function-level (Mean/Median) & 0/0": f'Function-level (Mean/Median) & {int(stats["function_level_loc_mean"])} LoC / {int(stats["function_level_loc_median"])} LoC',
    }

    # Apply replacements
    for old_text, new_text in replacements.items():
        template_content = template_content.replace(old_text, new_text)

    # Write the populated template
    with open(output_path, "w") as f:
        f.write(template_content)

    print(f"Generated LaTeX table: {output_path}")


def main():
    current_dir = Path(__file__).parent.parent
    latex_dir = current_dir / "latex"
    data_dir = current_dir / "data"

    # Set up paths
    csv_path = data_dir / "dataset_statistics_comprehensive.csv"
    template_path = latex_dir / "TEMPLATE_dataset_summary.tex"
    output_path = latex_dir / "dataset_summary_generated.tex"

    # Check if CSV exists
    if not csv_path.exists():
        print(f"Error: CSV file not found at {csv_path}")
        print(
            "Please run generate_dataset_stats_csv.py first to generate the statistics CSV."
        )
        return

    print(f"Loading statistics from {csv_path}...")
    stats = load_statistics_csv(str(csv_path))
    print(f"Loaded {len(stats)} statistics")

    print("Populating LaTeX template...")
    populate_latex_template(stats, str(template_path), str(output_path))

    print("Done!")


if __name__ == "__main__":
    main()
