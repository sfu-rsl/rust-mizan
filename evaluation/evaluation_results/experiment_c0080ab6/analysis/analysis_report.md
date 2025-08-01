# RustMizan Benchmark Analysis Report

- **Model:** claude-3-7-sonnet-20250219

**Mutations Applied:** malignant-blocks

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
| JSON Validity Rate | 91.9% |
| Vulnerability Detection Accuracy | 57.8% |
| Samples with At Least One Correct CWE Identified | 9.8% (17 out of 173) |
| Samples with At Least One Correct Vulnerable Function Identified | 10.4% (18 out of 173) |
| Samples with At Least One Correct Vulnerable Line Identified | 10.4% (18 out of 173) |
| CWE Type Macro F1 | 0.060 |
| Vulnerable Functions Macro F1 | 0.071 |
| Vulnerable Lines Macro F1 | 0.038 |

## Analysis of Vulnerable Samples

*Analysis focused only on samples that contain vulnerabilities*

| Metric | Value |
|--------|-------|
| Total Vulnerable Samples | 95 |
| JSON Validity Rate | 96.8% |
| Vulnerability Detection Accuracy | 94.7% |
| At Least One Correct CWE Identified | 17.9% (17 out of 95) |
| At Least One Correct Vulnerable Function Identified | 18.9% (18 out of 95) |
| At Least One Correct Vulnerable Line Identified | 18.9% (18 out of 95) |

### Detailed F1 Scores (Vulnerable Samples Only)

| Metric | F1 Score |
|--------|----------|
| CWE Type Macro F1 | 0.103 |
| Vulnerable Functions Macro F1 | 0.122 |
| Vulnerable Lines Macro F1 | 0.065 |

## Analysis by Granularity

| Granularity | Samples | JSON Validity | Vulnerability Accuracy | At Least One CWE | At Least One Function | At Least One Line |
|-------------|---------|---------------|------------------------|------------------|----------------------|-------------------|
| Function | 57 | 100.0% | 66.7% | 10.5% (6 out of 57) | 12.3% (7 out of 57) | 17.5% (10 out of 57) |
| File | 39 | 97.4% | 59.0% | 12.8% (5 out of 39) | 10.3% (4 out of 39) | 5.1% (2 out of 39) |
| Crate | 77 | 83.1% | 50.6% | 7.8% (6 out of 77) | 9.1% (7 out of 77) | 7.8% (6 out of 77) |

### Detailed F1 Scores by Granularity

| Granularity | CWE Macro F1 | Functions Macro F1 | Lines Macro F1 |
|-------------|--------------|-------------------|----------------|
| Function | 0.056 | 0.076 | 0.060 |
| File | 0.073 | 0.064 | 0.018 |
| Crate | 0.045 | 0.057 | 0.025 |

## Complete Granularity Analysis

*Analysis of vulnerabilities with all three granularities (function, file, crate) and valid JSON responses*

Found 16 vulnerabilities with all three granularities and valid JSON responses.

Total samples in complete analysis: 87 (29 samples per granularity)

| Granularity | Samples | Vulnerability Accuracy | At Least One CWE | At Least One Function | At Least One Line |
|-------------|---------|------------------------|------------------|----------------------|-------------------|
| Function | 29 | 65.5% | 10.3% (3 out of 29) | 6.9% (2 out of 29) | 10.3% (3 out of 29) |
| File | 29 | 65.5% | 13.8% (4 out of 29) | 13.8% (4 out of 29) | 6.9% (2 out of 29) |
| Crate | 29 | 55.2% | 6.9% (2 out of 29) | 13.8% (4 out of 29) | 6.9% (2 out of 29) |

### Detailed F1 Scores (Complete Granularity)

| Granularity | CWE Macro F1 | Functions Macro F1 | Lines Macro F1 |
|-------------|--------------|-------------------|----------------|
| Function | 0.052 | 0.040 | 0.047 |
| File | 0.075 | 0.086 | 0.025 |
| Crate | 0.037 | 0.093 | 0.021 |

## Performance by CWE Type

Total unique CWE types: 18

| CWE | Samples | JSON Validity | Vulnerability Accuracy | At Least One CWE | At Least One Function | At Least One Line |
|-----|---------|---------------|------------------------|------------------|----------------------|-------------------|
| CWE-416 | 13 | 92.3% | 84.6% | 0.0% (0 out of 13) | 7.7% (1 out of 13) | 7.7% (1 out of 13) |
| CWE-129 | 12 | 100.0% | 100.0% | 0.0% (0 out of 12) | 16.7% (2 out of 12) | 8.3% (1 out of 12) |
| CWE-362 | 11 | 90.9% | 90.9% | 0.0% (0 out of 11) | 0.0% (0 out of 11) | 18.2% (2 out of 11) |
| CWE-787 | 10 | 100.0% | 100.0% | 90.0% (9 out of 10) | 0.0% (0 out of 10) | 0.0% (0 out of 10) |
| CWE-908 | 9 | 100.0% | 88.9% | 0.0% (0 out of 9) | 66.7% (6 out of 9) | 33.3% (3 out of 9) |

### Detailed F1 Scores by CWE Type

| CWE | CWE F1 | Functions F1 | Lines F1 |
|-----|--------|--------------|----------|
| CWE-416 | 0.000 | 0.038 | 0.015 |
| CWE-129 | 0.000 | 0.167 | 0.056 |
| CWE-362 | 0.000 | 0.000 | 0.029 |
| CWE-787 | 0.523 | 0.000 | 0.000 |
| CWE-908 | 0.000 | 0.298 | 0.080 |

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
