#![doc= " General trait implementations for `BitVec`.\n\nThe operator traits are defined in the `ops` module.\n!"]

use super::*;
use crate::{
    order::BitOrder,
    store::BitStore,
};
use alloc::{
    borrow::{
        Borrow,
        BorrowMut,
    },
    boxed::Box,
    vec::Vec,
};
use core::{
    cmp::Ordering,
    fmt::{
        self,
        Binary,
        Debug,
        Display,
        Formatter,
        LowerHex,
        Octal,
        UpperHex,
    },
    hash::{
        Hash,
        Hasher,
    },
};

#[doc= " Signifies that `BitSlice` is the borrowed form of `BitVec`."]
impl<O, T> Borrow<BitSlice<O, T>> for BitVec<O, T>
where
    O: BitOrder,
    T: BitStore {
    #[doc= " Borrows the `BitVec` as a `BitSlice`."]
    #[doc= ""]
    #[doc= " # Parameters"]
    #[doc= ""]
    #[doc= " - `&self`"]
    #[doc= ""]
    #[doc= " # Returns"]
    #[doc= ""]
    #[doc= " A borrowed `BitSlice` of the vector."]
    #[doc= ""]
    #[doc= " # Examples"]
    #[doc= ""]
    #[doc= " ```rust"]
    #[doc= " use bitvec::prelude::*;"]
    #[doc= " use std::borrow::Borrow;"]
    #[doc= ""]
    #[doc= " let bv = bitvec![0; 13];"]
    #[doc= " let bs: &BitSlice = bv.borrow();"]
    #[doc= " assert!(!bs[10]);"]
    #[doc= " ```"]
    fn borrow(&self) -> &BitSlice<O, T> {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

#[doc= " Signifies that `BitSlice` is the borrowed form of `BitVec`."]
impl<O, T> BorrowMut<BitSlice<O, T>> for BitVec<O, T>
where
    O: BitOrder,
    T: BitStore {
    #[doc= " Mutably borrows the `BitVec` as a `BitSlice`."]
    #[doc= ""]
    #[doc= " # Parameters"]
    #[doc= ""]
    #[doc= " - `&mut self`"]
    #[doc= ""]
    #[doc= " # Returns"]
    #[doc= ""]
    #[doc= " A mutably borrowed `BitSlice` of the vector."]
    #[doc= ""]
    #[doc= " # Examples"]
    #[doc= ""]
    #[doc= " ```rust"]
    #[doc= " use bitvec::prelude::*;"]
    #[doc= " use std::borrow::BorrowMut;"]
    #[doc= ""]
    #[doc= " let mut bv = bitvec![0; 13];"]
    #[doc= " let bs: &mut BitSlice = bv.borrow_mut();"]
    #[doc= " assert!(!bs[10]);"]
    #[doc= " bs.set(10, true);"]
    #[doc= " assert!(bs[10]);"]
    #[doc= " ```"]
    fn borrow_mut(&mut self) -> &mut BitSlice<O, T> {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<O, T> Clone for BitVec<O, T>
where
    O: BitOrder,
    T: BitStore {
    fn clone(&self) -> Self {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    fn clone_from(&mut self, other: &Self) {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<O, T> Eq for BitVec<O, T>
where
    O: BitOrder,
    T: BitStore { }

impl<O, T> Ord for BitVec<O, T>
where
    O: BitOrder,
    T: BitStore {
    fn cmp(&self, rhs: &Self) -> Ordering {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

#[doc= " Tests if two `BitVec`s are semantically — not bitwise — equal.\n\nIt is valid to compare two vectors of different order or element types.\n\nThe equality condition requires that they have the same number of stored bits\nand that each pair of bits in semantic order are identical.\n*"]
impl<A, B, C, D> PartialEq<BitVec<C, D>> for BitVec<A, B>
where
    A: BitOrder,
    B: BitStore,
    C: BitOrder,
    D: BitStore {
    #[doc= " Performs a comparison by `==`."]
    #[doc= ""]
    #[doc= " # Parameters"]
    #[doc= ""]
    #[doc= " - `&self`"]
    #[doc= " - `rhs`: The other vector to compare."]
    #[doc= ""]
    #[doc= " # Returns"]
    #[doc= ""]
    #[doc= " Whether the vectors compare equal."]
    #[doc= ""]
    #[doc= " # Examples"]
    #[doc= ""]
    #[doc= " ```rust"]
    #[doc= " use bitvec::prelude::*;"]
    #[doc= ""]
    #[doc= " let l: BitVec<Lsb0, u16> = bitvec![Lsb0, u16; 0, 1, 0, 1];"]
    #[doc= " let r: BitVec<Msb0, u32> = bitvec![Msb0, u32; 0, 1, 0, 1];"]
    #[doc= " assert!(l == r);"]
    #[doc= " ```"]
    #[doc= ""]
    #[doc= " This example uses the same types to prove that raw, bitwise, values are"]
    #[doc= " not used for equality comparison."]
    #[doc= ""]
    #[doc= " ```rust"]
    #[doc= " use bitvec::prelude::*;"]
    #[doc= ""]
    #[doc= " let l: BitVec<Msb0, u8> = bitvec![Msb0, u8; 0, 1, 0, 1];"]
    #[doc= " let r: BitVec<Lsb0, u8> = bitvec![Lsb0, u8; 0, 1, 0, 1];"]
    #[doc= ""]
    #[doc= " assert_eq!(l, r);"]
    #[doc= " assert_ne!(l.as_slice(), r.as_slice());"]
    #[doc= " ```"]
    fn eq(&self, rhs: &BitVec<C, D>) -> bool {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<A, B, C, D> PartialEq<BitSlice<C, D>> for BitVec<A, B>
where
    A: BitOrder,
    B: BitStore,
    C: BitOrder,
    D: BitStore {
    fn eq(&self, rhs: &BitSlice<C, D>) -> bool {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<A, B, C, D> PartialEq<BitVec<C, D>> for BitSlice<A, B>
where
    A: BitOrder,
    B: BitStore,
    C: BitOrder,
    D: BitStore {
    fn eq(&self, rhs: &BitVec<C, D>) -> bool {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<A, B, C, D> PartialEq<&BitSlice<C, D>> for BitVec<A, B>
where
    A: BitOrder,
    B: BitStore,
    C: BitOrder,
    D: BitStore {
    fn eq(&self, rhs: &&BitSlice<C, D>) -> bool {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<A, B, C, D> PartialEq<BitVec<C, D>> for &BitSlice<A, B>
where
    A: BitOrder,
    B: BitStore,
    C: BitOrder,
    D: BitStore {
    fn eq(&self, rhs: &BitVec<C, D>) -> bool {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

#[doc= " Compares two `BitVec`s by semantic — not bitwise — ordering.\n\nThe comparison sorts by testing each index for one vector to have a set bit\nwhere the other vector has an unset bit. If the vectors are different, the\nvector with the set bit sorts greater than the vector with the unset bit.\n\nIf one of the vectors is exhausted before they differ, the longer vector is\ngreater.\n*"]
impl<A, B, C, D> PartialOrd<BitVec<C, D>> for BitVec<A, B>
where
    A: BitOrder,
    B: BitStore,
    C: BitOrder,
    D: BitStore {
    #[doc= " Performs a comparison by `<` or `>`."]
    #[doc= ""]
    #[doc= " # Parameters"]
    #[doc= ""]
    #[doc= " - `&self`"]
    #[doc= " - `rhs`: The other vector to compare."]
    #[doc= ""]
    #[doc= " # Returns"]
    #[doc= ""]
    #[doc= " The relative ordering of the two vectors."]
    #[doc= ""]
    #[doc= " # Examples"]
    #[doc= ""]
    #[doc= " ```rust"]
    #[doc= " use bitvec::prelude::*;"]
    #[doc= ""]
    #[doc= " let a = bitvec![0, 1, 0, 0];"]
    #[doc= " let b = bitvec![0, 1, 0, 1];"]
    #[doc= " let c = bitvec![0, 1, 0, 1, 1];"]
    #[doc= " assert!(a < b);"]
    #[doc= " assert!(b < c);"]
    #[doc= " ```"]
    fn partial_cmp(&self, rhs: &BitVec<C, D>) -> Option<Ordering> {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<A, B, C, D> PartialOrd<BitSlice<C, D>> for BitVec<A, B>
where
    A: BitOrder,
    B: BitStore,
    C: BitOrder,
    D: BitStore {
    fn partial_cmp(&self, rhs: &BitSlice<C, D>) -> Option<Ordering> {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<A, B, C, D> PartialOrd<BitVec<C, D>> for BitSlice<A, B>
where
    A: BitOrder,
    B: BitStore,
    C: BitOrder,
    D: BitStore {
    fn partial_cmp(&self, rhs: &BitVec<C, D>) -> Option<Ordering> {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<A, B, C, D> PartialOrd<&BitSlice<C, D>> for BitVec<A, B>
where
    A: BitOrder,
    B: BitStore,
    C: BitOrder,
    D: BitStore {
    fn partial_cmp(&self, rhs: &&BitSlice<C, D>) -> Option<Ordering> {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<A, B, C, D> PartialOrd<BitVec<C, D>> for &BitSlice<A, B>
where
    A: BitOrder,
    B: BitStore,
    C: BitOrder,
    D: BitStore {
    fn partial_cmp(&self, rhs: &BitVec<C, D>) -> Option<Ordering> {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<O, T> AsMut<BitSlice<O, T>> for BitVec<O, T>
where
    O: BitOrder,
    T: BitStore {
    fn as_mut(&mut self) -> &mut BitSlice<O, T> {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<O, T> AsRef<BitSlice<O, T>> for BitVec<O, T>
where
    O: BitOrder,
    T: BitStore {
    fn as_ref(&self) -> &BitSlice<O, T> {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<O, T> From<&BitSlice<O, T>> for BitVec<O, T>
where
    O: BitOrder,
    T: BitStore {
    fn from(src: &BitSlice<O, T>) -> Self {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

#[doc= " Builds a `BitVec` out of a slice of `bool`.\n\nThis is primarily for the `bitvec!` macro; it is not recommended for general\nuse.\n*"]
impl<O, T> From<&[bool]> for BitVec<O, T>
where
    O: BitOrder,
    T: BitStore {
    fn from(src: &[bool]) -> Self {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<O, T> From<BitBox<O, T>> for BitVec<O, T>
where
    O: BitOrder,
    T: BitStore {
    fn from(src: BitBox<O, T>) -> Self {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<O, T> From<&[T]> for BitVec<O, T>
where
    O: BitOrder,
    T: BitStore {
    fn from(src: &[T]) -> Self {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<O, T> From<Box<[T]>> for BitVec<O, T>
where
    O: BitOrder,
    T: BitStore {
    fn from(src: Box<[T]>) -> Self {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<O, T> Into<Box<[T]>> for BitVec<O, T>
where
    O: BitOrder,
    T: BitStore {
    fn into(self) -> Box<[T]> {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

#[doc= " Builds a `BitVec` out of a `Vec` of elements.\n\nThis moves the memory as-is from the source buffer into the new `BitVec`. The\nsource buffer will be unchanged by this operation, so you don't need to worry\nabout using the correct order type.\n*"]
impl<O, T> From<Vec<T>> for BitVec<O, T>
where
    O: BitOrder,
    T: BitStore {
    fn from(src: Vec<T>) -> Self {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<O, T> Into<Vec<T>> for BitVec<O, T>
where
    O: BitOrder,
    T: BitStore {
    fn into(self) -> Vec<T> {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<O, T> Default for BitVec<O, T>
where
    O: BitOrder,
    T: BitStore {
    fn default() -> Self {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<O, T> Binary for BitVec<O, T>
where
    O: BitOrder,
    T: BitStore {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

#[doc= " Prints the `BitVec` for debugging.\n\nThe output is of the form `BitVec<O, T> [ELT, *]`, where `<O, T>` is the order\nand element type, with square brackets on each end of the bits and all the live\nelements in the vector printed in binary. The printout is always in semantic\norder, and may not reflect the underlying store. To see the underlying store,\nuse `format!(\"{:?}\", self.as_slice());` instead.\n\nThe alternate character `{:#?}` prints each element on its own line, rather than\nseparated by a space.\n*"]
impl<O, T> Debug for BitVec<O, T>
where
    O: BitOrder,
    T: BitStore {
    #[doc= " Renders the `BitVec` type header and contents for debug."]
    #[doc= ""]
    #[doc= " # Examples"]
    #[doc= ""]
    #[doc= " ```rust"]
    #[doc= " use bitvec::prelude::*;"]
    #[doc= ""]
    #[doc= " let bv = bitvec![Lsb0, u16;"]
    #[doc= "   0, 1, 0, 1, 0, 0, 0, 0, 1, 1, 1, 1, 0, 1, 0, 1"]
    #[doc= " ];"]
    #[doc= " assert_eq!("]
    #[doc= "   \"BitVec<Lsb0, u16> [0101000011110101]\","]
    #[doc= "   &format!(\"{:?}\", bv)"]
    #[doc= " );"]
    #[doc= " ```"]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

#[doc= " Prints the `BitVec` for displaying.\n\nThis prints each element in turn, formatted in binary in semantic order (so the\nfirst bit seen is printed first and the last bit seen printed last). Each\nelement of storage is separated by a space for ease of reading.\n\nThe alternate character `{:#}` prints each element on its own line.\n\nTo see the in-memory representation, use `AsRef` to get access to the raw\nelements and print that slice instead.\n*"]
impl<O, T> Display for BitVec<O, T>
where
    O: BitOrder,
    T: BitStore {
    #[doc= " Renders the `BitVec` contents for display."]
    #[doc= ""]
    #[doc= " # Examples"]
    #[doc= ""]
    #[doc= " ```rust"]
    #[doc= " use bitvec::prelude::*;"]
    #[doc= ""]
    #[doc= " let bv = bitvec![Msb0, u8; 0, 1, 0, 0, 1, 0, 1, 1, 0, 1];"]
    #[doc= " assert_eq!(\"[01001011, 01]\", &format!(\"{}\", bv));"]
    #[doc= " ```"]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<O, T> LowerHex for BitVec<O, T>
where
    O: BitOrder,
    T: BitStore {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<O, T> Octal for BitVec<O, T>
where
    O: BitOrder,
    T: BitStore {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<O, T> UpperHex for BitVec<O, T>
where
    O: BitOrder,
    T: BitStore {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

#[doc= " Writes the contents of the `BitVec`, in semantic bit order, into a hasher."]
impl<O, T> Hash for BitVec<O, T>
where
    O: BitOrder,
    T: BitStore {
    #[doc= " Writes each bit of the `BitVec`, as a full `bool`, into the hasher."]
    #[doc= ""]
    #[doc= " # Parameters"]
    #[doc= ""]
    #[doc= " - `&self`"]
    #[doc= " - `hasher`: The hashing pool into which the vector is written."]
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

#[doc= " `BitVec` is safe to move across thread boundaries, as is `&mut BitVec`."]
unsafe impl<O, T> Send for BitVec<O, T>
where
    O: BitOrder,
    T: BitStore { }

#[doc= " `&BitVec` is safe to move across thread boundaries."]
unsafe impl<O, T> Sync for BitVec<O, T>
where
    O: BitOrder,
    T: BitStore { }
