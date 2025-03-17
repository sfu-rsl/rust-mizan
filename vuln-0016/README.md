# Vulnerability: CVE-2020-35887

| **Information**       | **Details**                                                                            |
| --------------------- | -------------------------------------------------------------------------------------- |
| **CVE**               | [CVE-2020-35887](https://rustsec.org/advisories/RUSTSEC-2020-0034.html)                |
| **Vulnerable Commit** | [efa2141](https://github.com/sjep/array/tree/efa214159eaad2abda7b072f278d678f8788c307) |
| **Fixed Commit**      | None                                                                                   |
| **Variants**          | - [vuln-crate](vuln-crate)                                                             |
|                       | - [vuln-function](vuln-function)                                                       |

### Vulnerable Lines

```rust
impl<T> Index<usize> for Array<T> {
    type Output = T;

    fn index<'a>(&'a self, idx: usize) -> &'a Self::Output {

        unsafe {
	        // If `idx` is out of bounds, `wrapping_offset`
	        // can lead to reading invalid memory (buffer overflow)
            self.ptr.wrapping_offset(idx as isize).as_ref()
        }.unwrap()
    }
}

impl<T> IndexMut<usize> for Array<T> {

    fn index_mut<'a>(&'a mut self, idx: usize) -> &'a mut Self::Output {

        unsafe {
	        // If `idx` is out of bounds, `wrapping_offset`
	        // can lead to reading invalid memory (buffer overflow)
            self.ptr.wrapping_offset(idx as isize).as_mut()
        }.unwrap()
    }
}
```
