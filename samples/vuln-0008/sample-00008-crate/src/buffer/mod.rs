//! Memory buffer API
//!
//! The `Buffer` trait defined here allows the memory buffer of the queue to
//! be defined independently from the queue implementation.

pub mod dynamic;

/// All buffers must implement this trait to be used with any of the queues.
pub trait Buffer<T> {
    /// Return the size of the buffer
    fn size(&self) -> usize;

    /// Return a pointer to data at the given index. It is expected that this
    /// function use modular arithmetic since `idx` may refer to a location
    /// beyond the end of the buffer.
    fn at(&self, idx: usize) -> *const T;

    /// Return a mutable pointer to data at the given index. It is expected
    /// that this function use modular arithmetic since `idx` may refer to a
    /// location beyond the end of the buffer.
    fn at_mut(&mut self, idx: usize) -> *mut T;
}
