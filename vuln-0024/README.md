# Vulnerability: CVE-2020-35892

| **Information**       | **Details**                                                                                            |
| --------------------- | ------------------------------------------------------------------------------------------------------ |
| **CVE**               | [CVE-2020-35892](https://rustsec.org/advisories/RUSTSEC-2020-0039.html)                                |
| **Vulnerable Commit** | [f1b18e1](https://github.com/nathansizemore/simple-slab/tree/f1b18e1ed42b5477d43c837155998d566fdaf461) |
| **Fixed Commit**      | [5e0524c](https://github.com/nathansizemore/simple-slab/tree/5e0524c1db836e2192e1cd818848d96937c0b587) |
| **Variants**          | - [vuln-crate](vuln-crate)                                                                             |
|                       | - [vuln-function](vuln-function)                                                                       |
|                       | - [fixed-crate](fixed-crate)                                                                           |
|                       | - [fixed-function](fixed-function)                                                                     |

### Vulnerable Lines

`src/lib.rs`

```rust
impl<T> Index<usize> for Slab<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        // The function does not validate whether `index` is within the valid range of allocated memory.
        unsafe { &(*(self.mem.offset(index as isize))) }
    }
}
```
