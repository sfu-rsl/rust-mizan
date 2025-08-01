# RustMizan Benchmark Analysis Report

- **Model:** gpt-4.1-2025-04-14

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
| JSON Validity Rate | 79.2% |
| Vulnerability Detection Accuracy | 45.1% |
| Samples with At Least One Correct CWE Identified | 8.7% (15 out of 173) |
| Samples with At Least One Correct Vulnerable Function Identified | 13.3% (23 out of 173) |
| Samples with At Least One Correct Vulnerable Line Identified | 12.7% (22 out of 173) |
| CWE Type Macro F1 | 0.054 |
| Vulnerable Functions Macro F1 | 0.120 |
| Vulnerable Lines Macro F1 | 0.108 |

## Analysis of Vulnerable Samples

*Analysis focused only on samples that contain vulnerabilities*

| Metric | Value |
|--------|-------|
| Total Vulnerable Samples | 95 |
| JSON Validity Rate | 80.0% |
| Vulnerability Detection Accuracy | 35.8% |
| At Least One Correct CWE Identified | 15.8% (15 out of 95) |
| At Least One Correct Vulnerable Function Identified | 24.2% (23 out of 95) |
| At Least One Correct Vulnerable Line Identified | 23.2% (22 out of 95) |

### Detailed F1 Scores (Vulnerable Samples Only)

| Metric | F1 Score |
|--------|----------|
| CWE Type Macro F1 | 0.098 |
| Vulnerable Functions Macro F1 | 0.216 |
| Vulnerable Lines Macro F1 | 0.195 |

## Analysis by Granularity

| Granularity | Samples | JSON Validity | Vulnerability Accuracy | At Least One CWE | At Least One Function | At Least One Line |
|-------------|---------|---------------|------------------------|------------------|----------------------|-------------------|
| Function | 57 | 100.0% | 64.9% | 14.0% (8 out of 57) | 22.8% (13 out of 57) | 21.1% (12 out of 57) |
| File | 39 | 71.8% | 30.8% | 5.1% (2 out of 39) | 7.7% (3 out of 39) | 5.1% (2 out of 39) |
| Crate | 77 | 67.5% | 37.7% | 6.5% (5 out of 77) | 9.1% (7 out of 77) | 10.4% (8 out of 77) |

### Detailed F1 Scores by Granularity

| Granularity | CWE Macro F1 | Functions Macro F1 | Lines Macro F1 |
|-------------|--------------|-------------------|----------------|
| Function | 0.080 | 0.198 | 0.187 |
| File | 0.019 | 0.042 | 0.019 |
| Crate | 0.028 | 0.046 | 0.044 |

## Complete Granularity Analysis

*Analysis of vulnerabilities with all three granularities (function, file, crate) and valid JSON responses*

Found 9 vulnerabilities with all three granularities and valid JSON responses.

Total samples in complete analysis: 45 (15 samples per granularity)

| Granularity | Samples | Vulnerability Accuracy | At Least One CWE | At Least One Function | At Least One Line |
|-------------|---------|------------------------|------------------|----------------------|-------------------|
| Function | 15 | 60.0% | 20.0% (3 out of 15) | 40.0% (6 out of 15) | 26.7% (4 out of 15) |
| File | 15 | 46.7% | 13.3% (2 out of 15) | 20.0% (3 out of 15) | 13.3% (2 out of 15) |
| Crate | 15 | 40.0% | 13.3% (2 out of 15) | 20.0% (3 out of 15) | 13.3% (2 out of 15) |

### Detailed F1 Scores (Complete Granularity)

| Granularity | CWE Macro F1 | Functions Macro F1 | Lines Macro F1 |
|-------------|--------------|-------------------|----------------|
| Function | 0.122 | 0.318 | 0.233 |
| File | 0.050 | 0.110 | 0.050 |
| Crate | 0.089 | 0.073 | 0.050 |

## Performance by CWE Type

Total unique CWE types: 18

| CWE | Samples | JSON Validity | Vulnerability Accuracy | At Least One CWE | At Least One Function | At Least One Line |
|-----|---------|---------------|------------------------|------------------|----------------------|-------------------|
| CWE-416 | 13 | 76.9% | 23.1% | 15.4% (2 out of 13) | 15.4% (2 out of 13) | 15.4% (2 out of 13) |
| CWE-129 | 12 | 41.7% | 16.7% | 0.0% (0 out of 12) | 0.0% (0 out of 12) | 0.0% (0 out of 12) |
| CWE-362 | 11 | 72.7% | 18.2% | 9.1% (1 out of 11) | 9.1% (1 out of 11) | 18.2% (2 out of 11) |
| CWE-787 | 10 | 90.0% | 20.0% | 0.0% (0 out of 10) | 10.0% (1 out of 10) | 0.0% (0 out of 10) |
| CWE-908 | 9 | 88.9% | 77.8% | 22.2% (2 out of 9) | 66.7% (6 out of 9) | 44.4% (4 out of 9) |

### Detailed F1 Scores by CWE Type

| CWE | CWE F1 | Functions F1 | Lines F1 |
|-----|--------|--------------|----------|
| CWE-416 | 0.077 | 0.115 | 0.128 |
| CWE-129 | 0.000 | 0.000 | 0.000 |
| CWE-362 | 0.030 | 0.061 | 0.131 |
| CWE-787 | 0.000 | 0.100 | 0.000 |
| CWE-908 | 0.072 | 0.452 | 0.272 |

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
