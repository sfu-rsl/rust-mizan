# RustMizan Benchmark Analysis Report

- **Model:** claude-3-7-sonnet-20250219

**Mutations Applied:** None (vanilla dataset)

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
| JSON Validity Rate | 89.6% |
| Vulnerability Detection Accuracy | 53.2% |
| Samples with At Least One Correct CWE Identified | 9.2% (16 out of 173) |
| Samples with At Least One Correct Vulnerable Function Identified | 18.5% (32 out of 173) |
| Samples with At Least One Correct Vulnerable Line Identified | 17.9% (31 out of 173) |
| CWE Type Macro F1 | 0.075 |
| Vulnerable Functions Macro F1 | 0.158 |
| Vulnerable Lines Macro F1 | 0.142 |

## Analysis of Vulnerable Samples

*Analysis focused only on samples that contain vulnerabilities*

| Metric | Value |
|--------|-------|
| Total Vulnerable Samples | 95 |
| JSON Validity Rate | 92.6% |
| Vulnerability Detection Accuracy | 88.4% |
| At Least One Correct CWE Identified | 16.8% (16 out of 95) |
| At Least One Correct Vulnerable Function Identified | 33.7% (32 out of 95) |
| At Least One Correct Vulnerable Line Identified | 32.6% (31 out of 95) |

### Detailed F1 Scores (Vulnerable Samples Only)

| Metric | F1 Score |
|--------|----------|
| CWE Type Macro F1 | 0.133 |
| Vulnerable Functions Macro F1 | 0.279 |
| Vulnerable Lines Macro F1 | 0.250 |

## Analysis by Granularity

| Granularity | Samples | JSON Validity | Vulnerability Accuracy | At Least One CWE | At Least One Function | At Least One Line |
|-------------|---------|---------------|------------------------|------------------|----------------------|-------------------|
| Function | 57 | 98.2% | 61.4% | 14.0% (8 out of 57) | 29.8% (17 out of 57) | 28.1% (16 out of 57) |
| File | 39 | 94.9% | 53.8% | 7.7% (3 out of 39) | 17.9% (7 out of 39) | 15.4% (6 out of 39) |
| Crate | 77 | 80.5% | 46.8% | 6.5% (5 out of 77) | 10.4% (8 out of 77) | 11.7% (9 out of 77) |

### Detailed F1 Scores by Granularity

| Granularity | CWE Macro F1 | Functions Macro F1 | Lines Macro F1 |
|-------------|--------------|-------------------|----------------|
| Function | 0.102 | 0.250 | 0.233 |
| File | 0.060 | 0.107 | 0.062 |
| Crate | 0.045 | 0.079 | 0.082 |

## Complete Granularity Analysis

*Analysis of vulnerabilities with all three granularities (function, file, crate) and valid JSON responses*

Found 16 vulnerabilities with all three granularities and valid JSON responses.

Total samples in complete analysis: 87 (29 samples per granularity)

| Granularity | Samples | Vulnerability Accuracy | At Least One CWE | At Least One Function | At Least One Line |
|-------------|---------|------------------------|------------------|----------------------|-------------------|
| Function | 29 | 65.5% | 17.2% (5 out of 29) | 31.0% (9 out of 29) | 27.6% (8 out of 29) |
| File | 29 | 58.6% | 6.9% (2 out of 29) | 24.1% (7 out of 29) | 20.7% (6 out of 29) |
| Crate | 29 | 51.7% | 10.3% (3 out of 29) | 17.2% (5 out of 29) | 17.2% (5 out of 29) |

### Detailed F1 Scores (Complete Granularity)

| Granularity | CWE Macro F1 | Functions Macro F1 | Lines Macro F1 |
|-------------|--------------|-------------------|----------------|
| Function | 0.121 | 0.239 | 0.225 |
| File | 0.057 | 0.144 | 0.083 |
| Crate | 0.080 | 0.140 | 0.123 |

## Performance by CWE Type

Total unique CWE types: 18

| CWE | Samples | JSON Validity | Vulnerability Accuracy | At Least One CWE | At Least One Function | At Least One Line |
|-----|---------|---------------|------------------------|------------------|----------------------|-------------------|
| CWE-416 | 13 | 92.3% | 76.9% | 23.1% (3 out of 13) | 15.4% (2 out of 13) | 38.5% (5 out of 13) |
| CWE-129 | 12 | 100.0% | 100.0% | 0.0% (0 out of 12) | 0.0% (0 out of 12) | 0.0% (0 out of 12) |
| CWE-362 | 11 | 63.6% | 63.6% | 9.1% (1 out of 11) | 0.0% (0 out of 11) | 9.1% (1 out of 11) |
| CWE-787 | 10 | 100.0% | 80.0% | 40.0% (4 out of 10) | 40.0% (4 out of 10) | 20.0% (2 out of 10) |
| CWE-908 | 9 | 100.0% | 100.0% | 11.1% (1 out of 9) | 66.7% (6 out of 9) | 66.7% (6 out of 9) |

### Detailed F1 Scores by CWE Type

| CWE | CWE F1 | Functions F1 | Lines F1 |
|-----|--------|--------------|----------|
| CWE-416 | 0.179 | 0.090 | 0.269 |
| CWE-129 | 0.000 | 0.000 | 0.000 |
| CWE-362 | 0.061 | 0.000 | 0.091 |
| CWE-787 | 0.267 | 0.367 | 0.113 |
| CWE-908 | 0.056 | 0.478 | 0.259 |

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
