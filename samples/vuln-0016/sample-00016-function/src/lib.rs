use std::ops::{Index, IndexMut, Range};

pub struct Array<T> {
    size: usize,
    ptr: *mut T,
}

impl<T> Index<usize> for Array<T> {
    type Output = T;

    fn index<'a>(&'a self, idx: usize) -> &'a Self::Output {

        unsafe {
            self.ptr.wrapping_offset(idx as isize).as_ref()
        }.unwrap()
    }
}

impl<T> IndexMut<usize> for Array<T> {

    fn index_mut<'a>(&'a mut self, idx: usize) -> &'a mut Self::Output {

        unsafe {
            self.ptr.wrapping_offset(idx as isize).as_mut()
        }.unwrap()
    }
}
