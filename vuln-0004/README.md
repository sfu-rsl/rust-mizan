# Vulnerability: CVE-2020-35891

| **Information**       | **Details**                                                                                       |
| --------------------- | --------------------------------------------------------------------------------------------------|
| **CVE**               | [CVE-2020-35891](https://rustsec.org/advisories/RUSTSEC-2020-0038.html)                           |
| **Vulnerable Commit** | [e1f1e3b](https://github.com/maciejhirsz/ordnung/commit/e1f1e3bda332dae33b76ca4be00dba46265e4cbe) |
| **Fixed Commit**      | None                                                                                              |
| **Variants**          | - [vuln-crate](vuln-crate)                                                                        |
|                       | - [vuln-file](vuln-file)                                                                          |
|                       | - [vuln-function](vuln-function)                                                                  |

### Vulnerable Lines

```rust
// Not panic-safe and causes double-free when an index larger than the length is provided.
self.with(move |v| v.remove(index))

// Panic from v.remove(index) frees a temporary vector created beforehand and the same 
// vector is freed again  when the user vector is dropped while stack unwinding.
let r = f(&mut stdvec);
```
