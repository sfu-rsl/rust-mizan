# Vulnerability: CVE-2020-25792

| **Information**       | **Details**                                                                                    |
| --------------------- | ---------------------------------------------------------------------------------------------- |
| **CVE**               | [CVE-2020-25792](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2020-25792)                |
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
pub fn pair(value1: A, value2: A) -> Self {
	let mut buffer = Self {
		origin: 0.into(),
		length: 2,
		// This does not initialize the memory properly
		data: MaybeUninit::uninit(),
	};
	unsafe {
		// `force_write` assumes memory is valid but doesn't check array size which leads to potential out-of-bounds write
		buffer.force_write(0.into(), value1);
		// Same
		buffer.force_write(1.into(), value2);
	}
	buffer
}
```

`src/sized_chunk/mod.rs`

```rust
pub fn pair(left: A, right: A) -> Self {
	let mut chunk = Self {
		left: 0,
		right: 2,
		// This does not initialize the memory properly
		data: MaybeUninit::uninit(),
	};
	unsafe {
		// `force_write` assumes memory is valid but doesn't check array size which leads to potential out-of-bounds write
		Chunk::force_write(0, left, &mut chunk);
		// Same
		Chunk::force_write(1, right, &mut chunk);
	}
	chunk
}
```
