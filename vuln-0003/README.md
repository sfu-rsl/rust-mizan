# Vulnerability: CVE-2018-21000

| **Information**       | **Details**                                                                                                  |
| --------------------- | ------------------------------------------------------------------------------------------------------------ |
| **CVE**               | [CVE-2018-21000](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2018-21000)                              |
| **Vulnerable Commit** | [c79ebfd](https://github.com/nabijaczleweli/safe-transmute-rs/tree/c79ebfdb5858982af59a78df471c7cad7a78fd23) |
| **Fixed Commit**      | [a134e06](https://github.com/nabijaczleweli/safe-transmute-rs/tree/a134e06d740f9d7c287f74c0af2cd06206774364) |
| **Variants**          | - [fixed-crate](fixed-crate)                                                                                 |
|                       | - [fixed-file](fixed-file)                                                                                   |
|                       | - [fixed-function](fixed-function)                                                                           |
|                       | - [vuln-crate](vuln-crate)                                                                                   |
|                       | - [vuln-file](vuln-file)                                                                                     |
|                       | - [vuln-function](vuln-function)                                                                             |

### Vulnerable Lines

```rust
// Incorrect argument order in `Vec::from_raw_parts` leads to buffer overflow
// Should be `Vec::from_raw_parts(ptr as *mut u8, len, capacity)`
Vec::from_raw_parts(ptr as *mut u8, capacity, len)
```