# RustMizan Benchmark Analysis Report

- **Model:** claude-3-7-sonnet-20250219

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
| JSON Validity Rate | 95.4% |
| Vulnerability Detection Accuracy | 55.5% |
| Samples with At Least One Correct CWE Identified | 5.8% (10 out of 173) |
| Samples with At Least One Correct Vulnerable Function Identified | 21.4% (37 out of 173) |
| Samples with At Least One Correct Vulnerable Line Identified | 19.1% (33 out of 173) |
| CWE Type Macro F1 | 0.041 |
| Vulnerable Functions Macro F1 | 0.170 |
| Vulnerable Lines Macro F1 | 0.132 |

## Analysis of Vulnerable Samples

*Analysis focused only on samples that contain vulnerabilities*

| Metric | Value |
|--------|-------|
| Total Vulnerable Samples | 95 |
| JSON Validity Rate | 96.8% |
| Vulnerability Detection Accuracy | 93.7% |
| At Least One Correct CWE Identified | 10.5% (10 out of 95) |
| At Least One Correct Vulnerable Function Identified | 38.9% (37 out of 95) |
| At Least One Correct Vulnerable Line Identified | 34.7% (33 out of 95) |

### Detailed F1 Scores (Vulnerable Samples Only)

| Metric | F1 Score |
|--------|----------|
| CWE Type Macro F1 | 0.073 |
| Vulnerable Functions Macro F1 | 0.305 |
| Vulnerable Lines Macro F1 | 0.237 |

## Analysis by Granularity

| Granularity | Samples | JSON Validity | Vulnerability Accuracy | At Least One CWE | At Least One Function | At Least One Line |
|-------------|---------|---------------|------------------------|------------------|----------------------|-------------------|
| Function | 57 | 100.0% | 63.2% | 8.8% (5 out of 57) | 31.6% (18 out of 57) | 31.6% (18 out of 57) |
| File | 39 | 100.0% | 53.8% | 7.7% (3 out of 39) | 17.9% (7 out of 39) | 12.8% (5 out of 39) |
| Crate | 77 | 89.6% | 50.6% | 2.6% (2 out of 77) | 15.6% (12 out of 77) | 13.0% (10 out of 77) |

### Detailed F1 Scores by Granularity

| Granularity | CWE Macro F1 | Functions Macro F1 | Lines Macro F1 |
|-------------|--------------|-------------------|----------------|
| Function | 0.058 | 0.280 | 0.243 |
| File | 0.060 | 0.118 | 0.062 |
| Crate | 0.014 | 0.098 | 0.073 |

## Complete Granularity Analysis

*Analysis of vulnerabilities with all three granularities (function, file, crate) and valid JSON responses*

Found 16 vulnerabilities with all three granularities and valid JSON responses.

Total samples in complete analysis: 87 (29 samples per granularity)

| Granularity | Samples | Vulnerability Accuracy | At Least One CWE | At Least One Function | At Least One Line |
|-------------|---------|------------------------|------------------|----------------------|-------------------|
| Function | 29 | 62.1% | 10.3% (3 out of 29) | 31.0% (9 out of 29) | 20.7% (6 out of 29) |
| File | 29 | 58.6% | 10.3% (3 out of 29) | 24.1% (7 out of 29) | 17.2% (5 out of 29) |
| Crate | 29 | 55.2% | 3.4% (1 out of 29) | 27.6% (8 out of 29) | 20.7% (6 out of 29) |

### Detailed F1 Scores (Complete Granularity)

| Granularity | CWE Macro F1 | Functions Macro F1 | Lines Macro F1 |
|-------------|--------------|-------------------|----------------|
| Function | 0.069 | 0.239 | 0.177 |
| File | 0.080 | 0.159 | 0.083 |
| Crate | 0.023 | 0.176 | 0.101 |

## Performance by CWE Type

Total unique CWE types: 18

| CWE | Samples | JSON Validity | Vulnerability Accuracy | At Least One CWE | At Least One Function | At Least One Line |
|-----|---------|---------------|------------------------|------------------|----------------------|-------------------|
| CWE-416 | 13 | 92.3% | 92.3% | 23.1% (3 out of 13) | 38.5% (5 out of 13) | 38.5% (5 out of 13) |
| CWE-129 | 12 | 100.0% | 100.0% | 0.0% (0 out of 12) | 0.0% (0 out of 12) | 0.0% (0 out of 12) |
| CWE-362 | 11 | 90.9% | 90.9% | 0.0% (0 out of 11) | 9.1% (1 out of 11) | 18.2% (2 out of 11) |
| CWE-787 | 10 | 100.0% | 70.0% | 0.0% (0 out of 10) | 40.0% (4 out of 10) | 20.0% (2 out of 10) |
| CWE-908 | 9 | 100.0% | 100.0% | 0.0% (0 out of 9) | 66.7% (6 out of 9) | 55.6% (5 out of 9) |

### Detailed F1 Scores by CWE Type

| CWE | CWE F1 | Functions F1 | Lines F1 |
|-----|--------|--------------|----------|
| CWE-416 | 0.154 | 0.256 | 0.193 |
| CWE-129 | 0.000 | 0.000 | 0.000 |
| CWE-362 | 0.000 | 0.091 | 0.152 |
| CWE-787 | 0.000 | 0.400 | 0.100 |
| CWE-908 | 0.000 | 0.463 | 0.220 |

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
