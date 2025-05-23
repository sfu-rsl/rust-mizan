# Vulnerability: CVE-2020-36432

| **Information**       | **Details**                                                                                    |
| --------------------- | ---------------------------------------------------------------------------------------------- |
| **CVE**               | [CVE-2020-36432](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2020-36432)                |
| **Vulnerable Commit** | [a533f2a](https://gitlab.com/dvshapkin/alg-ds/-/tree/a533f2a1520dc1a3688e8bd3a1e7c0b60eb5f3a9) |
| **Fixed Commit**      | N/A                                                                                            |
| **Variants**          | - [vuln-crate](vuln-crate)                                                                     |
|                       | - [vuln-file](vuln-file)                                                                       |
|                       | - [vuln-function](vuln-function)                                                               |

### Vulnerable Lines

```rust
/// Memory allocation for data buffer.
///
fn alloc(rows: usize, cols: usize) -> &'a mut [T] {
	unsafe {
		let buf = alloc::alloc(layout::<T>(rows * cols).unwrap()) as *mut T;
		let slice = std::slice::from_raw_parts_mut(buf, rows * cols);
		// The allocated buffer is uninitialized
		Self::fill_with(slice, T::default());
		slice
	}
}

/// Fills data buffer with a `value`.
///
fn fill_with(buf: &mut [T], value: T) {
	for e in buf {
		// This assumes that the buffer is already initialized
		*e = value.clone();
	}
}
```

Note: This vulnerability remains unpatched, as no fixed commit has been provided.
