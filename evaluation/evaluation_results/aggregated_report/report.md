# RustMizan Benchmark

## 1. Vanilla Dataset Performance

### 1.1 Model Comparison

| Experiment        | JSON Validity | Vulnerability Accuracy | At Least One Correct CWE Identified | At Least One Correct Vulnerable Function Identified | At Least One Correct Vulnerable Line Identified |
| ----------------- | ------------- | ---------------------- | ----------------------------------- | --------------------------------------------------- | ----------------------------------------------- |
| gpt-on-vanilla    | 80.9%         | 45.7%                  | 6.9% (12 out of 173)                | 13.9% (24 out of 173)                               | 15.6% (27 out of 173)                           |
| claude-on-vanilla | 89.6%         | 53.2%                  | 9.2% (16 out of 173)                | 18.5% (32 out of 173)                               | 17.9% (31 out of 173)                           |

### 1.2 GPT-4.1 Analysis by Granularity

| Granularity | Samples | JSON Validity | Vulnerability Accuracy | At Least One CWE    | At Least One Function | At Least One Line    |
| ----------- | ------- | ------------- | ---------------------- | ------------------- | --------------------- | -------------------- |
| Function    | 57      | 98.2%         | 61.4%                  | 10.5% (6 out of 57) | 21.1% (12 out of 57)  | 26.3% (15 out of 57) |
| File        | 39      | 69.2%         | 35.9%                  | 5.1% (2 out of 39)  | 10.3% (4 out of 39)   | 7.7% (3 out of 39)   |
| Crate       | 77      | 74.0%         | 39.0%                  | 5.2% (4 out of 77)  | 10.4% (8 out of 77)   | 11.7% (9 out of 77)  |

### 1.3 Claude-3.7-Sonnet Analysis by Granularity

| Granularity | Samples | JSON Validity | Vulnerability Accuracy | At Least One CWE    | At Least One Function | At Least One Line    |
| ----------- | ------- | ------------- | ---------------------- | ------------------- | --------------------- | -------------------- |
| Function    | 57      | 98.2%         | 61.4%                  | 14.0% (8 out of 57) | 29.8% (17 out of 57)  | 28.1% (16 out of 57) |
| File        | 39      | 94.9%         | 53.8%                  | 7.7% (3 out of 39)  | 17.9% (7 out of 39)   | 15.4% (6 out of 39)  |
| Crate       | 77      | 80.5%         | 46.8%                  | 6.5% (5 out of 77)  | 10.4% (8 out of 77)   | 11.7% (9 out of 77)  |

### 1.4 GPT-4.1 Performance by CWE Type

| CWE     | Samples | JSON Validity | Vulnerability Accuracy | At Least One CWE    | At Least One Function | At Least One Line   |
| ------- | ------- | ------------- | ---------------------- | ------------------- | --------------------- | ------------------- |
| CWE-416 | 13      | 76.9%         | 15.4%                  | 7.7% (1 out of 13)  | 7.7% (1 out of 13)    | 15.4% (2 out of 13) |
| CWE-129 | 12      | 41.7%         | 16.7%                  | 0.0% (0 out of 12)  | 8.3% (1 out of 12)    | 8.3% (1 out of 12)  |
| CWE-362 | 11      | 90.9%         | 18.2%                  | 9.1% (1 out of 11)  | 9.1% (1 out of 11)    | 18.2% (2 out of 11) |
| CWE-787 | 10      | 90.0%         | 20.0%                  | 10.0% (1 out of 10) | 10.0% (1 out of 10)   | 20.0% (2 out of 10) |
| CWE-908 | 9       | 88.9%         | 77.8%                  | 22.2% (2 out of 9)  | 66.7% (6 out of 9)    | 33.3% (3 out of 9)  |

### 1.5 Claude-3.7-Sonnet Performance by CWE Type

| CWE     | Samples | JSON Validity | Vulnerability Accuracy | At Least One CWE    | At Least One Function | At Least One Line   |
| ------- | ------- | ------------- | ---------------------- | ------------------- | --------------------- | ------------------- |
| CWE-416 | 13      | 92.3%         | 76.9%                  | 23.1% (3 out of 13) | 15.4% (2 out of 13)   | 38.5% (5 out of 13) |
| CWE-129 | 12      | 100.0%        | 100.0%                 | 0.0% (0 out of 12)  | 0.0% (0 out of 12)    | 0.0% (0 out of 12)  |
| CWE-362 | 11      | 63.6%         | 63.6%                  | 9.1% (1 out of 11)  | 0.0% (0 out of 11)    | 9.1% (1 out of 11)  |
| CWE-787 | 10      | 100.0%        | 80.0%                  | 40.0% (4 out of 10) | 40.0% (4 out of 10)   | 20.0% (2 out of 10) |
| CWE-908 | 9       | 100.0%        | 100.0%                 | 11.1% (1 out of 9)  | 66.7% (6 out of 9)    | 66.7% (6 out of 9)  |

**Figures:**

- `figures/gpt_vanilla_overall.png` - GPT Overall Performance
- `figures/claude_vanilla_overall.png` - Claude Overall Performance
- `figures/gpt_claude_vanilla_comparison.png` - Direct Model Comparison

**Full Reports:**

- [GPT Vanilla Detailed Analysis](../experiment_7dd2c700/analysis/analysis_report.md) - Complete performance breakdown including F1 scores, complete granularity analysis, and additional figures
- [Claude Vanilla Detailed Analysis](../experiment_7f0feb42/analysis/analysis_report.md) - Complete performance breakdown including F1 scores, complete granularity analysis, and additional figures
- [GPT vs Claude Comparison](../gpt-claude-vanilla/comparison_report.md) - Detailed comparison with F1 scores and valid JSON analysis

### 1.6 Findings

**Key Observations:**

- Claude consistently outperforms GPT across all metrics
- Both models struggle with CWE identification (less than 10% success rates of identifying at least one CWE)
- Function-level granularity provides optimal performance for both models
- CWE-908 (Use of Uninitialized Resource) shows the best performance for both models, while CWE-416 (Use After Free) and CWE-129 (Buffer Under-read) are particularly challenging

> The finding on CWEs needs further investigation to make sure the CWE types are the actual reason for the performance differences

## 2. Individual Mutation Effects Analysis

### 2.1 GPT-4.1 Individual Mutation Impact

#### Analysis of Samples with Valid JSON Across All Experiments

_Analysis focused only on the 105 samples where all experiments produced valid JSON responses_

| Experiment                    | Vulnerability Accuracy | At Least One Correct CWE Identified | At Least One Correct Vulnerable Function Identified | At Least One Correct Vulnerable Line Identified |
| ----------------------------- | ---------------------- | ----------------------------------- | --------------------------------------------------- | ----------------------------------------------- |
| format-expanded               | 60.0%                  | 9.5% (10 out of 105)                | 20.0% (21 out of 105)                               | 16.2% (17 out of 105)                           |
| format-compact                | 61.0%                  | 7.6% (8 out of 105)                 | 16.2% (17 out of 105)                               | 17.1% (18 out of 105)                           |
| mizan-mut-arithmetic-identity | 61.0%                  | 6.7% (7 out of 105)                 | 18.1% (19 out of 105)                               | 16.2% (17 out of 105)                           |
| mizan-mut-derive-reorder      | 57.1%                  | 7.6% (8 out of 105)                 | 15.2% (16 out of 105)                               | 12.4% (13 out of 105)                           |
| mizan-mut-for-to-while        | 62.9%                  | 5.7% (6 out of 105)                 | 16.2% (17 out of 105)                               | 16.2% (17 out of 105)                           |
| mizan-mut-if-else-reorder     | 58.1%                  | 7.6% (8 out of 105)                 | 16.2% (17 out of 105)                               | 14.3% (15 out of 105)                           |
| mizan-mut-trait-bound-reorder | 53.3%                  | 4.8% (5 out of 105)                 | 15.2% (16 out of 105)                               | 13.3% (14 out of 105)                           |
| mizan-mut-use-reorder         | 60.0%                  | 8.6% (9 out of 105)                 | 19.0% (20 out of 105)                               | 14.3% (15 out of 105)                           |
| mizan-mut-while-to-loop       | 58.1%                  | 6.7% (7 out of 105)                 | 18.1% (19 out of 105)                               | 15.2% (16 out of 105)                           |
| benign-blocks                 | 60.0%                  | 7.6% (8 out of 105)                 | 19.0% (20 out of 105)                               | 16.2% (17 out of 105)                           |
| benign-comments               | 62.9%                  | 6.7% (7 out of 105)                 | 19.0% (20 out of 105)                               | 17.1% (18 out of 105)                           |
| benign-rename-fn              | 58.1%                  | 7.6% (8 out of 105)                 | 16.2% (17 out of 105)                               | 16.2% (17 out of 105)                           |
| benign-rename-var             | 64.8%                  | 7.6% (8 out of 105)                 | 19.0% (20 out of 105)                               | 15.2% (16 out of 105)                           |
| malignant-blocks              | 72.4%                  | 7.6% (8 out of 105)                 | 17.1% (18 out of 105)                               | 14.3% (15 out of 105)                           |
| malignant-comments            | 44.8%                  | 1.9% (2 out of 105)                 | 4.8% (5 out of 105)                                 | 5.7% (6 out of 105)                             |
| malignant-rename-fn           | 60.0%                  | 5.7% (6 out of 105)                 | 17.1% (18 out of 105)                               | 15.2% (16 out of 105)                           |
| malignant-rename-var          | 62.9%                  | 8.6% (9 out of 105)                 | 21.9% (23 out of 105)                               | 21.0% (22 out of 105)                           |

### 2.2 Claude-3.7-Sonnet Individual Mutation Impact

#### Analysis of Samples with Valid JSON Across All Experiments

_Analysis focused only on the 137 samples where all experiments produced valid JSON responses_

| Experiment                    | Vulnerability Accuracy | At Least One Correct CWE Identified | At Least One Correct Vulnerable Function Identified | At Least One Correct Vulnerable Line Identified |
| ----------------------------- | ---------------------- | ----------------------------------- | --------------------------------------------------- | ----------------------------------------------- |
| vanilla                       | 57.7%                  | 10.9% (15 out of 137)               | 22.6% (31 out of 137)                               | 21.9% (30 out of 137)                           |
| format-expanded               | 58.4%                  | 9.5% (13 out of 137)                | 22.6% (31 out of 137)                               | 25.5% (35 out of 137)                           |
| format-compact                | 58.4%                  | 6.6% (9 out of 137)                 | 21.2% (29 out of 137)                               | 20.4% (28 out of 137)                           |
| mizan-mut-arithmetic-identity | 59.1%                  | 8.0% (11 out of 137)                | 21.9% (30 out of 137)                               | 21.2% (29 out of 137)                           |
| mizan-mut-derive-reorder      | 58.4%                  | 7.3% (10 out of 137)                | 24.8% (34 out of 137)                               | 21.9% (30 out of 137)                           |
| mizan-mut-for-to-while        | 59.1%                  | 11.7% (16 out of 137)               | 22.6% (31 out of 137)                               | 23.4% (32 out of 137)                           |
| mizan-mut-if-else-reorder     | 59.9%                  | 8.8% (12 out of 137)                | 24.1% (33 out of 137)                               | 23.4% (32 out of 137)                           |
| mizan-mut-trait-bound-reorder | 59.1%                  | 10.9% (15 out of 137)               | 21.9% (30 out of 137)                               | 19.7% (27 out of 137)                           |
| mizan-mut-use-reorder         | 59.1%                  | 8.8% (12 out of 137)                | 22.6% (31 out of 137)                               | 23.4% (32 out of 137)                           |
| mizan-mut-while-to-loop       | 60.6%                  | 8.0% (11 out of 137)                | 23.4% (32 out of 137)                               | 21.2% (29 out of 137)                           |
| benign-blocks                 | 57.7%                  | 10.9% (15 out of 137)               | 28.5% (39 out of 137)                               | 27.7% (38 out of 137)                           |
| benign-comments               | 59.1%                  | 12.4% (17 out of 137)               | 22.6% (31 out of 137)                               | 23.4% (32 out of 137)                           |
| benign-rename-fn              | 58.4%                  | 7.3% (10 out of 137)                | 21.9% (30 out of 137)                               | 23.4% (32 out of 137)                           |
| benign-rename-var             | 61.3%                  | 13.9% (19 out of 137)               | 25.5% (35 out of 137)                               | 24.8% (34 out of 137)                           |
| malignant-blocks              | 59.1%                  | 10.9% (15 out of 137)               | 11.7% (16 out of 137)                               | 11.7% (16 out of 137)                           |
| malignant-comments            | 54.0%                  | 8.8% (12 out of 137)                | 19.0% (26 out of 137)                               | 19.7% (27 out of 137)                           |
| malignant-rename-fn           | 56.9%                  | 9.5% (13 out of 137)                | 22.6% (31 out of 137)                               | 21.9% (30 out of 137)                           |
| malignant-rename-var          | 58.4%                  | 12.4% (17 out of 137)               | 24.8% (34 out of 137)                               | 26.3% (36 out of 137)                           |

**Figures:**

- `figures/gpt_individual_mutations.png` - GPT Individual Mutation Effects
- `figures/claude_individual_mutations.png` - Claude Individual Mutation Effects

**Full Reports:**

- [GPT Individual Mutations Analysis](../gpt-each-mutation/comparison_report.md) - Complete tables including overall performance, vulnerable samples analysis, F1 scores, and additional figures
- [Claude Individual Mutations Analysis](../claude-each-mutation/comparison_report.md) - Complete tables including overall performance, vulnerable samples analysis, F1 scores, and additional figures

### 2.3 Findings

**Benign Mutations Impact Assessment:**

- No significant negative impact on model performance for most neutral and benign mutations
  - For GPT, the ability to identify at least one vulnerable function is generally between 15.2% and 21.9%
  - For Claude, the ability to identify at least one vulnerable function is generally between 21.2% and 25.5% (except for `benign-blocks` which has a higher rate of 28.5%)

**Malignant Mutations Impact Assessment:**

- **Most detrimental mutation on the model's ability to identify at least one vulnerable function**:
  - For GPT, `malignant-comments` causes the most significant drop (from ~19% baseline to 4.8%)
  - For Claude, `malignant-blocks` has the largest negative impact (from ~22.6% baseline to 11.7%)
