fn from_slice_mut<T>(slice: &mut [T]) -> *const T {
    // Vulnerability here 
    slice.as_ptr()
}  