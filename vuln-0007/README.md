# Vulnerability: CVE-2020-35890

| **Information**       | **Details**                                                                                       |
|-----------------------|---------------------------------------------------------------------------------------------------|
| **CVE**               | [CVE-2020-35890](https://rustsec.org/advisories/RUSTSEC-2020-0038.html)                           |
| **Vulnerable Commit** | [e1f1e3b](https://github.com/maciejhirsz/ordnung/commit/e1f1e3bda332dae33b76ca4be00dba46265e4cbe) |
| **Fixed Commit**      | None                                                                                              |
| **Variants**          | - [vuln-crate](vuln-crate)                                                                        |
|                       | - [vuln-file](vuln-file)                                                                          |
|                       | - [vuln-function](vuln-function)                                                                  |

### Vulnerable lines

https://github.com/maciejhirsz/ordnung/issues/8

1. OOB is possible because `MASK_HI` is 0 in 32-bit build
    ```rust
    const MASK_HI: usize = !(core::u32::MAX as usize);
    ```
    len / cap packing doesn't work in a 32-bit environment; the capacity will be always zero. Checks like this will always
    pass in a 32-bit environment:
    ```rust
    if (capacity & MASK_HI) != 0 {
        panic!("compact Vec capacity out of bounds");
    }
    ```
2. Allocator layout mismatch in 64-bit build with large capacity.
    
    ```rust
    const MASK_LO: usize = core::u32::MAX as usize;
    ```
    `with_capacity` function allocates a standard Rust vector with the given capacity
    and passes it to from_stdvec_unchecked
    ```rust
    pub fn with_capacity(capacity: usize) -> Self {
        Self::from_stdvec_unchecked(StdVec::with_capacity(capacity))
    }
    ```
   However, the capacity of the standard Rust vector is preprocessed as `cap & MASK_LO`
    ```rust
    fn from_stdvec_unchecked(stdvec: StdVec<T>) -> Self {
        let mut stdvec = ManuallyDrop::new(stdvec);
    
        let ptr = stdvec.as_mut_ptr();
        let len = stdvec.len();
        let cap = stdvec.capacity();
    
        let ptr = slice_from_raw_parts_mut(ptr, len & MASK_LO | (cap & MASK_LO) << 32);
    
        Vec {
            ptr: unsafe { NonNull::new_unchecked(ptr) },
        }
    }
    
    #[inline]
    fn parts(&self) -> (usize, usize) {
        let parts = unsafe { &*(self.ptr.as_ptr() as *const [()]) }.len();

        (parts & MASK_LO, (parts & MASK_HI) >> 32)
    }
    ```
    This preprocessed capacity is used for dropping:
    ```rust
    impl<T> core::ops::Drop for Vec<T> {
        fn drop(&mut self) {
            let (len, cap) = self.parts();

            unsafe {
                StdVec::from_raw_parts(self.as_mut_ptr(), len, cap);
            }
        }
    }

    ```
    `MASK_LO` is `core::u32::MAX`, which is `(1 << 32) - 1`.

    The following code would create a `compact::Vec`, which would make a memory allocation of size `(1 << 32) + 4` 
    bytes. But since the capacity of the standard Rust vector is preprocessed as 
    `cap & MASK_LO == ((1 << 32) + 4) & MASK_LO == 4` is only 4, dropping this 
    `compact::Vec` would result in a memory deallocation of size 4 bytes:
    ```rust
    let _ = Vec::<u8>::with_capacity((1 << 32) + 4);
    ```
3. Creating a reference that points to an invalid value is already an undefined behavior, even if it is not 
dereferenced.

    ```rust
    let ptr = slice_from_raw_parts_mut(ptr, len & MASK_LO | (cap & MASK_LO) << 32); 
   
    // ...

   NonNull::new_unchecked(slice_from_raw_parts_mut( 
     ptr as *mut T, 
     (len & MASK_LO) | ((capacity & MASK_LO) << 32), 
   ))
   ```