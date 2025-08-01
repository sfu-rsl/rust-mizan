# RustMizan Multi-Experiment Comparison Report

## Experiment Overview

| Experiment Name | Experiment ID | Model | Mutations Applied |
|-----------------|---------------|-------|-------------------|
| format-expanded | d5b8be06 | gpt-4.1-2025-04-14 | format-expanded |
| format-compact | 7b480732 | gpt-4.1-2025-04-14 | format-compact |
| mizan-mut-arithmetic-identity | daa4c3ef | gpt-4.1-2025-04-14 | mizan-mut-arithmetic-identity |
| mizan-mut-derive-reorder | b858dcb6 | gpt-4.1-2025-04-14 | mizan-mut-derive-reorder |
| mizan-mut-for-to-while | b00c9c15 | gpt-4.1-2025-04-14 | mizan-mut-for-to-while |
| mizan-mut-if-else-reorder | 14c3df9e | gpt-4.1-2025-04-14 | mizan-mut-if-else-reorder |
| mizan-mut-trait-bound-reorder | 9f52a84a | gpt-4.1-2025-04-14 | mizan-mut-trait-bound-reorder |
| mizan-mut-use-reorder | 2d5639de | gpt-4.1-2025-04-14 | mizan-mut-use-reorder |
| mizan-mut-while-to-loop | 3c2a53da | gpt-4.1-2025-04-14 | mizan-mut-while-to-loop |
| benign-blocks | 0a7031e1 | gpt-4.1-2025-04-14 | benign-blocks |
| benign-comments | d686abf5 | gpt-4.1-2025-04-14 | benign-comments |
| benign-rename-fn | 983db75a | gpt-4.1-2025-04-14 | benign-rename-fn |
| benign-rename-var | 03a047fc | gpt-4.1-2025-04-14 | benign-rename-var |
| malignant-blocks | 8756cb83 | gpt-4.1-2025-04-14 | malignant-blocks |
| malignant-comments | d843cf24 | gpt-4.1-2025-04-14 | malignant-comments |
| malignant-rename-fn | d0f5777d | gpt-4.1-2025-04-14 | malignant-rename-fn |
| malignant-rename-var | 9b5685e9 | gpt-4.1-2025-04-14 | malignant-rename-var |

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
| format-expanded | 78.0% | 44.5% | 9.2% (16 out of 173) | 16.2% (28 out of 173) | 13.9% (24 out of 173) |
| format-compact | 74.0% | 45.1% | 6.9% (12 out of 173) | 12.1% (21 out of 173) | 13.9% (24 out of 173) |
| mizan-mut-arithmetic-identity | 78.0% | 45.1% | 5.8% (10 out of 173) | 12.7% (22 out of 173) | 12.1% (21 out of 173) |
| mizan-mut-derive-reorder | 79.2% | 43.4% | 7.5% (13 out of 173) | 12.1% (21 out of 173) | 11.0% (19 out of 173) |
| mizan-mut-for-to-while | 82.1% | 46.8% | 6.4% (11 out of 173) | 12.7% (22 out of 173) | 13.3% (23 out of 173) |
| mizan-mut-if-else-reorder | 79.2% | 45.1% | 8.7% (15 out of 173) | 13.3% (23 out of 173) | 12.7% (22 out of 173) |
| mizan-mut-trait-bound-reorder | 78.6% | 41.6% | 6.9% (12 out of 173) | 12.7% (22 out of 173) | 12.1% (21 out of 173) |
| mizan-mut-use-reorder | 77.5% | 45.1% | 7.5% (13 out of 173) | 13.9% (24 out of 173) | 11.6% (20 out of 173) |
| mizan-mut-while-to-loop | 76.3% | 43.4% | 6.9% (12 out of 173) | 13.3% (23 out of 173) | 12.1% (21 out of 173) |
| benign-blocks | 80.3% | 46.2% | 8.1% (14 out of 173) | 16.2% (28 out of 173) | 13.9% (24 out of 173) |
| benign-comments | 80.3% | 48.0% | 7.5% (13 out of 173) | 15.0% (26 out of 173) | 15.0% (26 out of 173) |
| benign-rename-fn | 80.3% | 45.1% | 7.5% (13 out of 173) | 12.1% (21 out of 173) | 13.3% (23 out of 173) |
| benign-rename-var | 80.3% | 46.8% | 7.5% (13 out of 173) | 13.3% (23 out of 173) | 12.1% (21 out of 173) |
| malignant-blocks | 80.9% | 56.6% | 8.7% (15 out of 173) | 12.7% (22 out of 173) | 12.1% (21 out of 173) |
| malignant-comments | 81.5% | 35.8% | 3.5% (6 out of 173) | 3.5% (6 out of 173) | 5.2% (9 out of 173) |
| malignant-rename-fn | 80.3% | 45.1% | 6.4% (11 out of 173) | 13.3% (23 out of 173) | 13.3% (23 out of 173) |
| malignant-rename-var | 80.9% | 46.8% | 9.8% (17 out of 173) | 17.3% (30 out of 173) | 16.2% (28 out of 173) |

### F1 Score Comparison

| Experiment | CWE Macro F1 | Functions Macro F1 | Lines Macro F1 |
|------------|--------------|-------------------|----------------|
| format-expanded | 0.057 | 0.149 | 0.120 |
| format-compact | 0.042 | 0.120 | 0.122 |
| mizan-mut-arithmetic-identity | 0.039 | 0.122 | 0.104 |
| mizan-mut-derive-reorder | 0.048 | 0.114 | 0.098 |
| mizan-mut-for-to-while | 0.037 | 0.116 | 0.109 |
| mizan-mut-if-else-reorder | 0.054 | 0.120 | 0.108 |
| mizan-mut-trait-bound-reorder | 0.045 | 0.117 | 0.099 |
| mizan-mut-use-reorder | 0.050 | 0.120 | 0.090 |
| mizan-mut-while-to-loop | 0.044 | 0.125 | 0.099 |
| benign-blocks | 0.063 | 0.152 | 0.131 |
| benign-comments | 0.051 | 0.138 | 0.125 |
| benign-rename-fn | 0.050 | 0.103 | 0.103 |
| benign-rename-var | 0.050 | 0.123 | 0.103 |
| malignant-blocks | 0.055 | 0.109 | 0.078 |
| malignant-comments | 0.027 | 0.033 | 0.037 |
| malignant-rename-fn | 0.039 | 0.124 | 0.112 |
| malignant-rename-var | 0.059 | 0.152 | 0.121 |

## Vulnerable Samples Only Comparison

*Performance metrics calculated only on samples that contain vulnerabilities*

| Experiment | JSON Validity | Vulnerability Accuracy | At Least One Correct CWE Identified | At Least One Correct Vulnerable Function Identified | At Least One Correct Vulnerable Line Identified |
|------------|---------------|------------------------|--------------------------------------|-----------------------------------------------------|--------------------------------------------------|
| format-expanded | 78.9% | 38.9% | 16.8% (16 out of 95) | 29.5% (28 out of 95) | 25.3% (24 out of 95) |
| format-compact | 70.5% | 34.7% | 12.6% (12 out of 95) | 22.1% (21 out of 95) | 25.3% (24 out of 95) |
| mizan-mut-arithmetic-identity | 76.8% | 35.8% | 10.5% (10 out of 95) | 23.2% (22 out of 95) | 22.1% (21 out of 95) |
| mizan-mut-derive-reorder | 76.8% | 33.7% | 13.7% (13 out of 95) | 22.1% (21 out of 95) | 20.0% (19 out of 95) |
| mizan-mut-for-to-while | 83.2% | 37.9% | 11.6% (11 out of 95) | 23.2% (22 out of 95) | 24.2% (23 out of 95) |
| mizan-mut-if-else-reorder | 80.0% | 35.8% | 15.8% (15 out of 95) | 24.2% (23 out of 95) | 23.2% (22 out of 95) |
| mizan-mut-trait-bound-reorder | 76.8% | 33.7% | 12.6% (12 out of 95) | 23.2% (22 out of 95) | 22.1% (21 out of 95) |
| mizan-mut-use-reorder | 75.8% | 38.9% | 13.7% (13 out of 95) | 25.3% (24 out of 95) | 21.1% (20 out of 95) |
| mizan-mut-while-to-loop | 75.8% | 34.7% | 12.6% (12 out of 95) | 24.2% (23 out of 95) | 22.1% (21 out of 95) |
| benign-blocks | 82.1% | 40.0% | 14.7% (14 out of 95) | 29.5% (28 out of 95) | 25.3% (24 out of 95) |
| benign-comments | 81.1% | 38.9% | 13.7% (13 out of 95) | 27.4% (26 out of 95) | 27.4% (26 out of 95) |
| benign-rename-fn | 81.1% | 35.8% | 13.7% (13 out of 95) | 22.1% (21 out of 95) | 24.2% (23 out of 95) |
| benign-rename-var | 82.1% | 36.8% | 13.7% (13 out of 95) | 24.2% (23 out of 95) | 22.1% (21 out of 95) |
| malignant-blocks | 82.1% | 54.7% | 15.8% (15 out of 95) | 23.2% (22 out of 95) | 22.1% (21 out of 95) |
| malignant-comments | 83.2% | 20.0% | 6.3% (6 out of 95) | 6.3% (6 out of 95) | 9.5% (9 out of 95) |
| malignant-rename-fn | 81.1% | 35.8% | 11.6% (11 out of 95) | 24.2% (23 out of 95) | 24.2% (23 out of 95) |
| malignant-rename-var | 83.2% | 42.1% | 17.9% (17 out of 95) | 31.6% (30 out of 95) | 29.5% (28 out of 95) |

### F1 Score Comparison (Vulnerable Samples Only)

| Experiment | CWE Macro F1 | Functions Macro F1 | Lines Macro F1 |
|------------|--------------|-------------------|----------------|
| format-expanded | 0.103 | 0.268 | 0.215 |
| format-compact | 0.081 | 0.230 | 0.232 |
| mizan-mut-arithmetic-identity | 0.072 | 0.225 | 0.193 |
| mizan-mut-derive-reorder | 0.091 | 0.214 | 0.183 |
| mizan-mut-for-to-while | 0.067 | 0.208 | 0.196 |
| mizan-mut-if-else-reorder | 0.098 | 0.216 | 0.195 |
| mizan-mut-trait-bound-reorder | 0.085 | 0.219 | 0.184 |
| mizan-mut-use-reorder | 0.093 | 0.223 | 0.168 |
| mizan-mut-while-to-loop | 0.080 | 0.230 | 0.182 |
| benign-blocks | 0.112 | 0.272 | 0.233 |
| benign-comments | 0.093 | 0.249 | 0.226 |
| benign-rename-fn | 0.090 | 0.185 | 0.187 |
| benign-rename-var | 0.089 | 0.220 | 0.183 |
| malignant-blocks | 0.099 | 0.196 | 0.140 |
| malignant-comments | 0.049 | 0.058 | 0.066 |
| malignant-rename-fn | 0.070 | 0.224 | 0.203 |
| malignant-rename-var | 0.105 | 0.269 | 0.214 |

## Analysis of Samples with Valid JSON Across All Experiments

*Analysis focused only on the 105 samples where all experiments produced valid JSON responses*

| Experiment | Vulnerability Accuracy | At Least One Correct CWE Identified | At Least One Correct Vulnerable Function Identified | At Least One Correct Vulnerable Line Identified |
|------------|------------------------|--------------------------------------|-----------------------------------------------------|--------------------------------------------------|
| format-expanded | 60.0% | 9.5% (10 out of 105) | 20.0% (21 out of 105) | 16.2% (17 out of 105) |
| format-compact | 61.0% | 7.6% (8 out of 105) | 16.2% (17 out of 105) | 17.1% (18 out of 105) |
| mizan-mut-arithmetic-identity | 61.0% | 6.7% (7 out of 105) | 18.1% (19 out of 105) | 16.2% (17 out of 105) |
| mizan-mut-derive-reorder | 57.1% | 7.6% (8 out of 105) | 15.2% (16 out of 105) | 12.4% (13 out of 105) |
| mizan-mut-for-to-while | 62.9% | 5.7% (6 out of 105) | 16.2% (17 out of 105) | 16.2% (17 out of 105) |
| mizan-mut-if-else-reorder | 58.1% | 7.6% (8 out of 105) | 16.2% (17 out of 105) | 14.3% (15 out of 105) |
| mizan-mut-trait-bound-reorder | 53.3% | 4.8% (5 out of 105) | 15.2% (16 out of 105) | 13.3% (14 out of 105) |
| mizan-mut-use-reorder | 60.0% | 8.6% (9 out of 105) | 19.0% (20 out of 105) | 14.3% (15 out of 105) |
| mizan-mut-while-to-loop | 58.1% | 6.7% (7 out of 105) | 18.1% (19 out of 105) | 15.2% (16 out of 105) |
| benign-blocks | 60.0% | 7.6% (8 out of 105) | 19.0% (20 out of 105) | 16.2% (17 out of 105) |
| benign-comments | 62.9% | 6.7% (7 out of 105) | 19.0% (20 out of 105) | 17.1% (18 out of 105) |
| benign-rename-fn | 58.1% | 7.6% (8 out of 105) | 16.2% (17 out of 105) | 16.2% (17 out of 105) |
| benign-rename-var | 64.8% | 7.6% (8 out of 105) | 19.0% (20 out of 105) | 15.2% (16 out of 105) |
| malignant-blocks | 72.4% | 7.6% (8 out of 105) | 17.1% (18 out of 105) | 14.3% (15 out of 105) |
| malignant-comments | 44.8% | 1.9% (2 out of 105) | 4.8% (5 out of 105) | 5.7% (6 out of 105) |
| malignant-rename-fn | 60.0% | 5.7% (6 out of 105) | 17.1% (18 out of 105) | 15.2% (16 out of 105) |
| malignant-rename-var | 62.9% | 8.6% (9 out of 105) | 21.9% (23 out of 105) | 21.0% (22 out of 105) |

### F1 Score Comparison (Valid JSON Samples)

| Experiment | CWE Macro F1 | Functions Macro F1 | Lines Macro F1 |
|------------|--------------|-------------------|----------------|
| format-expanded | 0.040 | 0.150 | 0.117 |
| format-compact | 0.034 | 0.125 | 0.126 |
| mizan-mut-arithmetic-identity | 0.037 | 0.145 | 0.121 |
| mizan-mut-derive-reorder | 0.041 | 0.124 | 0.104 |
| mizan-mut-for-to-while | 0.024 | 0.132 | 0.121 |
| mizan-mut-if-else-reorder | 0.036 | 0.128 | 0.110 |
| mizan-mut-trait-bound-reorder | 0.028 | 0.122 | 0.097 |
| mizan-mut-use-reorder | 0.045 | 0.141 | 0.098 |
| mizan-mut-while-to-loop | 0.033 | 0.135 | 0.105 |
| benign-blocks | 0.040 | 0.155 | 0.130 |
| benign-comments | 0.036 | 0.143 | 0.121 |
| benign-rename-fn | 0.040 | 0.118 | 0.109 |
| benign-rename-var | 0.039 | 0.145 | 0.113 |
| malignant-blocks | 0.039 | 0.132 | 0.089 |
| malignant-comments | 0.010 | 0.041 | 0.043 |
| malignant-rename-fn | 0.030 | 0.140 | 0.120 |
| malignant-rename-var | 0.045 | 0.172 | 0.136 |

### Vulnerable Samples Performance (Valid JSON Samples)

| Experiment | Vulnerability Accuracy | At Least One Correct CWE Identified | At Least One Correct Vulnerable Function Identified | At Least One Correct Vulnerable Line Identified |
|------------|------------------------|--------------------------------------|-----------------------------------------------------|--------------------------------------------------|
| format-expanded | 48.1% | 18.5% (10 out of 54) | 38.9% (21 out of 54) | 31.5% (17 out of 54) |
| format-compact | 48.1% | 14.8% (8 out of 54) | 31.5% (17 out of 54) | 33.3% (18 out of 54) |
| mizan-mut-arithmetic-identity | 51.9% | 13.0% (7 out of 54) | 35.2% (19 out of 54) | 31.5% (17 out of 54) |
| mizan-mut-derive-reorder | 44.4% | 14.8% (8 out of 54) | 29.6% (16 out of 54) | 24.1% (13 out of 54) |
| mizan-mut-for-to-while | 50.0% | 11.1% (6 out of 54) | 31.5% (17 out of 54) | 31.5% (17 out of 54) |
| mizan-mut-if-else-reorder | 42.6% | 14.8% (8 out of 54) | 31.5% (17 out of 54) | 27.8% (15 out of 54) |
| mizan-mut-trait-bound-reorder | 40.7% | 9.3% (5 out of 54) | 29.6% (16 out of 54) | 25.9% (14 out of 54) |
| mizan-mut-use-reorder | 53.7% | 16.7% (9 out of 54) | 37.0% (20 out of 54) | 27.8% (15 out of 54) |
| mizan-mut-while-to-loop | 46.3% | 13.0% (7 out of 54) | 35.2% (19 out of 54) | 29.6% (16 out of 54) |
| benign-blocks | 50.0% | 14.8% (8 out of 54) | 37.0% (20 out of 54) | 31.5% (17 out of 54) |
| benign-comments | 48.1% | 13.0% (7 out of 54) | 37.0% (20 out of 54) | 33.3% (18 out of 54) |
| benign-rename-fn | 46.3% | 14.8% (8 out of 54) | 31.5% (17 out of 54) | 31.5% (17 out of 54) |
| benign-rename-var | 50.0% | 14.8% (8 out of 54) | 37.0% (20 out of 54) | 29.6% (16 out of 54) |
| malignant-blocks | 68.5% | 14.8% (8 out of 54) | 33.3% (18 out of 54) | 27.8% (15 out of 54) |
| malignant-comments | 20.4% | 3.7% (2 out of 54) | 9.3% (5 out of 54) | 11.1% (6 out of 54) |
| malignant-rename-fn | 48.1% | 11.1% (6 out of 54) | 33.3% (18 out of 54) | 29.6% (16 out of 54) |
| malignant-rename-var | 57.4% | 16.7% (9 out of 54) | 42.6% (23 out of 54) | 40.7% (22 out of 54) |

## Figures

The following figures have been generated and saved to the `figures/` directory:

1. **figure1_overall_comparison.png** - Overall Performance Comparison: Simple Metrics
2. **figure2_f1_comparison.png** - F1 Score Comparison
3. **figure3_vulnerable_comparison.png** - Performance on Vulnerable Samples Only
4. **figure4_vulnerable_f1_comparison.png** - F1 Score Comparison (Vulnerable Samples Only)
5. **figure5_valid_json_comparison.png** - Performance on Samples with Valid JSON Across All Experiments
6. **figure6_valid_json_f1_comparison.png** - F1 Score Comparison on Samples with Valid JSON Across All Experiments
