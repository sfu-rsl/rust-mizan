# Vulnerability: CVE-2020-25796

| **Information**       | **Details**                                                                                    |
| --------------------- | ---------------------------------------------------------------------------------------------- |
| **CVE**               | [CVE-2020-25796](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2020-25796)                |
| **Vulnerable Commit** | [40aa74b](https://github.com/bodil/sized-chunks/tree/40aa74b824688a4d4b1e1c65a50c679abb58b41e) |
| **Fixed Commit**      | [9f66983](https://github.com/bodil/sized-chunks/tree/9f66983f058944da5f402202ac5708089051a237) |
| **Variants**          | - [fixed-crate](fixed-crate)                                                                   |
|                       | - [fixed-file](fixed-file)                                                                     |
|                       | - [fixed-function](fixed-function)                                                             |
|                       | - [vuln-crate](vuln-crate)                                                                     |
|                       | - [vuln-file](vuln-file)                                                                       |
|                       | - [vuln-function](vuln-function)                                                               |

### Vulnerable Lines

`src/inline_array/mod.rs`

```rust
#[inline]
#[must_use]
unsafe fn len_const(&self) -> *const usize {
    // If `data` is not aligned, this can cause undefined behavior.
    (&self.data) as *const _ as *const usize
}

#[inline]
#[must_use]
pub(crate) unsafe fn len_mut(&mut self) -> *mut usize {
    // If `data` is not aligned, this can cause undefined behavior.
    (&mut self.data) as *mut _ as *mut usize
}

#[inline]
#[must_use]
pub(crate) unsafe fn data(&self) -> *const A {
    // If `data` is not aligned, this can cause undefined behavior.
    self.len_const().add(1) as *const _ as *const A
}

#[inline]
#[must_use]
unsafe fn data_mut(&mut self) -> *mut A {
    // If `data` is not aligned, this can cause undefined behavior.
    self.len_mut().add(1) as *mut _ as *mut A
}

#[inline]
#[must_use]
unsafe fn ptr_at(&self, index: usize) -> *const A {
    // If `data` is not aligned, this can cause undefined behavior.
    self.data().add(index)
}

#[inline]
#[must_use]
unsafe fn ptr_at_mut(&mut self, index: usize) -> *mut A {
    // If `data` is not aligned, this can cause undefined behavior.
    self.data_mut().add(index)
}

#[inline]
unsafe fn read_at(&self, index: usize) -> A {
    // If `data` is not aligned, this can cause undefined behavior.
    ptr::read(self.ptr_at(index))
}

#[inline]
unsafe fn write_at(&mut self, index: usize, value: A) {
    // If `data` is not aligned, this can cause undefined behavior.
    ptr::write(self.ptr_at_mut(index), value);
}
```
