#![doc= " General trait implementations for `BitBox`\n\nThe operator traits are defined in the `ops` module.\n!"]

use crate::{
    boxed::BitBox,
    order::BitOrder,
    slice::BitSlice,
    store::BitStore,
    vec::BitVec,
};
use alloc::{
    borrow::{
        Borrow,
        BorrowMut,
    },
    boxed::Box,
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

impl<O, T> From<BitVec<O, T>> for BitBox<O, T>
where
    O: BitOrder,
    T: BitStore {
    fn from(src: BitVec<O, T>) -> Self {
        src.into_boxed_bitslice()
    }
}
