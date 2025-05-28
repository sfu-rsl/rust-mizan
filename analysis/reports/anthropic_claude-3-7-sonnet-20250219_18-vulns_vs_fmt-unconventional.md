# LLM Training Data Contamination Experiment Analysis

**Model:** anthropic - claude-3-7-sonnet-20250219  
**Original Dataset:** 18-vulns  
**Mutated Dataset:** fmt-unconventional  
**Mutation:** Applied unconventional Rust formatting style to the entire workspace  
**Report Generated:** 2025-05-28 14:32:28

# Mutation: Applied unconventional Rust formatting style to the entire workspace

## anthropic `claude-3-7-sonnet-20250219` on `18-vulns`:

### Check CVE task:
- Correctly predicted CVE existence: 12 out of 18 (66.7%)
> The percentage of times the model correctly predicted whether a crate has CVEs in a given year

### Identify crate task:
- Crate names: 73 out of 73 (100.0%)
> The percentage of times the model correctly identified the crate name from source code
- Years: 68 out of 73 (93.2%)
> The percentage of times the model correctly identified the crate publication year
- CVE existence: 42 out of 73 (57.5%)
> The percentage of times the model correctly predicted whether the crate has CVEs
- At least one CVE correct: 13 out of 73 (17.8%)
> The percentage of times the model correctly identified at least one actual CVE
- Failed to generate valid results for 12 samples out of 85

### Vulnerability detection task:
- Vulnerability existence: 44 out of 68 (64.7%)
> The percentage of times the model correctly detected whether a vulnerability exists
- At least one CWE type correct: 13 out of 68 (19.1%)
> The percentage of times the model correctly identified at least one CWE type
- At least one vulnerable function correct: 11 out of 68 (16.2%)
> The percentage of times the model correctly identified at least one vulnerable function
- At least one vulnerable line correct: 14 out of 68 (20.6%)
> The percentage of times the model correctly identified at least one vulnerable line
- Failed to generate valid results for 17 samples out of 85


## anthropic `claude-3-7-sonnet-20250219` on `fmt-unconventional`:

### Identify crate task:
- Crate names: 74 out of 74 (100.0%)
> The percentage of times the model correctly identified the crate name from source code
- Years: 69 out of 74 (93.2%)
> The percentage of times the model correctly identified the crate publication year
- CVE existence: 37 out of 74 (50.0%)
> The percentage of times the model correctly predicted whether the crate has CVEs
- At least one CVE correct: 13 out of 74 (17.6%)
> The percentage of times the model correctly identified at least one actual CVE
- Failed to generate valid results for 11 samples out of 85

### Vulnerability detection task:
- Vulnerability existence: 48 out of 72 (66.7%)
> The percentage of times the model correctly detected whether a vulnerability exists
- At least one CWE type correct: 17 out of 72 (23.6%)
> The percentage of times the model correctly identified at least one CWE type
- At least one vulnerable function correct: 16 out of 72 (22.2%)
> The percentage of times the model correctly identified at least one vulnerable function
- At least one vulnerable line correct: 19 out of 72 (26.4%)
> The percentage of times the model correctly identified at least one vulnerable line
- Failed to generate valid results for 13 samples out of 85


## Identify Crate Task - Performance by Code Level:

**Metric explanations:**
- **Crate Name (%):** The percentage of times the model correctly identified the crate name
- **Year (%):** The percentage of times the model correctly identified the crate publication year
- **CVE (%):** The percentage of times the model correctly identified at least one actual CVE

| Dataset | Level | Crate Name (%) | Year (%) | CVE (%) |
| --- | --- | --- | --- | --- |
| 18-vulns | crate | 100.0 | 92.9 | 7.1 |
| 18-vulns | file | 100.0 | 92.3 | 23.1 |
| 18-vulns | function | 100.0 | 93.5 | 19.6 |
| fmt-unconventional | crate | 100.0 | 85.7 | 14.3 |
| fmt-unconventional | file | 100.0 | 92.3 | 15.4 |
| fmt-unconventional | function | 100.0 | 95.7 | 19.1 |

## Vulnerability Detection Task - Performance by Code Level:

**Metric explanations:**
- **Existence (%):** The percentage of times the model correctly detected whether a vulnerability exists
- **CWE Type (%):** The percentage of times the model correctly identified at least one CWE type
- **Functions (%):** The percentage of times the model correctly identified at least one vulnerable function
- **Lines (%):** The percentage of times the model correctly identified at least one vulnerable line

| Dataset | Level | Existence (%) | CWE Type (%) | Functions (%) | Lines (%) |
| --- | --- | --- | --- | --- | --- |
| 18-vulns | crate | 100.0 | 33.3 | 13.3 | 33.3 |
| 18-vulns | file | 92.3 | 30.8 | 15.4 | 23.1 |
| 18-vulns | function | 42.5 | 10.0 | 17.5 | 15.0 |
| fmt-unconventional | crate | 100.0 | 46.2 | 15.4 | 30.8 |
| fmt-unconventional | file | 100.0 | 28.6 | 28.6 | 42.9 |
| fmt-unconventional | function | 46.7 | 15.6 | 22.2 | 20.0 |
