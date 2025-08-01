# RustMizan Benchmark Analysis Report

- **Model:** gpt-4.1-2025-04-14

**Mutations Applied:** benign-comments

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
| JSON Validity Rate | 80.3% |
| Vulnerability Detection Accuracy | 48.0% |
| Samples with At Least One Correct CWE Identified | 7.5% (13 out of 173) |
| Samples with At Least One Correct Vulnerable Function Identified | 15.0% (26 out of 173) |
| Samples with At Least One Correct Vulnerable Line Identified | 15.0% (26 out of 173) |
| CWE Type Macro F1 | 0.051 |
| Vulnerable Functions Macro F1 | 0.138 |
| Vulnerable Lines Macro F1 | 0.125 |

## Analysis of Vulnerable Samples

*Analysis focused only on samples that contain vulnerabilities*

| Metric | Value |
|--------|-------|
| Total Vulnerable Samples | 95 |
| JSON Validity Rate | 81.1% |
| Vulnerability Detection Accuracy | 38.9% |
| At Least One Correct CWE Identified | 13.7% (13 out of 95) |
| At Least One Correct Vulnerable Function Identified | 27.4% (26 out of 95) |
| At Least One Correct Vulnerable Line Identified | 27.4% (26 out of 95) |

### Detailed F1 Scores (Vulnerable Samples Only)

| Metric | F1 Score |
|--------|----------|
| CWE Type Macro F1 | 0.093 |
| Vulnerable Functions Macro F1 | 0.249 |
| Vulnerable Lines Macro F1 | 0.226 |

## Analysis by Granularity

| Granularity | Samples | JSON Validity | Vulnerability Accuracy | At Least One CWE | At Least One Function | At Least One Line |
|-------------|---------|---------------|------------------------|------------------|----------------------|-------------------|
| Function | 57 | 98.2% | 63.2% | 8.8% (5 out of 57) | 22.8% (13 out of 57) | 22.8% (13 out of 57) |
| File | 39 | 66.7% | 35.9% | 5.1% (2 out of 39) | 10.3% (4 out of 39) | 5.1% (2 out of 39) |
| Crate | 77 | 74.0% | 42.9% | 7.8% (6 out of 77) | 11.7% (9 out of 77) | 14.3% (11 out of 77) |

### Detailed F1 Scores by Granularity

| Granularity | CWE Macro F1 | Functions Macro F1 | Lines Macro F1 |
|-------------|--------------|-------------------|----------------|
| Function | 0.050 | 0.185 | 0.185 |
| File | 0.034 | 0.047 | 0.019 |
| Crate | 0.039 | 0.087 | 0.080 |

## Complete Granularity Analysis

*Analysis of vulnerabilities with all three granularities (function, file, crate) and valid JSON responses*

Found 8 vulnerabilities with all three granularities and valid JSON responses.

Total samples in complete analysis: 39 (13 samples per granularity)

| Granularity | Samples | Vulnerability Accuracy | At Least One CWE | At Least One Function | At Least One Line |
|-------------|---------|------------------------|------------------|----------------------|-------------------|
| Function | 13 | 61.5% | 15.4% (2 out of 13) | 38.5% (5 out of 13) | 23.1% (3 out of 13) |
| File | 13 | 53.8% | 15.4% (2 out of 13) | 30.8% (4 out of 13) | 15.4% (2 out of 13) |
| Crate | 13 | 61.5% | 23.1% (3 out of 13) | 30.8% (4 out of 13) | 23.1% (3 out of 13) |

### Detailed F1 Scores (Complete Granularity)

| Granularity | CWE Macro F1 | Functions Macro F1 | Lines Macro F1 |
|-------------|--------------|-------------------|----------------|
| Function | 0.090 | 0.291 | 0.192 |
| File | 0.103 | 0.142 | 0.058 |
| Crate | 0.121 | 0.244 | 0.167 |

## Performance by CWE Type

Total unique CWE types: 18

| CWE | Samples | JSON Validity | Vulnerability Accuracy | At Least One CWE | At Least One Function | At Least One Line |
|-----|---------|---------------|------------------------|------------------|----------------------|-------------------|
| CWE-416 | 13 | 76.9% | 7.7% | 7.7% (1 out of 13) | 0.0% (0 out of 13) | 7.7% (1 out of 13) |
| CWE-129 | 12 | 41.7% | 16.7% | 0.0% (0 out of 12) | 8.3% (1 out of 12) | 8.3% (1 out of 12) |
| CWE-362 | 11 | 90.9% | 18.2% | 0.0% (0 out of 11) | 0.0% (0 out of 11) | 18.2% (2 out of 11) |
| CWE-787 | 10 | 80.0% | 30.0% | 10.0% (1 out of 10) | 10.0% (1 out of 10) | 10.0% (1 out of 10) |
| CWE-908 | 9 | 88.9% | 77.8% | 11.1% (1 out of 9) | 66.7% (6 out of 9) | 33.3% (3 out of 9) |

### Detailed F1 Scores by CWE Type

| CWE | CWE F1 | Functions F1 | Lines F1 |
|-----|--------|--------------|----------|
| CWE-416 | 0.051 | 0.000 | 0.077 |
| CWE-129 | 0.000 | 0.042 | 0.048 |
| CWE-362 | 0.000 | 0.000 | 0.143 |
| CWE-787 | 0.050 | 0.100 | 0.050 |
| CWE-908 | 0.037 | 0.452 | 0.222 |

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
