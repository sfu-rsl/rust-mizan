# RustMizan Benchmark Analysis Report

- **Model:** gpt-4.1-2025-04-14

**Mutations Applied:** malignant-rename-var

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
| JSON Validity Rate | 80.9% |
| Vulnerability Detection Accuracy | 46.8% |
| Samples with At Least One Correct CWE Identified | 9.8% (17 out of 173) |
| Samples with At Least One Correct Vulnerable Function Identified | 17.3% (30 out of 173) |
| Samples with At Least One Correct Vulnerable Line Identified | 16.2% (28 out of 173) |
| CWE Type Macro F1 | 0.059 |
| Vulnerable Functions Macro F1 | 0.152 |
| Vulnerable Lines Macro F1 | 0.121 |

## Analysis of Vulnerable Samples

*Analysis focused only on samples that contain vulnerabilities*

| Metric | Value |
|--------|-------|
| Total Vulnerable Samples | 95 |
| JSON Validity Rate | 83.2% |
| Vulnerability Detection Accuracy | 42.1% |
| At Least One Correct CWE Identified | 17.9% (17 out of 95) |
| At Least One Correct Vulnerable Function Identified | 31.6% (30 out of 95) |
| At Least One Correct Vulnerable Line Identified | 29.5% (28 out of 95) |

### Detailed F1 Scores (Vulnerable Samples Only)

| Metric | F1 Score |
|--------|----------|
| CWE Type Macro F1 | 0.105 |
| Vulnerable Functions Macro F1 | 0.269 |
| Vulnerable Lines Macro F1 | 0.214 |

## Analysis by Granularity

| Granularity | Samples | JSON Validity | Vulnerability Accuracy | At Least One CWE | At Least One Function | At Least One Line |
|-------------|---------|---------------|------------------------|------------------|----------------------|-------------------|
| Function | 57 | 100.0% | 63.2% | 8.8% (5 out of 57) | 26.3% (15 out of 57) | 26.3% (15 out of 57) |
| File | 39 | 69.2% | 35.9% | 7.7% (3 out of 39) | 15.4% (6 out of 39) | 12.8% (5 out of 39) |
| Crate | 77 | 72.7% | 40.3% | 11.7% (9 out of 77) | 11.7% (9 out of 77) | 10.4% (8 out of 77) |

### Detailed F1 Scores by Granularity

| Granularity | CWE Macro F1 | Functions Macro F1 | Lines Macro F1 |
|-------------|--------------|-------------------|----------------|
| Function | 0.053 | 0.223 | 0.200 |
| File | 0.047 | 0.095 | 0.069 |
| Crate | 0.045 | 0.063 | 0.037 |

## Complete Granularity Analysis

*Analysis of vulnerabilities with all three granularities (function, file, crate) and valid JSON responses*

Found 9 vulnerabilities with all three granularities and valid JSON responses.

Total samples in complete analysis: 45 (15 samples per granularity)

| Granularity | Samples | Vulnerability Accuracy | At Least One CWE | At Least One Function | At Least One Line |
|-------------|---------|------------------------|------------------|----------------------|-------------------|
| Function | 15 | 66.7% | 26.7% (4 out of 15) | 40.0% (6 out of 15) | 33.3% (5 out of 15) |
| File | 15 | 60.0% | 20.0% (3 out of 15) | 33.3% (5 out of 15) | 26.7% (4 out of 15) |
| Crate | 15 | 53.3% | 26.7% (4 out of 15) | 26.7% (4 out of 15) | 6.7% (1 out of 15) |

### Detailed F1 Scores (Complete Granularity)

| Granularity | CWE Macro F1 | Functions Macro F1 | Lines Macro F1 |
|-------------|--------------|-------------------|----------------|
| Function | 0.156 | 0.344 | 0.261 |
| File | 0.122 | 0.203 | 0.134 |
| Crate | 0.127 | 0.143 | 0.033 |

## Performance by CWE Type

Total unique CWE types: 18

| CWE | Samples | JSON Validity | Vulnerability Accuracy | At Least One CWE | At Least One Function | At Least One Line |
|-----|---------|---------------|------------------------|------------------|----------------------|-------------------|
| CWE-416 | 13 | 76.9% | 23.1% | 23.1% (3 out of 13) | 23.1% (3 out of 13) | 23.1% (3 out of 13) |
| CWE-129 | 12 | 41.7% | 25.0% | 0.0% (0 out of 12) | 16.7% (2 out of 12) | 16.7% (2 out of 12) |
| CWE-362 | 11 | 90.9% | 36.4% | 9.1% (1 out of 11) | 9.1% (1 out of 11) | 18.2% (2 out of 11) |
| CWE-787 | 10 | 90.0% | 10.0% | 0.0% (0 out of 10) | 10.0% (1 out of 10) | 0.0% (0 out of 10) |
| CWE-908 | 9 | 88.9% | 77.8% | 33.3% (3 out of 9) | 66.7% (6 out of 9) | 55.6% (5 out of 9) |

### Detailed F1 Scores by CWE Type

| CWE | CWE F1 | Functions F1 | Lines F1 |
|-----|--------|--------------|----------|
| CWE-416 | 0.141 | 0.198 | 0.182 |
| CWE-129 | 0.000 | 0.083 | 0.081 |
| CWE-362 | 0.018 | 0.030 | 0.121 |
| CWE-787 | 0.000 | 0.100 | 0.000 |
| CWE-908 | 0.104 | 0.428 | 0.227 |

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
