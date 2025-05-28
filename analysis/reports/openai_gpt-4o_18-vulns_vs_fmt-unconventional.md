# LLM Training Data Contamination Experiment Analysis

**Model:** openai - gpt-4o  
**Original Dataset:** 18-vulns  
**Mutated Dataset:** fmt-unconventional  
**Mutation:** Applied unconventional Rust formatting style to the entire workspace  
**Report Generated:** 2025-05-28 14:36:13

# Mutation: Applied unconventional Rust formatting style to the entire workspace

## openai `gpt-4o` on `18-vulns`:

### Check CVE task:
- Correctly predicted CVE existence: 10 out of 18 (55.6%)
> The percentage of times the model correctly predicted whether a crate has CVEs in a given year

### Identify crate task:
- Crate names: 35 out of 50 (70.0%)
> The percentage of times the model correctly identified the crate name from source code
- Years: 3 out of 50 (6.0%)
> The percentage of times the model correctly identified the crate publication year
- CVE existence: 14 out of 50 (28.0%)
> The percentage of times the model correctly predicted whether the crate has CVEs
- At least one CVE correct: 0 out of 50 (0.0%)
> The percentage of times the model correctly identified at least one actual CVE
- Failed to generate valid results for 35 samples out of 85

### Vulnerability detection task:
- Vulnerability existence: 31 out of 44 (70.5%)
> The percentage of times the model correctly detected whether a vulnerability exists
- At least one CWE type correct: 8 out of 44 (18.2%)
> The percentage of times the model correctly identified at least one CWE type
- At least one vulnerable function correct: 21 out of 44 (47.7%)
> The percentage of times the model correctly identified at least one vulnerable function
- At least one vulnerable line correct: 14 out of 44 (31.8%)
> The percentage of times the model correctly identified at least one vulnerable line
- Failed to generate valid results for 41 samples out of 85


## openai `gpt-4o` on `fmt-unconventional`:

### Identify crate task:
- Crate names: 27 out of 37 (73.0%)
> The percentage of times the model correctly identified the crate name from source code
- Years: 3 out of 37 (8.1%)
> The percentage of times the model correctly identified the crate publication year
- CVE existence: 11 out of 37 (29.7%)
> The percentage of times the model correctly predicted whether the crate has CVEs
- At least one CVE correct: 1 out of 37 (2.7%)
> The percentage of times the model correctly identified at least one actual CVE
- Failed to generate valid results for 48 samples out of 85

### Vulnerability detection task:
- Vulnerability existence: 19 out of 31 (61.3%)
> The percentage of times the model correctly detected whether a vulnerability exists
- At least one CWE type correct: 7 out of 31 (22.6%)
> The percentage of times the model correctly identified at least one CWE type
- At least one vulnerable function correct: 12 out of 31 (38.7%)
> The percentage of times the model correctly identified at least one vulnerable function
- At least one vulnerable line correct: 6 out of 31 (19.4%)
> The percentage of times the model correctly identified at least one vulnerable line
- Failed to generate valid results for 54 samples out of 85


## Identify Crate Task - Performance by Code Level:

**Metric explanations:**
- **Crate Name (%):** The percentage of times the model correctly identified the crate name
- **Year (%):** The percentage of times the model correctly identified the crate publication year
- **CVE (%):** The percentage of times the model correctly identified at least one actual CVE

| Dataset | Level | Crate Name (%) | Year (%) | CVE (%) |
| --- | --- | --- | --- | --- |
| 18-vulns | crate | 100.0 | 0.0 | 0.0 |
| 18-vulns | file | 90.9 | 9.1 | 0.0 |
| 18-vulns | function | 57.6 | 6.1 | 0.0 |
| fmt-unconventional | crate | 83.3 | 16.7 | 0.0 |
| fmt-unconventional | file | 87.5 | 12.5 | 12.5 |
| fmt-unconventional | function | 65.2 | 4.3 | 0.0 |

## Vulnerability Detection Task - Performance by Code Level:

**Metric explanations:**
- **Existence (%):** The percentage of times the model correctly detected whether a vulnerability exists
- **CWE Type (%):** The percentage of times the model correctly identified at least one CWE type
- **Functions (%):** The percentage of times the model correctly identified at least one vulnerable function
- **Lines (%):** The percentage of times the model correctly identified at least one vulnerable line

| Dataset | Level | Existence (%) | CWE Type (%) | Functions (%) | Lines (%) |
| --- | --- | --- | --- | --- | --- |
| 18-vulns | crate | 83.3 | 16.7 | 50.0 | 33.3 |
| 18-vulns | file | 88.9 | 33.3 | 66.7 | 33.3 |
| 18-vulns | function | 62.1 | 13.8 | 41.4 | 31.0 |
| fmt-unconventional | crate | 100.0 | 33.3 | 66.7 | 66.7 |
| fmt-unconventional | file | 100.0 | 66.7 | 66.7 | 0.0 |
| fmt-unconventional | function | 52.0 | 16.0 | 32.0 | 16.0 |
