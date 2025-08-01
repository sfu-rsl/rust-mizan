# RustMizan Benchmark Analysis Report

- **Model:** claude-3-7-sonnet-20250219

**Mutations Applied:** benign-rename-var

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
| JSON Validity Rate | 89.6% |
| Vulnerability Detection Accuracy | 56.6% |
| Samples with At Least One Correct CWE Identified | 11.6% (20 out of 173) |
| Samples with At Least One Correct Vulnerable Function Identified | 21.4% (37 out of 173) |
| Samples with At Least One Correct Vulnerable Line Identified | 20.8% (36 out of 173) |
| CWE Type Macro F1 | 0.100 |
| Vulnerable Functions Macro F1 | 0.179 |
| Vulnerable Lines Macro F1 | 0.150 |

## Analysis of Vulnerable Samples

*Analysis focused only on samples that contain vulnerabilities*

| Metric | Value |
|--------|-------|
| Total Vulnerable Samples | 95 |
| JSON Validity Rate | 94.7% |
| Vulnerability Detection Accuracy | 92.6% |
| At Least One Correct CWE Identified | 21.1% (20 out of 95) |
| At Least One Correct Vulnerable Function Identified | 38.9% (37 out of 95) |
| At Least One Correct Vulnerable Line Identified | 37.9% (36 out of 95) |

### Detailed F1 Scores (Vulnerable Samples Only)

| Metric | F1 Score |
|--------|----------|
| CWE Type Macro F1 | 0.172 |
| Vulnerable Functions Macro F1 | 0.309 |
| Vulnerable Lines Macro F1 | 0.258 |

## Analysis by Granularity

| Granularity | Samples | JSON Validity | Vulnerability Accuracy | At Least One CWE | At Least One Function | At Least One Line |
|-------------|---------|---------------|------------------------|------------------|----------------------|-------------------|
| Function | 57 | 96.5% | 64.9% | 14.0% (8 out of 57) | 33.3% (19 out of 57) | 31.6% (18 out of 57) |
| File | 39 | 97.4% | 61.5% | 12.8% (5 out of 39) | 20.5% (8 out of 39) | 23.1% (9 out of 39) |
| Crate | 77 | 80.5% | 48.1% | 9.1% (7 out of 77) | 13.0% (10 out of 77) | 11.7% (9 out of 77) |

### Detailed F1 Scores by Granularity

| Granularity | CWE Macro F1 | Functions Macro F1 | Lines Macro F1 |
|-------------|--------------|-------------------|----------------|
| Function | 0.105 | 0.276 | 0.245 |
| File | 0.103 | 0.139 | 0.105 |
| Crate | 0.071 | 0.086 | 0.067 |

## Complete Granularity Analysis

*Analysis of vulnerabilities with all three granularities (function, file, crate) and valid JSON responses*

Found 14 vulnerabilities with all three granularities and valid JSON responses.

Total samples in complete analysis: 75 (25 samples per granularity)

| Granularity | Samples | Vulnerability Accuracy | At Least One CWE | At Least One Function | At Least One Line |
|-------------|---------|------------------------|------------------|----------------------|-------------------|
| Function | 25 | 68.0% | 12.0% (3 out of 25) | 36.0% (9 out of 25) | 28.0% (7 out of 25) |
| File | 25 | 68.0% | 12.0% (3 out of 25) | 28.0% (7 out of 25) | 28.0% (7 out of 25) |
| Crate | 25 | 56.0% | 16.0% (4 out of 25) | 16.0% (4 out of 25) | 16.0% (4 out of 25) |

### Detailed F1 Scores (Complete Granularity)

| Granularity | CWE Macro F1 | Functions Macro F1 | Lines Macro F1 |
|-------------|--------------|-------------------|----------------|
| Function | 0.093 | 0.251 | 0.225 |
| File | 0.107 | 0.176 | 0.108 |
| Crate | 0.133 | 0.080 | 0.069 |

## Performance by CWE Type

Total unique CWE types: 18

| CWE | Samples | JSON Validity | Vulnerability Accuracy | At Least One CWE | At Least One Function | At Least One Line |
|-----|---------|---------------|------------------------|------------------|----------------------|-------------------|
| CWE-416 | 13 | 84.6% | 84.6% | 46.2% (6 out of 13) | 46.2% (6 out of 13) | 61.5% (8 out of 13) |
| CWE-129 | 12 | 100.0% | 100.0% | 0.0% (0 out of 12) | 8.3% (1 out of 12) | 16.7% (2 out of 12) |
| CWE-362 | 11 | 90.9% | 81.8% | 9.1% (1 out of 11) | 0.0% (0 out of 11) | 9.1% (1 out of 11) |
| CWE-787 | 10 | 100.0% | 90.0% | 30.0% (3 out of 10) | 50.0% (5 out of 10) | 20.0% (2 out of 10) |
| CWE-908 | 9 | 100.0% | 100.0% | 0.0% (0 out of 9) | 66.7% (6 out of 9) | 55.6% (5 out of 9) |

### Detailed F1 Scores by CWE Type

| CWE | CWE F1 | Functions F1 | Lines F1 |
|-----|--------|--------------|----------|
| CWE-416 | 0.410 | 0.274 | 0.293 |
| CWE-129 | 0.000 | 0.083 | 0.111 |
| CWE-362 | 0.061 | 0.000 | 0.091 |
| CWE-787 | 0.200 | 0.467 | 0.113 |
| CWE-908 | 0.000 | 0.441 | 0.229 |

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
