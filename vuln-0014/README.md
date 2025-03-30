# Vulnerability: CVE-2019-16139

| **Information**       | **Details**                                                                                     |
| --------------------- | ----------------------------------------------------------------------------------------------- |
| **CVE**               | [RUSTSEC-2019-0015](https://rustsec.org/advisories/RUSTSEC-2019-0015.html)                         |
| **Vulnerable Commit** | [0b2b4e5](https://github.com/servo/rust-smallvec/tree/0b2b4e53a5cf86740b2515907dd53252b3b59401) |
| **Fixed Commit**      | [9998ba0](https://github.com/servo/rust-smallvec/tree/9998ba0694a6b51aa6604748b00b6a98f0a0039e) |
| **Variants**          | - [fixed-crate](fixed-crate)                                                                    |
|                       | - [fixed-function](fixed-function)                                                              |
|                       | - [vuln-crate](vuln-crate)                                                                        |
|                       | - [vuln-function](vuln-function)                                                                |

### Vulnerable Lines

```rust
// Does not guarantee a uniqueness of lifetime, allowing a different's arena index to be used on it
pub unsafe fn SmallArena::new(_: &'tag mut (), capacity: usize) -> Self
```
