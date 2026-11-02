# Vulnerability: CVE-2020-36442

| **Information**       | **Details**                                                                                  |
| --------------------- | -------------------------------------------------------------------------------------------- |
| **CVE**               | [CVE-2020-36442](https://www.cve.org/CVERecord?id=CVE-2020-36442)                            |
| **Vulnerable Commit** | [0b46851](https://github.com/maciejhirsz/beef/tree/0b4685143e680749991c295836d8d09565fd6814) |
| **Fixed Commit**      | [8e970aa](https://github.com/maciejhirsz/beef/tree/8e970aaa60471a845a309c0fe82ebe59779341ca) |
| **Variants**          | - [fixed-crate](fixed-crate)                                                                 |
|                       | - [fixed-file](fixed-file)                                                                   |
|                       | - [fixed-function](fixed-function)                                                           |
|                       | - [vuln-crate](vuln-crate)                                                                   |
|                       | - [vuln-file](vuln-file)                                                                     |
|                       | - [vuln-function](vuln-function)                                                             |

### Vulnerable Lines

`src/generic.rs`

```rust
// The issue is that the bound on Send implementation for Cow is not bounded by the Sync. Without this it's possible to create references to a non-Sync object like Cell through the use of a borrowed Cow.

unsafe impl<T: Beef + Send + ?Sized, U: Capacity> Send for Cow<'_, T, U> {}
```
