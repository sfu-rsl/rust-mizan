use std::mem;
use std::ptr;

use super::buffer::Buffer;

#[cfg(all(feature="unstable", any(target_arch = "x86", target_arch = "x86_64")))]
#[inline(always)]
pub fn pause() {
    unsafe { asm!("PAUSE") };
}

#[cfg(all(not(feature="unstable"), any(target_arch = "x86", target_arch = "x86_64")))]
#[inline(always)]
pub fn pause() {
    // nop
}

#[cfg(all(not(target_arch = "x86"), not(target_arch = "x86_64")))]
#[inline(always)]
pub fn pause() {
    // nop
}

pub fn alloc<T>(size: usize) -> *mut T {
    let mut vec = Vec::with_capacity(size);
    let ptr = vec.as_mut_ptr();
    mem::forget(vec);
    ptr
}

pub fn dealloc<T>(ptr: *mut T, size: usize) {
    unsafe { Vec::from_raw_parts(ptr, 0, size); }
}

#[inline(always)]
pub fn buf_write<T, B: Buffer<T>>(buf: &mut B, head: usize, value: T) {
    unsafe { ptr::write(buf.at_mut(head), value) }
}

#[inline(always)]
pub fn buf_read<T, B: Buffer<T>>(buf: &B, tail: usize) -> T {
    unsafe { ptr::read(buf.at(tail)) }
}
