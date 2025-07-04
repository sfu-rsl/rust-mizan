pub struct Array<T> {
    size: usize,
    ptr: *mut T,
}

unsafe impl<T> Sync for Array<T>{}
unsafe impl<T> Send for Array<T>{}
