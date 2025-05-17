# Vulnerability: RUSTSEC-2025-0033

| **Information**       | **Details**                                                                                                  |
| --------------------- | ------------------------------------------------------------------------------------------------------------ |
| **CVE**               | [RUSTSEC-2025-0033](https://rustsec.org/advisories/RUSTSEC-2025-0033.html)                                   |
| **Vulnerable Commit** | [71b2f51](https://github.com/pombredanne/scanner-rs/commit/2893e45f8d60692a11e8584ba01eb38fe7798fea)         |
| **Fixed Commit**      | [54043c8](https://github.com/pombredanne/scanner-rs/pull/1/commits/277de79e21477ebc2aa82247ffc2fcad1a5c9203) |
| **Variants**          | - [vuln-crate](vuln-crate)                                                                                   |
|                       |                                                                                                              |

### Vulnerable Lines

`src/lib.rs`

```rust
/// Match::get() and Match::ptr() lack sufficient bounds checks, leading to potential out of bounds reads
impl<'a> Match<'a> {
	/// Get the pointer to the location that was matched.
	/// VULNERABILITY: offset() results in Undefined Behaviour if the pointer is not inside a valid allocated block or is not in bounds.
	/// See https://doc.rust-lang.org/std/primitive.pointer.html#method.offset for more information.
	#[inline]
	pub fn ptr(&self) -> *const u8 {
		unsafe { self.haystack.as_ptr().offset(self.at as isize) }
	}

	/// Get a pointer from the store array.
	/// VULNERABILITY: offset() results in Undefined Behaviour if the pointer is not inside a valid allocated block or is not in bounds.
	/// See https://doc.rust-lang.org/std/primitive.pointer.html#method.offset for more information.
	#[inline]
	pub fn get<T>(&self, idx: usize) -> *const T {
		unsafe { self.haystack.as_ptr().offset(self.store[idx] as isize) as *const T }
	}
}
```
