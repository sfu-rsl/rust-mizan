# RustMizan Multi-Experiment Comparison Report

## Experiment Overview

| Experiment Name | Experiment ID | Model | Mutations Applied |
|-----------------|---------------|-------|-------------------|
| gpt-on-vanilla | 7dd2c700 | gpt-4.1-2025-04-14 | None (vanilla) |
| claude-on-vanilla | 7f0feb42 | claude-3-7-sonnet-20250219 | None (vanilla) |

## Metric Definitions

- **JSON Validity Rate:** Percentage of samples with valid JSON responses that could be parsed
- **Vulnerability Detection Accuracy:** Percentage of samples where the model correctly identified whether code is vulnerable or not
- **At Least One Correct CWE:** Percentage of samples where the model identified at least one correct CWE type
- **At Least One Correct Function:** Percentage of samples where the model identified at least one correct vulnerable function
- **At Least One Correct Line:** Percentage of samples where the model identified at least one correct vulnerable line
- **Macro F1 Score:** Average F1 score across all samples for the given task (CWE/Functions/Lines)

## Overall Performance Comparison

| Experiment | JSON Validity | Vulnerability Accuracy | At Least One Correct CWE Identified | At Least One Correct Vulnerable Function Identified | At Least One Correct Vulnerable Line Identified |
|------------|---------------|------------------------|--------------------------------------|-----------------------------------------------------|--------------------------------------------------|
| gpt-on-vanilla | 80.9% | 45.7% | 6.9% (12 out of 173) | 13.9% (24 out of 173) | 15.6% (27 out of 173) |
| claude-on-vanilla | 89.6% | 53.2% | 9.2% (16 out of 173) | 18.5% (32 out of 173) | 17.9% (31 out of 173) |

### F1 Score Comparison

| Experiment | CWE Macro F1 | Functions Macro F1 | Lines Macro F1 |
|------------|--------------|-------------------|----------------|
| gpt-on-vanilla | 0.042 | 0.114 | 0.123 |
| claude-on-vanilla | 0.075 | 0.158 | 0.142 |

## Vulnerable Samples Only Comparison

*Performance metrics calculated only on samples that contain vulnerabilities*

| Experiment | JSON Validity | Vulnerability Accuracy | At Least One Correct CWE Identified | At Least One Correct Vulnerable Function Identified | At Least One Correct Vulnerable Line Identified |
|------------|---------------|------------------------|--------------------------------------|-----------------------------------------------------|--------------------------------------------------|
| gpt-on-vanilla | 82.1% | 35.8% | 12.6% (12 out of 95) | 25.3% (24 out of 95) | 28.4% (27 out of 95) |
| claude-on-vanilla | 92.6% | 88.4% | 16.8% (16 out of 95) | 33.7% (32 out of 95) | 32.6% (31 out of 95) |

### F1 Score Comparison (Vulnerable Samples Only)

| Experiment | CWE Macro F1 | Functions Macro F1 | Lines Macro F1 |
|------------|--------------|-------------------|----------------|
| gpt-on-vanilla | 0.076 | 0.205 | 0.221 |
| claude-on-vanilla | 0.133 | 0.279 | 0.250 |

## Analysis of Samples with Valid JSON Across All Experiments

*Analysis focused only on the 126 samples where all experiments produced valid JSON responses*

| Experiment | Vulnerability Accuracy | At Least One Correct CWE Identified | At Least One Correct Vulnerable Function Identified | At Least One Correct Vulnerable Line Identified |
|------------|------------------------|--------------------------------------|-----------------------------------------------------|--------------------------------------------------|
| gpt-on-vanilla | 55.6% | 9.5% (12 out of 126) | 19.0% (24 out of 126) | 21.4% (27 out of 126) |
| claude-on-vanilla | 60.3% | 10.3% (13 out of 126) | 24.6% (31 out of 126) | 22.2% (28 out of 126) |

### F1 Score Comparison (Valid JSON Samples)

| Experiment | CWE Macro F1 | Functions Macro F1 | Lines Macro F1 |
|------------|--------------|-------------------|----------------|
| gpt-on-vanilla | 0.047 | 0.127 | 0.137 |
| claude-on-vanilla | 0.071 | 0.187 | 0.157 |

### Vulnerable Samples Performance (Valid JSON Samples)

| Experiment | Vulnerability Accuracy | At Least One Correct CWE Identified | At Least One Correct Vulnerable Function Identified | At Least One Correct Vulnerable Line Identified |
|------------|------------------------|--------------------------------------|-----------------------------------------------------|--------------------------------------------------|
| gpt-on-vanilla | 46.6% | 16.4% (12 out of 73) | 32.9% (24 out of 73) | 37.0% (27 out of 73) |
| claude-on-vanilla | 94.5% | 17.8% (13 out of 73) | 42.5% (31 out of 73) | 38.4% (28 out of 73) |

## Figures

The following figures have been generated and saved to the `figures/` directory:

1. **figure1_overall_comparison.png** - Overall Performance Comparison: Simple Metrics
2. **figure2_f1_comparison.png** - F1 Score Comparison
3. **figure3_vulnerable_comparison.png** - Performance on Vulnerable Samples Only
4. **figure4_vulnerable_f1_comparison.png** - F1 Score Comparison (Vulnerable Samples Only)
5. **figure5_valid_json_comparison.png** - Performance on Samples with Valid JSON Across All Experiments
6. **figure6_valid_json_f1_comparison.png** - F1 Score Comparison on Samples with Valid JSON Across All Experiments
