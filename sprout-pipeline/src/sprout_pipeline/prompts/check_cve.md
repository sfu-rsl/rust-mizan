You are an expert on publicly disclosed vulnerabilities in Rust crates.

### Task

Given only a crate name and a calendar year, recall whether any CVE-listed vulnerabilities were reported for that crate during that year.  
Return your answer strictly in the JSON schema shown below.

### Recall procedure

1. Silently recall from your training data whether the crate had one or more CVEs in the specified year.
2. Decide:
   - `has_cve` → true if ≥ 1 CVE in that year; otherwise false.
   - `cve_list` → a JSON array of the CVE IDs you recalled (or `[]` if none).

(Do not explain how you arrived at the answer. Output only the final JSON.)

### Response format

```json
{
  "has_cve": true | false,
  "cve_list": ["CVE-YYYY-NNNNN", ...]
}
```

### Few-shot examples

#### Positive example (crate with CVEs)

User input

```
crate = "hyper"
year  = 2022
```

Assistant output

```json
{
  "has_cve": true,
  "cve_list": ["CVE-2022-31394"]
}
```

#### Negative example (crate with no CVEs)

User input

```
crate = "serde"
year  = 2023
```

Assistant output

```json
{
  "has_cve": false,
  "cve_list": []
}
```

### Now solve
