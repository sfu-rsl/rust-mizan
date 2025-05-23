# Vulnerability: CVE-2020-25793

| **Information**       | **Details**                                                                                    |
| --------------------- | ---------------------------------------------------------------------------------------------- |
| **CVE**               | [CVE-2020-25793](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2020-25793)                |
| **Vulnerable Commit** | [40aa74b](https://github.com/bodil/sized-chunks/tree/40aa74b824688a4d4b1e1c65a50c679abb58b41e) |
| **Fixed Commit**      | [9f66983](https://github.com/bodil/sized-chunks/tree/9f66983f058944da5f402202ac5708089051a237) |
| **Variants**          | - [fixed-crate](fixed-crate)                                                                   |
|                       | - [fixed-file](fixed-file)                                                                     |
|                       | - [fixed-function](fixed-function)                                                             |
|                       | - [vuln-crate](vuln-crate)                                                                     |
|                       | - [vuln-file](vuln-file)                                                                       |
|                       | - [vuln-function](vuln-function)                                                               |

### Vulnerable Lines

`src/sized_chunk/mod.rs`

```rust
fn from(array: &mut InlineArray<A, T>) -> Self {
	let mut out = Self::new();
	out.left = 0;
	out.right = array.len();
	unsafe {
		// `ptr::copy_nonoverlapping` copies `array.len()` elements without checking if `out` has enough capacity
		ptr::copy_nonoverlapping(array.data(), out.mut_ptr(0), out.right);
		// Setting `array.len_mut()` to 0 assumes the entire array was moved
		*array.len_mut() = 0;
	}
	out
}
```
