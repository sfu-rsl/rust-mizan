import sys
import json
import pandas as pd
from pathlib import Path
from typing import Dict, List, Any

sys.path.append(str(Path(__file__).parent.parent))

from common.data_utils import get_vanilla_experiment_ids
from common.metrics import compute_experiment_metrics


def load_vulnerability_mapping() -> Dict[str, Dict[str, str]]:
    """Load the vulnerability mapping JSON file."""
    current_dir = Path(__file__).parent.parent
    mapping_path = current_dir / "data" / "vulnerability_mapping.json"
    
    if not mapping_path.exists():
        raise FileNotFoundError(f"Vulnerability mapping not found: {mapping_path}")
    
    with open(mapping_path, 'r', encoding='utf-8') as f:
        return json.load(f)


def get_model_short_names() -> Dict[str, str]:
    """Map full model names to short names for display."""
    return {
        "claude-3-7-sonnet": "Claude 3.7",
        "gpt-4.1": "GPT-4.1", 
        "gemini-1.5-pro": "Gemini 1.5 Pro",
        "deepseek-chat": "DeepSeek"
    }


def format_cwe_for_latex(cwe: str, description: str) -> str:
    """Format CWE information for LaTeX display - just return CWE ID."""
    if cwe == "Unknown":
        return "Unknown"
    return cwe


def get_hit_status(df: pd.DataFrame, vuln_id: str, granularity: str, model: str) -> str:
    """Get hit status for a specific vulnerability, granularity, and model."""
    
    # Map display model names to actual model names in data
    model_mapping = {
        "Claude 3.7 Sonnet": "Claude 3.7 Sonnet",
        "GPT-4.1": "GPT-4.1", 
        "Gemini 1.5 Pro": "Gemini 1.5 Pro",
        "DeepSeek-V3.1": "DeepSeek-V3.1"
    }
    
    actual_model_name = model_mapping.get(model, model)
    
    # Filter for the specific combination
    filtered = df[
        (df["vuln_id"] == vuln_id) & 
        (df["granularity"] == granularity) & 
        (df["model"] == actual_model_name) &
        (df["is_vulnerable_gt"] == True)
    ]
    
    if filtered.empty:
        # No sample exists for this combination
        return "\\cellna"
    
    # Check if any sample was correctly identified (hit)
    has_hit = (filtered["function_tp"] > 0).any()
    return "\\cellyes" if has_hit else "\\cellno"


def generate_table_rows(experiment_data: Dict[str, pd.DataFrame], vulnerability_mapping: Dict) -> str:
    """Generate LaTeX table rows from experiment data."""
    
    # Get actual model names from the data
    available_models = list(experiment_data.keys())
    print(f"Available models in data: {available_models}")
    
    granularities = ["crate", "file", "function"]
    
    # Combine all data for analysis
    combined_df = pd.concat([df for df in experiment_data.values()], ignore_index=True)
    
    # Get unique vulnerability IDs that have data
    available_vulns = sorted(combined_df["vuln_id"].unique())
    
    table_rows = []
    
    for vuln_id in available_vulns:
        if vuln_id not in vulnerability_mapping:
            continue
            
        vuln_info = vulnerability_mapping[vuln_id]
        cve_id = vuln_info["cve_id"]
        cwe_info = format_cwe_for_latex(vuln_info["cwe"], vuln_info["description"])
        
        # Build the row
        row_parts = [f"% Row for {vuln_id}"]
        row_parts.append(f"& \\vulnId{{{cve_id}}} & {cwe_info} & ")
        
        # Add results for each model across granularities
        model_results = []
        for model in available_models:
            granularity_results = []
            for granularity in granularities:
                status = get_hit_status(combined_df, vuln_id, granularity, model)
                granularity_results.append(status)
            model_results.append(" & ".join(granularity_results))
        
        row_parts.append("& " + " \n& ".join(model_results) + " \\\\")
        row_parts.append("\\hline")
        
        table_rows.append("\n".join(row_parts))
    
    return "\n".join(table_rows)


def generate_compact_table() -> None:
    """Generate the compact hit@1 table with real data."""
    
    # Load data
    print("Loading experiment data...")
    vanilla_experiments = get_vanilla_experiment_ids()
    print(f"Vanilla experiments: {vanilla_experiments}")
    experiment_ids = list(vanilla_experiments.values())
    model_names = list(vanilla_experiments.keys())
    
    # Get common samples and compute metrics for all experiments
    experiment_data = compute_experiment_metrics(experiment_ids, model_names)
    print(f"Loaded data for models: {list(experiment_data.keys())}")
    
    # Debug: Check what's in each dataframe
    for model, df in experiment_data.items():
        print(f"Model {model}: {len(df)} rows, unique vulns: {df['vuln_id'].nunique() if 'vuln_id' in df.columns else 'no vuln_id column'}")
        if len(df) > 0:
            print(f"  Sample columns: {list(df.columns)[:10]}")
            print(f"  Sample unique models in data: {df['model'].unique() if 'model' in df.columns else 'no model column'}")
    
    print("Loading vulnerability mapping...")
    vulnerability_mapping = load_vulnerability_mapping()
    
    print("Generating table rows...")
    table_rows = generate_table_rows(experiment_data, vulnerability_mapping)
    
    # Load template
    current_dir = Path(__file__).parent.parent
    template_path = current_dir / "latex_formatters" / "TEMPLATE_hit_at_1.tex"
    
    with open(template_path, 'r', encoding='utf-8') as f:
        template_content = f.read()
    
    # Replace placeholder with actual data
    final_content = template_content.replace("{TABLE_ROWS}", table_rows)
    
    # Save to latex directory
    latex_dir = current_dir / "latex"
    latex_dir.mkdir(exist_ok=True)
    output_path = latex_dir / "hit_at_1_compact_generated.tex"
    
    with open(output_path, 'w', encoding='utf-8') as f:
        f.write(final_content)
    
    print(f"Compact table generated: {output_path}")
    
    # Count vulnerabilities processed
    vuln_count = len([line for line in table_rows.split('\n') if line.strip().startswith('% Row for')])
    print(f"Processed {vuln_count} vulnerabilities")
    
    # Debug: show sample of table rows
    print("\nSample of generated rows:")
    sample_lines = table_rows.split('\n')[:20]
    for line in sample_lines:
        if line.strip():
            print(f"  {line}")


if __name__ == "__main__":
    generate_compact_table()