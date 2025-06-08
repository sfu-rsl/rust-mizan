# Vulnerability: RUSTSEC-2025-0003

| **Information**       | **Details**                                                                                         |
| --------------------- | --------------------------------------------------------------------------------------------------- |
| **RUSTSEC**           | [RUSTSEC-2025-0003](https://rustsec.org/advisories/RUSTSEC-2025-0003)                               |
| **Vulnerable Commit** | [83a49b8](https://github.com/aldanor/fast-float-rust/tree/83a49b8b5a530d8e950a6dbfdc11089d213b9ac9) |
| **Variants**          | - [vuln-crate](vuln-crate)                                                                          |
|                       | - [vuln-function](vuln-function)                                                                    |

### Vulnerable Lines

`src/common.rs`

```rust
/// The first() function does not have a bounds check when accessing memory.
/// It specifically dereferences a pointer offset by self.ptr which could lead to an
/// invalid memory access if an empty string input is provided.
impl<'a> AsciiStr<'a> {
    #[inline]
    pub fn first(&self) -> u8 {
        // Vulnerability: There is no bounds check on the self.ptr which could lead to an invalid memory access
        unsafe { *self.ptr }
    }
}
```
