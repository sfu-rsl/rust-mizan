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

    /// Get the raw index for a logical index.
    #[inline]
    fn raw(&self, index: usize) -> RawIndex<N> {
        self.origin + index
    }

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

    /// Copy the value at a raw index, discarding ownership of the copied value
    #[inline]
    unsafe fn force_read(&self, index: RawIndex<N>) -> A {
        core::ptr::read(self.ptr(index))
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

    /// Test if the ring buffer is full.
    #[inline]
    #[must_use]
    pub fn is_full(&self) -> bool {
        self.len() == Self::CAPACITY
    }

    /// Push a value to the back of the buffer.
    ///
    /// Panics if the capacity of the buffer is exceeded.
    ///
    /// Time: O(1)
    pub fn push_back(&mut self, value: A) {
        if self.is_full() {
            panic!("RingBuffer::push_back: can't push to a full buffer")
        } else {
            unsafe { self.force_write(self.raw(self.length), value) }
            self.length += 1;
        }
    }

    /// Insert multiple values at index `index`, shifting all the following values
    /// to the right.
    ///
    /// Panics if the index is out of bounds or the chunk doesn't have room for
    /// all the values.
    ///
    /// Time: O(m+n) where m is the number of elements inserted and n is the number
    /// of elements following the insertion index. Calling `insert`
    /// repeatedly would be O(m*n).
    pub fn insert_from<Iterable, I>(&mut self, index: usize, iter: Iterable)
    where
        Iterable: IntoIterator<Item = A, IntoIter = I>,
        I: ExactSizeIterator<Item = A>,
    {
        let iter = iter.into_iter();
        let insert_size = iter.len();
        if self.len() + insert_size > Self::CAPACITY {
            panic!(
                "Chunk::insert_from: chunk cannot fit {} elements",
                insert_size
            );
        }
        if index > self.len() {
            panic!("Chunk::insert_from: index out of bounds");
        }
        if index == self.len() {
            self.extend(iter);
            return;
        }
        let right_count = self.len() - index;
        // Check which side has fewer elements to shift.
        if right_count < index {
            // Shift to the right.
            let mut i = self.raw(self.len() - 1);
            let target = self.raw(index);
            while i != target {
                unsafe { self.force_write(i + insert_size, self.force_read(i)) };
                i -= 1;
            }
            unsafe { self.force_write(target + insert_size, self.force_read(target)) };
            self.length += insert_size;
        } else {
            // Shift to the left.
            self.origin -= insert_size;
            self.length += insert_size;
            for i in self.range().take(index) {
                unsafe { self.force_write(i, self.force_read(i + insert_size)) };
            }
        }
        let mut index = self.raw(index);
        for value in iter {
            unsafe { self.force_write(index, value) };
            index += 1;
        }
    }
}

impl<A, N: ChunkLength<A>> Extend<A> for RingBuffer<A, N> {
    #[inline]
    fn extend<I: IntoIterator<Item = A>>(&mut self, iter: I) {
        for item in iter {
            self.push_back(item);
        }
    }
}
