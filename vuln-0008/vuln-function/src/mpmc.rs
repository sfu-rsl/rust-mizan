//! Multiple-producer multiple-consumer queue
//!
//! The MPMC queue allows for pushing from one thread and popping from another.
//! Both the producer and consumer ends of the queue may be accessed by
//! multiple threads. In other words, both `MPMCProducer` and `MPMCConsumer`
//! are `Send` and `Sync`.

use std::cell::UnsafeCell;
use std::marker::PhantomData;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

use super::{Consumer, Producer, PushError, TryPushError, PopError, TryPopError};
use super::buffer::Buffer;
use crate::util::{pause, buf_read, buf_write};

//#[repr(C)]
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
unsafe impl<T, B: Buffer<T>> Sync for MPMCProducer<T, B> {}

