# Vulnerability: CVE-2020-35867

| **Information**       | **Details**                                                                                     |
| --------------------- | ----------------------------------------------------------------------------------------------- |
| **CVE**               | [CVE-2020-35867](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2020-35867)                 |
| **Vulnerable Commit** | [156fa9f](https://github.com/rusqlite/rusqlite/commit/156fa9fcf23cadd5db3bdeeca11e5a25cd813e5b) |
| **Fixed Commit**      | [54043c8](https://github.com/rusqlite/rusqlite/commit/54043c803c83517aa54b463b098a61dfa28c54f4) |
| **Variants**          | - [fixed-crate](fixed-crate)                                                                    |
|                       | - [vuln-crate](vuln-crate)                                                                      |

### Vulnerable Lines

`src/vtab/mod.rs`

```rust
impl Connection {
    /// `feature = "vtab"` Register a virtual table implementation.
    ///
    /// Step 3 of [Creating New Virtual Table Implementations](https://sqlite.org/vtab.html#creating_new_virtual_table_implementations).
    pub fn create_module<T: VTab>(
        &self,
        module_name: &str,
        // VULNERABILITY: The module reference is not 'static, but SQLite assumes it is.
		// This can lead to UB if the module is dropped while SQLite still holds a reference.
        module: &Module<T>,
        aux: Option<T::Aux>,
    ) -> Result<()> {
        self.db.borrow_mut().create_module(module_name, module, aux)
    }
}

impl InnerConnection {
    fn create_module<T: VTab>(
        &mut self,
        module_name: &str,
        // VULNERABILITY: Same issue here
        module: &Module<T>,
        aux: Option<T::Aux>,
    ) -> Result<()> {
        let c_name = str_to_cstring(module_name)?;
        let r = match aux {
            Some(aux) => {
                let boxed_aux: *mut T::Aux = Box::into_raw(Box::new(aux));
                unsafe {
                    ffi::sqlite3_create_module_v2(
                        self.db(),
                        c_name.as_ptr(),
                        // VULNERABILITY: SQLite expects this to be 'static
                        &module.base,
                        boxed_aux as *mut c_void,
                        Some(free_boxed_value::<T::Aux>),
                    )
                }
            }
            None => unsafe {
                ffi::sqlite3_create_module_v2(
                    self.db(),
                    c_name.as_ptr(),
                    // VULNERABILITY: SQLite expects this to be 'static
                    &module.base,
                    ptr::null_mut(),
                    None,
                )
            },
        };
        self.decode_result(r)
    }
}
```
