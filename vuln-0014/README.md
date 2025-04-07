# Vulnerability: CVE-2019-16139

| **Information**       | **Details**                                                                                     |
| --------------------- | ----------------------------------------------------------------------------------------------- |
| **CVE**               | [CVE-2019-16139](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2019-16139)                         |
| **Vulnerable Commit** | [947fa6e](https://github.com/llogiq/compact_arena/tree/947fa6ebd24c9b03f7673400ef7e76fe6bb816b8) |
| **Fixed Commit**      | [04348f7](https://github.com/llogiq/compact_arena/tree/04348f76a453473a9f80220f1b9e35e488b21c42) |
| **Variants**          | - [fixed-crate](fixed-crate)                                                                    |
|                       | - [fixed-function](fixed-function)                                                              |
|                       | - [vuln-crate](vuln-crate)                                                                        |
|                       | - [vuln-function](vuln-function)                                                                |

### Vulnerable Lines

```rust
// This function does not guarantee a uniqueness of the lifetime 'tag. This lifetime is used as a marker to ensure that any index accesses are legal. It was possible for two lifetimes to merge and to use an index from one arena onto another (index out of bounds).
pub unsafe fn SmallArena::new(_: &'tag mut (), capacity: usize) -> Self

// Note that no changes were made to SmallArena::new. The safeguards were added to the application that interfaces with the macro
macrorules! mk_arena 
```