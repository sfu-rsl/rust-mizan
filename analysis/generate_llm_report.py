#!/usr/bin/env python3
"""
Generate LLM Training Data Contamination Experiment Analysis Report

This script analyzes the results from three experiments testing LLM performance
on Rust memory safety vulnerabilities, comparing original and mutated datasets.
"""

import pandas as pd
import warnings
import argparse
import os
from datetime import datetime

warnings.filterwarnings("ignore")


def extract_level_from_code_sample(code_sample):
    """Extract the level (crate/file/function) from code sample path"""
    if "vuln-crate" in code_sample:
        return "crate"
    elif "vuln-file" in code_sample:
        return "file"
    elif "vuln-function" in code_sample:
        return "function"
    else:
        return "function"


def parse_cve_list(cve_str):
    """Parse CVE list from string representation"""
    if pd.isna(cve_str) or cve_str == "[]":
        return []
    if isinstance(cve_str, str):
        cve_str = cve_str.strip("[]").replace("'", "").replace('"', "")
        if cve_str:
            return [cve.strip() for cve in cve_str.split(",")]
    return []


def load_dataset(check_cve_path, identify_crate_path, vuln_path):
    """Load all CSV files for a dataset"""
    data = {}

    if check_cve_path:
        try:
            data["check_cve"] = pd.read_csv(check_cve_path)
        except:
            data["check_cve"] = None
    else:
        data["check_cve"] = None

    try:
        data["identify_crate"] = pd.read_csv(identify_crate_path)
        if data["identify_crate"] is not None:
            data["identify_crate"]["level"] = data["identify_crate"][
                "code_sample"
            ].apply(extract_level_from_code_sample)
            # Parse CVE lists
            data["identify_crate"]["cve_tp_list"] = data["identify_crate"][
                "cve_tp"
            ].apply(parse_cve_list)
            data["identify_crate"]["has_cve_tp"] = data["identify_crate"][
                "cve_tp_list"
            ].apply(lambda x: len(x) > 0)
    except:
        data["identify_crate"] = None

    try:
        data["vuln"] = pd.read_csv(vuln_path)
        if data["vuln"] is not None:
            data["vuln"]["level"] = data["vuln"]["code_sample"].apply(
                extract_level_from_code_sample
            )
            # Add flags for having at least one true positive
            data["vuln"]["has_cwe_tp"] = data["vuln"]["cwe_tp"] > 0
            data["vuln"]["has_func_tp"] = data["vuln"]["func_tp"] > 0
            data["vuln"]["has_line_tp"] = data["vuln"]["line_tp"] > 0
    except:
        data["vuln"] = None

    return data


def generate_dataset_summary_md(
    data,
    dataset_name,
    model_provider,
    model_name,
    num_check_cve,
    num_identify_crate,
    num_vuln,
):
    """Generate Markdown for dataset summary"""
    md = f"\n## {model_provider} `{model_name}` on `{dataset_name}`:\n\n"

    # Check CVE task (only for original dataset)
    if data["check_cve"] is not None:
        df = data["check_cve"]
        total = len(df)
        correct = df["has_cve_correct"].sum()
        missing = num_check_cve - total

        md += "### Check CVE task:\n"
        md += f"- Correctly predicted CVE existence: {correct} out of {total} ({correct/total*100:.1f}%)\n"
        md += "> The percentage of times the model correctly predicted whether a crate has CVEs in a given year\n"
        if missing > 0:
            md += f"- Failed to generate valid results for {missing} samples out of {num_check_cve}\n"
        md += "\n"

    # Identify crate task
    if data["identify_crate"] is not None:
        df = data["identify_crate"]
        total = len(df)
        crate_correct = df["crate_name_correct"].sum()
        year_correct = df["year_correct"].sum()
        cve_correct = df["has_cve_correct"].sum()
        cve_tp_count = df["has_cve_tp"].sum()
        missing = num_identify_crate - total

        md += "### Identify crate task:\n"
        md += f"- Crate names: {crate_correct} out of {total} ({crate_correct/total*100:.1f}%)\n"
        md += "> The percentage of times the model correctly identified the crate name from source code\n"
        md += (
            f"- Years: {year_correct} out of {total} ({year_correct/total*100:.1f}%)\n"
        )
        md += "> The percentage of times the model correctly identified the crate publication year\n"
        md += f"- CVE existence: {cve_correct} out of {total} ({cve_correct/total*100:.1f}%)\n"
        md += "> The percentage of times the model correctly predicted whether the crate has CVEs\n"
        md += f"- At least one CVE correct: {cve_tp_count} out of {total} ({cve_tp_count/total*100:.1f}%)\n"
        md += "> The percentage of times the model correctly identified at least one actual CVE\n"
        if missing > 0:
            md += f"- Failed to generate valid results for {missing} samples out of {num_identify_crate}\n"
        md += "\n"

    # Vulnerability detection task
    if data["vuln"] is not None:
        df = data["vuln"]
        total = len(df)
        existence_correct = df["existence_correct"].sum()
        cwe_tp_count = df["has_cwe_tp"].sum()
        func_tp_count = df["has_func_tp"].sum()
        line_tp_count = df["has_line_tp"].sum()
        missing = num_vuln - total

        md += "### Vulnerability detection task:\n"
        md += f"- Vulnerability existence: {existence_correct} out of {total} ({existence_correct/total*100:.1f}%)\n"
        md += "> The percentage of times the model correctly detected whether a vulnerability exists\n"
        md += f"- At least one CWE type correct: {cwe_tp_count} out of {total} ({cwe_tp_count/total*100:.1f}%)\n"
        md += "> The percentage of times the model correctly identified at least one CWE type\n"
        md += f"- At least one vulnerable function correct: {func_tp_count} out of {total} ({func_tp_count/total*100:.1f}%)\n"
        md += "> The percentage of times the model correctly identified at least one vulnerable function\n"
        md += f"- At least one vulnerable line correct: {line_tp_count} out of {total} ({line_tp_count/total*100:.1f}%)\n"
        md += "> The percentage of times the model correctly identified at least one vulnerable line\n"
        if missing > 0:
            md += f"- Failed to generate valid results for {missing} samples out of {num_vuln}\n"
        md += "\n"

    return md


def create_level_comparison_identify(
    original_data, mutated_data, original_name, mutated_name
):
    """Create identify crate task comparison by level"""
    results = []

    for dataset_name, data in [
        (original_name, original_data),
        (mutated_name, mutated_data),
    ]:
        if data["identify_crate"] is not None:
            df = data["identify_crate"]

            for level in ["crate", "file", "function"]:
                level_df = df[df["level"] == level]
                if len(level_df) > 0:
                    total = len(level_df)
                    crate_correct = level_df["crate_name_correct"].sum()
                    year_correct = level_df["year_correct"].sum()
                    cve_tp_correct = level_df["has_cve_tp"].sum()

                    results.append(
                        {
                            "Dataset": dataset_name,
                            "Level": level,
                            "Crate Name (%)": f"{crate_correct/total*100:.1f}",
                            "Year (%)": f"{year_correct/total*100:.1f}",
                            "CVE (%)": f"{cve_tp_correct/total*100:.1f}",
                        }
                    )

    return pd.DataFrame(results)


def create_level_comparison_vuln(
    original_data, mutated_data, original_name, mutated_name
):
    """Create vulnerability detection task comparison by level"""
    results = []

    for dataset_name, data in [
        (original_name, original_data),
        (mutated_name, mutated_data),
    ]:
        if data["vuln"] is not None:
            df = data["vuln"]

            for level in ["crate", "file", "function"]:
                level_df = df[df["level"] == level]
                if len(level_df) > 0:
                    total = len(level_df)
                    existence_correct = level_df["existence_correct"].sum()
                    cwe_tp_count = level_df["has_cwe_tp"].sum()
                    func_tp_count = level_df["has_func_tp"].sum()
                    line_tp_count = level_df["has_line_tp"].sum()

                    results.append(
                        {
                            "Dataset": dataset_name,
                            "Level": level,
                            "Existence (%)": f"{existence_correct/total*100:.1f}",
                            "CWE Type (%)": f"{cwe_tp_count/total*100:.1f}",
                            "Functions (%)": f"{func_tp_count/total*100:.1f}",
                            "Lines (%)": f"{line_tp_count/total*100:.1f}",
                        }
                    )

    return pd.DataFrame(results)


def dataframe_to_markdown_table(df):
    """Convert pandas DataFrame to Markdown table"""
    # Get column headers
    headers = "| " + " | ".join(df.columns) + " |"
    separator = "|" + "|".join([" --- " for _ in df.columns]) + "|"

    # Get rows
    rows = []
    for _, row in df.iterrows():
        row_str = "| " + " | ".join(str(val) for val in row) + " |"
        rows.append(row_str)

    # Combine all parts
    table = "\n".join([headers, separator] + rows)
    return table


def generate_report(config):
    """Generate the full Markdown report"""
    # Load datasets
    original_data = load_dataset(
        config["original_check_cve"],
        config["original_identify_crate"],
        config["original_vuln"],
    )
    mutated_data = load_dataset(
        None, config["mutated_identify_crate"], config["mutated_vuln"]
    )

    # Start building Markdown content
    content = f"""# LLM Training Data Contamination Experiment Analysis

**Model:** {config['model_provider']} - {config['model_name']}  
**Original Dataset:** {config['original_dataset_name']}  
**Mutated Dataset:** {config['mutated_dataset_name']}  
**Mutation:** {config['mutation_description']}  
**Report Generated:** {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}

# Mutation: {config['mutation_description']}
"""

    # Add dataset summaries
    content += generate_dataset_summary_md(
        original_data,
        config["original_dataset_name"],
        config["model_provider"],
        config["model_name"],
        config["num_check_cve"],
        config["num_identify_crate"],
        config["num_vuln"],
    )

    content += generate_dataset_summary_md(
        mutated_data,
        config["mutated_dataset_name"],
        config["model_provider"],
        config["model_name"],
        config["num_check_cve"],
        config["num_identify_crate"],
        config["num_vuln"],
    )

    # Add identify crate task by level
    content += "\n## Identify Crate Task - Performance by Code Level:\n\n"
    content += """**Metric explanations:**
- **Crate Name (%):** The percentage of times the model correctly identified the crate name
- **Year (%):** The percentage of times the model correctly identified the crate publication year
- **CVE (%):** The percentage of times the model correctly identified at least one actual CVE

"""

    identify_df = create_level_comparison_identify(
        original_data,
        mutated_data,
        config["original_dataset_name"],
        config["mutated_dataset_name"],
    )
    content += dataframe_to_markdown_table(identify_df) + "\n"

    # Add vulnerability detection task by level
    content += "\n## Vulnerability Detection Task - Performance by Code Level:\n\n"
    content += """**Metric explanations:**
- **Existence (%):** The percentage of times the model correctly detected whether a vulnerability exists
- **CWE Type (%):** The percentage of times the model correctly identified at least one CWE type
- **Functions (%):** The percentage of times the model correctly identified at least one vulnerable function
- **Lines (%):** The percentage of times the model correctly identified at least one vulnerable line

"""

    vuln_df = create_level_comparison_vuln(
        original_data,
        mutated_data,
        config["original_dataset_name"],
        config["mutated_dataset_name"],
    )
    content += dataframe_to_markdown_table(vuln_df) + "\n"

    return content


def main():
    parser = argparse.ArgumentParser(
        description="Generate LLM experiment analysis report"
    )
    parser.add_argument(
        "--config", type=str, help="Path to configuration file (optional)"
    )
    args = parser.parse_args()

    # Default configuration (can be overridden by config file)
    config = {
        # Model Information
        "model_provider": "openai",
        "model_name": "gpt-4o",
        # Dataset names
        "original_dataset_name": "18-vulns",
        "mutated_dataset_name": "fmt-unconventional",
        # Dataset paths
        "original_check_cve": "../variants/18-vulns/datasets/check_cve/openai-gpt-4o/2025-05-27T19-13-14.csv",
        "original_identify_crate": "../variants/18-vulns/datasets/identify_crate/openai-gpt-4o/2025-05-27T20-46-24.csv",
        "original_vuln": "../variants/18-vulns/datasets/vuln/openai-gpt-4o/2025-05-27T20-46-33.csv",
        "mutated_identify_crate": "../variants/fmt-unconventional/datasets/identify_crate/openai-gpt-4o/2025-05-27T21-19-56.csv",
        "mutated_vuln": "../variants/fmt-unconventional/datasets/vuln/openai-gpt-4o/2025-05-27T21-20-01.csv",
        # Mutation description
        "mutation_description": "Applied unconventional Rust formatting style to the entire workspace",
        # Sample counts
        "num_check_cve": 15,
        "num_identify_crate": 85,
        "num_vuln": 85,
    }

    # Load config file if provided
    if args.config:
        import json

        with open(args.config, "r") as f:
            config.update(json.load(f))

    # Generate report
    md_report = generate_report(config)

    # Create output directory if it doesn't exist
    os.makedirs("reports", exist_ok=True)

    # Save report
    output_filename = f"reports/{config['model_provider']}_{config['model_name']}_{config['original_dataset_name']}_vs_{config['mutated_dataset_name']}.md"
    with open(output_filename, "w") as f:
        f.write(md_report)

    print(f"Report generated successfully: {output_filename}")


if __name__ == "__main__":
    main()
