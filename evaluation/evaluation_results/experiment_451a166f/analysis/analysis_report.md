# RustMizan Benchmark Analysis Report

- **Model:** claude-3-7-sonnet-20250219

**Mutations Applied:** malignant-comments

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
| JSON Validity Rate | 90.8% |
| Vulnerability Detection Accuracy | 48.0% |
| Samples with At Least One Correct CWE Identified | 7.5% (13 out of 173) |
| Samples with At Least One Correct Vulnerable Function Identified | 16.8% (29 out of 173) |
| Samples with At Least One Correct Vulnerable Line Identified | 18.5% (32 out of 173) |
| CWE Type Macro F1 | 0.060 |
| Vulnerable Functions Macro F1 | 0.142 |
| Vulnerable Lines Macro F1 | 0.141 |

## Analysis of Vulnerable Samples

*Analysis focused only on samples that contain vulnerabilities*

| Metric | Value |
|--------|-------|
| Total Vulnerable Samples | 95 |
| JSON Validity Rate | 94.7% |
| Vulnerability Detection Accuracy | 78.9% |
| At Least One Correct CWE Identified | 13.7% (13 out of 95) |
| At Least One Correct Vulnerable Function Identified | 30.5% (29 out of 95) |
| At Least One Correct Vulnerable Line Identified | 33.7% (32 out of 95) |

### Detailed F1 Scores (Vulnerable Samples Only)

| Metric | F1 Score |
|--------|----------|
| CWE Type Macro F1 | 0.105 |
| Vulnerable Functions Macro F1 | 0.248 |
| Vulnerable Lines Macro F1 | 0.246 |

## Analysis by Granularity

| Granularity | Samples | JSON Validity | Vulnerability Accuracy | At Least One CWE | At Least One Function | At Least One Line |
|-------------|---------|---------------|------------------------|------------------|----------------------|-------------------|
| Function | 57 | 98.2% | 54.4% | 7.0% (4 out of 57) | 22.8% (13 out of 57) | 21.1% (12 out of 57) |
| File | 39 | 94.9% | 48.7% | 7.7% (3 out of 39) | 15.4% (6 out of 39) | 12.8% (5 out of 39) |
| Crate | 77 | 83.1% | 42.9% | 7.8% (6 out of 77) | 13.0% (10 out of 77) | 19.5% (15 out of 77) |

### Detailed F1 Scores by Granularity

| Granularity | CWE Macro F1 | Functions Macro F1 | Lines Macro F1 |
|-------------|--------------|-------------------|----------------|
| Function | 0.044 | 0.195 | 0.170 |
| File | 0.064 | 0.110 | 0.088 |
| Crate | 0.058 | 0.090 | 0.118 |

## Complete Granularity Analysis

*Analysis of vulnerabilities with all three granularities (function, file, crate) and valid JSON responses*

Found 15 vulnerabilities with all three granularities and valid JSON responses.

Total samples in complete analysis: 81 (27 samples per granularity)

| Granularity | Samples | Vulnerability Accuracy | At Least One CWE | At Least One Function | At Least One Line |
|-------------|---------|------------------------|------------------|----------------------|-------------------|
| Function | 27 | 55.6% | 3.7% (1 out of 27) | 22.2% (6 out of 27) | 11.1% (3 out of 27) |
| File | 27 | 55.6% | 3.7% (1 out of 27) | 22.2% (6 out of 27) | 18.5% (5 out of 27) |
| Crate | 27 | 55.6% | 7.4% (2 out of 27) | 18.5% (5 out of 27) | 25.9% (7 out of 27) |

### Detailed F1 Scores (Complete Granularity)

| Granularity | CWE Macro F1 | Functions Macro F1 | Lines Macro F1 |
|-------------|--------------|-------------------|----------------|
| Function | 0.025 | 0.165 | 0.104 |
| File | 0.037 | 0.158 | 0.127 |
| Crate | 0.062 | 0.121 | 0.146 |

## Performance by CWE Type

Total unique CWE types: 18

| CWE | Samples | JSON Validity | Vulnerability Accuracy | At Least One CWE | At Least One Function | At Least One Line |
|-----|---------|---------------|------------------------|------------------|----------------------|-------------------|
| CWE-416 | 13 | 92.3% | 69.2% | 30.8% (4 out of 13) | 0.0% (0 out of 13) | 23.1% (3 out of 13) |
| CWE-129 | 12 | 91.7% | 91.7% | 0.0% (0 out of 12) | 16.7% (2 out of 12) | 16.7% (2 out of 12) |
| CWE-362 | 11 | 81.8% | 63.6% | 9.1% (1 out of 11) | 9.1% (1 out of 11) | 27.3% (3 out of 11) |
| CWE-787 | 10 | 100.0% | 70.0% | 10.0% (1 out of 10) | 40.0% (4 out of 10) | 20.0% (2 out of 10) |
| CWE-908 | 9 | 100.0% | 88.9% | 11.1% (1 out of 9) | 55.6% (5 out of 9) | 33.3% (3 out of 9) |

### Detailed F1 Scores by CWE Type

| CWE | CWE F1 | Functions F1 | Lines F1 |
|-----|--------|--------------|----------|
| CWE-416 | 0.205 | 0.000 | 0.205 |
| CWE-129 | 0.000 | 0.167 | 0.125 |
| CWE-362 | 0.061 | 0.061 | 0.167 |
| CWE-787 | 0.100 | 0.400 | 0.133 |
| CWE-908 | 0.032 | 0.407 | 0.207 |

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
