#![doc= " Operator trait implementations."]

use super::*;
use crate::{
    order::BitOrder,
    store::BitStore,
};
use core::ops::{
    Deref,
    DerefMut,
    Shl,
    ShlAssign,
    Shr,
    ShrAssign,
};

#[doc= " Reborrows the `BitVec` as a `BitSlice`.\n\nThis mimics the separation between `Vec<T>` and `[T]`.\n*"]
impl<O, T> Deref for BitVec<O, T>
where
    O: BitOrder,
    T: BitStore {
    type Target = BitSlice<O, T>;

    #[doc= " Dereferences `&BitVec` down to `&BitSlice`."]
    #[doc= ""]
    #[doc= " # Examples"]
    #[doc= ""]
    #[doc= " ```rust"]
    #[doc= " use bitvec::prelude::*;"]
    #[doc= ""]
    #[doc= " let bv: BitVec = bitvec![1; 4];"]
    #[doc= " let bref: &BitSlice = &bv;"]
    #[doc= " assert!(bref[2]);"]
    #[doc= " ```"]
    fn deref(&self) -> &Self::Target {
        self.as_bitslice()
    }
}

#[doc= " Mutably reborrows the `BitVec` as a `BitSlice`.\n\nThis mimics the separation between `Vec<T>` and `[T]`.\n*"]
impl<O, T> DerefMut for BitVec<O, T>
where
    O: BitOrder,
    T: BitStore {
    #[doc= " Dereferences `&mut BitVec` down to `&mut BitSlice`."]
    #[doc= ""]
    #[doc= " # Examples"]
    #[doc= ""]
    #[doc= " ```rust"]
    #[doc= " use bitvec::prelude::*;"]
    #[doc= ""]
    #[doc= " let mut bv: BitVec = bitvec![0; 6];"]
    #[doc= " let bref: &mut BitSlice = &mut bv;"]
    #[doc= " assert!(!bref[5]);"]
    #[doc= " bref.set(5, true);"]
    #[doc= " assert!(bref[5]);"]
    #[doc= " ```"]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut_bitslice()
    }
}

