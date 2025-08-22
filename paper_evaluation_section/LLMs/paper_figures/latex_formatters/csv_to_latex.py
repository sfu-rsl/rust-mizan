#!/usr/bin/env python3
import pandas as pd


def format_vanilla_performance_table():
    """Convert vanilla performance CSV to LaTeX format"""

    # Read the CSV file
    csv_path = "../tables/table_7_1_vanilla_performance.csv"
    df = pd.read_csv(csv_path)

    # Remove the Samples column as it's not in the LaTeX version
    if "Samples" in df.columns:
        df = df.drop("Samples", axis=1)

    # Define model order: GPT, Gemini, Claude, DeepSeek
    model_order = ["GPT-4.1", "Gemini 1.5 Pro", "Claude 3.7 Sonnet", "DeepSeek-V3.1"]

    # Reorder rows based on model order (models are rows, not columns)
    df_reordered = []
    for model in model_order:
        matching_rows = df[df["Model"] == model]
        if len(matching_rows) > 0:
            df_reordered.append(matching_rows)

    if df_reordered:
        df = pd.concat(df_reordered, ignore_index=True)

    # Column mapping for headers
    column_mapping = {"Binary Accuracy": "Binary Acc.", "Hit@1-Function": "Hit@1-Func"}

    # Create LaTeX table
    latex_content = []
    latex_content.append("\\begin{table}[htbp]")
    latex_content.append("\\centering")
    latex_content.append(
        "\\caption{Overall LLM performance on vanilla RustMizan dataset}"
    )
    latex_content.append("\\label{tab:vanilla_performance}")
    latex_content.append("\\begin{tabular}{lccccc}")
    latex_content.append("\\toprule")

    # Headers
    headers = []
    for col in df.columns:
        if col == "Model":
            headers.append("\\textbf{Model}")
        else:
            header_name = column_mapping.get(col, col)
            headers.append(f"\\textbf{{{header_name}}}")

    latex_content.append(" & ".join(headers) + " \\\\")
    latex_content.append("\\midrule")

    # Data rows
    for _, row in df.iterrows():
        row_data = []
        for col in df.columns:
            if col == "Model":
                row_data.append(str(row[col]))
            else:
                value = row[col]
                # Format numbers to 3 decimal places
                if isinstance(value, (int, float)):
                    row_data.append(f"{value:.3f}")
                else:
                    row_data.append(str(value))

        latex_content.append(" & ".join(row_data) + " \\\\")

    latex_content.append("\\bottomrule")
    latex_content.append("\\end{tabular}")
    latex_content.append("\\end{table}")

    # Write to LaTeX file
    output_path = "../latex/vanilla_performance_generated.tex"
    with open(output_path, "w") as f:
        f.write("\n".join(latex_content))

    print(f"LaTeX table generated: {output_path}")


def format_transformation_impact_table():
    """Convert transformation impact CSV to LaTeX format"""

    # Read the CSV file
    csv_path = "../tables/transformation_impact_table.csv"
    df = pd.read_csv(csv_path)

    # Define model order: GPT, Gemini, Claude, DeepSeek
    model_order = ["GPT-4.1", "Gemini 1.5 Pro", "Claude 3.7 Sonnet", "DeepSeek-V3.1"]
    model_columns = [col for col in model_order if col in df.columns]

    # Define transformation order and mapping
    transformation_order = [
        # Vanilla
        "vanilla (baseline)",
        # Formatting Mutations
        "Format Compact",
        "Format Expanded",
        # AST-based Mutations
        "For→While",
        "While→Loop",
        "If-Else Reorder",
        "Derive Reorder",
        "Trait Bound Reorder",
        "Use Reorder",
        "Arithmetic Identity",
        # Benign Insertions
        "Benign Comments",
        "Benign Blocks",
        # Benign rename
        "Benign Rename Fn",
        "Benign Rename Var",
        # Malignant Insertions
        "Malignant Comments",
        "Malignant Blocks",
        # Malignant rename
        "Malignant Rename Fn",
        "Malignant Rename Var",
    ]

    # Reorder dataframe based on transformation order
    df_ordered = []
    for transform in transformation_order:
        matching_rows = df[df["Transformation"] == transform]
        if len(matching_rows) > 0:
            df_ordered.append(matching_rows)
    df = pd.concat(df_ordered, ignore_index=True)

    # Find the most negative delta for each model to highlight in red
    def extract_delta(value_str):
        """Extract delta from format like '0.364 (-0.113)'"""
        if "(" in str(value_str) and ")" in str(value_str):
            delta_part = str(value_str).split("(")[1].split(")")[0]
            return float(delta_part)
        return 0.0

    # Find minimum delta for each model (most negative)
    min_deltas = {}
    for model in model_columns:
        deltas = []
        for _, row in df.iterrows():
            if row["Transformation"] != "vanilla (baseline)":  # Skip baseline row
                delta = extract_delta(row[model])
                deltas.append(delta)
        if deltas:
            min_deltas[model] = min(deltas)

    # Create LaTeX table
    latex_content = []
    latex_content.append("\\begin{table}[htbp]")
    latex_content.append("\\centering")
    latex_content.append(
        "\\caption{Transformation impact on model performance (Hit@1-Function metric)}"
    )
    latex_content.append("\\label{tab:transformation_impact}")
    latex_content.append("\\begin{tabular}{lcccc}")
    latex_content.append("\\toprule")

    # Headers
    headers = ["\\textbf{Transformation}"]
    for model in model_columns:
        headers.append(f"\\textbf{{{model}}}")
    latex_content.append(" & ".join(headers) + " \\\\")
    latex_content.append("\\midrule")

    # Data rows with grouping
    current_group = None
    for _, row in df.iterrows():
        transform_name = row["Transformation"]

        # Determine group for spacing
        if transform_name == "vanilla (baseline)":
            new_group = "vanilla"
        elif transform_name in ["Format Compact", "Format Expanded"]:
            new_group = "formatting"
        elif transform_name in [
            "For→While",
            "While→Loop",
            "If-Else Reorder",
            "Derive Reorder",
            "Trait Bound Reorder",
            "Use Reorder",
            "Arithmetic Identity",
        ]:
            new_group = "ast"
        elif transform_name in ["Benign Comments", "Benign Blocks"]:
            new_group = "benign_insert"
        elif transform_name in ["Benign Rename Fn", "Benign Rename Var"]:
            new_group = "benign_rename"
        elif transform_name in ["Malignant Comments", "Malignant Blocks"]:
            new_group = "malignant_insert"
        elif transform_name in ["Malignant Rename Fn", "Malignant Rename Var"]:
            new_group = "malignant_rename"
        else:
            new_group = "other"

        # Add spacing between groups
        if current_group is not None and current_group != new_group:
            if (
                (current_group == "vanilla" and new_group == "formatting")
                or (current_group == "ast" and new_group == "benign_insert")
                or (
                    current_group == "benign_rename" and new_group == "malignant_insert"
                )
            ):
                # More aggressive separator
                latex_content.append("\\addlinespace[4pt]")
            elif (
                (current_group == "formatting" and new_group == "ast")
                or (current_group == "benign_insert" and new_group == "benign_rename")
                or (
                    current_group == "malignant_insert"
                    and new_group == "malignant_rename"
                )
            ):
                # Small subtle separator
                latex_content.append("\\addlinespace[2pt]")

        current_group = new_group
        row_data = [transform_name]

        for model in model_columns:
            value_str = str(row[model])

            # Check if this is the most negative delta for this model
            if row["Transformation"] != "vanilla (baseline)":
                delta = extract_delta(value_str)
                if delta == min_deltas.get(model, 0) and delta < 0:
                    # Color the most negative delta red
                    value_str = f"\\textcolor{{red}}{{{value_str}}}"

            row_data.append(value_str)

        latex_content.append(" & ".join(row_data) + " \\\\")

    latex_content.append("\\bottomrule")
    latex_content.append("\\end{tabular}")
    latex_content.append("\\end{table}")

    # Write to LaTeX file
    output_path = "../latex/transformation_impact_generated.tex"
    with open(output_path, "w") as f:
        f.write("\n".join(latex_content))

    print(f"LaTeX table generated: {output_path}")


if __name__ == "__main__":
    format_vanilla_performance_table()
    format_transformation_impact_table()
