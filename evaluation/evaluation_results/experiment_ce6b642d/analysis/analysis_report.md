# RustMizan Benchmark Analysis Report

- **Model:** claude-3-7-sonnet-20250219

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
| JSON Validity Rate | 96.0% |
| Vulnerability Detection Accuracy | 56.6% |
| Samples with At Least One Correct CWE Identified | 6.9% (12 out of 173) |
| Samples with At Least One Correct Vulnerable Function Identified | 20.2% (35 out of 173) |
| Samples with At Least One Correct Vulnerable Line Identified | 18.5% (32 out of 173) |
| CWE Type Macro F1 | 0.054 |
| Vulnerable Functions Macro F1 | 0.159 |
| Vulnerable Lines Macro F1 | 0.128 |

## Analysis of Vulnerable Samples

*Analysis focused only on samples that contain vulnerabilities*

| Metric | Value |
|--------|-------|
| Total Vulnerable Samples | 95 |
| JSON Validity Rate | 95.8% |
| Vulnerability Detection Accuracy | 92.6% |
| At Least One Correct CWE Identified | 12.6% (12 out of 95) |
| At Least One Correct Vulnerable Function Identified | 36.8% (35 out of 95) |
| At Least One Correct Vulnerable Line Identified | 33.7% (32 out of 95) |

### Detailed F1 Scores (Vulnerable Samples Only)

| Metric | F1 Score |
|--------|----------|
| CWE Type Macro F1 | 0.098 |
| Vulnerable Functions Macro F1 | 0.290 |
| Vulnerable Lines Macro F1 | 0.234 |

## Analysis by Granularity

| Granularity | Samples | JSON Validity | Vulnerability Accuracy | At Least One CWE | At Least One Function | At Least One Line |
|-------------|---------|---------------|------------------------|------------------|----------------------|-------------------|
| Function | 57 | 100.0% | 63.2% | 8.8% (5 out of 57) | 31.6% (18 out of 57) | 31.6% (18 out of 57) |
| File | 39 | 97.4% | 53.8% | 7.7% (3 out of 39) | 17.9% (7 out of 39) | 10.3% (4 out of 39) |
| Crate | 77 | 92.2% | 53.2% | 5.2% (4 out of 77) | 13.0% (10 out of 77) | 13.0% (10 out of 77) |

### Detailed F1 Scores by Granularity

| Granularity | CWE Macro F1 | Functions Macro F1 | Lines Macro F1 |
|-------------|--------------|-------------------|----------------|
| Function | 0.056 | 0.280 | 0.254 |
| File | 0.068 | 0.105 | 0.046 |
| Crate | 0.040 | 0.082 | 0.066 |

## Complete Granularity Analysis

*Analysis of vulnerabilities with all three granularities (function, file, crate) and valid JSON responses*

Found 15 vulnerabilities with all three granularities and valid JSON responses.

Total samples in complete analysis: 81 (27 samples per granularity)

| Granularity | Samples | Vulnerability Accuracy | At Least One CWE | At Least One Function | At Least One Line |
|-------------|---------|------------------------|------------------|----------------------|-------------------|
| Function | 27 | 59.3% | 7.4% (2 out of 27) | 29.6% (8 out of 27) | 22.2% (6 out of 27) |
| File | 27 | 59.3% | 11.1% (3 out of 27) | 22.2% (6 out of 27) | 14.8% (4 out of 27) |
| Crate | 27 | 55.6% | 7.4% (2 out of 27) | 22.2% (6 out of 27) | 18.5% (5 out of 27) |

### Detailed F1 Scores (Complete Granularity)

| Granularity | CWE Macro F1 | Functions Macro F1 | Lines Macro F1 |
|-------------|--------------|-------------------|----------------|
| Function | 0.049 | 0.239 | 0.190 |
| File | 0.099 | 0.133 | 0.066 |
| Crate | 0.062 | 0.146 | 0.081 |

## Performance by CWE Type

Total unique CWE types: 18

| CWE | Samples | JSON Validity | Vulnerability Accuracy | At Least One CWE | At Least One Function | At Least One Line |
|-----|---------|---------------|------------------------|------------------|----------------------|-------------------|
| CWE-416 | 13 | 84.6% | 84.6% | 15.4% (2 out of 13) | 23.1% (3 out of 13) | 38.5% (5 out of 13) |
| CWE-129 | 12 | 100.0% | 100.0% | 0.0% (0 out of 12) | 0.0% (0 out of 12) | 0.0% (0 out of 12) |
| CWE-362 | 11 | 90.9% | 90.9% | 0.0% (0 out of 11) | 9.1% (1 out of 11) | 18.2% (2 out of 11) |
| CWE-787 | 10 | 100.0% | 80.0% | 20.0% (2 out of 10) | 50.0% (5 out of 10) | 0.0% (0 out of 10) |
| CWE-908 | 9 | 100.0% | 100.0% | 0.0% (0 out of 9) | 66.7% (6 out of 9) | 66.7% (6 out of 9) |

### Detailed F1 Scores by CWE Type

| CWE | CWE F1 | Functions F1 | Lines F1 |
|-----|--------|--------------|----------|
| CWE-416 | 0.128 | 0.154 | 0.203 |
| CWE-129 | 0.000 | 0.000 | 0.000 |
| CWE-362 | 0.000 | 0.091 | 0.152 |
| CWE-787 | 0.167 | 0.450 | 0.000 |
| CWE-908 | 0.000 | 0.441 | 0.270 |

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
