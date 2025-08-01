# RustMizan Analysis Scripts

This directory contains Python scripts for analyzing RustMizan benchmark results.

## Scripts

### 1. Mizan dataset Analyzer (`dataset_analyzer.py`)

```bash
python dataset_analyzer.py
```

### 2. Single Experiment Analyzer (`single_experiment_analyzer.py`)

```bash
python single_experiment_analyzer.py <experiment_id>
```

### 3. Multi-Experiment Comparison Analyzer (`multi_experiment_analyzer.py`)

```bash
python multi_experiment_analyzer.py <output_directory_name> <name1:experiment_id1> <name2:experiment_id2> [name3:experiment_id3] ...
```

Examples:

```bash
# Compare vanilla vs mutated dataset
python multi_experiment_analyzer.py vanilla_vs_mutated vanilla:5edf96b1 benign_mutations:90f048fd

# Compare multiple model performances
python multi_experiment_analyzer.py model_comparison gpt4:exp1 claude:exp2 llama:exp3

# Compare different mutation strategies
python multi_experiment_analyzer.py mutation_analysis baseline:abc123 light_mutations:def456 heavy_mutations:ghi789
```

## Notes

- Both scripts must be run from the `evaluation/analysis/` directory
- Experiment IDs should be provided WITHOUT the 'experiment\_' prefix
- Multi-experiment analyzer can handle 18+ experiments but visualizations may become crowded
- For large numbers of experiments, consider splitting into smaller comparison groups
