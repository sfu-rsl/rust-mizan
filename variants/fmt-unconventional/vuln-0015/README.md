# Vulnerability: CVE-2020-35886

| **Information**       | **Details**                                                                            |
| --------------------- | -------------------------------------------------------------------------------------- |
| **CVE**               | [CVE-2020-35886](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2020-35886)        |
| **Vulnerable Commit** | [efa2141](https://github.com/sjep/array/tree/efa214159eaad2abda7b072f278d678f8788c307) |
| **Fixed Commit**      | None                                                                                   |
| **Variants**          | - [vuln-crate](vuln-crate)                                                             |
|                       | - [vuln-function](vuln-function)                                                       |

### Vulnerable Lines

```rust
// Treats `T` as `Sync` and `Send` even if it's not thread-safe
unsafe impl<T> Sync for Array<T>{}
unsafe impl<T> Send for Array<T>{}
```
