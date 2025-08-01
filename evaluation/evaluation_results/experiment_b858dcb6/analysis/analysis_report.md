# RustMizan Benchmark Analysis Report

- **Model:** gpt-4.1-2025-04-14

**Mutations Applied:** mizan-mut-derive-reorder

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
| JSON Validity Rate | 79.2% |
| Vulnerability Detection Accuracy | 43.4% |
| Samples with At Least One Correct CWE Identified | 7.5% (13 out of 173) |
| Samples with At Least One Correct Vulnerable Function Identified | 12.1% (21 out of 173) |
| Samples with At Least One Correct Vulnerable Line Identified | 11.0% (19 out of 173) |
| CWE Type Macro F1 | 0.048 |
| Vulnerable Functions Macro F1 | 0.114 |
| Vulnerable Lines Macro F1 | 0.098 |

## Analysis of Vulnerable Samples

*Analysis focused only on samples that contain vulnerabilities*

| Metric | Value |
|--------|-------|
| Total Vulnerable Samples | 95 |
| JSON Validity Rate | 76.8% |
| Vulnerability Detection Accuracy | 33.7% |
| At Least One Correct CWE Identified | 13.7% (13 out of 95) |
| At Least One Correct Vulnerable Function Identified | 22.1% (21 out of 95) |
| At Least One Correct Vulnerable Line Identified | 20.0% (19 out of 95) |

### Detailed F1 Scores (Vulnerable Samples Only)

| Metric | F1 Score |
|--------|----------|
| CWE Type Macro F1 | 0.091 |
| Vulnerable Functions Macro F1 | 0.214 |
| Vulnerable Lines Macro F1 | 0.183 |

## Analysis by Granularity

| Granularity | Samples | JSON Validity | Vulnerability Accuracy | At Least One CWE | At Least One Function | At Least One Line |
|-------------|---------|---------------|------------------------|------------------|----------------------|-------------------|
| Function | 57 | 100.0% | 59.6% | 15.8% (9 out of 57) | 21.1% (12 out of 57) | 19.3% (11 out of 57) |
| File | 39 | 71.8% | 35.9% | 2.6% (1 out of 39) | 7.7% (3 out of 39) | 5.1% (2 out of 39) |
| Crate | 77 | 67.5% | 35.1% | 3.9% (3 out of 77) | 7.8% (6 out of 77) | 7.8% (6 out of 77) |

### Detailed F1 Scores by Granularity

| Granularity | CWE Macro F1 | Functions Macro F1 | Lines Macro F1 |
|-------------|--------------|-------------------|----------------|
| Function | 0.091 | 0.191 | 0.175 |
| File | 0.013 | 0.039 | 0.017 |
| Crate | 0.012 | 0.042 | 0.035 |

## Complete Granularity Analysis

*Analysis of vulnerabilities with all three granularities (function, file, crate) and valid JSON responses*

Found 8 vulnerabilities with all three granularities and valid JSON responses.

Total samples in complete analysis: 39 (13 samples per granularity)

| Granularity | Samples | Vulnerability Accuracy | At Least One CWE | At Least One Function | At Least One Line |
|-------------|---------|------------------------|------------------|----------------------|-------------------|
| Function | 13 | 69.2% | 23.1% (3 out of 13) | 30.8% (4 out of 13) | 15.4% (2 out of 13) |
| File | 13 | 53.8% | 7.7% (1 out of 13) | 23.1% (3 out of 13) | 15.4% (2 out of 13) |
| Crate | 13 | 30.8% | 0.0% (0 out of 13) | 7.7% (1 out of 13) | 0.0% (0 out of 13) |

### Detailed F1 Scores (Complete Granularity)

| Granularity | CWE Macro F1 | Functions Macro F1 | Lines Macro F1 |
|-------------|--------------|-------------------|----------------|
| Function | 0.141 | 0.236 | 0.115 |
| File | 0.038 | 0.117 | 0.052 |
| Crate | 0.000 | 0.026 | 0.000 |

## Performance by CWE Type

Total unique CWE types: 18

| CWE | Samples | JSON Validity | Vulnerability Accuracy | At Least One CWE | At Least One Function | At Least One Line |
|-----|---------|---------------|------------------------|------------------|----------------------|-------------------|
| CWE-416 | 13 | 76.9% | 15.4% | 7.7% (1 out of 13) | 7.7% (1 out of 13) | 7.7% (1 out of 13) |
| CWE-129 | 12 | 41.7% | 16.7% | 0.0% (0 out of 12) | 0.0% (0 out of 12) | 0.0% (0 out of 12) |
| CWE-362 | 11 | 72.7% | 27.3% | 9.1% (1 out of 11) | 9.1% (1 out of 11) | 18.2% (2 out of 11) |
| CWE-787 | 10 | 90.0% | 20.0% | 0.0% (0 out of 10) | 10.0% (1 out of 10) | 0.0% (0 out of 10) |
| CWE-908 | 9 | 77.8% | 77.8% | 22.2% (2 out of 9) | 66.7% (6 out of 9) | 33.3% (3 out of 9) |

### Detailed F1 Scores by CWE Type

| CWE | CWE F1 | Functions F1 | Lines F1 |
|-----|--------|--------------|----------|
| CWE-416 | 0.051 | 0.062 | 0.077 |
| CWE-129 | 0.000 | 0.000 | 0.000 |
| CWE-362 | 0.020 | 0.040 | 0.111 |
| CWE-787 | 0.000 | 0.100 | 0.000 |
| CWE-908 | 0.111 | 0.415 | 0.237 |

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
