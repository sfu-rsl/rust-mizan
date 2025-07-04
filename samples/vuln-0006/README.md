# Vulnerability: CVE-2021-25900

| **Information**       | **Details**                                                                                     |
| --------------------- | ----------------------------------------------------------------------------------------------- |
| **CVE**               | [CVE-2021-25900](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2021-25900)                 |
| **Vulnerable Commit** | [0b2b4e5](https://github.com/servo/rust-smallvec/tree/0b2b4e53a5cf86740b2515907dd53252b3b59401) |
| **Fixed Commit**      | [9998ba0](https://github.com/servo/rust-smallvec/tree/9998ba0694a6b51aa6604748b00b6a98f0a0039e) |
| **Variants**          | - [fixed-crate](fixed-crate)                                                                    |
|                       | - [fixed-file](fixed-file)                                                                      |
|                       | - [fixed-function](fixed-function)                                                              |
|                       | - [vuln-crate](vuln-crate)                                                                      |
|                       | - [vuln-file](vuln-file)                                                                        |
|                       | - [vuln-function](vuln-function)                                                                |

### Vulnerable Lines

```rust
// Setting the length to 0 before reserving capacity makes `reserve()` a no-op,
// because `reserve(n)` compares `len` with `capacity`, and `len = 0` means
// it will never trigger a reallocation.
self.set_len(0);

// This reserve call is ineffective because `self.set_len(0)` set length to zero
self.reserve(1);
```
