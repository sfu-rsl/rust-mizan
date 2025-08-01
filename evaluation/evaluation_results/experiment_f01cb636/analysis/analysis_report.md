# RustMizan Benchmark Analysis Report

- **Model:** claude-3-7-sonnet-20250219

**Mutations Applied:** benign-blocks

## Metric Definitions

- **JSON Validity Rate:** Percentage of samples with valid JSON responses that could be parsed
- **Vulnerability Detection Accuracy:** Percentage of samples where the model correctly identified whether code is vulnerable or not
- **At Least One Correct CWE:** Percentage of samples where the model identified at least one correct CWE type
- **At Least One Correct Function:** Percentage of samples where the model identified at least one correct vulnerable function
- **At Least One Correct Line:** Percentage of samples where the model identified at least one correct vulnerable line
- **Macro F1 Score:** Average F1 score across all samples for the given task (CWE/Functions/Lines)
- **Granularity Levels:** Function (single function), File (entire file), Crate (entire crate/package)

## Overall Performance Summary

| Metric | Value |
|--------|-------|
| Total Samples | 173 |
| JSON Validity Rate | 91.3% |
| Vulnerability Detection Accuracy | 53.8% |
| Samples with At Least One Correct CWE Identified | 11.6% (20 out of 173) |
| Samples with At Least One Correct Vulnerable Function Identified | 25.4% (44 out of 173) |
| Samples with At Least One Correct Vulnerable Line Identified | 26.0% (45 out of 173) |
| CWE Type Macro F1 | 0.086 |
| Vulnerable Functions Macro F1 | 0.225 |
| Vulnerable Lines Macro F1 | 0.197 |

## Analysis of Vulnerable Samples

*Analysis focused only on samples that contain vulnerabilities*

| Metric | Value |
|--------|-------|
| Total Vulnerable Samples | 95 |
| JSON Validity Rate | 94.7% |
| Vulnerability Detection Accuracy | 90.5% |
| At Least One Correct CWE Identified | 21.1% (20 out of 95) |
| At Least One Correct Vulnerable Function Identified | 46.3% (44 out of 95) |
| At Least One Correct Vulnerable Line Identified | 47.4% (45 out of 95) |

### Detailed F1 Scores (Vulnerable Samples Only)

| Metric | F1 Score |
|--------|----------|
| CWE Type Macro F1 | 0.152 |
| Vulnerable Functions Macro F1 | 0.394 |
| Vulnerable Lines Macro F1 | 0.347 |

## Analysis by Granularity

| Granularity | Samples | JSON Validity | Vulnerability Accuracy | At Least One CWE | At Least One Function | At Least One Line |
|-------------|---------|---------------|------------------------|------------------|----------------------|-------------------|
| Function | 57 | 100.0% | 64.9% | 14.0% (8 out of 57) | 35.1% (20 out of 57) | 35.1% (20 out of 57) |
| File | 39 | 100.0% | 53.8% | 12.8% (5 out of 39) | 23.1% (9 out of 39) | 23.1% (9 out of 39) |
| Crate | 77 | 80.5% | 45.5% | 9.1% (7 out of 77) | 19.5% (15 out of 77) | 20.8% (16 out of 77) |

### Detailed F1 Scores by Granularity

| Granularity | CWE Macro F1 | Functions Macro F1 | Lines Macro F1 |
|-------------|--------------|-------------------|----------------|
| Function | 0.096 | 0.303 | 0.277 |
| File | 0.077 | 0.166 | 0.156 |
| Crate | 0.067 | 0.152 | 0.121 |

## Complete Granularity Analysis

*Analysis of vulnerabilities with all three granularities (function, file, crate) and valid JSON responses*

Found 16 vulnerabilities with all three granularities and valid JSON responses.

Total samples in complete analysis: 87 (29 samples per granularity)

| Granularity | Samples | Vulnerability Accuracy | At Least One CWE | At Least One Function | At Least One Line |
|-------------|---------|------------------------|------------------|----------------------|-------------------|
| Function | 29 | 65.5% | 17.2% (5 out of 29) | 34.5% (10 out of 29) | 27.6% (8 out of 29) |
| File | 29 | 58.6% | 10.3% (3 out of 29) | 27.6% (8 out of 29) | 24.1% (7 out of 29) |
| Crate | 29 | 51.7% | 10.3% (3 out of 29) | 34.5% (10 out of 29) | 31.0% (9 out of 29) |

### Detailed F1 Scores (Complete Granularity)

| Granularity | CWE Macro F1 | Functions Macro F1 | Lines Macro F1 |
|-------------|--------------|-------------------|----------------|
| Function | 0.126 | 0.251 | 0.228 |
| File | 0.057 | 0.201 | 0.182 |
| Crate | 0.063 | 0.277 | 0.186 |

## Performance by CWE Type

Total unique CWE types: 18

| CWE | Samples | JSON Validity | Vulnerability Accuracy | At Least One CWE | At Least One Function | At Least One Line |
|-----|---------|---------------|------------------------|------------------|----------------------|-------------------|
| CWE-416 | 13 | 92.3% | 92.3% | 46.2% (6 out of 13) | 38.5% (5 out of 13) | 61.5% (8 out of 13) |
| CWE-129 | 12 | 100.0% | 100.0% | 0.0% (0 out of 12) | 25.0% (3 out of 12) | 16.7% (2 out of 12) |
| CWE-362 | 11 | 90.9% | 90.9% | 18.2% (2 out of 11) | 9.1% (1 out of 11) | 27.3% (3 out of 11) |
| CWE-787 | 10 | 100.0% | 70.0% | 20.0% (2 out of 10) | 40.0% (4 out of 10) | 40.0% (4 out of 10) |
| CWE-908 | 9 | 100.0% | 100.0% | 0.0% (0 out of 9) | 66.7% (6 out of 9) | 33.3% (3 out of 9) |

### Detailed F1 Scores by CWE Type

| CWE | CWE F1 | Functions F1 | Lines F1 |
|-----|--------|--------------|----------|
| CWE-416 | 0.359 | 0.256 | 0.409 |
| CWE-129 | 0.000 | 0.222 | 0.167 |
| CWE-362 | 0.121 | 0.091 | 0.227 |
| CWE-787 | 0.133 | 0.400 | 0.247 |
| CWE-908 | 0.000 | 0.435 | 0.170 |

## Figures

The following figures have been generated and saved to the `figures/` directory:

1. **figure1_overall_simple.png** - Overall Performance: Simple Metrics
2. **figure2_overall_detailed.png** - Overall Performance: Detailed Metrics
3. **figure3_vulnerable_samples_simple.png** - Performance on Vulnerable Samples Only: Simple Metrics
4. **figure4_vulnerable_samples_detailed.png** - Performance on Vulnerable Samples Only: Detailed Metrics
5. **figure5_granularity_simple.png** - Performance by Granularity: Simple Metrics
6. **figure6_granularity_detailed.png** - Performance by Granularity: Detailed Metrics
7. **figure7_complete_granularity_simple.png** - Complete Granularity Analysis: Simple Metrics
8. **figure8_complete_granularity_detailed.png** - Complete Granularity Analysis: Detailed Metrics
