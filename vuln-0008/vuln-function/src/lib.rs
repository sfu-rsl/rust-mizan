use std::cell::UnsafeCell;
use std::marker::PhantomData;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

/// All buffers must implement this trait to be used with any of the queues.
trait Buffer<T> {
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


struct MPMCQueue<T, B: Buffer<T>> {
    head: AtomicUsize,
    next_head: AtomicUsize,
    _pad1: [u8; 48],
    tail: AtomicUsize,
    next_tail: AtomicUsize,
    _pad2: [u8; 48],
    buf: B,
    ok: AtomicBool,
    _marker: PhantomData<T>
}

unsafe impl<T, B: Buffer<T>> Sync for MPMCQueue<T, B> {}

/// Consumer end of the queue. Implements the trait `Consumer<T>`.
pub struct MPMCConsumer<T, B: Buffer<T>> {
    queue: Arc<UnsafeCell<MPMCQueue<T, B>>>,
}

unsafe impl<T, B: Buffer<T>> Send for MPMCConsumer<T, B> {}
unsafe impl<T, B: Buffer<T>> Sync for MPMCConsumer<T, B> {}

/// Producer end of the queue. Implements the trait `Producer<T>`.
pub struct MPMCProducer<T, B: Buffer<T>> {
    queue: Arc<UnsafeCell<MPMCQueue<T, B>>>,
}

unsafe impl<T, B: Buffer<T>> Send for MPMCProducer<T, B> {}
unsafe impl<T   , B: Buffer<T>> Sync for MPMCProducer<T, B> {}