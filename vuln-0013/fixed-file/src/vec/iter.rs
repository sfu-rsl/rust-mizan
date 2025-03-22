#![doc= " Iteration processes for `BitVec`."]

use super::*;
use crate::{
    order::BitOrder,
    store::BitStore,
};
use core::iter::{
    FromIterator,
    FusedIterator,
};

#[doc= " Extends a `BitVec` with the contents of another bitstream.\n\nAt present, this just calls `.push()` in a loop. When specialization becomes\navailable, it will be able to more intelligently perform bulk moves from the\nsource into `self` when the source is `BitSlice`-compatible.\n*"]
impl<O, T> Extend<bool> for BitVec<O, T>
where
    O: BitOrder,
    T: BitStore {
    #[doc= " Extends a `BitVec` from another bitstream."]
    #[doc= ""]
    #[doc= " # Parameters"]
    #[doc= ""]
    #[doc= " - `&mut self`"]
    #[doc= " - `src`: A source bitstream."]
    #[doc= ""]
    #[doc= " # Type Parameters"]
    #[doc= ""]
    #[doc= " - `I: IntoIterator<Item=bool>`: The source bitstream with which to"]
    #[doc= "   extend `self`."]
    #[doc= ""]
    #[doc= " # Examples"]
    #[doc= ""]
    #[doc= " ```rust"]
    #[doc= " use bitvec::prelude::*;"]
    #[doc= ""]
    #[doc= " let mut bv = bitvec![Msb0, u8; 0; 4];"]
    #[doc= " bv.extend(bitvec![1; 4]);"]
    #[doc= " assert_eq!(0x0F, bv.as_slice()[0]);"]
    #[doc= " ```"]
    fn extend<I: IntoIterator<Item = bool>>(&mut self, src: I) {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

#[doc= " Permits the construction of a `BitVec` by using `.collect()` on an iterator"]
#[doc= " of `bool`."]
impl<O, T> FromIterator<bool> for BitVec<O, T>
where
    O: BitOrder,
    T: BitStore {
    #[doc= " Collects an iterator of `bool` into a vector."]
    #[doc= ""]
    #[doc= " # Examples"]
    #[doc= ""]
    #[doc= " ```rust"]
    #[doc= " use bitvec::prelude::*;"]
    #[doc= ""]
    #[doc= " use std::iter::repeat;"]
    #[doc= " let bv: BitVec<Msb0, u8> ="]
    #[doc= "     repeat(true).take(4).chain(repeat(false).take(4)).collect();"]
    #[doc= " assert_eq!(bv.as_slice()[0], 0xF0);"]
    #[doc= " ```"]
    fn from_iter<I: IntoIterator<Item = bool>>(src: I) -> Self {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

#[doc= " Produces an iterator over all the bits in the vector.\n\nThis iterator follows the ordering in the vector type, and implements\n`ExactSizeIterator`, since `BitVec`s always know exactly how large they are, and\n`DoubleEndedIterator`, since they have known ends.\n*"]
impl<O, T> IntoIterator for BitVec<O, T>
where
    O: BitOrder,
    T: BitStore {
    type IntoIter = IntoIter<O, T>;
    type Item = bool;

    #[doc= " Iterates over the vector."]
    #[doc= ""]
    #[doc= " # Examples"]
    #[doc= ""]
    #[doc= " ```rust"]
    #[doc= " use bitvec::prelude::*;"]
    #[doc= ""]
    #[doc= " let bv = bitvec![Msb0, u8; 1, 1, 1, 1, 0, 0, 0, 0];"]
    #[doc= " let mut count = 0;"]
    #[doc= " for bit in bv {"]
    #[doc= "   if bit { count += 1; }"]
    #[doc= " }"]
    #[doc= " assert_eq!(count, 4);"]
    #[doc= " ```"]
    fn into_iter(self) -> Self::IntoIter {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<'a, O, T> IntoIterator for &'a BitVec<O, T>
where
    O: BitOrder,
    T: 'a + BitStore {
    type IntoIter = <&'a BitSlice<O, T> as IntoIterator>::IntoIter;
    type Item = &'a bool;

    fn into_iter(self) -> Self::IntoIter {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

#[doc= " State keeper for draining iteration.\n\n# Type Parameters\n\n- `O: BitOrder`: The ordering type of the underlying vector.\n- `T: 'a + BitStore`: The storage type of the underlying vector.\n\n# Lifetimes\n\n- `'a`: The lifetime of the underlying vector.\n*"]
pub(crate) struct Drain<'a, O, T>
where
    O: BitOrder,
    T: 'a + BitStore {
    #[doc= " Current remaining range to remove."]
    pub(super) iter: crate::slice::iter::Iter<'a, O, T>,
}

impl<'a, O, T> Drain<'a, O, T>
where
    O: BitOrder,
    T: 'a + BitStore {
    #[doc= " Fills the drain span with another iterator."]
    #[doc= ""]
    #[doc= " If the stream exhausts before the drain is filled, then the tail"]
    #[doc= " elements move downwards; otherwise, the tail stays put and the drain is"]
    #[doc= " filled."]
    #[doc= ""]
    #[doc= " # Parameters"]
    #[doc= ""]
    #[doc= " - `&mut self`"]
    #[doc= " - `stream`: The source of bits to fill into the drain."]
    #[doc= ""]
    #[doc= " # Returns"]
    #[doc= ""]
    #[doc= " - `true` if the drain was filled before the `stream` exhausted."]
    #[doc= " - `false` if the `stream` exhausted early, and the tail was moved down."]
    #[doc= ""]
    #[doc= " # Type Parameters"]
    #[doc= ""]
    #[doc= " - `I: Iterator<Item=bool>`: A provider of bits."]
    unsafe fn fill<I: Iterator<Item = bool>>(&mut self, stream: &mut I) -> bool {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    #[doc= " Moves the tail span farther back in the vector."]
    #[doc= ""]
    #[doc= " # Parameters"]
    #[doc= ""]
    #[doc= " - `&mut self`"]
    #[doc= " - `by`: The amount by which to move the tail span."]
    unsafe fn move_tail(&mut self, by: usize) {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<'a, O, T> DoubleEndedIterator for Drain<'a, O, T>
where
    O: BitOrder,
    T: 'a + BitStore {
    fn next_back(&mut self) -> Option<Self::Item> {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<'a, O, T> ExactSizeIterator for Drain<'a, O, T>
where
    O: BitOrder,
    T: 'a + BitStore { }

impl<'a, O, T> FusedIterator for Drain<'a, O, T>
where
    O: BitOrder,
    T: 'a + BitStore { }

impl<'a, O, T> Iterator for Drain<'a, O, T>
where
    O: BitOrder,
    T: 'a + BitStore {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    fn count(self) -> usize {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    fn last(mut self) -> Option<Self::Item> {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<'a, O, T> Drop for Drain<'a, O, T>
where
    O: BitOrder,
    T: 'a + BitStore {
    fn drop(&mut self) {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

#[doc= " A consuming iterator for `BitVec`."]
#[repr(C)]
pub struct IntoIter<O, T>
where
    O: BitOrder,
    T: BitStore {
    #[doc= " Owning descriptor for the allocation. This is not modified by"]
    #[doc= " iteration."]
    pub(super) bitvec: BitVec<O, T>,
}

impl<O, T> IntoIter<O, T>
where
    O: BitOrder,
    T: BitStore {
    fn iterator(&self) -> <&BitSlice<O, T> as IntoIterator>::IntoIter {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<O, T> DoubleEndedIterator for IntoIter<O, T>
where
    O: BitOrder,
    T: BitStore {
    fn next_back(&mut self) -> Option<Self::Item> {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<O, T> ExactSizeIterator for IntoIter<O, T>
where
    O: BitOrder,
    T: BitStore { }

impl<O, T> FusedIterator for IntoIter<O, T>
where
    O: BitOrder,
    T: BitStore { }

impl<O, T> Iterator for IntoIter<O, T>
where
    O: BitOrder,
    T: BitStore {
    type Item = bool;

    #[doc= " Advances the iterator by one, returning the first bit in it (if any)."]
    #[doc= ""]
    #[doc= " # Parameters"]
    #[doc= ""]
    #[doc= " - `&mut self`"]
    #[doc= ""]
    #[doc= " # Returns"]
    #[doc= ""]
    #[doc= " The leading bit in the iterator, if any."]
    #[doc= ""]
    #[doc= " # Examples"]
    #[doc= ""]
    #[doc= " ```rust"]
    #[doc= " use bitvec::prelude::*;"]
    #[doc= ""]
    #[doc= " let bv = bitvec![1, 0];"]
    #[doc= " let mut iter = bv.iter();"]
    #[doc= " assert!(iter.next().unwrap());"]
    #[doc= " assert!(!iter.next().unwrap());"]
    #[doc= " assert!(iter.next().is_none());"]
    #[doc= " ```"]
    fn next(&mut self) -> Option<Self::Item> {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    #[doc= " Hints at the number of bits remaining in the iterator."]
    #[doc= ""]
    #[doc= " Because the exact size is always known, this always produces"]
    #[doc= " `(len, Some(len))`."]
    #[doc= ""]
    #[doc= " # Parameters"]
    #[doc= ""]
    #[doc= " - `&self`"]
    #[doc= ""]
    #[doc= " # Returns"]
    #[doc= ""]
    #[doc= " - `usize`: The minimum bits remaining."]
    #[doc= " - `Option<usize>`: The maximum bits remaining."]
    #[doc= ""]
    #[doc= " # Examples"]
    #[doc= ""]
    #[doc= " ```rust"]
    #[doc= " use bitvec::prelude::*;"]
    #[doc= ""]
    #[doc= " let bv = bitvec![0, 1];"]
    #[doc= " let mut iter = bv.iter();"]
    #[doc= " assert_eq!(iter.size_hint(), (2, Some(2)));"]
    #[doc= " iter.next();"]
    #[doc= " assert_eq!(iter.size_hint(), (1, Some(1)));"]
    #[doc= " iter.next();"]
    #[doc= " assert_eq!(iter.size_hint(), (0, Some(0)));"]
    #[doc= " ```"]
    fn size_hint(&self) -> (usize, Option<usize>) {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    #[doc= " Counts how many bits are live in the iterator, consuming it."]
    #[doc= ""]
    #[doc= " You are probably looking to use this on a borrowed iterator rather than"]
    #[doc= " an owning iterator. See [`BitSlice`]."]
    #[doc= ""]
    #[doc= " # Parameters"]
    #[doc= ""]
    #[doc= " - `self`"]
    #[doc= ""]
    #[doc= " # Returns"]
    #[doc= ""]
    #[doc= " The number of bits in the iterator."]
    #[doc= ""]
    #[doc= " # Examples"]
    #[doc= ""]
    #[doc= " ```rust"]
    #[doc= " use bitvec::prelude::*;"]
    #[doc= " let bv = bitvec![Msb0, u8; 0, 1, 0, 1, 0];"]
    #[doc= " assert_eq!(bv.into_iter().count(), 5);"]
    #[doc= " ```"]
    #[doc= ""]
    #[doc= " [`BitSlice`]: ../struct.BitSlice.html#method.iter"]
    fn count(self) -> usize {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    #[doc= " Advances the iterator by `n` bits, starting from zero."]
    #[doc= ""]
    #[doc= " # Parameters"]
    #[doc= ""]
    #[doc= " - `&mut self`"]
    #[doc= " - `n`: The number of bits to skip, before producing the next bit after"]
    #[doc= "   skips. If this overshoots the iterator’s remaining length, then the"]
    #[doc= "   iterator is marked empty before returning `None`."]
    #[doc= ""]
    #[doc= " # Returns"]
    #[doc= ""]
    #[doc= " If `n` does not overshoot the iterator’s bounds, this produces the `n`th"]
    #[doc= " bit after advancing the iterator to it, discarding the intermediate"]
    #[doc= " bits."]
    #[doc= ""]
    #[doc= " If `n` does overshoot the iterator’s bounds, this empties the iterator"]
    #[doc= " and returns `None`."]
    #[doc= ""]
    #[doc= " # Examples"]
    #[doc= ""]
    #[doc= " ```rust"]
    #[doc= " use bitvec::prelude::*;"]
    #[doc= " let bv = bitvec![Msb0, u8; 0, 0, 0, 1];"]
    #[doc= " let mut iter = bv.into_iter();"]
    #[doc= " assert_eq!(iter.len(), 4);"]
    #[doc= " assert!(iter.nth(3).unwrap());"]
    #[doc= " assert!(iter.nth(0).is_none());"]
    #[doc= " ```"]
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    #[doc= " Consumes the iterator, returning only the last bit."]
    #[doc= ""]
    #[doc= " # Examples"]
    #[doc= ""]
    #[doc= " ```rust"]
    #[doc= " use bitvec::prelude::*;"]
    #[doc= " let bv = bitvec![Msb0, u8; 0, 0, 0, 1];"]
    #[doc= " assert!(bv.into_iter().last().unwrap());"]
    #[doc= " ```"]
    #[doc= ""]
    #[doc= " Empty iterators return `None`"]
    #[doc= ""]
    #[doc= " ```rust"]
    #[doc= " use bitvec::prelude::*;"]
    #[doc= " assert!(bitvec![].into_iter().last().is_none());"]
    #[doc= " ```"]
    fn last(mut self) -> Option<Self::Item> {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

#[doc= " A splicing iterator for `BitVec`.\n\nThis removes a segment from the vector and inserts another bitstream into its\nspot. Any bits from the original `BitVec` after the removed segment are kept,\nafter the inserted bitstream.\n\nOnly the removed segment is available for iteration.\n\n# Type Parameters\n\n- `I: Iterator<Item=bool>`: Any bitstream. This will be used to fill the\n  removed span.\n*"]
pub(crate) struct Splice<'a, O, T, I>
where
    O: BitOrder,
    T: 'a + BitStore,
    I: Iterator<Item = bool> {
    pub(super) drain: Drain<'a, O, T>,
    pub(super) splice: I,
}

impl<'a, O, T, I> DoubleEndedIterator for Splice<'a, O, T, I>
where
    O: BitOrder,
    T: 'a + BitStore,
    I: Iterator<Item = bool> {
    fn next_back(&mut self) -> Option<Self::Item> {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<'a, O, T, I> ExactSizeIterator for Splice<'a, O, T, I>
where
    O: BitOrder,
    T: 'a + BitStore,
    I: Iterator<Item = bool> { }

impl<'a, O, T, I> FusedIterator for Splice<'a, O, T, I>
where
    O: BitOrder,
    T: 'a + BitStore,
    I: Iterator<Item = bool> { }

impl<'a, O, T, I> Iterator for Splice<'a, O, T, I>
where
    O: BitOrder,
    T: 'a + BitStore,
    I: Iterator<Item = bool> {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    fn count(self) -> usize {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    fn last(mut self) -> Option<Self::Item> {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<'a, O, T, I> Drop for Splice<'a, O, T, I>
where
    O: BitOrder,
    T: 'a + BitStore,
    I: Iterator<Item = bool> {
    fn drop(&mut self) {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}
