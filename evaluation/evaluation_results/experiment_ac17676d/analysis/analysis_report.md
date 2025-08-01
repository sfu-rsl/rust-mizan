# RustMizan Benchmark Analysis Report

- **Model:** claude-3-7-sonnet-20250219

**Mutations Applied:** format-compact

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
| JSON Validity Rate | 93.6% |
| Vulnerability Detection Accuracy | 55.5% |
| Samples with At Least One Correct CWE Identified | 5.8% (10 out of 173) |
| Samples with At Least One Correct Vulnerable Function Identified | 18.5% (32 out of 173) |
| Samples with At Least One Correct Vulnerable Line Identified | 17.3% (30 out of 173) |
| CWE Type Macro F1 | 0.038 |
| Vulnerable Functions Macro F1 | 0.154 |
| Vulnerable Lines Macro F1 | 0.123 |

## Analysis of Vulnerable Samples

*Analysis focused only on samples that contain vulnerabilities*

| Metric | Value |
|--------|-------|
| Total Vulnerable Samples | 95 |
| JSON Validity Rate | 95.8% |
| Vulnerability Detection Accuracy | 89.5% |
| At Least One Correct CWE Identified | 10.5% (10 out of 95) |
| At Least One Correct Vulnerable Function Identified | 33.7% (32 out of 95) |
| At Least One Correct Vulnerable Line Identified | 31.6% (30 out of 95) |

### Detailed F1 Scores (Vulnerable Samples Only)

| Metric | F1 Score |
|--------|----------|
| CWE Type Macro F1 | 0.068 |
| Vulnerable Functions Macro F1 | 0.275 |
| Vulnerable Lines Macro F1 | 0.219 |

## Analysis by Granularity

| Granularity | Samples | JSON Validity | Vulnerability Accuracy | At Least One CWE | At Least One Function | At Least One Line |
|-------------|---------|---------------|------------------------|------------------|----------------------|-------------------|
| Function | 57 | 100.0% | 61.4% | 7.0% (4 out of 57) | 33.3% (19 out of 57) | 29.8% (17 out of 57) |
| File | 39 | 94.9% | 59.0% | 10.3% (4 out of 39) | 15.4% (6 out of 39) | 10.3% (4 out of 39) |
| Crate | 77 | 88.3% | 49.4% | 2.6% (2 out of 77) | 9.1% (7 out of 77) | 11.7% (9 out of 77) |

### Detailed F1 Scores by Granularity

| Granularity | CWE Macro F1 | Functions Macro F1 | Lines Macro F1 |
|-------------|--------------|-------------------|----------------|
| Function | 0.047 | 0.288 | 0.236 |
| File | 0.060 | 0.090 | 0.039 |
| Crate | 0.015 | 0.066 | 0.064 |

## Complete Granularity Analysis

*Analysis of vulnerabilities with all three granularities (function, file, crate) and valid JSON responses*

Found 15 vulnerabilities with all three granularities and valid JSON responses.

Total samples in complete analysis: 81 (27 samples per granularity)

| Granularity | Samples | Vulnerability Accuracy | At Least One CWE | At Least One Function | At Least One Line |
|-------------|---------|------------------------|------------------|----------------------|-------------------|
| Function | 27 | 63.0% | 11.1% (3 out of 27) | 37.0% (10 out of 27) | 25.9% (7 out of 27) |
| File | 27 | 59.3% | 7.4% (2 out of 27) | 22.2% (6 out of 27) | 14.8% (4 out of 27) |
| Crate | 27 | 55.6% | 3.7% (1 out of 27) | 14.8% (4 out of 27) | 18.5% (5 out of 27) |

### Detailed F1 Scores (Complete Granularity)

| Granularity | CWE Macro F1 | Functions Macro F1 | Lines Macro F1 |
|-------------|--------------|-------------------|----------------|
| Function | 0.074 | 0.276 | 0.198 |
| File | 0.043 | 0.130 | 0.056 |
| Crate | 0.025 | 0.114 | 0.074 |

## Performance by CWE Type

Total unique CWE types: 18

| CWE | Samples | JSON Validity | Vulnerability Accuracy | At Least One CWE | At Least One Function | At Least One Line |
|-----|---------|---------------|------------------------|------------------|----------------------|-------------------|
| CWE-416 | 13 | 92.3% | 76.9% | 23.1% (3 out of 13) | 15.4% (2 out of 13) | 30.8% (4 out of 13) |
| CWE-129 | 12 | 100.0% | 100.0% | 0.0% (0 out of 12) | 0.0% (0 out of 12) | 0.0% (0 out of 12) |
| CWE-362 | 11 | 90.9% | 90.9% | 9.1% (1 out of 11) | 9.1% (1 out of 11) | 18.2% (2 out of 11) |
| CWE-787 | 10 | 90.0% | 80.0% | 0.0% (0 out of 10) | 40.0% (4 out of 10) | 10.0% (1 out of 10) |
| CWE-908 | 9 | 100.0% | 100.0% | 0.0% (0 out of 9) | 66.7% (6 out of 9) | 66.7% (6 out of 9) |

### Detailed F1 Scores by CWE Type

| CWE | CWE F1 | Functions F1 | Lines F1 |
|-----|--------|--------------|----------|
| CWE-416 | 0.141 | 0.115 | 0.163 |
| CWE-129 | 0.000 | 0.000 | 0.000 |
| CWE-362 | 0.061 | 0.091 | 0.182 |
| CWE-787 | 0.000 | 0.400 | 0.029 |
| CWE-908 | 0.000 | 0.441 | 0.261 |

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
