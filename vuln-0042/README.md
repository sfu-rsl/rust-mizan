# Vulnerability: RUSTSEC-2025-0002

| **Information**       | **Details**                                                                                               |
|-----------------------|-----------------------------------------------------------------------------------------------------------|
| **RUSTSEC**           | [RUSTSEC-2025-0002](https://rustsec.org/advisories/RUSTSEC-2025-0002.html)                                |
| **Vulnerable Commit** | [83a49b8](https://github.com/Alexhuszagh/fast-float-rust/commit/83a49b8b5a530d8e950a6dbfdc11089d213b9ac9) |
| **Fixed Commit**      | [31d1abf](https://github.com/Alexhuszagh/fast-float-rust/commit/31d1abf7c6232f35eb069c64b8290e6fa92802b6) |
| **Variants**          | - [vuln-crate](vuln-crate)                                                                                |
|                       | - [vuln-function](vuln-function)                                                                          |
|                       | - [fixed-crate](fixed-crate)                                                                              |
|                       | - [fixed-function](fixed-function)                                                                        |

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
