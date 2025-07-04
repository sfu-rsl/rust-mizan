# Vulnerability: CVE-2020-35868

| **Information**       | **Details**                                                                                     |
| --------------------- | ----------------------------------------------------------------------------------------------- |
| **CVE**               | [CVE-2020-35868](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2020-35868)                 |
| **Vulnerable Commit** | [156fa9f](https://github.com/rusqlite/rusqlite/commit/156fa9fcf23cadd5db3bdeeca11e5a25cd813e5b) |
| **Fixed Commit**      | [54043c8](https://github.com/rusqlite/rusqlite/commit/54043c803c83517aa54b463b098a61dfa28c54f4) |
| **Variants**          | - [fixed-crate](fixed-crate)                                                                    |
|                       | - [vuln-crate](vuln-crate)                                                                      |

### Vulnerable Lines

`src/unlock_notify.rs`

```rust
// `fired` and `wait` functions signatures imply exclusive (`&mut`) access to `UnlockNotification`, but the object was actually shared across threads and aliased

#[cfg(feature = "unlock_notify")]
#[allow(clippy::mutex_atomic)]
impl UnlockNotification {
    fn new() -> UnlockNotification {
        UnlockNotification {
            cond: Condvar::new(),
            mutex: Mutex::new(false),
        }
    }

	// VULNERABILITY: &mut self implies unique ownership but the object is shared across threads
    fn fired(&mut self) {
        *self.mutex.lock().unwrap() = true;
        self.cond.notify_one();
    }

	// VULNERABILITY: &mut self implies unique ownership but the object is shared across threads
    fn wait(&mut self) {
        let mut fired = self.mutex.lock().unwrap();
        while !*fired {
            fired = self.cond.wait(fired).unwrap();
        }
    }
}

/// This function is an unlock-notify callback
#[cfg(feature = "unlock_notify")]
unsafe extern "C" fn unlock_notify_cb(ap_arg: *mut *mut c_void, n_arg: c_int) {
    use std::slice::from_raw_parts;
    let args = from_raw_parts(ap_arg, n_arg as usize);
    for arg in args {
        let _ = catch_unwind(|| {
	        // VULNERABILITY: This line casts a shared raw pointer to a &mut which is only valid if no aliases exist
            let un: &mut UnlockNotification = &mut *(*arg as *mut UnlockNotification);
            un.fired()
        });
    }
}
```
