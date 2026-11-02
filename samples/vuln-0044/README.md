# Vulnerability: CVE-2020-35885

| **Information**       | **Details**                                                                                               |
| --------------------- | --------------------------------------------------------------------------------------------------------- |
| **CVE**               | [CVE-2020-35885](https://nvd.nist.gov/vuln/detail/CVE-2020-35885)                                         |
| **Vulnerable Commit** | [8458c22](https://github.com/pigeonhands/rust-arch/tree/8458c22a161cb8996659fd124de49972f8164712/alpm-rs) |
| **Fixed Commit**      |                                                                                                           |
| **Variants**          | - [vuln-crate](vuln-crate)                                                                                |
|                       | - [vuln-file](vuln-file)                                                                                  |
|                       | - [vuln-function](vuln-function)                                                                          |

### Vulnerable Lines

`src/macros.rs`

```rust
// The issue is that the implementation of StrcCtx drops the memory not allocated by it. Which in turn creates problems of double free and use after free for the client programs.

// creates a raw c-string
// and deallocates it in the deconstructor
pub struct StrcCtx{
    pub ptr: *mut c_char,
}

impl StrcCtx {
    pub fn new(s: &str) -> StrcCtx{
        StrcCtx{
            ptr: strc_noctx!(s),
        }
    }
}

impl Drop for StrcCtx{
    fn drop(&mut self) {
        unsafe{
             let _ = str_fromraw!(self.ptr);
        }
    }
}
```
