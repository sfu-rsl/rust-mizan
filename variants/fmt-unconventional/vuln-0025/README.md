# Vulnerability: CVE-2020-35893

| **Information**       | **Details**                                                                                            |
| --------------------- | ------------------------------------------------------------------------------------------------------ |
| **CVE**               | [CVE-2020-35893](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2020-35893)                        |
| **Vulnerable Commit** | [f1b18e1](https://github.com/nathansizemore/simple-slab/tree/f1b18e1ed42b5477d43c837155998d566fdaf461) |
| **Fixed Commit**      | [5e0524c](https://github.com/nathansizemore/simple-slab/tree/5e0524c1db836e2192e1cd818848d96937c0b587) |
| **Variants**          | - [vuln-crate](vuln-crate)                                                                             |
|                       | - [vuln-function](vuln-function)                                                                       |
|                       | - [fixed-crate](fixed-crate)                                                                           |
|                       | - [fixed-function](fixed-function)                                                                     |

### Vulnerable Lines

`src/lib.rs`

```rust
/// Removes the element at `offset`.
///
/// # Panics
///
/// * If `offset` is out of bounds.
#[inline]
pub fn remove(&mut self, offset: usize) -> T {
	assert!(offset < self.len, "Offset out of bounds");

	let elem: T;
	let last_elem: T;
	let elem_ptr: *mut T;
	let last_elem_ptr: *mut T;

	unsafe {
		elem_ptr = self.mem.offset(offset as isize);
		// Off-by-one error
		last_elem_ptr = self.mem.offset(self.len as isize);

		elem = ptr::read(elem_ptr);
		// Reads from an invalid memory address
		last_elem = ptr::read(last_elem_ptr);

		ptr::write(elem_ptr, last_elem);
	}

	self.len -= 1;
	return elem;
}
```
