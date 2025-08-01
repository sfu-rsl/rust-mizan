# RustMizan Benchmark Analysis Report

- **Model:** gpt-4.1-2025-04-14

**Mutations Applied:** mizan-mut-while-to-loop

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
| JSON Validity Rate | 76.3% |
| Vulnerability Detection Accuracy | 43.4% |
| Samples with At Least One Correct CWE Identified | 6.9% (12 out of 173) |
| Samples with At Least One Correct Vulnerable Function Identified | 13.3% (23 out of 173) |
| Samples with At Least One Correct Vulnerable Line Identified | 12.1% (21 out of 173) |
| CWE Type Macro F1 | 0.044 |
| Vulnerable Functions Macro F1 | 0.125 |
| Vulnerable Lines Macro F1 | 0.099 |

## Analysis of Vulnerable Samples

*Analysis focused only on samples that contain vulnerabilities*

| Metric | Value |
|--------|-------|
| Total Vulnerable Samples | 95 |
| JSON Validity Rate | 75.8% |
| Vulnerability Detection Accuracy | 34.7% |
| At Least One Correct CWE Identified | 12.6% (12 out of 95) |
| At Least One Correct Vulnerable Function Identified | 24.2% (23 out of 95) |
| At Least One Correct Vulnerable Line Identified | 22.1% (21 out of 95) |

### Detailed F1 Scores (Vulnerable Samples Only)

| Metric | F1 Score |
|--------|----------|
| CWE Type Macro F1 | 0.080 |
| Vulnerable Functions Macro F1 | 0.230 |
| Vulnerable Lines Macro F1 | 0.182 |

## Analysis by Granularity

| Granularity | Samples | JSON Validity | Vulnerability Accuracy | At Least One CWE | At Least One Function | At Least One Line |
|-------------|---------|---------------|------------------------|------------------|----------------------|-------------------|
| Function | 57 | 100.0% | 52.6% | 7.0% (4 out of 57) | 19.3% (11 out of 57) | 17.5% (10 out of 57) |
| File | 39 | 66.7% | 38.5% | 0.0% (0 out of 39) | 7.7% (3 out of 39) | 5.1% (2 out of 39) |
| Crate | 77 | 63.6% | 39.0% | 10.4% (8 out of 77) | 11.7% (9 out of 77) | 11.7% (9 out of 77) |

### Detailed F1 Scores by Granularity

| Granularity | CWE Macro F1 | Functions Macro F1 | Lines Macro F1 |
|-------------|--------------|-------------------|----------------|
| Function | 0.038 | 0.172 | 0.151 |
| File | 0.000 | 0.035 | 0.015 |
| Crate | 0.047 | 0.070 | 0.050 |

## Complete Granularity Analysis

*Analysis of vulnerabilities with all three granularities (function, file, crate) and valid JSON responses*

Found 6 vulnerabilities with all three granularities and valid JSON responses.

Total samples in complete analysis: 30 (10 samples per granularity)

| Granularity | Samples | Vulnerability Accuracy | At Least One CWE | At Least One Function | At Least One Line |
|-------------|---------|------------------------|------------------|----------------------|-------------------|
| Function | 10 | 70.0% | 0.0% (0 out of 10) | 30.0% (3 out of 10) | 10.0% (1 out of 10) |
| File | 10 | 50.0% | 0.0% (0 out of 10) | 20.0% (2 out of 10) | 10.0% (1 out of 10) |
| Crate | 10 | 60.0% | 20.0% (2 out of 10) | 20.0% (2 out of 10) | 10.0% (1 out of 10) |

### Detailed F1 Scores (Complete Granularity)

| Granularity | CWE Macro F1 | Functions Macro F1 | Lines Macro F1 |
|-------------|--------------|-------------------|----------------|
| Function | 0.000 | 0.178 | 0.080 |
| File | 0.000 | 0.117 | 0.040 |
| Crate | 0.117 | 0.150 | 0.080 |

## Performance by CWE Type

Total unique CWE types: 18

| CWE | Samples | JSON Validity | Vulnerability Accuracy | At Least One CWE | At Least One Function | At Least One Line |
|-----|---------|---------------|------------------------|------------------|----------------------|-------------------|
| CWE-416 | 13 | 76.9% | 23.1% | 7.7% (1 out of 13) | 0.0% (0 out of 13) | 7.7% (1 out of 13) |
| CWE-129 | 12 | 41.7% | 16.7% | 0.0% (0 out of 12) | 0.0% (0 out of 12) | 0.0% (0 out of 12) |
| CWE-362 | 11 | 72.7% | 9.1% | 0.0% (0 out of 11) | 0.0% (0 out of 11) | 0.0% (0 out of 11) |
| CWE-787 | 10 | 90.0% | 20.0% | 10.0% (1 out of 10) | 20.0% (2 out of 10) | 10.0% (1 out of 10) |
| CWE-908 | 9 | 77.8% | 77.8% | 33.3% (3 out of 9) | 77.8% (7 out of 9) | 44.4% (4 out of 9) |

### Detailed F1 Scores by CWE Type

| CWE | CWE F1 | Functions F1 | Lines F1 |
|-----|--------|--------------|----------|
| CWE-416 | 0.051 | 0.000 | 0.051 |
| CWE-129 | 0.000 | 0.000 | 0.000 |
| CWE-362 | 0.000 | 0.000 | 0.000 |
| CWE-787 | 0.022 | 0.117 | 0.017 |
| CWE-908 | 0.139 | 0.488 | 0.284 |

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
