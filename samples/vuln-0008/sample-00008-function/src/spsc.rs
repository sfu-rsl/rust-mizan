//! Single-producer single-consumer queue
//!
//! The SPSC queue allows for pushing from one thread and popping from another.
//! Each end of the queue can only be owned and accessed from a single thread.
//! In other words, both the `SPSCProducer` and `SPSCConsumer` are `Send` and
//! `!Sync`.

use std::cell::UnsafeCell;
use std::marker::PhantomData;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

use super::{Consumer, Producer, PushError, TryPushError, PopError, TryPopError};
use super::buffer::Buffer;
use crate::util::{pause, buf_read, buf_write};

//#[repr(C)]
struct SPSCQueue<T, B: Buffer<T>> {
    head: AtomicUsize,
    _pad1: [u8; 56],
    tail: AtomicUsize,
    _pad2: [u8; 56],
    buf: B,
    ok: AtomicBool,
    _marker: PhantomData<T>
}

unsafe impl<T, B: Buffer<T>> Sync for SPSCQueue<T, B> {}

/// Consumer end of the queue. Implements the trait `Consumer<T>`.
pub struct SPSCConsumer<T, B: Buffer<T>> {
    queue: Arc<UnsafeCell<SPSCQueue<T, B>>>,
}

unsafe impl<T, B: Buffer<T>> Send for SPSCConsumer<T, B> {}

/// Producer end of the queue. Implements the trait `Producer<T>`.
pub struct SPSCProducer<T, B: Buffer<T>> {
    queue: Arc<UnsafeCell<SPSCQueue<T, B>>>,
}

unsafe impl<T, B: Buffer<T>> Send for SPSCProducer<T, B> {}

