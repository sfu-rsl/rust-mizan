# Adding a New Vulnerability to the Dataset

1. Identify the Vulnerability
   - Use the CVE identifier from MITRE, not the RustSec-assigned ID.
2. Create a Directory
   - Create a new folder named `vuln-XXXX` (increment the ID based on the latest entry) in the `samples/` directory.
   - This folder will contain all variants of code samples for this CVE.
3. Find Vulnerable and Fixed Commits
   - Vulnerable commit:
     - If clear from GitHub issue, use the commit before the fix.
     - If unclear, use the version before the Patched version listed by RustSec.
   - Fixed commit:
     - Use the commit corresponding to the Patched version from RustSec.
     - If no patched version is listed, skip generating fixed samples.
4. Generate Vulnerable Code Samples
   - From the vulnerable commit, create samples following the naming convention:
     - `sample-0XXXX-crate`: full crate (where XXXX is the 4-digit vuln ID, e.g., 0043)
     - `sample-0XXXX-file`: minimal crate with the vulnerable file
     - `sample-0XXXX-function`: minimal crate with the vulnerable function
   - Update each sample's `Cargo.toml` to use the correct package name (e.g., `name = "sample-00043-crate"`)
   - Ensure all crates compile:
     - Apply minimal changes if needed (e.g. fixing outdated syntax)
5. Generate Fixed Code Samples (if fix exists)
   - From the fixed commit, create samples with the naming convention:
     - `sample-1XXXX-crate`, `sample-1XXXX-file`, `sample-1XXXX-function`
   - Note: The first digit `1` indicates these are fixed versions
6. Write the `README.md`
   - Include:
     - CVE ID
     - Crate name
     - Commit links (before and after the fix)
     - List of included sample variants
     - Explanation of the vulnerability:
       - Include a code snippet with a comment pointing out the vulnerability
       - Justify why this code is vulnerable (refer to CVE, RustSec, or GitHub issue)
7. Handle Dependencies (if needed)
   - If your code samples require external dependencies (that's normally the case when a crate depends on other crates from the project's cargo workspace):
     - Place the dependency crates in the `samples/deps/` directory
     - Update the `deps` field in `mizan.json` for each code sample that needs them
8. Update `mizan.json`
   - Add an entry for the CVE with:
     - Code sample paths (keep the original format, e.g., "vuln-0043/vuln-crate")
     - `is_vulnerability` flag
     - CWE type(s)
     - Mapping of file → vulnerable functions
     - Mapping of file → vulnerable line numbers
     - `deps` field: array of dependency names from `samples/deps/` (empty array if no deps)
   - When unsure, prefer over-reporting:
     - Include both the vulnerable API and functions that call it

## Notes

- All crates must compile. If needed, make minimal edits without changing behavior.
- The `README.md` should follow the structure of existing entries.
- Vulnerable line/function annotations should capture all relevant surface area, not just the line that panics.
- Only use official, peer-reviewed fixes.
- If no fix exists, only include vulnerable samples.

## Naming Convention

The naming convention is designed to be clear to developers but not immediately obvious to LLMs:

- Vulnerable samples: `sample-0XXXX-level` (first digit is 0)
- Fixed samples: `sample-1XXXX-level` (first digit is 1)
- Where XXXX is the 4-digit vulnerability ID and level is `function`, `file`, or `crate`
