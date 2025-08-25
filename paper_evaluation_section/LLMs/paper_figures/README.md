# Paper Figures and Tables

This directory contains the code for generating all figures and tables used in the RustMizan paper. The evaluation follows a principled micro-averaging approach designed specifically for vulnerability detection tasks.

## Usage

```bash
# Clean previous processed experiment results
find ../evaluation_results -name "processed_results.csv" -delete;
rm -f latex/*_generated.tex;


# Generate vulnerability mapping (run once or when mizan.json changes)
python common/vulnerability_utils.py;

# Generate LaTeX tables and figures
python scripts/process_experiments.py;
python scripts/generate_vanilla_analysis.py;
python scripts/generate_granularity_analysis.py;
python scripts/generate_compact_success_at_1_table.py;
python scripts/generate_transformation_analysis.py;


# For the dataset statistics section
python scripts/generate_dataset_summary.py;
python scripts/generate_cwe_table.py;
python scripts/generate_loc_boxplot.py;
```

## Scoring

## Sample-Level Metrics

### 1. Binary Vulnerability Detection

Task: Classify code as vulnerable or non-vulnerable (`is_vulnerable: true/false`)

Metric: Standard binary accuracy

### 2. CWE Type Classification

Task: Identify vulnerability types (`cwe_type: ["CWE-XXX", ...]`)

Approach: Multi-label set-based classification

- Predicted CWEs: `{CWE-416, CWE-119}`
- Actual CWEs: `{CWE-416, CWE-787}`
- TP = |intersection| = 1 (CWE-416)
- FP = |predicted - actual| = 1 (CWE-119)
- FN = |actual - predicted| = 1 (CWE-787)

### 3. Function Localization

Task: Identify vulnerable functions (`vulnerable_functions: {file: [signatures]}`)

Approach: Set-based matching with (file, signature) tuples

### 4. Line Localization

Task: Identify vulnerable lines (`vulnerable_lines: {file: [line_numbers]}`)

Approach: Set-based matching with (file, line) tuples

### 5. Success@1 Metrics

Task: Did the model find at least one correct element?

Application: Only computed for vulnerable samples since non-vulnerable samples have no "vulnerable" elements.

- Success@1-Function: 1.0 if function_tp > 0, else 0.0
- Success@1-Line: 1.0 if line_tp > 0, else 0.0

## Experiment-Level Aggregation

We use micro-averaging by summing TP/FP/FN across all samples before computing aggregate metrics:

```python
tp_total = sum(sample_tp for all samples)
fp_total = sum(sample_fp for all samples)
fn_total = sum(sample_fn for all samples)

aggregate_f1 = 2 * precision * recall / (precision + recall)
aggregate_precision = tp_total / (tp_total + fp_total)
aggregate_recall = tp_total / (tp_total + fn_total)
```

This is because complex vulnerabilities (affecting more functions/lines) naturally contribute more to the score. This aligns with our domain goals and reflects real-world complexity where some vulnerabilities are inherently more complex.

> To ensure fair comparison when comparing across multiple experiments (models sometimes fail to produce valid JSON for some samples due to context length limits or other issues), we only include samples where all models produced valid JSON outputs. This ensures that the comparison is based on a common set of samples.
