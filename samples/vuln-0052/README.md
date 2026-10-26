# Vulnerability: CVE-2019-15551

| **Information**       | **Details**                                                                                        |
| --------------------- | -------------------------------------------------------------------------------------------------- |
| **CVE**               | [CVE-2019-15551](https://www.cve.org/CVERecord?id=CVE-2019-15551)                                  |
| **Vulnerable Commit** | [88b62b65](https://github.com/servo/rust-smallvec/commit/88b62b65fae5bc144d9e9cacf0778c8ed1ed509b) |
| **Fixed Commit**      | [4ba0d0f6](https://github.com/servo/rust-smallvec/commit/4ba0d0f689440963e38b8adbe7fc2cabc6e573d5) |
| **Variants**          | - [vuln-crate](vuln-crate)                                                                         |
|                       | - [vuln-function](vuln-function)                                                                   |
|                       | - [fixed-crate](fixed-crate)                                                                       |
|                       | - [fixed-function](fixed-function)                                                                 |

### Vulnerable Lines

`lib.rs`

```rust
// If `grow` is called on a spilled SmallVec with `new_cap` equal to the current capacity, 
// the function (incorrectly) deallocates the internal heap buffer which leaves a dangling 
// pointer inside SmallVec, leading to a double free or use-after-free on subsequent 
// accesses or during drop. 

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
            self.data = SmallVecData::from_inline(mem::uninitialized());
            ptr::copy_nonoverlapping(ptr, self.data.inline_mut().ptr_mut(), len);
            self.capacity = len;
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
        // Vulnerable line: if new_cap == cap, we deallocate instead of performing a no-op
        deallocate(ptr, cap);
    }
}
```
