# RustMizan Benchmark Analysis Report

- **Model:** claude-3-7-sonnet-20250219

**Mutations Applied:** mizan-mut-if-else-reorder

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
| JSON Validity Rate | 94.8% |
| Vulnerability Detection Accuracy | 56.6% |
| Samples with At Least One Correct CWE Identified | 8.1% (14 out of 173) |
| Samples with At Least One Correct Vulnerable Function Identified | 20.2% (35 out of 173) |
| Samples with At Least One Correct Vulnerable Line Identified | 19.1% (33 out of 173) |
| CWE Type Macro F1 | 0.062 |
| Vulnerable Functions Macro F1 | 0.162 |
| Vulnerable Lines Macro F1 | 0.138 |

## Analysis of Vulnerable Samples

*Analysis focused only on samples that contain vulnerabilities*

| Metric | Value |
|--------|-------|
| Total Vulnerable Samples | 95 |
| JSON Validity Rate | 96.8% |
| Vulnerability Detection Accuracy | 92.6% |
| At Least One Correct CWE Identified | 14.7% (14 out of 95) |
| At Least One Correct Vulnerable Function Identified | 36.8% (35 out of 95) |
| At Least One Correct Vulnerable Line Identified | 34.7% (33 out of 95) |

### Detailed F1 Scores (Vulnerable Samples Only)

| Metric | F1 Score |
|--------|----------|
| CWE Type Macro F1 | 0.111 |
| Vulnerable Functions Macro F1 | 0.289 |
| Vulnerable Lines Macro F1 | 0.246 |

## Analysis by Granularity

| Granularity | Samples | JSON Validity | Vulnerability Accuracy | At Least One CWE | At Least One Function | At Least One Line |
|-------------|---------|---------------|------------------------|------------------|----------------------|-------------------|
| Function | 57 | 98.2% | 64.9% | 8.8% (5 out of 57) | 31.6% (18 out of 57) | 31.6% (18 out of 57) |
| File | 39 | 97.4% | 56.4% | 5.1% (2 out of 39) | 17.9% (7 out of 39) | 12.8% (5 out of 39) |
| Crate | 77 | 90.9% | 50.6% | 9.1% (7 out of 77) | 13.0% (10 out of 77) | 13.0% (10 out of 77) |

### Detailed F1 Scores by Granularity

| Granularity | CWE Macro F1 | Functions Macro F1 | Lines Macro F1 |
|-------------|--------------|-------------------|----------------|
| Function | 0.058 | 0.280 | 0.255 |
| File | 0.038 | 0.114 | 0.062 |
| Crate | 0.070 | 0.081 | 0.074 |

## Complete Granularity Analysis

*Analysis of vulnerabilities with all three granularities (function, file, crate) and valid JSON responses*

Found 15 vulnerabilities with all three granularities and valid JSON responses.

Total samples in complete analysis: 81 (27 samples per granularity)

| Granularity | Samples | Vulnerability Accuracy | At Least One CWE | At Least One Function | At Least One Line |
|-------------|---------|------------------------|------------------|----------------------|-------------------|
| Function | 27 | 59.3% | 11.1% (3 out of 27) | 33.3% (9 out of 27) | 29.6% (8 out of 27) |
| File | 27 | 63.0% | 7.4% (2 out of 27) | 25.9% (7 out of 27) | 18.5% (5 out of 27) |
| Crate | 27 | 55.6% | 11.1% (3 out of 27) | 18.5% (5 out of 27) | 14.8% (4 out of 27) |

### Detailed F1 Scores (Complete Granularity)

| Granularity | CWE Macro F1 | Functions Macro F1 | Lines Macro F1 |
|-------------|--------------|-------------------|----------------|
| Function | 0.074 | 0.276 | 0.252 |
| File | 0.056 | 0.164 | 0.089 |
| Crate | 0.111 | 0.118 | 0.094 |

## Performance by CWE Type

Total unique CWE types: 18

| CWE | Samples | JSON Validity | Vulnerability Accuracy | At Least One CWE | At Least One Function | At Least One Line |
|-----|---------|---------------|------------------------|------------------|----------------------|-------------------|
| CWE-416 | 13 | 92.3% | 92.3% | 23.1% (3 out of 13) | 30.8% (4 out of 13) | 38.5% (5 out of 13) |
| CWE-129 | 12 | 100.0% | 91.7% | 8.3% (1 out of 12) | 8.3% (1 out of 12) | 8.3% (1 out of 12) |
| CWE-362 | 11 | 90.9% | 90.9% | 0.0% (0 out of 11) | 18.2% (2 out of 11) | 18.2% (2 out of 11) |
| CWE-787 | 10 | 100.0% | 80.0% | 20.0% (2 out of 10) | 40.0% (4 out of 10) | 10.0% (1 out of 10) |
| CWE-908 | 9 | 100.0% | 100.0% | 11.1% (1 out of 9) | 66.7% (6 out of 9) | 66.7% (6 out of 9) |

### Detailed F1 Scores by CWE Type

| CWE | CWE F1 | Functions F1 | Lines F1 |
|-----|--------|--------------|----------|
| CWE-416 | 0.179 | 0.197 | 0.274 |
| CWE-129 | 0.083 | 0.083 | 0.056 |
| CWE-362 | 0.000 | 0.152 | 0.127 |
| CWE-787 | 0.167 | 0.400 | 0.100 |
| CWE-908 | 0.032 | 0.441 | 0.239 |

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
