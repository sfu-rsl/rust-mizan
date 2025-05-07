# Adding a New Vulnerability to the Dataset

1. Identify the Vulnerability
   - Pick a vulnerability from `datasets/` folder
   - Use the CVE identifier from MITRE, not the RustSec-assigned ID.
   - Skip CVEs that are already included in the dataset.
2. Create a Directory
   - Create a new folder named `vuln-XXXX` (increment the ID based on the latest entry).
   - This folder will contain all variants of code samples for this CVE.
3. Find Vulnerable and Fixed Commits
   - Vulnerable commit:
     - If clear from GitHub issue, use the commit before the fix.
     - If unclear, use the version before the Patched version listed by RustSec.
   - Fixed commit:
     - Use the commit corresponding to the Patched version from RustSec.
     - If no patched version is listed, skip generating fixed samples.
4. Generate Vulnerable Code Samples
   - From the vulnerable commit:
     - `vuln-crate`: full crate.
     - `vuln-file`: minimal crate with the vulnerable file.
     - `vuln-function`: minimal crate with the vulnerable function.
   - Ensure all crates compile:
     - Apply minimal changes if needed (e.g. fixing outdated syntax).
5. Generate Fixed Code Samples (if fix exists)
   - From the fixed commit:
     - `fixed-crate`, `fixed-file`, `fixed-function`
6. Write the `README.md`
   - Include:
     - CVE ID
     - Crate name
     - Commit links (before and after the fix)
     - List of included sample variants
     - Explanation of the vulnerability:
       - Include a code snippet with a comment pointing out the vulnerability
       - Justify why this code is vulnerable (refer to CVE, RustSec, or GitHub issue)
7. Update `mizan.json`
   - Add an entry for the CVE with:
     - Code sample paths
     - `is_vulnerability` flag
     - CWE type(s)
     - Mapping of file → vulnerable functions
     - Mapping of file → vulnerable line numbers
   - When unsure, prefer over-reporting:
     - Include both the vulnerable API and functions that call it
8. Mark the CVE as Done
   - In `datasets`, mark the CVE as complete.

## Notes

- All crates must compile. If needed, make minimal edits without changing behavior.
- The `README.md` should follow the structure of existing entries.
- Vulnerable line/function annotations should capture all relevant surface area, not just the line that panics.
- Only use official, peer-reviewed fixes.
- If no fix exists, only include vulnerable samples.

> [!Note]
> In some cases, it might be challenging or unrealistic to add the whole crate/module (e.g., when dealing with the standard library). If unsure, please open an issue for discussion.
