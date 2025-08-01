# RustMizan Benchmark Analysis Report

- **Model:** claude-3-7-sonnet-20250219

**Mutations Applied:** malignant-rename-fn

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
| JSON Validity Rate | 90.2% |
| Vulnerability Detection Accuracy | 53.8% |
| Samples with At Least One Correct CWE Identified | 8.7% (15 out of 173) |
| Samples with At Least One Correct Vulnerable Function Identified | 18.5% (32 out of 173) |
| Samples with At Least One Correct Vulnerable Line Identified | 18.5% (32 out of 173) |
| CWE Type Macro F1 | 0.068 |
| Vulnerable Functions Macro F1 | 0.158 |
| Vulnerable Lines Macro F1 | 0.140 |

## Analysis of Vulnerable Samples

*Analysis focused only on samples that contain vulnerabilities*

| Metric | Value |
|--------|-------|
| Total Vulnerable Samples | 95 |
| JSON Validity Rate | 93.7% |
| Vulnerability Detection Accuracy | 89.5% |
| At Least One Correct CWE Identified | 15.8% (15 out of 95) |
| At Least One Correct Vulnerable Function Identified | 33.7% (32 out of 95) |
| At Least One Correct Vulnerable Line Identified | 33.7% (32 out of 95) |

### Detailed F1 Scores (Vulnerable Samples Only)

| Metric | F1 Score |
|--------|----------|
| CWE Type Macro F1 | 0.120 |
| Vulnerable Functions Macro F1 | 0.276 |
| Vulnerable Lines Macro F1 | 0.246 |

## Analysis by Granularity

| Granularity | Samples | JSON Validity | Vulnerability Accuracy | At Least One CWE | At Least One Function | At Least One Line |
|-------------|---------|---------------|------------------------|------------------|----------------------|-------------------|
| Function | 57 | 98.2% | 63.2% | 12.3% (7 out of 57) | 31.6% (18 out of 57) | 28.1% (16 out of 57) |
| File | 39 | 97.4% | 51.3% | 10.3% (4 out of 39) | 15.4% (6 out of 39) | 15.4% (6 out of 39) |
| Crate | 77 | 80.5% | 48.1% | 5.2% (4 out of 77) | 10.4% (8 out of 77) | 13.0% (10 out of 77) |

### Detailed F1 Scores by Granularity

| Granularity | CWE Macro F1 | Functions Macro F1 | Lines Macro F1 |
|-------------|--------------|-------------------|----------------|
| Function | 0.094 | 0.263 | 0.233 |
| File | 0.073 | 0.099 | 0.071 |
| Crate | 0.032 | 0.074 | 0.076 |

## Complete Granularity Analysis

*Analysis of vulnerabilities with all three granularities (function, file, crate) and valid JSON responses*

Found 16 vulnerabilities with all three granularities and valid JSON responses.

Total samples in complete analysis: 87 (29 samples per granularity)

| Granularity | Samples | Vulnerability Accuracy | At Least One CWE | At Least One Function | At Least One Line |
|-------------|---------|------------------------|------------------|----------------------|-------------------|
| Function | 29 | 65.5% | 17.2% (5 out of 29) | 34.5% (10 out of 29) | 24.1% (7 out of 29) |
| File | 29 | 55.2% | 10.3% (3 out of 29) | 20.7% (6 out of 29) | 20.7% (6 out of 29) |
| Crate | 29 | 55.2% | 6.9% (2 out of 29) | 17.2% (5 out of 29) | 20.7% (6 out of 29) |

### Detailed F1 Scores (Complete Granularity)

| Granularity | CWE Macro F1 | Functions Macro F1 | Lines Macro F1 |
|-------------|--------------|-------------------|----------------|
| Function | 0.126 | 0.242 | 0.206 |
| File | 0.080 | 0.133 | 0.095 |
| Crate | 0.046 | 0.129 | 0.111 |

## Performance by CWE Type

Total unique CWE types: 18

| CWE | Samples | JSON Validity | Vulnerability Accuracy | At Least One CWE | At Least One Function | At Least One Line |
|-----|---------|---------------|------------------------|------------------|----------------------|-------------------|
| CWE-416 | 13 | 92.3% | 84.6% | 30.8% (4 out of 13) | 15.4% (2 out of 13) | 38.5% (5 out of 13) |
| CWE-129 | 12 | 100.0% | 100.0% | 8.3% (1 out of 12) | 0.0% (0 out of 12) | 0.0% (0 out of 12) |
| CWE-362 | 11 | 81.8% | 81.8% | 9.1% (1 out of 11) | 0.0% (0 out of 11) | 9.1% (1 out of 11) |
| CWE-787 | 10 | 100.0% | 80.0% | 20.0% (2 out of 10) | 40.0% (4 out of 10) | 20.0% (2 out of 10) |
| CWE-908 | 9 | 100.0% | 100.0% | 0.0% (0 out of 9) | 66.7% (6 out of 9) | 55.6% (5 out of 9) |

### Detailed F1 Scores by CWE Type

| CWE | CWE F1 | Functions F1 | Lines F1 |
|-----|--------|--------------|----------|
| CWE-416 | 0.218 | 0.090 | 0.231 |
| CWE-129 | 0.056 | 0.000 | 0.000 |
| CWE-362 | 0.061 | 0.000 | 0.091 |
| CWE-787 | 0.133 | 0.400 | 0.113 |
| CWE-908 | 0.000 | 0.441 | 0.236 |

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
