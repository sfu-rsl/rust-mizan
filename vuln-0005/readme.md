# Vulnerability: CVE-2020-35916

| **Information**       | **Details**                                                                                    |
| --------------------- | ---------------------------------------------------------------------------------------------- |
| **CVE**               | [CVE-2020-35916](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2020-35916)                |
| **Vulnerable Commit** | [94b6749](https://github.com/HeroicKatora/image/tree/94b674913107ca2b72a7eae81d95e96c783ca433) |
| **Fixed Commit**      | [5cbe1e6](https://github.com/image-rs/image/tree/5cbe1e6767d11aff3f14c7ad69a06b04e8d583c7)     |
| **Variants**          | - [fixed-crate](fixed-crate)                                                                   |
|                       | - [fixed-file](fixed-file)                                                                     |
|                       | - [fixed-function](fixed-function)                                                             |
|                       | - [vuln-crate](vuln-crate)                                                                     |
|                       | - [vuln-file](vuln-file)                                                                       |
|                       | - [vuln-function](vuln-function)                                                               |

## Vulnerable Lines

```rust
// The function promises a mutable reference while returning a const one.
fn from_slice_mut(slice: &mut [T]) -> &mut $ident<T> {
    assert_eq!(slice.len(), $channels);

    // Vulnerability here
    unsafe { &mut *(slice.as_ptr() as *mut $ident<T>) }
}

// This usually gets caught by the compiler
```
