# Paper Figures and Tables

## Usage

```bash
# Clean previous processed experiment results and figures
find ../evaluation_results -name "processed_results.csv" -delete;
rm -rf tables/ figures/;
rm -rf latex_formatters/latex_tables/;

# Generate vulnerability mapping (run once or when mizan.json changes)
python common/vulnerability_utils.py;

# Generate tables and figures
python scripts/process_experiments.py;
python scripts/generate_vanilla_analysis.py;
python scripts/generate_granularity_analysis.py;
python scripts/generate_compact_hit_at_1_table.py;
python scripts/generate_transformation_analysis.py;

# After generating tables with the scripts above, convert them to LaTeX format:
cd latex_formatters/
python csv_to_latex.py
```

## LaTeX Formatting Notes

### Transformation Impact Table

- First row shows vanilla baseline values
- All other rows show actual score and delta: "0.400 (+0.000)"
- The value with the most negative delta should be colored red in LaTeX

## Statistical Decisions

- F1 Score Edge Cases:
  - No predictions and no actual: return 1.0 (perfect score)
  - No predictions made: return 0.0 (zero recall)
  - No actual positives but predictions made: return 0.0 if false positives exist, else 1.0
  - Standard case: 2 _ precision _ recall / (precision + recall)
- TP/FP/FN Computation:
  - TP: intersection of predicted and actual sets
  - FP: predicted items not in actual set
  - FN: actual items not in predicted set
  - Uses set operations for exact matching (todo: allow partial matches for functions, allow "regions" for lines)
- F1 Score Aggregation:
  - macro-aggregation: Sum TP/FP/FN across all samples, then compute F1 (we are using this)
  - micro-aggregation: Average per-sample F1 scores
- Hit@1-Function Logic:
  - Only applies to vulnerable samples (is_vulnerable = True)
  - Non-vulnerable samples get 0.0 score
  - Returns 1.0 if any function correctly identified (tp > 0), else 0.0
  - Sample-level metric, not entity-level
- Entity Identification:
  - Functions: identified by (file_path, function_name) tuples
  - Lines: identified by (file_path, line_number) tuples
  - CWE: identified by string values
  - Exact matching required for all entities
- Fair Comparison:
  - Only uses samples common to all experiments being compared
  - Only includes samples with valid JSON responses
  - Maintains same evaluation set across all models
