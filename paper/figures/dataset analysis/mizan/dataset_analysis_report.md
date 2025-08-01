# RustMizan Dataset Analysis Report

**Dataset Version:** 0.1.0
**Rust Version:** rustc 1.84.1
**Analysis Date:** 2025-07-30

## Dataset Statistics

| Metric                | Value     |
| --------------------- | --------- |
| Total Vulnerabilities | 42        |
| Total Code Samples    | 173       |
| Vulnerable Samples    | 95        |
| Fixed Samples         | 78        |
| Year Range            | 2018-2025 |
| Unique Authors        | 5         |
| Unique CWE Types      | 18        |

## Vulnerability Analysis

### Top 10 CWE Types

| CWE Type | Frequency | Percentage |
| -------- | --------- | ---------- |
| CWE-416  | 13        | 12.6%      |
| CWE-129  | 12        | 11.7%      |
| CWE-362  | 11        | 10.7%      |
| CWE-787  | 10        | 9.7%       |
| CWE-908  | 9         | 8.7%       |
| CWE-119  | 8         | 7.8%       |
| CWE-401  | 8         | 7.8%       |
| CWE-125  | 7         | 6.8%       |
| CWE-415  | 6         | 5.8%       |
| CWE-400  | 3         | 2.9%       |

### Vulnerabilities by Year

| Year | Count |
| ---- | ----- |
| 2018 | 2     |
| 2019 | 4     |
| 2020 | 28    |
| 2021 | 1     |
| 2023 | 1     |
| 2025 | 6     |

## Code Sample Analysis

### Granularity Level Statistics

| Granularity | Samples | Avg Lines | Avg Rust Files |
| ----------- | ------- | --------- | -------------- |
| Function    | 57      | 193.1     | 1.9            |
| File        | 39      | 1389.2    | 6.4            |
| Crate       | 77      | 4445.2    | 17.4           |

## Figures

The following figures have been generated and saved to the `figures/` directory:

1. **figure1_sample_distribution.png** - Distribution of Vulnerable vs Fixed Code Samples
2. **figure2_cwe_distribution.png** - Top 10 CWE Types in Dataset
3. **figure3_vulnerabilities_by_year.png** - Distribution of Vulnerabilities by Year
4. **figure4_granularity_distribution.png** - Distribution of Code Samples by Granularity Level
5. **figure5_lines_by_granularity.png** - Lines of Code Distribution by Granularity Level
