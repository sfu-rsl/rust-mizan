//! Single-producer multiple-consumer queue
//!
//! The SPMC queue allows for pushing from one thread and popping from another.
//! The producer end of the queue may be accessed by a single thread while the
//! consumer end may be accessed by multiple threads. In other words,
//! `SPMCProducer` is `Send` and `!Sync` while `SPMCConsumer` is `Send` and
//! `Sync`.

use std::cell::UnsafeCell;
use std::marker::PhantomData;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

use super::{Consumer, Producer, PushError, TryPushError, PopError, TryPopError};
use super::buffer::Buffer;
use crate::util::{pause, buf_read, buf_write};

//#[repr(C)]
struct SPMCQueue<T, B: Buffer<T>> {
    head: AtomicUsize,
    _pad1: [u8; 56],
    tail: AtomicUsize,
    next_tail: AtomicUsize,
    _pad2: [u8; 48],
    buf: B,
    ok: AtomicBool,
    _marker: PhantomData<T>
}

unsafe impl<T, B: Buffer<T>> Sync for SPMCQueue<T, B> {}

/// Consumer end of the queue. Implements the trait `Consumer<T>`.
pub struct SPMCConsumer<T, B: Buffer<T>> {
    queue: Arc<UnsafeCell<SPMCQueue<T, B>>>,
}

unsafe impl<T, B: Buffer<T>> Send for SPMCConsumer<T, B> {}
unsafe impl<T, B: Buffer<T>> Sync for SPMCConsumer<T, B> {}

/// Producer end of the queue. Implements the trait `Producer<T>`.
pub struct SPMCProducer<T, B: Buffer<T>> {
    queue: Arc<UnsafeCell<SPMCQueue<T, B>>>,
}

unsafe impl<T, B: Buffer<T>> Send for SPMCProducer<T, B> {}
unsafe impl<T, B: Buffer<T>> Sync for SPMCProducer<T, B> {}

