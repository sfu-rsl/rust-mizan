# Vulnerability: CVE-2020-26235

| **Information**       | **Details**                                                                                   |
| --------------------- | --------------------------------------------------------------------------------------------- |
| **CVE**               | [CVE-2020-26235](https://www.cve.org/CVERecord?id=CVE-2020-26235)                             |
| **Vulnerable Commit** | [4eeedcf](https://github.com/chronotope/chrono/tree/4eeedcfcc409f19d965f477d767d05f3418c4df1) |
| **Fixed Commit**      | [0b7feac](https://github.com/chronotope/chrono/tree/0b7feacb5482076b4efe1b6bcf720abfc82eb476) |
| **Variants**          | - [vuln-crate](vuln-crate)                                                                    |
|                       | - [vuln-function](vuln-function)                                                              |
|                       | - [fixed-crate](fixed-crate)                                                                  |
|                       | - [fixed-function](fixed-function)                                                            |

### Vulnerable Lines

`src/sys/unix.rs`

```rust
// The issues is that the localtime_r() function of the libc library changes the environment without using lock and hence it is not thread safe. So if any program uses this library in a multithread program, it will create data race and null pointer reference.

// Vulnerable function
pub fn time_to_local_tm(sec: i64, tm: &mut Tm) {
    unsafe {
        let sec = sec as time_t;
        let mut out = mem::zeroed();
        // Vulnerable line
        if libc::localtime_r(&sec, &mut out).is_null() {
            panic!("localtime_r failed: {}", io::Error::last_os_error());
        }
        #[cfg(any(target_os = "solaris", target_os = "illumos"))]
        let gmtoff = {
            tzset();
            // < 0 means we don't know; assume we're not in DST.
            if out.tm_isdst == 0 {
                // timezone is seconds west of UTC, tm_gmtoff is seconds east
                -timezone
            } else if out.tm_isdst > 0 {
                -altzone
            } else {
                -timezone
            }
        };
        #[cfg(not(any(target_os = "solaris", target_os = "illumos")))]
        let gmtoff = out.tm_gmtoff;
        tm_to_rust_tm(&out, gmtoff as i32, tm);
    }
}

```
