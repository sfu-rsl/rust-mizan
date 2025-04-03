# Vulnerability: CVE-2020-36215

| **Information**       | **Details**                                                                                            |
| --------------------- | ------------------------------------------------------------------------------------------------------ |
| **CVE**               | [CVE-2020-36215](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2020-36215)                        |
| **Vulnerable Commit** | [1b91c14](https://github.com/AdrienChampion/hashconsing/tree/1b91c147fa232a816daa2ae58557d197a662ab01) |
| **Fixed Commit**      | [4b2903c](https://github.com/AdrienChampion/hashconsing/tree/4b2903c9f91e0d81732174def757ac23a218e07a) |
| **Variants**          | - [fixed-crate](fixed-crate)                                                                           |
|                       | - [fixed-file](fixed-file)                                                                             |
|                       | - [fixed-function](fixed-function)                                                                     |
|                       | - [vuln-crate](vuln-crate)                                                                             |
|                       | - [vuln-file](vuln-file)                                                                               |
|                       | - [vuln-function](vuln-function)                                                                       |

### Vulnerable Lines

```rust
// Treats `T` as `Sync` and `Send` even if it's not thread-safe
unsafe impl<T> Sync for HConsed<T> {}
unsafe impl<T> Send for HConsed<T> {}
```
