fn from_slice_mut<T>(slice: &mut [T]) -> *mut T {
    slice.as_mut_ptr()
}   