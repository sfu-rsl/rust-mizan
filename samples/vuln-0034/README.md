# Vulnerability: CVE-2020-35870

| **Information**       | **Details**                                                                                     |
| --------------------- | ----------------------------------------------------------------------------------------------- |
| **CVE**               | [CVE-2020-35870](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2020-35870)                 |
| **Vulnerable Commit** | [71b2f51](https://github.com/rusqlite/rusqlite/commit/71b2f5187b0cbace3f8b6ff53432ff2ca0defcf0) |
| **Fixed Commit**      | [54043c8](https://github.com/rusqlite/rusqlite/commit/54043c803c83517aa54b463b098a61dfa28c54f4) |
| **Variants**          | - [fixed-crate](fixed-crate)                                                                    |
|                       | - [vuln-crate](vuln-crate)                                                                      |

### Vulnerable Lines

`src/functions.rs`

```rust
// SQLite lets you add some extra data to function arguments using auxdata. Rusqlite had a way to do that with set_aux and get_aux. The issue is set_aux gave control of the data to SQLite and get_aux would still try to access it (even though SQLite could delete it at any time).

/// Sets the auxilliary data associated with a particular parameter. See
/// https://www.sqlite.org/c3ref/get_auxdata.html for a discussion of
/// this feature, or the unit tests of this module for an example.
pub fn set_aux<T: 'static>(&self, arg: c_int, value: T) {
	let boxed = Box::into_raw(Box::new(AuxData {
		id: TypeId::of::<T>(),
		value,
	}));
	unsafe {
		ffi::sqlite3_set_auxdata(
			self.ctx,
			arg,
			boxed as *mut c_void,
			Some(free_boxed_value::<AuxData<T>>),
		)
	};
}

/// Gets the auxilliary data that was associated with a given parameter
/// via `set_aux`. Returns `Ok(None)` if no data has been associated,
/// and .
pub fn get_aux<T: 'static>(&self, arg: c_int) -> Result<Option<&T>> {
	let p = unsafe { ffi::sqlite3_get_auxdata(self.ctx, arg) as *const AuxData<T> };
	if p.is_null() {
		Ok(None)
	} else {
		// VULNERABILITY: SQLite owns the memory for AuxData<T> after set_aux. This pointer can become invalid at any time
		let id = unsafe { (*p).id };
		if TypeId::of::<T>() != id {
			Err(Error::GetAuxWrongType)
		} else {
			// VULNERABILITY: This creates a reference to memory that may have already been freed (use-after-free)
			Ok(Some(unsafe { &(*p).value }))
		}
	}
}
```
