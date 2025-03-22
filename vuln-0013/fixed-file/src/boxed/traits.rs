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

impl<O, T> Borrow<BitSlice<O, T>> for BitBox<O, T>
where
    O: BitOrder,
    T: BitStore {
    fn borrow(&self) -> &BitSlice<O, T> {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<O, T> BorrowMut<BitSlice<O, T>> for BitBox<O, T>
where
    O: BitOrder,
    T: BitStore {
    fn borrow_mut(&mut self) -> &mut BitSlice<O, T> {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<O, T> Clone for BitBox<O, T>
where
    O: BitOrder,
    T: BitStore {
    fn clone(&self) -> Self {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<O, T> Eq for BitBox<O, T>
where
    O: BitOrder,
    T: BitStore { }

impl<O, T> Ord for BitBox<O, T>
where
    O: BitOrder,
    T: BitStore {
    fn cmp(&self, rhs: &Self) -> Ordering {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<A, B, C, D> PartialEq<BitBox<C, D>> for BitBox<A, B>
where
    A: BitOrder,
    B: BitStore,
    C: BitOrder,
    D: BitStore {
    fn eq(&self, rhs: &BitBox<C, D>) -> bool {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<A, B, C, D> PartialEq<BitSlice<C, D>> for BitBox<A, B>
where
    A: BitOrder,
    B: BitStore,
    C: BitOrder,
    D: BitStore {
    fn eq(&self, rhs: &BitSlice<C, D>) -> bool {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<A, B, C, D> PartialEq<BitBox<C, D>> for BitSlice<A, B>
where
    A: BitOrder,
    B: BitStore,
    C: BitOrder,
    D: BitStore {
    fn eq(&self, rhs: &BitBox<C, D>) -> bool {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<A, B, C, D> PartialOrd<BitBox<C, D>> for BitBox<A, B>
where
    A: BitOrder,
    B: BitStore,
    C: BitOrder,
    D: BitStore {
    fn partial_cmp(&self, rhs: &BitBox<C, D>) -> Option<Ordering> {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<A, B, C, D> PartialOrd<BitSlice<C, D>> for BitBox<A, B>
where
    A: BitOrder,
    B: BitStore,
    C: BitOrder,
    D: BitStore {
    fn partial_cmp(&self, rhs: &BitSlice<C, D>) -> Option<Ordering> {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<A, B, C, D> PartialOrd<BitBox<C, D>> for BitSlice<A, B>
where
    A: BitOrder,
    B: BitStore,
    C: BitOrder,
    D: BitStore {
    fn partial_cmp(&self, rhs: &BitBox<C, D>) -> Option<Ordering> {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<O, T> AsMut<BitSlice<O, T>> for BitBox<O, T>
where
    O: BitOrder,
    T: BitStore {
    fn as_mut(&mut self) -> &mut BitSlice<O, T> {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<O, T> AsRef<BitSlice<O, T>> for BitBox<O, T>
where
    O: BitOrder,
    T: BitStore {
    fn as_ref(&self) -> &BitSlice<O, T> {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<O, T> From<&BitSlice<O, T>> for BitBox<O, T>
where
    O: BitOrder,
    T: BitStore {
    fn from(src: &BitSlice<O, T>) -> Self {
        Self::from_bitslice(src)
    }
}

impl<O, T> From<&[T]> for BitBox<O, T>
where
    O: BitOrder,
    T: BitStore {
    fn from(src: &[T]) -> Self {
        Self::from_slice(src)
    }
}

impl<O, T> From<BitVec<O, T>> for BitBox<O, T>
where
    O: BitOrder,
    T: BitStore {
    fn from(src: BitVec<O, T>) -> Self {
        src.into_boxed_bitslice()
    }
}

impl<O, T> From<Box<[T]>> for BitBox<O, T>
where
    O: BitOrder,
    T: BitStore {
    fn from(src: Box<[T]>) -> Self {
        Self::from_boxed_slice(src)
    }
}

impl<O, T> Into<Box<[T]>> for BitBox<O, T>
where
    O: BitOrder,
    T: BitStore {
    fn into(self) -> Box<[T]> {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<O, T> Default for BitBox<O, T>
where
    O: BitOrder,
    T: BitStore {
    fn default() -> Self {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<O, T> Binary for BitBox<O, T>
where
    O: BitOrder,
    T: BitStore {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<O, T> Debug for BitBox<O, T>
where
    O: BitOrder,
    T: BitStore {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<O, T> Display for BitBox<O, T>
where
    O: BitOrder,
    T: BitStore {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<O, T> LowerHex for BitBox<O, T>
where
    O: BitOrder,
    T: BitStore {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<O, T> Octal for BitBox<O, T>
where
    O: BitOrder,
    T: BitStore {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<O, T> UpperHex for BitBox<O, T>
where
    O: BitOrder,
    T: BitStore {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<O, T> Hash for BitBox<O, T>
where
    O: BitOrder,
    T: BitStore {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

#[doc= " `BitBox` is safe to move across thread boundaries, as is `&mut BitBox`."]
unsafe impl<O, T> Send for BitBox<O, T>
where
    O: BitOrder,
    T: BitStore { }

#[doc= " `&BitBox` is safe to move across thread boundaries."]
unsafe impl<O, T> Sync for BitBox<O, T>
where
    O: BitOrder,
    T: BitStore { }
