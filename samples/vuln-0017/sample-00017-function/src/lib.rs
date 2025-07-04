use std::alloc::{alloc, alloc_zeroed, dealloc, Layout};

pub struct Array<T> {
    size: usize,
    ptr: *mut T,
}

impl<T> Array<T>
  where T: Clone {
    /// More generic initialization instantiating all elements as copies of some template
    pub fn new_from_template(size: usize, template: &T) -> Self {
        let objsize = std::mem::size_of::<T>();
        let layout = Layout::from_size_align(size * objsize, 8).unwrap();
        let ptr = unsafe {
            alloc(layout) as *mut T
        };
        for i in 0..size {
            unsafe {
                (*(ptr.wrapping_offset(i as isize))) = template.clone();
            }
        }
        Self{size, ptr}
    }
}
