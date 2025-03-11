# Vulnerability:  CVE-2020-35916	

| **Information**       | **Details**                                                                                |
| --------------------- | ------------------------------------------------------------------------------------------ |
| **CVE**               | [CVE-2020-35916](https://rustsec.org/advisories/RUSTSEC-2020-0073.html)                    |
| **Vulnerable Commit** | [94b6749](https://github.com/HeroicKatora/image/tree/94b674913107ca2b72a7eae81d95e96c783ca433) |
| **Fixed Commit**  | [5cbe1e6](https://github.com/image-rs/image/tree/5cbe1e6767d11aff3f14c7ad69a06b04e8d583c7) |
| **Variants**          | - [fixed-crate](fixed-crate)|
|                       | - [fixed-file](fixed-file)|
|                       | - [fixed-function](fixed-function)|
|                       | - [vuln-crate](vuln-crate)|
|                       | - [vuln-file](vuln-file)|
|                       | - [vuln-function](vuln-function)|

## Vulnerable Lines
```
// .as_ptr() returns a const* T, but we promise a mut* T
// Should use .as_mut_ptr()
fn from_slice_mut(slice: &mut [T]) -> &mut $ident<T> {
    assert_eq!(slice.len(), $channels);
    unsafe { &mut *(slice.as_ptr() as *mut $ident<T>) }
}
```