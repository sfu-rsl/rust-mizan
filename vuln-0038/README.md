# Vulnerability: RUSTSEC-2025-0033

| **Information**       | **Details**                                                                                          |
| --------------------- | ---------------------------------------------------------------------------------------------------- |
| **CVE**               | [RUSTSEC-2025-0033](https://rustsec.org/advisories/RUSTSEC-2025-0033.html)                           |
| **Vulnerable Commit** | [2893e45](https://github.com/pombredanne/scanner-rs/commit/2893e45f8d60692a11e8584ba01eb38fe7798fea) |
| **Variants**          | - [vuln-crate](vuln-crate)                                                                           |
|                       | - [vuln-function](vuln-function)                                                                     |

### Vulnerable Lines

`src/lib.rs`

```rust
/// Match::get() and Match::ptr() lack sufficient bounds checks, leading to potential out of bounds reads
impl<'a> Match<'a> {
	/// Get the pointer to the location that was matched.
	#[inline]
	pub fn ptr(&self) -> *const u8 {
		/// VULNERABILITY: offset() results in Undefined Behaviour if the pointer is not inside a valid allocated block or is not in bounds.
		/// See https://doc.rust-lang.org/std/primitive.pointer.html#method.offset for more information.
		unsafe { self.haystack.as_ptr().offset(self.at as isize) }
	}

	/// Get a pointer from the store array.
	#[inline]
	pub fn get<T>(&self, idx: usize) -> *const T {
		/// VULNERABILITY: offset() results in Undefined Behaviour if the pointer is not inside a valid allocated block or is not in bounds.
		/// See https://doc.rust-lang.org/std/primitive.pointer.html#method.offset for more information.
		unsafe { self.haystack.as_ptr().offset(self.store[idx] as isize) as *const T }
	}
}
```
