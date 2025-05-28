# LLM Training Data Contamination Experiment Analysis

**Model:** anthropic - claude-sonnet-4-20250514  
**Original Dataset:** 18-vulns  
**Mutated Dataset:** fmt-unconventional  
**Mutation:** Applied unconventional Rust formatting style to the entire workspace  
**Report Generated:** 2025-05-28 14:34:44

# Mutation: Applied unconventional Rust formatting style to the entire workspace

## anthropic `claude-sonnet-4-20250514` on `18-vulns`:

### Check CVE task:
- Correctly predicted CVE existence: 9 out of 18 (50.0%)
> The percentage of times the model correctly predicted whether a crate has CVEs in a given year

### Identify crate task:
- Crate names: 74 out of 79 (93.7%)
> The percentage of times the model correctly identified the crate name from source code
- Years: 74 out of 79 (93.7%)
> The percentage of times the model correctly identified the crate publication year
- CVE existence: 54 out of 79 (68.4%)
> The percentage of times the model correctly predicted whether the crate has CVEs
- At least one CVE correct: 19 out of 79 (24.1%)
> The percentage of times the model correctly identified at least one actual CVE
- Failed to generate valid results for 6 samples out of 85

### Vulnerability detection task:
- Vulnerability existence: 53 out of 76 (69.7%)
> The percentage of times the model correctly detected whether a vulnerability exists
- At least one CWE type correct: 14 out of 76 (18.4%)
> The percentage of times the model correctly identified at least one CWE type
- At least one vulnerable function correct: 15 out of 76 (19.7%)
> The percentage of times the model correctly identified at least one vulnerable function
- At least one vulnerable line correct: 16 out of 76 (21.1%)
> The percentage of times the model correctly identified at least one vulnerable line
- Failed to generate valid results for 9 samples out of 85


## anthropic `claude-sonnet-4-20250514` on `fmt-unconventional`:

### Identify crate task:
- Crate names: 73 out of 77 (94.8%)
> The percentage of times the model correctly identified the crate name from source code
- Years: 73 out of 77 (94.8%)
> The percentage of times the model correctly identified the crate publication year
- CVE existence: 55 out of 77 (71.4%)
> The percentage of times the model correctly predicted whether the crate has CVEs
- At least one CVE correct: 22 out of 77 (28.6%)
> The percentage of times the model correctly identified at least one actual CVE
- Failed to generate valid results for 8 samples out of 85

### Vulnerability detection task:
- Vulnerability existence: 51 out of 73 (69.9%)
> The percentage of times the model correctly detected whether a vulnerability exists
- At least one CWE type correct: 14 out of 73 (19.2%)
> The percentage of times the model correctly identified at least one CWE type
- At least one vulnerable function correct: 19 out of 73 (26.0%)
> The percentage of times the model correctly identified at least one vulnerable function
- At least one vulnerable line correct: 15 out of 73 (20.5%)
> The percentage of times the model correctly identified at least one vulnerable line
- Failed to generate valid results for 12 samples out of 85


## Identify Crate Task - Performance by Code Level:

**Metric explanations:**
- **Crate Name (%):** The percentage of times the model correctly identified the crate name
- **Year (%):** The percentage of times the model correctly identified the crate publication year
- **CVE (%):** The percentage of times the model correctly identified at least one actual CVE

| Dataset | Level | Crate Name (%) | Year (%) | CVE (%) |
| --- | --- | --- | --- | --- |
| 18-vulns | crate | 93.3 | 93.3 | 20.0 |
| 18-vulns | file | 92.9 | 92.9 | 21.4 |
| 18-vulns | function | 94.0 | 94.0 | 26.0 |
| fmt-unconventional | crate | 92.9 | 92.9 | 28.6 |
| fmt-unconventional | file | 100.0 | 100.0 | 28.6 |
| fmt-unconventional | function | 93.9 | 93.9 | 28.6 |

## Vulnerability Detection Task - Performance by Code Level:

**Metric explanations:**
- **Existence (%):** The percentage of times the model correctly detected whether a vulnerability exists
- **CWE Type (%):** The percentage of times the model correctly identified at least one CWE type
- **Functions (%):** The percentage of times the model correctly identified at least one vulnerable function
- **Lines (%):** The percentage of times the model correctly identified at least one vulnerable line

| Dataset | Level | Existence (%) | CWE Type (%) | Functions (%) | Lines (%) |
| --- | --- | --- | --- | --- | --- |
| 18-vulns | crate | 86.7 | 33.3 | 13.3 | 20.0 |
| 18-vulns | file | 76.9 | 15.4 | 23.1 | 23.1 |
| 18-vulns | function | 62.5 | 14.6 | 20.8 | 20.8 |
| fmt-unconventional | crate | 85.7 | 28.6 | 35.7 | 21.4 |
| fmt-unconventional | file | 100.0 | 16.7 | 33.3 | 25.0 |
| fmt-unconventional | function | 57.4 | 17.0 | 21.3 | 19.1 |
