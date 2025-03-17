# Vulnerability: CVE-2020-35888

| **Information**       | **Details**                                                                            |
| --------------------- | -------------------------------------------------------------------------------------- |
| **CVE**               | [CVE-2020-35888](https://rustsec.org/advisories/RUSTSEC-2020-0034.html)                |
| **Vulnerable Commit** | [efa2141](https://github.com/sjep/array/tree/efa214159eaad2abda7b072f278d678f8788c307) |
| **Fixed Commit**      | None                                                                                   |
| **Variants**          | - [vuln-crate](vuln-crate)                                                             |
|                       | - [vuln-function](vuln-function)                                                       |

### Vulnerable Lines

```rust
impl<T> Array<T>
  where T: Clone {
    /// More generic initialization instantiating all elements as copies of some template
    pub fn new_from_template(size: usize, template: &T) -> Self {
        let objsize = std::mem::size_of::<T>();
        let layout = Layout::from_size_align(size * objsize, 8).unwrap();
        let ptr = unsafe {
	        // The allocated memory is uninitialized
            alloc(layout) as *mut T
        };
        for i in 0..size {
            unsafe {
	            // If `template.clone()` panics, `Array<T>` will drop uninitialized memory
                (*(ptr.wrapping_offset(i as isize))) = template.clone();
            }
        }
        Self{size, ptr}
    }
}
```
