# Vulnerability: CVE-2020-35860

| **Information**       | **Details**                                                                                       |
| --------------------- | ------------------------------------------------------------------------------------------------- |
| **CVE**               | [CVE-2020-35860](https://www.cve.org/CVERecord?id=CVE-2020-35923)                                 |
| **Vulnerable Commit** | [31bae23](https://github.com/TomBebbington/cbox-rs/tree/31bae23ba2d42066c2ac2f9d45f19de4fb04185a) |
| **Fixed Commit**      |                                                                                                   |
| **Variants**          | - [vuln-crate](vuln-crate)                                                                        |
|                       | - [vuln-function](vuln-function)                                                                  |

### Vulnerable Lines

`src/lib.rs`

```rust
// The issues are as follows:
// 1. deref implementation for the CBox<str> calls from _ptr() on the user provided raw pointer without checking for null pointer which can cause dereference of the null raw pointer.
// 2. Within deref implementation, another function is called to create string from CStr without confirming if the initial raw pointer contained data of type u8. This can cause undefined behaviour.
// 3. The object of CSemiBox has a drop function implemented for the raw pointer it is accepting while creating a struct which can cause double free, in some cases.

impl<'a> Deref for CBox<str> {
    type Target = str;
    fn deref(&self) -> &str {
        unsafe {
            //Vulnerable lines 27-28
            let text = CStr::from_ptr(self.ptr);
            str::from_utf8_unchecked(text.to_bytes())
        }
    }
}

impl<'a, D:?Sized> Drop for CSemiBox<'a, D> where D:DisposeRef+'a {
    #[inline(always)]
    /// Run the destructor
    fn drop(&mut self) {
        // Vulnerable line
        unsafe { <D as DisposeRef>::dispose(self.ptr) }
    }
}

```
