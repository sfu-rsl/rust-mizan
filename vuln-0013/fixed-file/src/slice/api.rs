#![doc= " Reïmplementation of the `[T]` API.\n\nThis module tracks the [`slice`] primitive and [`core::slice`] module in the\nversion of Rust specified in the `rust-toolchain` file. It is required to\nprovide an exact or equivalent API surface matching the `Box<[T]>` type, to the\nextent that it is possible in the language. Where differences occur, they must\nbe documented in a section called `API Differences`.\n\n[`core::slice`]: https://doc.rust-lang.org/core/slice\n[`slice`]: https://doc.rust-lang.org/std/primitive.slice.html\n!"]

use crate::{
    index::BitIdx,
    order::BitOrder,
    slice::{
        iter::{
            Chunks,
            ChunksExact,
            ChunksExactMut,
            ChunksMut,
            Iter,
            IterMut,
            RChunks,
            RChunksExact,
            RChunksExactMut,
            RChunksMut,
            RSplit,
            RSplitMut,
            RSplitN,
            RSplitNMut,
            Split,
            SplitMut,
            SplitN,
            SplitNMut,
            Windows,
        },
        proxy::BitMut,
        BitSlice,
    },
    store::BitStore,
};
use core::{
    ops::{
        Range,
        RangeFrom,
        RangeFull,
        RangeTo,
    },
};
#[cfg(feature = "alloc")]
use crate::vec::BitVec;

#[doc= " Reimplementation of the `[T]` inherent-method API."]
impl<O, T> BitSlice<O, T>
where
    O: BitOrder,
    T: BitStore {
    #[doc= " Returns the number of bits in the slice."]
    #[doc= ""]
    #[doc= " # Original"]
    #[doc= ""]
    #[doc= " [`slice::len`](https://doc.rust-lang.org/std/primitive.slice.html#method.len)"]
    #[doc= ""]
    #[doc= " # Examples"]
    #[doc= ""]
    #[doc= " ```rust"]
    #[doc= " # use bitvec::prelude::*;"]
    #[doc= " let bits = 0u8.bits::<Local>();"]
    #[doc= " assert_eq!(bits.len(), 8);"]
    #[doc= " ```"]
    pub(crate) fn len(&self) -> usize {
        self.bitptr().len()
    }

    #[doc= " Returns `true` if the slice has a length of 0."]
    #[doc= ""]
    #[doc= " # Original"]
    #[doc= ""]
    #[doc= " [`slice::is_empty`](https://doc.rust-lang.org/std/primitive.slice.html#method.is_empty)"]
    #[doc= ""]
    #[doc= " # Examples"]
    #[doc= ""]
    #[doc= " ```rust"]
    #[doc= " # use bitvec::prelude::*;"]
    #[doc= " let bits = 0u8.bits::<Local>();"]
    #[doc= " assert!(!bits.is_empty());"]
    #[doc= ""]
    #[doc= " assert!(BitSlice::<Local, usize>::empty().is_empty())"]
    #[doc= " ```"]
    pub(crate) fn is_empty(&self) -> bool {
        self.bitptr().len() == 0
    }

    #[doc= " Returns an iterator over `chunk_size` bits of the slice at a time,"]
    #[doc= " starting at the beginning of the slice."]
    #[doc= ""]
    #[doc= " The chunks are slices and do not overlap. If `chunk_size` does not"]
    #[doc= " divide the length of the slice, then the last chunk will not have length"]
    #[doc= " `chunk_size`."]
    #[doc= ""]
    #[doc= " See [`chunks_exact`] for a variant of this iterator that returns chunks"]
    #[doc= " of always exactly `chunk_size` elements, and [`rchunks`] for the same"]
    #[doc= " iterator but starting at the end of the slice."]
    #[doc= ""]
    #[doc= " # Panics"]
    #[doc= ""]
    #[doc= " Panics if `chunk_size` is 0."]
    #[doc= ""]
    #[doc= " # Examples"]
    #[doc= ""]
    #[doc= " ```rust"]
    #[doc= " # use bitvec::prelude::*;"]
    #[doc= " let data = 0b001_010_10u8;"]
    #[doc= " let bits = data.bits::<Msb0>();"]
    #[doc= " let mut iter = bits.chunks(3);"]
    #[doc= " assert_eq!(iter.next().unwrap(), &bits[0 .. 3]);"]
    #[doc= " assert_eq!(iter.next().unwrap(), &bits[3 .. 6]);"]
    #[doc= " assert_eq!(iter.next().unwrap(), &bits[6 .. 8]);"]
    #[doc= " assert!(iter.next().is_none());"]
    #[doc= " ```"]
    #[doc= ""]
    #[doc= " [`chunks_exact`]: #method.chunks_exact"]
    #[doc= " [`rchunks`]: #method.rchunks"]
    pub(crate) fn chunks(&self, chunk_size: usize) -> Chunks<O, T> {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}
