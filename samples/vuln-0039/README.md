# Vulnerability: CVE-2019-15554

| **Information**       | **Details**                                                                                       |
|-----------------------|---------------------------------------------------------------------------------------------------|
| **CVE**               | [CVE-2019-15554](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2019-15554)                   |
| **Vulnerable Commit** | [19de501](https://github.com/servo/rust-smallvec/commit/19de50108d403efaa7cd979eac3bb97a4432fd4b) |
| **Fixed Commit**      | [830d32b](https://github.com/servo/rust-smallvec/commit/830d32bbdb7dc858d1f81d662fe6dd06219f0647) |
| **Variants**          | - [vuln-crate](vuln-crate)                                                                        |
|                       | - [vuln-function](vuln-function)                                                                  |
|                       | - [fixed-crate](fixed-crate)                                                                      |
|                       | - [fixed-function](fixed-function)                                                                |

### Vulnerable Lines

`lib.rs`

```rust
/// Re-allocate to set the capacity to `max(new_cap, inline_size())`.
///
/// Panics if `new_cap` is less than the vector's length.
pub fn grow(&mut self, new_cap: usize) {
    unsafe {
        let (ptr, &mut len, cap) = self.triple_mut();
        let unspilled = !self.spilled();
        assert!(new_cap >= len);
        if new_cap <= self.inline_size() {
            if unspilled {
                return;
            }
            // VULNERABILITY: Because` self.capacity` is not reset after this line, the header still advertises the old (large) heap capacity.
            ptr::copy_nonoverlapping(ptr, self.data.inline_mut().ptr_mut(), len);
        } else if new_cap != cap {
            let mut vec = Vec::with_capacity(new_cap);
            let new_alloc = vec.as_mut_ptr();
            mem::forget(vec);
            ptr::copy_nonoverlapping(ptr, new_alloc, len);
            self.data = SmallVecData::from_heap(new_alloc, len);
            self.capacity = new_cap;
            if unspilled {
                return;
            }
        }
        deallocate(ptr, cap);
    }
}
```
