# RustMizan Multi-Experiment Comparison Report

## Experiment Overview

| Experiment Name | Experiment ID | Model | Mutations Applied |
|-----------------|---------------|-------|-------------------|
| vanilla | 7f0feb42 | claude-3-7-sonnet-20250219 | None (vanilla) |
| format-expanded | d13af563 | claude-3-7-sonnet-20250219 | format-expanded |
| format-compact | ac17676d | claude-3-7-sonnet-20250219 | format-compact |
| mizan-mut-arithmetic-identity | bcf47760 | claude-3-7-sonnet-20250219 | mizan-mut-arithmetic-identity |
| mizan-mut-derive-reorder | d1a77f67 | claude-3-7-sonnet-20250219 | mizan-mut-derive-reorder |
| mizan-mut-for-to-while | f3669e76 | claude-3-7-sonnet-20250219 | mizan-mut-for-to-while |
| mizan-mut-if-else-reorder | b9c36e1c | claude-3-7-sonnet-20250219 | mizan-mut-if-else-reorder |
| mizan-mut-trait-bound-reorder | 4bdcd379 | claude-3-7-sonnet-20250219 | mizan-mut-trait-bound-reorder |
| mizan-mut-use-reorder | 1ff7f149 | claude-3-7-sonnet-20250219 | mizan-mut-use-reorder |
| mizan-mut-while-to-loop | ce6b642d | claude-3-7-sonnet-20250219 | mizan-mut-while-to-loop |
| benign-blocks | f01cb636 | claude-3-7-sonnet-20250219 | benign-blocks |
| benign-comments | a6f63139 | claude-3-7-sonnet-20250219 | benign-comments |
| benign-rename-fn | df68be8f | claude-3-7-sonnet-20250219 | benign-rename-fn |
| benign-rename-var | f6468fc8 | claude-3-7-sonnet-20250219 | benign-rename-var |
| malignant-blocks | c0080ab6 | claude-3-7-sonnet-20250219 | malignant-blocks |
| malignant-comments | 451a166f | claude-3-7-sonnet-20250219 | malignant-comments |
| malignant-rename-fn | 53589144 | claude-3-7-sonnet-20250219 | malignant-rename-fn |
| malignant-rename-var | 4a0c2d1c | claude-3-7-sonnet-20250219 | malignant-rename-var |

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
| vanilla | 89.6% | 53.2% | 9.2% (16 out of 173) | 18.5% (32 out of 173) | 17.9% (31 out of 173) |
| format-expanded | 85.5% | 49.1% | 8.1% (14 out of 173) | 18.5% (32 out of 173) | 21.4% (37 out of 173) |
| format-compact | 93.6% | 55.5% | 5.8% (10 out of 173) | 18.5% (32 out of 173) | 17.3% (30 out of 173) |
| mizan-mut-arithmetic-identity | 96.5% | 56.6% | 7.5% (13 out of 173) | 19.1% (33 out of 173) | 18.5% (32 out of 173) |
| mizan-mut-derive-reorder | 95.4% | 55.5% | 5.8% (10 out of 173) | 21.4% (37 out of 173) | 19.1% (33 out of 173) |
| mizan-mut-for-to-while | 94.8% | 54.9% | 10.4% (18 out of 173) | 19.1% (33 out of 173) | 19.7% (34 out of 173) |
| mizan-mut-if-else-reorder | 94.8% | 56.6% | 8.1% (14 out of 173) | 20.2% (35 out of 173) | 19.1% (33 out of 173) |
| mizan-mut-trait-bound-reorder | 96.5% | 56.1% | 8.7% (15 out of 173) | 19.1% (33 out of 173) | 17.3% (30 out of 173) |
| mizan-mut-use-reorder | 96.0% | 56.1% | 7.5% (13 out of 173) | 19.7% (34 out of 173) | 20.2% (35 out of 173) |
| mizan-mut-while-to-loop | 96.0% | 56.6% | 6.9% (12 out of 173) | 20.2% (35 out of 173) | 18.5% (32 out of 173) |
| benign-blocks | 91.3% | 53.8% | 11.6% (20 out of 173) | 25.4% (44 out of 173) | 26.0% (45 out of 173) |
| benign-comments | 90.2% | 54.9% | 11.6% (20 out of 173) | 19.7% (34 out of 173) | 20.8% (36 out of 173) |
| benign-rename-fn | 91.9% | 56.1% | 6.9% (12 out of 173) | 18.5% (32 out of 173) | 20.2% (35 out of 173) |
| benign-rename-var | 89.6% | 56.6% | 11.6% (20 out of 173) | 21.4% (37 out of 173) | 20.8% (36 out of 173) |
| malignant-blocks | 91.9% | 57.8% | 9.8% (17 out of 173) | 10.4% (18 out of 173) | 10.4% (18 out of 173) |
| malignant-comments | 90.8% | 48.0% | 7.5% (13 out of 173) | 16.8% (29 out of 173) | 18.5% (32 out of 173) |
| malignant-rename-fn | 90.2% | 53.8% | 8.7% (15 out of 173) | 18.5% (32 out of 173) | 18.5% (32 out of 173) |
| malignant-rename-var | 91.9% | 56.1% | 11.0% (19 out of 173) | 21.4% (37 out of 173) | 23.1% (40 out of 173) |

### F1 Score Comparison

| Experiment | CWE Macro F1 | Functions Macro F1 | Lines Macro F1 |
|------------|--------------|-------------------|----------------|
| vanilla | 0.075 | 0.158 | 0.142 |
| format-expanded | 0.071 | 0.176 | 0.176 |
| format-compact | 0.038 | 0.154 | 0.123 |
| mizan-mut-arithmetic-identity | 0.053 | 0.151 | 0.136 |
| mizan-mut-derive-reorder | 0.041 | 0.170 | 0.132 |
| mizan-mut-for-to-while | 0.081 | 0.156 | 0.129 |
| mizan-mut-if-else-reorder | 0.062 | 0.162 | 0.138 |
| mizan-mut-trait-bound-reorder | 0.063 | 0.145 | 0.123 |
| mizan-mut-use-reorder | 0.056 | 0.151 | 0.136 |
| mizan-mut-while-to-loop | 0.054 | 0.159 | 0.128 |
| benign-blocks | 0.086 | 0.225 | 0.197 |
| benign-comments | 0.087 | 0.176 | 0.189 |
| benign-rename-fn | 0.060 | 0.154 | 0.153 |
| benign-rename-var | 0.100 | 0.179 | 0.150 |
| malignant-blocks | 0.060 | 0.071 | 0.038 |
| malignant-comments | 0.060 | 0.142 | 0.141 |
| malignant-rename-fn | 0.068 | 0.158 | 0.140 |
| malignant-rename-var | 0.091 | 0.187 | 0.179 |

## Vulnerable Samples Only Comparison

*Performance metrics calculated only on samples that contain vulnerabilities*

| Experiment | JSON Validity | Vulnerability Accuracy | At Least One Correct CWE Identified | At Least One Correct Vulnerable Function Identified | At Least One Correct Vulnerable Line Identified |
|------------|---------------|------------------------|--------------------------------------|-----------------------------------------------------|--------------------------------------------------|
| vanilla | 92.6% | 88.4% | 16.8% (16 out of 95) | 33.7% (32 out of 95) | 32.6% (31 out of 95) |
| format-expanded | 85.3% | 81.1% | 14.7% (14 out of 95) | 33.7% (32 out of 95) | 38.9% (37 out of 95) |
| format-compact | 95.8% | 89.5% | 10.5% (10 out of 95) | 33.7% (32 out of 95) | 31.6% (30 out of 95) |
| mizan-mut-arithmetic-identity | 96.8% | 93.7% | 13.7% (13 out of 95) | 34.7% (33 out of 95) | 33.7% (32 out of 95) |
| mizan-mut-derive-reorder | 96.8% | 93.7% | 10.5% (10 out of 95) | 38.9% (37 out of 95) | 34.7% (33 out of 95) |
| mizan-mut-for-to-while | 94.7% | 90.5% | 18.9% (18 out of 95) | 34.7% (33 out of 95) | 35.8% (34 out of 95) |
| mizan-mut-if-else-reorder | 96.8% | 92.6% | 14.7% (14 out of 95) | 36.8% (35 out of 95) | 34.7% (33 out of 95) |
| mizan-mut-trait-bound-reorder | 96.8% | 92.6% | 15.8% (15 out of 95) | 34.7% (33 out of 95) | 31.6% (30 out of 95) |
| mizan-mut-use-reorder | 96.8% | 93.7% | 13.7% (13 out of 95) | 35.8% (34 out of 95) | 36.8% (35 out of 95) |
| mizan-mut-while-to-loop | 95.8% | 92.6% | 12.6% (12 out of 95) | 36.8% (35 out of 95) | 33.7% (32 out of 95) |
| benign-blocks | 94.7% | 90.5% | 21.1% (20 out of 95) | 46.3% (44 out of 95) | 47.4% (45 out of 95) |
| benign-comments | 94.7% | 89.5% | 21.1% (20 out of 95) | 35.8% (34 out of 95) | 37.9% (36 out of 95) |
| benign-rename-fn | 95.8% | 91.6% | 12.6% (12 out of 95) | 33.7% (32 out of 95) | 36.8% (35 out of 95) |
| benign-rename-var | 94.7% | 92.6% | 21.1% (20 out of 95) | 38.9% (37 out of 95) | 37.9% (36 out of 95) |
| malignant-blocks | 96.8% | 94.7% | 17.9% (17 out of 95) | 18.9% (18 out of 95) | 18.9% (18 out of 95) |
| malignant-comments | 94.7% | 78.9% | 13.7% (13 out of 95) | 30.5% (29 out of 95) | 33.7% (32 out of 95) |
| malignant-rename-fn | 93.7% | 89.5% | 15.8% (15 out of 95) | 33.7% (32 out of 95) | 33.7% (32 out of 95) |
| malignant-rename-var | 95.8% | 92.6% | 20.0% (19 out of 95) | 38.9% (37 out of 95) | 42.1% (40 out of 95) |

### F1 Score Comparison (Vulnerable Samples Only)

| Experiment | CWE Macro F1 | Functions Macro F1 | Lines Macro F1 |
|------------|--------------|-------------------|----------------|
| vanilla | 0.133 | 0.279 | 0.250 |
| format-expanded | 0.130 | 0.321 | 0.321 |
| format-compact | 0.068 | 0.275 | 0.219 |
| mizan-mut-arithmetic-identity | 0.097 | 0.274 | 0.247 |
| mizan-mut-derive-reorder | 0.073 | 0.305 | 0.237 |
| mizan-mut-for-to-while | 0.148 | 0.284 | 0.236 |
| mizan-mut-if-else-reorder | 0.111 | 0.289 | 0.246 |
| mizan-mut-trait-bound-reorder | 0.114 | 0.263 | 0.224 |
| mizan-mut-use-reorder | 0.101 | 0.272 | 0.246 |
| mizan-mut-while-to-loop | 0.098 | 0.290 | 0.234 |
| benign-blocks | 0.152 | 0.394 | 0.347 |
| benign-comments | 0.150 | 0.305 | 0.327 |
| benign-rename-fn | 0.104 | 0.269 | 0.267 |
| benign-rename-var | 0.172 | 0.309 | 0.258 |
| malignant-blocks | 0.103 | 0.122 | 0.065 |
| malignant-comments | 0.105 | 0.248 | 0.246 |
| malignant-rename-fn | 0.120 | 0.276 | 0.246 |
| malignant-rename-var | 0.159 | 0.327 | 0.313 |

## Analysis of Samples with Valid JSON Across All Experiments

*Analysis focused only on the 137 samples where all experiments produced valid JSON responses*

| Experiment | Vulnerability Accuracy | At Least One Correct CWE Identified | At Least One Correct Vulnerable Function Identified | At Least One Correct Vulnerable Line Identified |
|------------|------------------------|--------------------------------------|-----------------------------------------------------|--------------------------------------------------|
| vanilla | 57.7% | 10.9% (15 out of 137) | 22.6% (31 out of 137) | 21.9% (30 out of 137) |
| format-expanded | 58.4% | 9.5% (13 out of 137) | 22.6% (31 out of 137) | 25.5% (35 out of 137) |
| format-compact | 58.4% | 6.6% (9 out of 137) | 21.2% (29 out of 137) | 20.4% (28 out of 137) |
| mizan-mut-arithmetic-identity | 59.1% | 8.0% (11 out of 137) | 21.9% (30 out of 137) | 21.2% (29 out of 137) |
| mizan-mut-derive-reorder | 58.4% | 7.3% (10 out of 137) | 24.8% (34 out of 137) | 21.9% (30 out of 137) |
| mizan-mut-for-to-while | 59.1% | 11.7% (16 out of 137) | 22.6% (31 out of 137) | 23.4% (32 out of 137) |
| mizan-mut-if-else-reorder | 59.9% | 8.8% (12 out of 137) | 24.1% (33 out of 137) | 23.4% (32 out of 137) |
| mizan-mut-trait-bound-reorder | 59.1% | 10.9% (15 out of 137) | 21.9% (30 out of 137) | 19.7% (27 out of 137) |
| mizan-mut-use-reorder | 59.1% | 8.8% (12 out of 137) | 22.6% (31 out of 137) | 23.4% (32 out of 137) |
| mizan-mut-while-to-loop | 60.6% | 8.0% (11 out of 137) | 23.4% (32 out of 137) | 21.2% (29 out of 137) |
| benign-blocks | 57.7% | 10.9% (15 out of 137) | 28.5% (39 out of 137) | 27.7% (38 out of 137) |
| benign-comments | 59.1% | 12.4% (17 out of 137) | 22.6% (31 out of 137) | 23.4% (32 out of 137) |
| benign-rename-fn | 58.4% | 7.3% (10 out of 137) | 21.9% (30 out of 137) | 23.4% (32 out of 137) |
| benign-rename-var | 61.3% | 13.9% (19 out of 137) | 25.5% (35 out of 137) | 24.8% (34 out of 137) |
| malignant-blocks | 59.1% | 10.9% (15 out of 137) | 11.7% (16 out of 137) | 11.7% (16 out of 137) |
| malignant-comments | 54.0% | 8.8% (12 out of 137) | 19.0% (26 out of 137) | 19.7% (27 out of 137) |
| malignant-rename-fn | 56.9% | 9.5% (13 out of 137) | 22.6% (31 out of 137) | 21.9% (30 out of 137) |
| malignant-rename-var | 58.4% | 12.4% (17 out of 137) | 24.8% (34 out of 137) | 26.3% (36 out of 137) |

### F1 Score Comparison (Valid JSON Samples)

| Experiment | CWE Macro F1 | Functions Macro F1 | Lines Macro F1 |
|------------|--------------|-------------------|----------------|
| vanilla | 0.078 | 0.174 | 0.157 |
| format-expanded | 0.069 | 0.183 | 0.185 |
| format-compact | 0.040 | 0.163 | 0.131 |
| mizan-mut-arithmetic-identity | 0.055 | 0.162 | 0.149 |
| mizan-mut-derive-reorder | 0.049 | 0.185 | 0.145 |
| mizan-mut-for-to-while | 0.083 | 0.174 | 0.147 |
| mizan-mut-if-else-reorder | 0.060 | 0.180 | 0.160 |
| mizan-mut-trait-bound-reorder | 0.077 | 0.157 | 0.138 |
| mizan-mut-use-reorder | 0.061 | 0.163 | 0.151 |
| mizan-mut-while-to-loop | 0.058 | 0.173 | 0.143 |
| benign-blocks | 0.071 | 0.229 | 0.191 |
| benign-comments | 0.079 | 0.181 | 0.191 |
| benign-rename-fn | 0.057 | 0.166 | 0.163 |
| benign-rename-var | 0.106 | 0.192 | 0.163 |
| malignant-blocks | 0.061 | 0.068 | 0.036 |
| malignant-comments | 0.062 | 0.146 | 0.140 |
| malignant-rename-fn | 0.068 | 0.175 | 0.150 |
| malignant-rename-var | 0.096 | 0.196 | 0.185 |

### Vulnerable Samples Performance (Valid JSON Samples)

| Experiment | Vulnerability Accuracy | At Least One Correct CWE Identified | At Least One Correct Vulnerable Function Identified | At Least One Correct Vulnerable Line Identified |
|------------|------------------------|--------------------------------------|-----------------------------------------------------|--------------------------------------------------|
| vanilla | 96.0% | 20.0% (15 out of 75) | 41.3% (31 out of 75) | 40.0% (30 out of 75) |
| format-expanded | 96.0% | 17.3% (13 out of 75) | 41.3% (31 out of 75) | 46.7% (35 out of 75) |
| format-compact | 92.0% | 12.0% (9 out of 75) | 38.7% (29 out of 75) | 37.3% (28 out of 75) |
| mizan-mut-arithmetic-identity | 97.3% | 14.7% (11 out of 75) | 40.0% (30 out of 75) | 38.7% (29 out of 75) |
| mizan-mut-derive-reorder | 97.3% | 13.3% (10 out of 75) | 45.3% (34 out of 75) | 40.0% (30 out of 75) |
| mizan-mut-for-to-while | 96.0% | 21.3% (16 out of 75) | 41.3% (31 out of 75) | 42.7% (32 out of 75) |
| mizan-mut-if-else-reorder | 96.0% | 16.0% (12 out of 75) | 44.0% (33 out of 75) | 42.7% (32 out of 75) |
| mizan-mut-trait-bound-reorder | 96.0% | 20.0% (15 out of 75) | 40.0% (30 out of 75) | 36.0% (27 out of 75) |
| mizan-mut-use-reorder | 97.3% | 16.0% (12 out of 75) | 41.3% (31 out of 75) | 42.7% (32 out of 75) |
| mizan-mut-while-to-loop | 97.3% | 14.7% (11 out of 75) | 42.7% (32 out of 75) | 38.7% (29 out of 75) |
| benign-blocks | 96.0% | 20.0% (15 out of 75) | 52.0% (39 out of 75) | 50.7% (38 out of 75) |
| benign-comments | 96.0% | 22.7% (17 out of 75) | 41.3% (31 out of 75) | 42.7% (32 out of 75) |
| benign-rename-fn | 96.0% | 13.3% (10 out of 75) | 40.0% (30 out of 75) | 42.7% (32 out of 75) |
| benign-rename-var | 100.0% | 25.3% (19 out of 75) | 46.7% (35 out of 75) | 45.3% (34 out of 75) |
| malignant-blocks | 97.3% | 20.0% (15 out of 75) | 21.3% (16 out of 75) | 21.3% (16 out of 75) |
| malignant-comments | 89.3% | 16.0% (12 out of 75) | 34.7% (26 out of 75) | 36.0% (27 out of 75) |
| malignant-rename-fn | 96.0% | 17.3% (13 out of 75) | 41.3% (31 out of 75) | 40.0% (30 out of 75) |
| malignant-rename-var | 97.3% | 22.7% (17 out of 75) | 45.3% (34 out of 75) | 48.0% (36 out of 75) |

## Figures

The following figures have been generated and saved to the `figures/` directory:

1. **figure1_overall_comparison.png** - Overall Performance Comparison: Simple Metrics
2. **figure2_f1_comparison.png** - F1 Score Comparison
3. **figure3_vulnerable_comparison.png** - Performance on Vulnerable Samples Only
4. **figure4_vulnerable_f1_comparison.png** - F1 Score Comparison (Vulnerable Samples Only)
5. **figure5_valid_json_comparison.png** - Performance on Samples with Valid JSON Across All Experiments
6. **figure6_valid_json_f1_comparison.png** - F1 Score Comparison on Samples with Valid JSON Across All Experiments
