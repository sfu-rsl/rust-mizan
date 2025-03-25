# Vulnerability: CVE-2020-35711

| **Information**       | **Details**                                                                                 |
| --------------------- | ------------------------------------------------------------------------------------------- |
| **CVE**               | [CVE-2020-35711](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2020-35711)             |
| **Vulnerable Commit** | [b5ec44c](https://github.com/vorner/arc-swap/tree/b5ec44cd5d9b4b4a491cf85c1cbb2b1d5cdae061) |
| **Fixed Commit**      | [03ffe98](https://github.com/vorner/arc-swap/tree/03ffe983171f9583d629d31a33b169631ec56934) |
| **Variants**          | - [fixed-crate](fixed-crate)                                                                |
|                       | - [fixed-file](fixed-file)                                                                  |
|                       | - [fixed-function](fixed-function)                                                          |
|                       | - [vuln-crate](vuln-crate)                                                                  |
|                       | - [vuln-file](vuln-file)                                                                    |
|                       | - [vuln-function](vuln-function)                                                            |

## Justification for CWE Type and Vulnerable Lines

- The [NVD entry](https://nvd.nist.gov/vuln/detail/CVE-2020-35711) does not have CWE types yet since it is awaiting analysis, but it is clear that this is a use-after-free (dangling pointer) vulnerability (CWE-416).

### Vulnerable Lines

```rust
// The assumption that `self.value` remains valid as long as the `guard` is alive is not enforced by the `Access` trait
unsafe { &*self.value }

// When guard is moved or when the underlying memory is altered, value can become a dangling pointer.
let value: *const _ = (self.projection)(&guard);

// The `Access` trait bound (`A: Access<T>`) does not enforce any safety guarantees about the validity of references when the guard is moved
A: Access<T>,
```
