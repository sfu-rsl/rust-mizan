//! Buffer which is allocated at run time

use super::Buffer;
use crate::util::{alloc, dealloc};

/// Holds data allocated from the heap at run time
pub struct DynamicBuffer<T> {
    ptr: *mut T,
    size: usize,
}

impl<T> DynamicBuffer<T> {
    /// Create a new `DynamicBuffer` of the given size. This method will
    /// return an error if the requested number of bytes could not be
    /// allocated.
    pub fn new(size: usize) -> Result<Self, &'static str> {
        if size > 0 {
            Ok(DynamicBuffer{ptr: alloc(size), size: size})
        } else {
            Err("Buffer size must be greater than 0")
        }
    }
}

impl<T> Drop for DynamicBuffer<T> {
    fn drop(&mut self) {
        dealloc(self.ptr, self.size);
    }
}

unsafe impl<T: Send> Send for DynamicBuffer<T> {}

impl<T> Buffer<T> for DynamicBuffer<T> {
    #[inline(always)]
    fn size(&self) -> usize {
        self.size
    }

    #[inline(always)]
    fn at(&self, idx: usize) -> *const T {
        let idx = idx % self.size;
        unsafe { self.ptr.offset(idx as isize) }
    }

    fn at_mut(&mut self, idx: usize) -> *mut T {
        let idx = idx % self.size;
        unsafe { self.ptr.offset(idx as isize) }
    }
}

/// Holds data allocated from the heap at run time. Similar to `DynamicBuffer`
/// except that the size must be a power of two. This will result in slightly
/// faster runtime performance due to the use of a mask instead of modulus
/// when computing buffer indexes.
pub struct DynamicBufferP2<T> {
    ptr: *mut T,
    mask: usize,
}

impl<T> DynamicBufferP2<T> {
    /// Create a new `DynamicBufferP2` of the given size. This method will
    /// return an error if the requested number of bytes could not be
    /// allocated or size is not a power of two.
    pub fn new(size: usize) -> Result<Self, &'static str> {
        match size {
            0 => Err("Buffer size must be greater than 0"),
            _ if ((size - 1) & size) == 0 =>
                Ok(DynamicBufferP2{ptr: alloc(size), mask: size - 1}),
            _ => Err("Buffer size must be a power of two")
        }
    }
}

impl<T> Drop for DynamicBufferP2<T> {
    fn drop(&mut self) {
        dealloc(self.ptr, self.mask + 1);
    }
}

unsafe impl<T: Send> Send for DynamicBufferP2<T> {}

impl<T> Buffer<T> for DynamicBufferP2<T> {
    #[inline(always)]
    fn size(&self) -> usize {
        self.mask + 1
    }

    #[inline(always)]
    fn at(&self, idx: usize) -> *const T {
        let idx = idx & self.mask;
        unsafe { self.ptr.offset(idx as isize) }
    }

    #[inline(always)]
    fn at_mut(&mut self, idx: usize) -> *mut T {
        let idx = idx & self.mask;
        unsafe { self.ptr.offset(idx as isize) }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use super::super::Buffer;

    #[test]
    fn bad_buf() {
        assert!(DynamicBuffer::<u8>::new(0).is_err());
        assert!(DynamicBufferP2::<u8>::new(0).is_err());
        assert!(DynamicBufferP2::<u8>::new(3).is_err());
    }

    #[test]
    fn p2_buf() {
        for i in 0..25 {
            let size = 1 << i;
            assert!(DynamicBufferP2::<u8>::new(size).is_ok());
        }
    }

    fn test_at<T, B: Buffer<T>>(buf: &mut B) {
        let size = buf.size();
        for i in 0..size {
            assert_eq!(buf.at(i), buf.at(i));
            assert_eq!(buf.at_mut(i), buf.at_mut(i));
            assert_eq!(buf.at(i), buf.at(size + i));
            assert_eq!(buf.at_mut(i), buf.at_mut(size + i));
            assert!(buf.at(i) != buf.at(i+1));
            assert!(buf.at_mut(i) != buf.at_mut(i+1));
        }
    }

    #[test]
    fn dynamic_buf() {
        let mut buf = DynamicBuffer::<i32>::new(3).unwrap();
        assert_eq!(buf.size(), 3);
        test_at(&mut buf);
    }

    #[test]
    fn dynamic_buf_p2() {
        let mut buf = DynamicBufferP2::<i32>::new(4).unwrap();
        assert_eq!(buf.size(), 4);
        test_at(&mut buf);
    }
}
