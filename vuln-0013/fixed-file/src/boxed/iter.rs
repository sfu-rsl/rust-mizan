#![doc= " Iteration processes for `BitBox`."]

use crate::{
    boxed::BitBox,
    order::BitOrder,
    slice::BitSlice,
    store::BitStore,
};
use core::iter::FusedIterator;

impl<O, T> IntoIterator for BitBox<O, T>
where
    O: BitOrder,
    T: BitStore {
    type IntoIter = IntoIter<O, T>;
    type Item = bool;

    fn into_iter(self) -> Self::IntoIter {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<'a, O, T> IntoIterator for &'a BitBox<O, T>
where
    O: BitOrder,
    T: 'a + BitStore {
    type IntoIter = <&'a BitSlice<O, T> as IntoIterator>::IntoIter;
    type Item = <Self::IntoIter as Iterator>::Item;

    fn into_iter(self) -> Self::IntoIter {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<'a, O, T> IntoIterator for &'a mut BitBox<O, T>
where
    O: BitOrder,
    T: 'a + BitStore {
    type IntoIter = <&'a mut BitSlice<O, T> as IntoIterator>::IntoIter;
    type Item = <Self::IntoIter as Iterator>::Item;

    fn into_iter(self) -> Self::IntoIter {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

#[doc= " State keeper for consuming iteration over a `BitBox`."]
#[repr(C)]
pub struct IntoIter<O, T>
where
    O: BitOrder,
    T: BitStore {
    #[doc= " Owning pointer to the full slab"]
    bitbox: BitBox<O, T>,
}

impl<O, T> IntoIter<O, T>
where
    O: BitOrder,
    T: BitStore { }

impl<O, T> Iterator for IntoIter<O, T>
where
    O: BitOrder,
    T: BitStore {
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
