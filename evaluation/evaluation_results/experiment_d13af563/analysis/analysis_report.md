# RustMizan Benchmark Analysis Report

- **Model:** claude-3-7-sonnet-20250219

**Mutations Applied:** format-expanded

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
| JSON Validity Rate | 85.5% |
| Vulnerability Detection Accuracy | 49.1% |
| Samples with At Least One Correct CWE Identified | 8.1% (14 out of 173) |
| Samples with At Least One Correct Vulnerable Function Identified | 18.5% (32 out of 173) |
| Samples with At Least One Correct Vulnerable Line Identified | 21.4% (37 out of 173) |
| CWE Type Macro F1 | 0.071 |
| Vulnerable Functions Macro F1 | 0.176 |
| Vulnerable Lines Macro F1 | 0.176 |

## Analysis of Vulnerable Samples

*Analysis focused only on samples that contain vulnerabilities*

| Metric | Value |
|--------|-------|
| Total Vulnerable Samples | 95 |
| JSON Validity Rate | 85.3% |
| Vulnerability Detection Accuracy | 81.1% |
| At Least One Correct CWE Identified | 14.7% (14 out of 95) |
| At Least One Correct Vulnerable Function Identified | 33.7% (32 out of 95) |
| At Least One Correct Vulnerable Line Identified | 38.9% (37 out of 95) |

### Detailed F1 Scores (Vulnerable Samples Only)

| Metric | F1 Score |
|--------|----------|
| CWE Type Macro F1 | 0.130 |
| Vulnerable Functions Macro F1 | 0.321 |
| Vulnerable Lines Macro F1 | 0.321 |

## Analysis by Granularity

| Granularity | Samples | JSON Validity | Vulnerability Accuracy | At Least One CWE | At Least One Function | At Least One Line |
|-------------|---------|---------------|------------------------|------------------|----------------------|-------------------|
| Function | 57 | 98.2% | 63.2% | 14.0% (8 out of 57) | 29.8% (17 out of 57) | 35.1% (20 out of 57) |
| File | 39 | 100.0% | 48.7% | 10.3% (4 out of 39) | 12.8% (5 out of 39) | 17.9% (7 out of 39) |
| Crate | 77 | 68.8% | 39.0% | 2.6% (2 out of 77) | 13.0% (10 out of 77) | 13.0% (10 out of 77) |

### Detailed F1 Scores by Granularity

| Granularity | CWE Macro F1 | Functions Macro F1 | Lines Macro F1 |
|-------------|--------------|-------------------|----------------|
| Function | 0.105 | 0.253 | 0.281 |
| File | 0.073 | 0.081 | 0.084 |
| Crate | 0.022 | 0.109 | 0.087 |

## Complete Granularity Analysis

*Analysis of vulnerabilities with all three granularities (function, file, crate) and valid JSON responses*

Found 16 vulnerabilities with all three granularities and valid JSON responses.

Total samples in complete analysis: 87 (29 samples per granularity)

| Granularity | Samples | Vulnerability Accuracy | At Least One CWE | At Least One Function | At Least One Line |
|-------------|---------|------------------------|------------------|----------------------|-------------------|
| Function | 29 | 65.5% | 17.2% (5 out of 29) | 31.0% (9 out of 29) | 31.0% (9 out of 29) |
| File | 29 | 55.2% | 10.3% (3 out of 29) | 17.2% (5 out of 29) | 24.1% (7 out of 29) |
| Crate | 29 | 58.6% | 3.4% (1 out of 29) | 17.2% (5 out of 29) | 17.2% (5 out of 29) |

### Detailed F1 Scores (Complete Granularity)

| Granularity | CWE Macro F1 | Functions Macro F1 | Lines Macro F1 |
|-------------|--------------|-------------------|----------------|
| Function | 0.138 | 0.222 | 0.255 |
| File | 0.075 | 0.109 | 0.112 |
| Crate | 0.034 | 0.152 | 0.120 |

## Performance by CWE Type

Total unique CWE types: 18

| CWE | Samples | JSON Validity | Vulnerability Accuracy | At Least One CWE | At Least One Function | At Least One Line |
|-----|---------|---------------|------------------------|------------------|----------------------|-------------------|
| CWE-416 | 13 | 76.9% | 76.9% | 38.5% (5 out of 13) | 7.7% (1 out of 13) | 46.2% (6 out of 13) |
| CWE-129 | 12 | 100.0% | 100.0% | 0.0% (0 out of 12) | 0.0% (0 out of 12) | 0.0% (0 out of 12) |
| CWE-362 | 11 | 45.5% | 45.5% | 9.1% (1 out of 11) | 0.0% (0 out of 11) | 9.1% (1 out of 11) |
| CWE-787 | 10 | 90.0% | 70.0% | 10.0% (1 out of 10) | 50.0% (5 out of 10) | 30.0% (3 out of 10) |
| CWE-908 | 9 | 100.0% | 88.9% | 11.1% (1 out of 9) | 55.6% (5 out of 9) | 55.6% (5 out of 9) |

### Detailed F1 Scores by CWE Type

| CWE | CWE F1 | Functions F1 | Lines F1 |
|-----|--------|--------------|----------|
| CWE-416 | 0.308 | 0.038 | 0.317 |
| CWE-129 | 0.000 | 0.000 | 0.000 |
| CWE-362 | 0.061 | 0.000 | 0.091 |
| CWE-787 | 0.067 | 0.500 | 0.217 |
| CWE-908 | 0.074 | 0.396 | 0.273 |

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
