# Vulnerability: CVE-2020-35871

| **Information**       | **Details**                                                                                     |
| --------------------- | ----------------------------------------------------------------------------------------------- |
| **CVE**               | [CVE-2020-35871](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2020-35871)                 |
| **Vulnerable Commit** | [71b2f51](https://github.com/rusqlite/rusqlite/commit/71b2f5187b0cbace3f8b6ff53432ff2ca0defcf0) |
| **Fixed Commit**      | [54043c8](https://github.com/rusqlite/rusqlite/commit/54043c803c83517aa54b463b098a61dfa28c54f4) |
| **Variants**          | - [fixed-crate](fixed-crate)                                                                    |
|                       | - [vuln-crate](vuln-crate)                                                                      |

### Vulnerable Lines

`src/functions.rs`

```rust
// The code returns shared references (`&T`) from a pointer managed by SQLite without enforcing thread safety bounds like `Send + Sync` on `T`.


/// Sets the auxilliary data associated with a particular parameter. See
/// https://www.sqlite.org/c3ref/get_auxdata.html for a discussion of
/// this feature, or the unit tests of this module for an example.
// VULNERABILITY: operates on shared state across threads but don’t require `T: Send + Sync`
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
// VULNERABILITY: operates on shared state across threads but don’t require `T: Send + Sync`
pub fn get_aux<T: 'static>(&self, arg: c_int) -> Result<Option<&T>> {
	let p = unsafe { ffi::sqlite3_get_auxdata(self.ctx, arg) as *const AuxData<T> };
	if p.is_null() {
		Ok(None)
	} else {
		// VULNERABILITY: Reading from shared data (`*p`) without any synchronization
		let id = unsafe { (*p).id };
		if TypeId::of::<T>() != id {
			Err(Error::GetAuxWrongType)
		} else {
			// VULNERABILITY: Returns a shared reference to `T` without any synchronization
			Ok(Some(unsafe { &(*p).value }))
		}
	}
}
```
