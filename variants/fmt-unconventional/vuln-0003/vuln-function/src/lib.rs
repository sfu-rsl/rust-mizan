use core::mem::forget;
use core::mem::size_of;



pub unsafe fn guarded_transmute_to_bytes_vec<T>(mut from: Vec<T>) -> Vec<u8> {
    let capacity = from.capacity() * size_of::<T>();
    let len = from.len() * size_of::<T>();
    let ptr = from.as_mut_ptr();
    forget(from);
    Vec::from_raw_parts(ptr as *mut u8, capacity, len)
}
