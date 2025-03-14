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

The majority of the functions are vulnerable as a result of improper handling of the vector capacity. In particular, 
anything that uses `compact::Vec::capacity` or the 2nd tuple argument from `compact::Vec::parts` can potentially do
out-of-bound accesses.

https://github.com/maciejhirsz/ordnung/issues/8

1. Out-of-bound access is possible because `MASK_HI` is 0 in 32-bit build
    ```rust
    const MASK_HI: usize = !(core::u32::MAX as usize);
    ```
    len / cap packing doesn't work in a 32-bit environment; `MASK_HI == 0` implies the value of `capacity & MASK_HI` 
    is 0. 
    
    Therefore, checks like these will always pass in a 32-bit environment:
    
    ```rust
    unsafe fn pack<T>(ptr: *mut T, len: usize, capacity: usize) -> NonNull<[T]> {
      if (capacity & MASK_HI) != 0 {
          panic!("compact Vec capacity out of bounds");
      }
    
      pack_unchecked(ptr, len, capacity)
    }
    // ...
    pub fn push(&mut self, val: T) {
        let ptr = self.as_mut_ptr();
        let (len, cap) = self.parts();

        if len == cap {
            // Code to reallocate the vector array buffer not shown.
            // When creating a compact::Vec from an existing, nonempty standard Rust vector, 
            // len will be nonzero. However, in a 32-bit environment, `cap` is always 0
            // Thus len == cap would always be false 
            // ...
        }
        unsafe {
            // Will write out-of-bounds because array buffer is never reallocated
            self.as_mut_ptr().add(len).write(val);
            self.set_len(len + 1);
        }
    }
    ```
     
    The following PoC was used by the reporter to demonstrate an OOB access (and failure to deallocate):

    ```rust
    use ordnung::compact::Vec as CompactVec;
    
    // ...
    
    let mut vec: Vec<u8> = Vec::with_capacity(1);
    vec.push(1);
    let mut compact_vec = CompactVec::from(vec);
    println!("pointer: {:p}", compact_vec.as_ptr());
    println!("len: {}", compact_vec.len());
    println!("capacity: {}", compact_vec.capacity());
    
    // Allows us to push elements over the boundary because `len != cap`
    for i in 0..4 {
        compact_vec.push(i);
        println!("Pushed {}", i);
    }
    println!("len: {}", compact_vec.len());
    println!("capacity: {}", compact_vec.capacity());

    // cap is 0, so no deallocation
    ```
   
    ```
    [TRACE] > alloc [address=0x9fb15d0, size=1, align=1]
    pointer: 0x9fb15d0
    len: 1
    capacity: 0
    Pushed 0
    Pushed 1
    Pushed 2
    Pushed 3
    len: 5
    capacity: 0
    ```

   This was remarked by the reporter as not a real security threat, as the code will scream and panic in the debug 
   build; however, they note it is still a soundness issue, since it allows undefined behavior in safe Rust code.

   If it's not meant for 32-bit systems, it shouldn't compile on 32-bit systems.

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
   
    Although the use cases for the crate talk about the maps being for <= 100 items, there is still bounds checking
    being done elsewhere in the crate. (And even if there wasn't bounds checking in the first place, there still 
    should be, as the intention of it being for small sizes is not enforced anywhere.)
3. Creating a reference that points to an invalid value is already an undefined behavior, even if it is not 
dereferenced. The [T] field is not directly used anywhere in the `compact` module

    ```rust
    let ptr = slice_from_raw_parts_mut(ptr, len & MASK_LO | (cap & MASK_LO) << 32); 
   
    // ...

    NonNull::new_unchecked(slice_from_raw_parts_mut( 
      ptr as *mut T, 
      (len & MASK_LO) | ((capacity & MASK_LO) << 32), 
    ))
    ```