// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! A fixed capacity ring buffer.
//!
//! See [`RingBuffer`](struct.RingBuffer.html)

use core::mem::MaybeUninit;

use typenum::U64;

pub use array_ops::{Array, ArrayMut, HasLength};

mod types;
use types::ChunkLength;

mod index;
use index::{IndexIter, RawIndex};

/// A fixed capacity ring buffer.
///
/// A ring buffer is an array where the first logical index is at some arbitrary
/// location inside the array, and the indices wrap around to the start of the
/// array once they overflow its bounds.
///
/// This gives us the ability to push to either the front or the end of the
/// array in constant time, at the cost of losing the ability to get a single
/// contiguous slice reference to the contents.
///
/// It differs from the [`Chunk`][Chunk] in that the latter will have mostly
/// constant time pushes, but may occasionally need to shift its contents around
/// to make room. They both have constant time pop, and they both have linear
/// time insert and remove.
///
/// The `RingBuffer` offers its own [`Slice`][Slice] and [`SliceMut`][SliceMut]
/// types to compensate for the loss of being able to take a slice, but they're
/// somewhat less efficient, so the general rule should be that you shouldn't
/// choose a `RingBuffer` if you rely heavily on slices - but if you don't,
/// it's probably a marginally better choice overall than [`Chunk`][Chunk].
///
/// # Feature Flag
///
/// To use this data structure, you need to enable the `ringbuffer` feature.
///
/// [Chunk]: ../sized_chunk/struct.Chunk.html
/// [Slice]: struct.Slice.html
/// [SliceMut]: struct.SliceMut.html
pub struct RingBuffer<A, N = U64>
where
    N: ChunkLength<A>,
{
    origin: RawIndex<N>,
    length: usize,
    data: MaybeUninit<N::SizedType>,
}

impl<A, N> HasLength for RingBuffer<A, N>
where
    N: ChunkLength<A>,
{
    /// Get the length of the ring buffer.
    #[inline]
    #[must_use]
    fn len(&self) -> usize {
        self.length
    }
}

impl<A, N> RingBuffer<A, N>
where
    N: ChunkLength<A>,
{
    /// The capacity of this ring buffer, as a `usize`.
    pub const CAPACITY: usize = N::USIZE;

    #[inline]
    unsafe fn ptr(&self, index: RawIndex<N>) -> *const A {
        debug_assert!(index.to_usize() < Self::CAPACITY);
        (&self.data as *const _ as *const A).add(index.to_usize())
    }

    #[inline]
    unsafe fn mut_ptr(&mut self, index: RawIndex<N>) -> *mut A {
        debug_assert!(index.to_usize() < Self::CAPACITY);
        (&mut self.data as *mut _ as *mut A).add(index.to_usize())
    }

    /// Write a value at a raw index without trying to drop what's already there
    #[inline]
    unsafe fn force_write(&mut self, index: RawIndex<N>, value: A) {
        core::ptr::write(self.mut_ptr(index), value)
    }

    /// Get an iterator over the raw indices of the buffer from left to right.
    #[inline]
    fn range(&self) -> IndexIter<N> {
        IndexIter {
            remaining: self.len(),
            left_index: self.origin,
            right_index: self.origin + self.len(),
        }
    }

    /// Construct an empty ring buffer.
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self {
            origin: 0.into(),
            length: 0,
            data: MaybeUninit::uninit(),
        }
    }
}

impl<A: Clone, N: ChunkLength<A>> Clone for RingBuffer<A, N> {
    fn clone(&self) -> Self {
        let mut out = Self::new();
        out.origin = self.origin;
        out.length = self.length;
        for index in out.range() {
            unsafe { out.force_write(index, (&*self.ptr(index)).clone()) };
        }
        out
    }
}
