# RustMizan Benchmark Analysis Report

- **Model:** gpt-4.1-2025-04-14

**Mutations Applied:** malignant-comments

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
| JSON Validity Rate | 81.5% |
| Vulnerability Detection Accuracy | 35.8% |
| Samples with At Least One Correct CWE Identified | 3.5% (6 out of 173) |
| Samples with At Least One Correct Vulnerable Function Identified | 3.5% (6 out of 173) |
| Samples with At Least One Correct Vulnerable Line Identified | 5.2% (9 out of 173) |
| CWE Type Macro F1 | 0.027 |
| Vulnerable Functions Macro F1 | 0.033 |
| Vulnerable Lines Macro F1 | 0.037 |

## Analysis of Vulnerable Samples

*Analysis focused only on samples that contain vulnerabilities*

| Metric | Value |
|--------|-------|
| Total Vulnerable Samples | 95 |
| JSON Validity Rate | 83.2% |
| Vulnerability Detection Accuracy | 20.0% |
| At Least One Correct CWE Identified | 6.3% (6 out of 95) |
| At Least One Correct Vulnerable Function Identified | 6.3% (6 out of 95) |
| At Least One Correct Vulnerable Line Identified | 9.5% (9 out of 95) |

### Detailed F1 Scores (Vulnerable Samples Only)

| Metric | F1 Score |
|--------|----------|
| CWE Type Macro F1 | 0.049 |
| Vulnerable Functions Macro F1 | 0.058 |
| Vulnerable Lines Macro F1 | 0.066 |

## Analysis by Granularity

| Granularity | Samples | JSON Validity | Vulnerability Accuracy | At Least One CWE | At Least One Function | At Least One Line |
|-------------|---------|---------------|------------------------|------------------|----------------------|-------------------|
| Function | 57 | 100.0% | 38.6% | 3.5% (2 out of 57) | 5.3% (3 out of 57) | 8.8% (5 out of 57) |
| File | 39 | 69.2% | 30.8% | 2.6% (1 out of 39) | 2.6% (1 out of 39) | 2.6% (1 out of 39) |
| Crate | 77 | 74.0% | 36.4% | 3.9% (3 out of 77) | 2.6% (2 out of 77) | 3.9% (3 out of 77) |

### Detailed F1 Scores by Granularity

| Granularity | CWE Macro F1 | Functions Macro F1 | Lines Macro F1 |
|-------------|--------------|-------------------|----------------|
| Function | 0.018 | 0.053 | 0.064 |
| File | 0.026 | 0.017 | 0.017 |
| Crate | 0.024 | 0.012 | 0.011 |

## Complete Granularity Analysis

*Analysis of vulnerabilities with all three granularities (function, file, crate) and valid JSON responses*

Found 9 vulnerabilities with all three granularities and valid JSON responses.

Total samples in complete analysis: 45 (15 samples per granularity)

| Granularity | Samples | Vulnerability Accuracy | At Least One CWE | At Least One Function | At Least One Line |
|-------------|---------|------------------------|------------------|----------------------|-------------------|
| Function | 15 | 33.3% | 0.0% (0 out of 15) | 6.7% (1 out of 15) | 6.7% (1 out of 15) |
| File | 15 | 33.3% | 6.7% (1 out of 15) | 0.0% (0 out of 15) | 0.0% (0 out of 15) |
| Crate | 15 | 33.3% | 6.7% (1 out of 15) | 6.7% (1 out of 15) | 6.7% (1 out of 15) |

### Detailed F1 Scores (Complete Granularity)

| Granularity | CWE Macro F1 | Functions Macro F1 | Lines Macro F1 |
|-------------|--------------|-------------------|----------------|
| Function | 0.000 | 0.067 | 0.053 |
| File | 0.067 | 0.000 | 0.000 |
| Crate | 0.067 | 0.019 | 0.019 |

## Performance by CWE Type

Total unique CWE types: 18

| CWE | Samples | JSON Validity | Vulnerability Accuracy | At Least One CWE | At Least One Function | At Least One Line |
|-----|---------|---------------|------------------------|------------------|----------------------|-------------------|
| CWE-416 | 13 | 76.9% | 7.7% | 0.0% (0 out of 13) | 0.0% (0 out of 13) | 0.0% (0 out of 13) |
| CWE-129 | 12 | 41.7% | 8.3% | 0.0% (0 out of 12) | 0.0% (0 out of 12) | 0.0% (0 out of 12) |
| CWE-362 | 11 | 90.9% | 18.2% | 0.0% (0 out of 11) | 0.0% (0 out of 11) | 0.0% (0 out of 11) |
| CWE-787 | 10 | 90.0% | 10.0% | 0.0% (0 out of 10) | 0.0% (0 out of 10) | 0.0% (0 out of 10) |
| CWE-908 | 9 | 88.9% | 33.3% | 0.0% (0 out of 9) | 11.1% (1 out of 9) | 11.1% (1 out of 9) |

### Detailed F1 Scores by CWE Type

| CWE | CWE F1 | Functions F1 | Lines F1 |
|-----|--------|--------------|----------|
| CWE-416 | 0.000 | 0.000 | 0.000 |
| CWE-129 | 0.000 | 0.000 | 0.000 |
| CWE-362 | 0.000 | 0.000 | 0.000 |
| CWE-787 | 0.000 | 0.000 | 0.000 |
| CWE-908 | 0.000 | 0.111 | 0.089 |

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
