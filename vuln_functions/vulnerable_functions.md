# Vulnerable Functions Dataset

This file contains the vulnerable functions extracted from the dataset.

## Format

The `vulnerable_functions.jsonl` file contains 42 lines (one per vulnerability). Each line is a JSON object with the following fields:

- `vulnerability_id`: Vulnerability identifier (e.g., "vuln-0001")
- `cve`: Link to the CVE record
- `crate_name`: Name of the vulnerable crate
- `year`: Year of the vulnerability
- `cwe_type`: Array of CWE classifications
- `function_signature`: The vulnerable function signature
- `source_file`: Source file path within the sample
- `vulnerable_code`: The complete Rust function code
