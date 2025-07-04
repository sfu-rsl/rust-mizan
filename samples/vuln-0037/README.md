# Vulnerability: CVE-2020-35873

| **Information**       | **Details**                                                                                     |
| --------------------- | ----------------------------------------------------------------------------------------------- |
| **CVE**               | [CVE-2020-35873](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2020-35873)                 |
| **Vulnerable Commit** | [156fa9f](https://github.com/rusqlite/rusqlite/commit/156fa9fcf23cadd5db3bdeeca11e5a25cd813e5b) |
| **Fixed Commit**      | [54043c8](https://github.com/rusqlite/rusqlite/commit/54043c803c83517aa54b463b098a61dfa28c54f4) |
| **Variants**          | - [fixed-crate](fixed-crate)                                                                    |
|                       | - [vuln-crate](vuln-crate)                                                                      |

### Vulnerable Lines

Not vulnerable but for reference

```rust
fn str_to_cstring(s: &str) -> Result<CString> {
    Ok(CString::new(s)?)
}
```

`src/session.rs`

```rust
/// Attach a table. `None` means all tables.
pub fn attach(&mut self, table: Option<&str>) -> Result<()> {
	let table = if let Some(table) = table {
		str_to_cstring(table)?.as_ptr()
	} else {
		ptr::null()
	};
	// VULNERABILITY: Unsafe use of a pointer to freed memory
	unsafe { check!(ffi::sqlite3session_attach(self.s, table)) };
	Ok(())
}


/// Load the difference between tables.
pub fn diff(&mut self, from: DatabaseName<'_>, table: &str) -> Result<()> {
	let from = from.to_cstring()?;
	// CString is created and immediately `.as_ptr()` is taken
    // but the CString is dropped at the end of this statement.
	let table = str_to_cstring(table)?.as_ptr();
	unsafe {
		let mut errmsg = ptr::null_mut();
		// VULNERABILITY: FFI call uses a dangling pointer to freed CString.
		let r =
			ffi::sqlite3session_diff(self.s, from.as_ptr(), table, &mut errmsg as *mut *mut _);
		if r != ffi::SQLITE_OK {
			let errmsg: *mut c_char = errmsg;
			let message = errmsg_to_string(&*errmsg);
			ffi::sqlite3_free(errmsg as *mut ::std::os::raw::c_void);
			return Err(error_from_sqlite_code(r, Some(message)));
		}
	}
	Ok(())
}
```
