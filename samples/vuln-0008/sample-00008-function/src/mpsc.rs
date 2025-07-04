//! Multiple-producer single-consumer queue
//!
//! The MPSC queue allows for pushing from one thread and popping from another.
//! The producer end of the queue may be accessed by multiple threads while
//! the consumer end may only be accessed by a single thread. In other words,
//! the `MPSCProducer` is `Send` and `Sync` while the `MPSCConsumer` is `Send`
//! and `!Sync`.

use std::cell::UnsafeCell;
use std::marker::PhantomData;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

use super::{Consumer, Producer, PushError, TryPushError, PopError, TryPopError};
use super::buffer::Buffer;
use crate::util::{pause, buf_read, buf_write};

//#[repr(C)]
struct MPSCQueue<T, B: Buffer<T>> {
    head: AtomicUsize,
    next_head: AtomicUsize,
    _pad1: [u8; 48],
    tail: AtomicUsize,
    _pad2: [u8; 56],
    buf: B,
    ok: AtomicBool,
    _marker: PhantomData<T>
}

unsafe impl<T, B: Buffer<T>> Sync for MPSCQueue<T, B> {}

/// Consumer end of the queue. Implements the trait `Consumer<T>`.
pub struct MPSCConsumer<T, B: Buffer<T>> {
    queue: Arc<UnsafeCell<MPSCQueue<T, B>>>,
}

unsafe impl<T, B: Buffer<T>> Send for MPSCConsumer<T, B> {}

/// Producer end of the queue. Implements the trait `Producer<T>`.
pub struct MPSCProducer<T, B: Buffer<T>> {
    queue: Arc<UnsafeCell<MPSCQueue<T, B>>>,
}

unsafe impl<T, B: Buffer<T>> Send for MPSCProducer<T, B> {}
unsafe impl<T, B: Buffer<T>> Sync for MPSCProducer<T, B> {}

