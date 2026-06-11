# Add a vulnerability

Adding a vulnerability means providing the compilable variants and the metadata; the existing tooling handles the rest. See the [Dataset](../dataset.md) page for the layout, the [naming convention](../dataset.md#naming-convention), and the `mizan.json` schema.

## Steps

1. **Identify the vulnerability.** Use the CVE identifier from MITRE, not the RustSec-assigned ID.
2. **Create a directory.** Make a new `vuln-XXXX` folder (increment the latest ID) under `samples/`. It will hold all variants for this CVE.
3. **Find the vulnerable and fixed commits.**
   - Vulnerable commit: the commit before the fix if clear from the GitHub issue, otherwise the version before the patched release listed by RustSec.
   - Fixed commit: the commit corresponding to the patched release. If no patched version is listed, skip the fixed samples.
4. **Generate the vulnerable samples.** From the vulnerable commit, create:
   - `sample-0XXXX-crate`: the full crate
   - `sample-0XXXX-file`: a minimal crate with the vulnerable file
   - `sample-0XXXX-function`: a minimal crate with the vulnerable function

   Set each sample's `Cargo.toml` package name to match (e.g. `name = "sample-00043-crate"`). Make sure every crate compiles, applying minimal changes if needed (e.g. fixing outdated syntax).

5. **Generate the fixed samples (if a fix exists).** From the fixed commit, create `sample-1XXXX-crate`, `sample-1XXXX-file`, `sample-1XXXX-function`. The leading `1` marks them as fixed.
6. **Write the sample `README.md`.** Include the CVE ID, crate name, before/after commit links, the list of variants, and an explanation of the vulnerability with a code snippet pointing out the vulnerable line and a justification (referencing the CVE, RustSec, or the GitHub issue).
7. **Handle dependencies (if needed).** If samples depend on other crates from the project's workspace, place those crates in `samples/deps/` and list them in the `deps` field of each sample in `mizan.json`.
8. **Update `mizan.json`.** Add an entry with the sample paths, `is_vulnerability` flag, CWE type(s), the file-to-vulnerable-functions map, the file-to-vulnerable-lines map, and the `deps` array (empty if none). When unsure, prefer over-reporting: include both the vulnerable API and the functions that call it.

## Notes

- All crates must compile. If needed, make minimal edits without changing behavior.
- Follow the structure of existing sample `README.md` files.
- Vulnerable line and function annotations should capture all relevant surface area, not just the line that panics.
- Only use official, peer-reviewed fixes. If no fix exists, include only the vulnerable samples.

## Naming convention

The sample naming convention (`sample-0XXXX` for vulnerable, `sample-1XXXX` for fixed) is documented on the [Dataset](../dataset.md#naming-convention) page.
