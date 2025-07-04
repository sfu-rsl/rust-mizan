//! Fast and lightweight Slab Allocator.

use std::ptr;

pub struct Slab<T> {
    capacity: usize,
    len: usize,
    mem: *mut T,
}

impl<T> Slab<T> {
    /// Removes the element at `offset`.
    ///
    /// # Panics
    ///
    /// * If `offset` is out of bounds.
    #[inline]
    pub fn remove(&mut self, offset: usize) -> T {
        assert!(offset < self.len, "Offset out of bounds");

        let elem: T;
        let last_elem: T;
        let elem_ptr: *mut T;
        let last_elem_ptr: *mut T;

        unsafe {
            elem_ptr = self.mem.offset(offset as isize);
            last_elem_ptr = self.mem.offset((self.len - 1) as isize);

            elem = ptr::read(elem_ptr);
            last_elem = ptr::read(last_elem_ptr);

            ptr::write(elem_ptr, last_elem);
        }

        self.len -= 1;
        return elem;
    }
}
