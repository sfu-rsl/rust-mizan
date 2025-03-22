#![doc= " Operator trait implementations."]

use crate::{
    boxed::BitBox,
    order::BitOrder,
    slice::BitSlice,
    store::BitStore,
};
use core::{
    ops::{
        BitAnd,
        BitAndAssign,
        BitOr,
        BitOrAssign,
        BitXor,
        BitXorAssign,
        Deref,
        DerefMut,
        Index,
        IndexMut,
        Not,
        Range,
        RangeFrom,
        RangeFull,
        RangeInclusive,
        RangeTo,
        RangeToInclusive,
        Shl,
        ShlAssign,
        Shr,
        ShrAssign,
    },
    slice,
};

impl<O, T, I> BitAnd<I> for BitBox<O, T>
where
    O: BitOrder,
    T: BitStore,
    I: IntoIterator<Item = bool> {
    type Output = Self;

    fn bitand(mut self, rhs: I) -> Self::Output {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<O, T, I> BitAndAssign<I> for BitBox<O, T>
where
    O: BitOrder,
    T: BitStore,
    I: IntoIterator<Item = bool> {
    fn bitand_assign(&mut self, rhs: I) {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<O, T, I> BitOr<I> for BitBox<O, T>
where
    O: BitOrder,
    T: BitStore,
    I: IntoIterator<Item = bool> {
    type Output = Self;

    fn bitor(mut self, rhs: I) -> Self::Output {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<O, T, I> BitOrAssign<I> for BitBox<O, T>
where
    O: BitOrder,
    T: BitStore,
    I: IntoIterator<Item = bool> {
    fn bitor_assign(&mut self, rhs: I) {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<O, T, I> BitXor<I> for BitBox<O, T>
where
    O: BitOrder,
    T: BitStore,
    I: IntoIterator<Item = bool> {
    type Output = Self;

    fn bitxor(mut self, rhs: I) -> Self::Output {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<O, T, I> BitXorAssign<I> for BitBox<O, T>
where
    O: BitOrder,
    T: BitStore,
    I: IntoIterator<Item = bool> {
    fn bitxor_assign(&mut self, rhs: I) {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<O, T> Deref for BitBox<O, T>
where
    O: BitOrder,
    T: BitStore {
    type Target = BitSlice<O, T>;

    fn deref(&self) -> &Self::Target {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<O, T> DerefMut for BitBox<O, T>
where
    O: BitOrder,
    T: BitStore {
    fn deref_mut(&mut self) -> &mut Self::Target {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<O, T> Drop for BitBox<O, T>
where
    O: BitOrder,
    T: BitStore {
    fn drop(&mut self) {
        let bp = self.bitptr();
        let ptr = bp.pointer().w();
        let len = bp.elements();
        let slice = unsafe {
            slice::from_raw_parts_mut(ptr, len)
        };
        drop(unsafe {
            Box::from_raw(slice as *mut [_])
        })
    }
}

impl<O, T> Index<usize> for BitBox<O, T>
where
    O: BitOrder,
    T: BitStore {
    type Output = bool;

    fn index(&self, index: usize) -> &Self::Output {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<O, T> Index<Range<usize>> for BitBox<O, T>
where
    O: BitOrder,
    T: BitStore {
    type Output = BitSlice<O, T>;

    fn index(&self, range: Range<usize>) -> &Self::Output {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<O, T> IndexMut<Range<usize>> for BitBox<O, T>
where
    O: BitOrder,
    T: BitStore {
    fn index_mut(&mut self, range: Range<usize>) -> &mut Self::Output {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<O, T> Index<RangeFrom<usize>> for BitBox<O, T>
where
    O: BitOrder,
    T: BitStore {
    type Output = BitSlice<O, T>;

    fn index(&self, range: RangeFrom<usize>) -> &Self::Output {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<O, T> IndexMut<RangeFrom<usize>> for BitBox<O, T>
where
    O: BitOrder,
    T: BitStore {
    fn index_mut(&mut self, range: RangeFrom<usize>) -> &mut Self::Output {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<O, T> Index<RangeFull> for BitBox<O, T>
where
    O: BitOrder,
    T: BitStore {
    type Output = BitSlice<O, T>;

    fn index(&self, _: RangeFull) -> &Self::Output {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<O, T> IndexMut<RangeFull> for BitBox<O, T>
where
    O: BitOrder,
    T: BitStore {
    fn index_mut(&mut self, _: RangeFull) -> &mut Self::Output {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<O, T> Index<RangeInclusive<usize>> for BitBox<O, T>
where
    O: BitOrder,
    T: BitStore {
    type Output = BitSlice<O, T>;

    fn index(&self, range: RangeInclusive<usize>) -> &Self::Output {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<O, T> IndexMut<RangeInclusive<usize>> for BitBox<O, T>
where
    O: BitOrder,
    T: BitStore {
    fn index_mut(&mut self, range: RangeInclusive<usize>) -> &mut Self::Output {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<O, T> Index<RangeTo<usize>> for BitBox<O, T>
where
    O: BitOrder,
    T: BitStore {
    type Output = BitSlice<O, T>;

    fn index(&self, range: RangeTo<usize>) -> &Self::Output {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<O, T> IndexMut<RangeTo<usize>> for BitBox<O, T>
where
    O: BitOrder,
    T: BitStore {
    fn index_mut(&mut self, range: RangeTo<usize>) -> &mut Self::Output {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<O, T> Index<RangeToInclusive<usize>> for BitBox<O, T>
where
    O: BitOrder,
    T: BitStore {
    type Output = BitSlice<O, T>;

    fn index(&self, range: RangeToInclusive<usize>) -> &Self::Output {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<O, T> IndexMut<RangeToInclusive<usize>> for BitBox<O, T>
where
    O: BitOrder,
    T: BitStore {
    fn index_mut(&mut self, range: RangeToInclusive<usize>) -> &mut Self::Output {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<O, T> Not for BitBox<O, T>
where
    O: BitOrder,
    T: BitStore {
    type Output = Self;

    fn not(mut self) -> Self::Output {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<O, T> Shl<usize> for BitBox<O, T>
where
    O: BitOrder,
    T: BitStore {
    type Output = Self;

    fn shl(mut self, shamt: usize) -> Self::Output {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<O, T> ShlAssign<usize> for BitBox<O, T>
where
    O: BitOrder,
    T: BitStore {
    fn shl_assign(&mut self, shamt: usize) {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<O, T> Shr<usize> for BitBox<O, T>
where
    O: BitOrder,
    T: BitStore {
    type Output = Self;

    fn shr(mut self, shamt: usize) -> Self::Output {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<O, T> ShrAssign<usize> for BitBox<O, T>
where
    O: BitOrder,
    T: BitStore {
    fn shr_assign(&mut self, shamt: usize) {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}
