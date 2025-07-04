# Vulnerability: CVE-2020-25794

| **Information**       | **Details**                                                                                    |
| --------------------- | ---------------------------------------------------------------------------------------------- |
| **CVE**               | [CVE-2020-25794](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2020-25794)                |
| **Vulnerable Commit** | [40aa74b](https://github.com/bodil/sized-chunks/tree/40aa74b824688a4d4b1e1c65a50c679abb58b41e) |
| **Fixed Commit**      | [9f66983](https://github.com/bodil/sized-chunks/tree/9f66983f058944da5f402202ac5708089051a237) |
| **Variants**          | - [fixed-crate](fixed-crate)                                                                   |
|                       | - [fixed-file](fixed-file)                                                                     |
|                       | - [fixed-function](fixed-function)                                                             |
|                       | - [vuln-crate](vuln-crate)                                                                     |
|                       | - [vuln-file](vuln-file)                                                                       |
|                       | - [vuln-function](vuln-function)                                                               |

### Vulnerable Lines

`src/ring_buffer/mod.rs`

```rust
impl<A: Clone, N: ChunkLength<A>> Clone for RingBuffer<A, N> {
    fn clone(&self) -> Self {
        let mut out = Self::new();
        out.origin = self.origin;
        out.length = self.length;
        for index in out.range() {
            // If `clone()` panics, the function drops more than it initialized.
            unsafe { out.force_write(index, (&*self.ptr(index)).clone()) };
        }
        out
    }
}
```
