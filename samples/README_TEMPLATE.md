# Vulnerability: CVE-YYYY-NNNNN

| **Information**   | **Details** |
| ----------------- | ----------- |
| **CVE / RUSTSEC**           | [CVE-YYYY-NNNNN](https://www.cve.org/CVERecord?id=CVE-YYYY-NNNN) / [RUSTSEC-YYYY-NNNN](https://rustsec.org/advisories/RUSTSEC-YYYY-NNNN.html) |
| **Crate**         | `crate-name` |
| **Vulnerable commit** | [abc1234](https://github.com/owner/repo/tree/<full-sha>) |
| **Fixed commit**  | [def5678](https://github.com/owner/repo/tree/<full-sha>) (or "No fix available") |
| **Variants**      | - Vulnerable: [crate](sample-0XXXX-crate), [file](sample-0XXXX-file), [function](sample-0XXXX-function) |
|                   | - Fixed: [crate](sample-1XXXX-crate), [file](sample-1XXXX-file), [function](sample-1XXXX-function) |

## Vulnerability

What the bug is and how it can be triggered.

## Vulnerable lines

```rust
// the vulnerable code, with the vulnerable line(s) marked
```

## Justification

Why these lines and this CWE type, with links to the CVE, RustSec advisory, or GitHub issue.