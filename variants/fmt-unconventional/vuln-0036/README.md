# Vulnerability: CVE-2020-35872

| **Information**       | **Details**                                                                                     |
| --------------------- | ----------------------------------------------------------------------------------------------- |
| **CVE**               | [CVE-2020-35872](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2020-35872)                 |
| **Vulnerable Commit** | [156fa9f](https://github.com/rusqlite/rusqlite/commit/156fa9fcf23cadd5db3bdeeca11e5a25cd813e5b) |
| **Fixed Commit**      | [71b2f51](https://github.com/rusqlite/rusqlite/commit/71b2f5187b0cbace3f8b6ff53432ff2ca0defcf0) |
| **Variants**          | - [fixed-crate](fixed-crate)                                                                    |
|                       | - [vuln-crate](vuln-crate)                                                                      |

### Vulnerable Lines

`src/functions.rs`

```rust
// The issue is that Rust’s default `repr(Rust)` layout is not stable across compiler versions or guaranteed to be C-compatible so casting such structures to/from `c_void` in FFI boundaries can lead to undefined behavior

/// Sets the auxilliary data associated with a particular parameter. See
/// https://www.sqlite.org/c3ref/get_auxdata.html for a discussion of
/// this feature, or the unit tests of this module for an example.
pub fn set_aux<T: 'static>(&self, arg: c_int, value: T) {
	let boxed = Box::into_raw(Box::new((std::any::TypeId::of::<T>(), value)));
	unsafe {
		// VULNERABILITY: Passes the repr(Rust) tuple to an external C function, leading to potential layout mismatch and undefined behavior
		ffi::sqlite3_set_auxdata(
			self.ctx,
			arg,
			boxed as *mut c_void,
			Some(free_boxed_value::<(std::any::TypeId, T)>),
		)
	};
}

/// Gets the auxilliary data that was associated with a given parameter
/// via `set_aux`. Returns `Ok(None)` if no data has been associated,
/// and .
pub fn get_aux<T: 'static>(&self, arg: c_int) -> Result<Option<&T>> {
	let p = unsafe { ffi::sqlite3_get_auxdata(self.ctx, arg) as *mut (std::any::TypeId, T) };
	if p.is_null() {
		Ok(None)
	} else {
		// VULNERABILITY: Dereferencing a possibly misaligned or invalid pointer
		let id_val = unsafe { &*p };
		// VULNERABILITY: Accessing a specific tuple element based on assumed layout, unsafe without repr(C)
		if std::any::TypeId::of::<T>() != id_val.0 {
			Err(Error::GetAuxWrongType)
		} else {
			Ok(Some(&id_val.1))
		}
	}
}
```
