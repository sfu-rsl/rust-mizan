# Vulnerability: CVE-2020-35869

| **Information**       | **Details**                                                                                     |
| --------------------- | ----------------------------------------------------------------------------------------------- |
| **CVE**               | [CVE-2020-35869](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2020-35869)                 |
| **Vulnerable Commit** | [156fa9f](https://github.com/rusqlite/rusqlite/commit/156fa9fcf23cadd5db3bdeeca11e5a25cd813e5b) |
| **Fixed Commit**      | [54043c8](https://github.com/rusqlite/rusqlite/commit/54043c803c83517aa54b463b098a61dfa28c54f4) |
| **Variants**          | - [fixed-crate](fixed-crate)                                                                    |
|                       | - [vuln-crate](vuln-crate)                                                                      |

### Vulnerable Lines

`src/trace.rs`

```rust
pub fn log(err_code: c_int, msg: &str) {
    let msg = CString::new(msg).expect("SQLite log messages cannot contain embedded zeroes");
    unsafe {
        // VULNERABILITY: msg.as_ptr() is passed directly as the format string.
        // If the message contains format specifiers like `%s`, `%n`, etc.,
        // SQLite's internal printf will interpret them
        ffi::sqlite3_log(err_code, msg.as_ptr());
    }
}
```
