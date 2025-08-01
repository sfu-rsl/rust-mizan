# RustMizan Benchmark Analysis Report

- **Model:** gpt-4.1-2025-04-14

**Mutations Applied:** format-expanded

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
| JSON Validity Rate | 78.0% |
| Vulnerability Detection Accuracy | 44.5% |
| Samples with At Least One Correct CWE Identified | 9.2% (16 out of 173) |
| Samples with At Least One Correct Vulnerable Function Identified | 16.2% (28 out of 173) |
| Samples with At Least One Correct Vulnerable Line Identified | 13.9% (24 out of 173) |
| CWE Type Macro F1 | 0.057 |
| Vulnerable Functions Macro F1 | 0.149 |
| Vulnerable Lines Macro F1 | 0.120 |

## Analysis of Vulnerable Samples

*Analysis focused only on samples that contain vulnerabilities*

| Metric | Value |
|--------|-------|
| Total Vulnerable Samples | 95 |
| JSON Validity Rate | 78.9% |
| Vulnerability Detection Accuracy | 38.9% |
| At Least One Correct CWE Identified | 16.8% (16 out of 95) |
| At Least One Correct Vulnerable Function Identified | 29.5% (28 out of 95) |
| At Least One Correct Vulnerable Line Identified | 25.3% (24 out of 95) |

### Detailed F1 Scores (Vulnerable Samples Only)

| Metric | F1 Score |
|--------|----------|
| CWE Type Macro F1 | 0.103 |
| Vulnerable Functions Macro F1 | 0.268 |
| Vulnerable Lines Macro F1 | 0.215 |

## Analysis by Granularity

| Granularity | Samples | JSON Validity | Vulnerability Accuracy | At Least One CWE | At Least One Function | At Least One Line |
|-------------|---------|---------------|------------------------|------------------|----------------------|-------------------|
| Function | 57 | 100.0% | 59.6% | 14.0% (8 out of 57) | 22.8% (13 out of 57) | 21.1% (12 out of 57) |
| File | 39 | 66.7% | 38.5% | 10.3% (4 out of 39) | 15.4% (6 out of 39) | 10.3% (4 out of 39) |
| Crate | 77 | 67.5% | 36.4% | 5.2% (4 out of 77) | 11.7% (9 out of 77) | 10.4% (8 out of 77) |

### Detailed F1 Scores by Granularity

| Granularity | CWE Macro F1 | Functions Macro F1 | Lines Macro F1 |
|-------------|--------------|-------------------|----------------|
| Function | 0.074 | 0.205 | 0.187 |
| File | 0.047 | 0.084 | 0.055 |
| Crate | 0.022 | 0.067 | 0.044 |

## Complete Granularity Analysis

*Analysis of vulnerabilities with all three granularities (function, file, crate) and valid JSON responses*

Found 7 vulnerabilities with all three granularities and valid JSON responses.

Total samples in complete analysis: 33 (11 samples per granularity)

| Granularity | Samples | Vulnerability Accuracy | At Least One CWE | At Least One Function | At Least One Line |
|-------------|---------|------------------------|------------------|----------------------|-------------------|
| Function | 11 | 54.5% | 18.2% (2 out of 11) | 36.4% (4 out of 11) | 18.2% (2 out of 11) |
| File | 11 | 63.6% | 27.3% (3 out of 11) | 45.5% (5 out of 11) | 27.3% (3 out of 11) |
| Crate | 11 | 54.5% | 0.0% (0 out of 11) | 27.3% (3 out of 11) | 18.2% (2 out of 11) |

### Detailed F1 Scores (Complete Granularity)

| Granularity | CWE Macro F1 | Functions Macro F1 | Lines Macro F1 |
|-------------|--------------|-------------------|----------------|
| Function | 0.091 | 0.288 | 0.136 |
| File | 0.107 | 0.252 | 0.151 |
| Crate | 0.000 | 0.179 | 0.059 |

## Performance by CWE Type

Total unique CWE types: 18

| CWE | Samples | JSON Validity | Vulnerability Accuracy | At Least One CWE | At Least One Function | At Least One Line |
|-----|---------|---------------|------------------------|------------------|----------------------|-------------------|
| CWE-416 | 13 | 69.2% | 15.4% | 7.7% (1 out of 13) | 7.7% (1 out of 13) | 7.7% (1 out of 13) |
| CWE-129 | 12 | 41.7% | 16.7% | 0.0% (0 out of 12) | 0.0% (0 out of 12) | 0.0% (0 out of 12) |
| CWE-362 | 11 | 90.9% | 18.2% | 0.0% (0 out of 11) | 0.0% (0 out of 11) | 9.1% (1 out of 11) |
| CWE-787 | 10 | 80.0% | 30.0% | 10.0% (1 out of 10) | 20.0% (2 out of 10) | 10.0% (1 out of 10) |
| CWE-908 | 9 | 88.9% | 88.9% | 44.4% (4 out of 9) | 88.9% (8 out of 9) | 55.6% (5 out of 9) |

### Detailed F1 Scores by CWE Type

| CWE | CWE F1 | Functions F1 | Lines F1 |
|-----|--------|--------------|----------|
| CWE-416 | 0.038 | 0.038 | 0.051 |
| CWE-129 | 0.000 | 0.000 | 0.000 |
| CWE-362 | 0.000 | 0.000 | 0.091 |
| CWE-787 | 0.050 | 0.200 | 0.100 |
| CWE-908 | 0.150 | 0.546 | 0.314 |

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
