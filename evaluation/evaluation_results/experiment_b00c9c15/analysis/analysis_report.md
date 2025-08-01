# RustMizan Benchmark Analysis Report

- **Model:** gpt-4.1-2025-04-14

**Mutations Applied:** mizan-mut-for-to-while

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
| JSON Validity Rate | 82.1% |
| Vulnerability Detection Accuracy | 46.8% |
| Samples with At Least One Correct CWE Identified | 6.4% (11 out of 173) |
| Samples with At Least One Correct Vulnerable Function Identified | 12.7% (22 out of 173) |
| Samples with At Least One Correct Vulnerable Line Identified | 13.3% (23 out of 173) |
| CWE Type Macro F1 | 0.037 |
| Vulnerable Functions Macro F1 | 0.116 |
| Vulnerable Lines Macro F1 | 0.109 |

## Analysis of Vulnerable Samples

*Analysis focused only on samples that contain vulnerabilities*

| Metric | Value |
|--------|-------|
| Total Vulnerable Samples | 95 |
| JSON Validity Rate | 83.2% |
| Vulnerability Detection Accuracy | 37.9% |
| At Least One Correct CWE Identified | 11.6% (11 out of 95) |
| At Least One Correct Vulnerable Function Identified | 23.2% (22 out of 95) |
| At Least One Correct Vulnerable Line Identified | 24.2% (23 out of 95) |

### Detailed F1 Scores (Vulnerable Samples Only)

| Metric | F1 Score |
|--------|----------|
| CWE Type Macro F1 | 0.067 |
| Vulnerable Functions Macro F1 | 0.208 |
| Vulnerable Lines Macro F1 | 0.196 |

## Analysis by Granularity

| Granularity | Samples | JSON Validity | Vulnerability Accuracy | At Least One CWE | At Least One Function | At Least One Line |
|-------------|---------|---------------|------------------------|------------------|----------------------|-------------------|
| Function | 57 | 96.5% | 59.6% | 8.8% (5 out of 57) | 17.5% (10 out of 57) | 17.5% (10 out of 57) |
| File | 39 | 74.4% | 38.5% | 5.1% (2 out of 39) | 10.3% (4 out of 39) | 10.3% (4 out of 39) |
| Crate | 77 | 75.3% | 41.6% | 5.2% (4 out of 77) | 10.4% (8 out of 77) | 11.7% (9 out of 77) |

### Detailed F1 Scores by Granularity

| Granularity | CWE Macro F1 | Functions Macro F1 | Lines Macro F1 |
|-------------|--------------|-------------------|----------------|
| Function | 0.042 | 0.161 | 0.159 |
| File | 0.023 | 0.053 | 0.034 |
| Crate | 0.025 | 0.067 | 0.066 |

## Complete Granularity Analysis

*Analysis of vulnerabilities with all three granularities (function, file, crate) and valid JSON responses*

Found 8 vulnerabilities with all three granularities and valid JSON responses.

Total samples in complete analysis: 39 (13 samples per granularity)

| Granularity | Samples | Vulnerability Accuracy | At Least One CWE | At Least One Function | At Least One Line |
|-------------|---------|------------------------|------------------|----------------------|-------------------|
| Function | 13 | 53.8% | 23.1% (3 out of 13) | 30.8% (4 out of 13) | 15.4% (2 out of 13) |
| File | 13 | 46.2% | 7.7% (1 out of 13) | 23.1% (3 out of 13) | 23.1% (3 out of 13) |
| Crate | 13 | 46.2% | 7.7% (1 out of 13) | 15.4% (2 out of 13) | 15.4% (2 out of 13) |

### Detailed F1 Scores (Complete Granularity)

| Granularity | CWE Macro F1 | Functions Macro F1 | Lines Macro F1 |
|-------------|--------------|-------------------|----------------|
| Function | 0.096 | 0.244 | 0.138 |
| File | 0.031 | 0.146 | 0.090 |
| Crate | 0.051 | 0.108 | 0.079 |

## Performance by CWE Type

Total unique CWE types: 18

| CWE | Samples | JSON Validity | Vulnerability Accuracy | At Least One CWE | At Least One Function | At Least One Line |
|-----|---------|---------------|------------------------|------------------|----------------------|-------------------|
| CWE-416 | 13 | 84.6% | 7.7% | 7.7% (1 out of 13) | 0.0% (0 out of 13) | 7.7% (1 out of 13) |
| CWE-129 | 12 | 41.7% | 33.3% | 0.0% (0 out of 12) | 0.0% (0 out of 12) | 0.0% (0 out of 12) |
| CWE-362 | 11 | 90.9% | 36.4% | 9.1% (1 out of 11) | 9.1% (1 out of 11) | 18.2% (2 out of 11) |
| CWE-787 | 10 | 90.0% | 20.0% | 0.0% (0 out of 10) | 10.0% (1 out of 10) | 0.0% (0 out of 10) |
| CWE-908 | 9 | 88.9% | 77.8% | 11.1% (1 out of 9) | 66.7% (6 out of 9) | 55.6% (5 out of 9) |

### Detailed F1 Scores by CWE Type

| CWE | CWE F1 | Functions F1 | Lines F1 |
|-----|--------|--------------|----------|
| CWE-416 | 0.038 | 0.000 | 0.062 |
| CWE-129 | 0.000 | 0.000 | 0.000 |
| CWE-362 | 0.020 | 0.052 | 0.127 |
| CWE-787 | 0.000 | 0.100 | 0.000 |
| CWE-908 | 0.028 | 0.441 | 0.288 |

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
