# Vulnerability: RUSTSEC-2025-0032

| **Information**       | **Details**                                                                                     |
| --------------------- | ----------------------------------------------------------------------------------------------- |
| **CVE**               | [RUSTSEC-2025-0032](https://rustsec.org/advisories/RUSTSEC-2025-0032.html)                         |
| **Vulnerable Commit** | [8115456d](https://gitlab.redox-os.org/redox-os/uefi/-/commit/8115456dad76c104848b01acaf3427d9a8a77798) |
| **Fixed Commit**      | [baa4dcc6](https://gitlab.redox-os.org/redox-os/uefi/-/commit/baa4dcc60e3aa061243442c14eec25beff8753ed) |
| **Variants**          | - [fixed-crate](fixed-crate)                                                                    |
|                       | - [fixed-file](fixed-file)                                                                      |
|                       | - [fixed-function](fixed-function)                                                              |
|                       | - [vuln-crate](vuln-crate)                                                                      |
|                       | - [vuln-file](vuln-file)                                                                        |
|                       | - [vuln-function](vuln-function)                                                                |

### Vulnerable lines

`crates/uefi_std/src/ffi.rs`

```rust
/// VULNERABILITY: a heap buffer overflow will occur if a pointer without a trailing 0 is passed in
pub fn nstr(wstring: *const u16) -> String {
    let mut string = String::new();

    let mut i = 0;
    loop {
        /// VULNERABILITY: a heap buffer overflow will occur if a pointer without a trailing 0 is passed in
        let w = unsafe { *wstring.offset(i) };
        i += 1;
        if w == 0 {
            break;
        }
        /// VULNERABILITY: a heap buffer overflow will occur if a pointer without a trailing 0 is passed in
        let c = unsafe { char::from_u32_unchecked(w as u32) };
        string.push(c);
    }

    string
}
```
