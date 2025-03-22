#![doc= " Operator trait implementations."]

use crate::{
    order::BitOrder,
    slice::BitSlice,
    store::BitStore,
};
use core::ops::{
    BitAndAssign,
    BitOrAssign,
    BitXorAssign,
    Index,
    IndexMut,
    Not,
    Range,
    RangeFrom,
    RangeFull,
    RangeInclusive,
    RangeTo,
    RangeToInclusive,
    ShlAssign,
    ShrAssign,
};

#[doc= " Performs the Boolean `AND` operation against another bitstream and writes\nthe result into `self`. If the other bitstream ends before `self,`, the\nremaining bits of `self` are cleared.\n\n# Type Parameters\n\n- `I: IntoIterator<Item=bool>`: A stream of bits, which may be a `BitSlice`\n  or some other bit producer as desired.\n*"]
impl<O, T, I> BitAndAssign<I> for BitSlice<O, T>
where
    O: BitOrder,
    T: BitStore,
    I: IntoIterator<Item = bool> {
    #[doc= " `AND`s a bitstream into a slice."]
    #[doc= ""]
    #[doc= " # Parameters"]
    #[doc= ""]
    #[doc= " - `&mut self`"]
    #[doc= " - `rhs`: The bitstream to `AND` into `self`."]
    #[doc= ""]
    #[doc= " # Examples"]
    #[doc= ""]
    #[doc= " ```rust"]
    #[doc= " use bitvec::prelude::*;"]
    #[doc= ""]
    #[doc= " let mut store = [0b0101_0100u8];"]
    #[doc= " let     other = [0b0011_0000u8];"]
    #[doc= " let lhs = store.bits_mut::<Msb0>();"]
    #[doc= " let rhs = other.bits::<Msb0>();"]
    #[doc= " lhs[.. 6] &= rhs[.. 4].iter().copied();"]
    #[doc= " assert_eq!(store[0], 0b0001_0000);"]
    #[doc= " ```"]
    fn bitand_assign(&mut self, rhs: I) {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

#[doc= " Performs the Boolean `OR` operation against another bitstream and writes the\nresult into `self`. If the other bitstream ends before `self`, the remaining\nbits of `self` are not affected.\n\n# Type Parameters\n\n- `I: IntoIterator<Item=bool>`: A stream of bits, which may be a `BitSlice`\n  or some other bit producer as desired.\n*"]
impl<O, T, I> BitOrAssign<I> for BitSlice<O, T>
where
    O: BitOrder,
    T: BitStore,
    I: IntoIterator<Item = bool> {
    #[doc= " `OR`s a bitstream into a slice."]
    #[doc= ""]
    #[doc= " # Parameters"]
    #[doc= ""]
    #[doc= " - `&mut self`"]
    #[doc= " - `rhs`: The bitstream to `OR` into `self`."]
    #[doc= ""]
    #[doc= " # Examples"]
    #[doc= ""]
    #[doc= " ```rust"]
    #[doc= " use bitvec::prelude::*;"]
    #[doc= ""]
    #[doc= " let mut store = [0b0101_0100u8];"]
    #[doc= " let     other = [0b0011_0000u8];"]
    #[doc= " let lhs = store.bits_mut::<Msb0>();"]
    #[doc= " let rhs = other.bits::<Msb0>();"]
    #[doc= " lhs[.. 6] |= rhs[.. 4].iter().copied();"]
    #[doc= " assert_eq!(store[0], 0b0111_0100);"]
    #[doc= " ```"]
    fn bitor_assign(&mut self, rhs: I) {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

#[doc= " Performs the Boolean `XOR` operation against another bitstream and writes\nthe result into `self`. If the other bitstream ends before `self`, the remaining\nbits of `self` are not affected.\n\n# Type Parameters\n\n- `I: IntoIterator<Item=bool>`: A stream of bits, which may be a `BitSlice`\n  or some other bit producer as desired.\n*"]
impl<O, T, I> BitXorAssign<I> for BitSlice<O, T>
where
    O: BitOrder,
    T: BitStore,
    I: IntoIterator<Item = bool> {
    #[doc= " `XOR`s a bitstream into a slice."]
    #[doc= ""]
    #[doc= " # Parameters"]
    #[doc= ""]
    #[doc= " - `&mut self`"]
    #[doc= " - `rhs`: The bitstream to `XOR` into `self`."]
    #[doc= ""]
    #[doc= " # Examples"]
    #[doc= ""]
    #[doc= " ```rust"]
    #[doc= " use bitvec::prelude::*;"]
    #[doc= ""]
    #[doc= " let mut store = [0b0101_0100u8];"]
    #[doc= " let     other = [0b0011_0000u8];"]
    #[doc= " let lhs = store.bits_mut::<Msb0>();"]
    #[doc= " let rhs = other.bits::<Msb0>();"]
    #[doc= " lhs[.. 6] ^= rhs[.. 4].iter().copied();"]
    #[doc= " assert_eq!(store[0], 0b0110_0100);"]
    #[doc= " ```"]
    fn bitxor_assign(&mut self, rhs: I) {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<O, T> Index<usize> for BitSlice<O, T>
where
    O: BitOrder,
    T: BitStore {
    type Output = bool;

    fn index(&self, place: usize) -> &Self::Output {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<O, T> Index<Range<usize>> for BitSlice<O, T>
where
    O: BitOrder,
    T: BitStore {
    type Output = Self;

    fn index(&self, range: Range<usize>) -> &Self {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<O, T> IndexMut<Range<usize>> for BitSlice<O, T>
where
    O: BitOrder,
    T: BitStore {
    fn index_mut(&mut self, range: Range<usize>) -> &mut Self {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<O, T> Index<RangeInclusive<usize>> for BitSlice<O, T>
where
    O: BitOrder,
    T: BitStore {
    type Output = Self;

    fn index(&self, range: RangeInclusive<usize>) -> &Self {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<O, T> IndexMut<RangeInclusive<usize>> for BitSlice<O, T>
where
    O: BitOrder,
    T: BitStore {
    fn index_mut(&mut self, range: RangeInclusive<usize>) -> &mut Self {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<O, T> Index<RangeFrom<usize>> for BitSlice<O, T>
where
    O: BitOrder,
    T: BitStore {
    type Output = Self;

    fn index(&self, range: RangeFrom<usize>) -> &Self {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<O, T> IndexMut<RangeFrom<usize>> for BitSlice<O, T>
where
    O: BitOrder,
    T: BitStore {
    fn index_mut(&mut self, range: RangeFrom<usize>) -> &mut Self {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<O, T> Index<RangeFull> for BitSlice<O, T>
where
    O: BitOrder,
    T: BitStore {
    type Output = Self;

    fn index(&self, _: RangeFull) -> &Self {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<O, T> IndexMut<RangeFull> for BitSlice<O, T>
where
    O: BitOrder,
    T: BitStore {
    fn index_mut(&mut self, _: RangeFull) -> &mut Self {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<O, T> Index<RangeTo<usize>> for BitSlice<O, T>
where
    O: BitOrder,
    T: BitStore {
    type Output = Self;

    fn index(&self, range: RangeTo<usize>) -> &Self {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<O, T> IndexMut<RangeTo<usize>> for BitSlice<O, T>
where
    O: BitOrder,
    T: BitStore {
    fn index_mut(&mut self, range: RangeTo<usize>) -> &mut Self {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<O, T> Index<RangeToInclusive<usize>> for BitSlice<O, T>
where
    O: BitOrder,
    T: BitStore {
    type Output = Self;

    fn index(&self, range: RangeToInclusive<usize>) -> &Self {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<O, T> IndexMut<RangeToInclusive<usize>> for BitSlice<O, T>
where
    O: BitOrder,
    T: BitStore {
    fn index_mut(&mut self, range: RangeToInclusive<usize>) -> &mut Self {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

#[doc= " Flips all bits in the slice, in place."]
impl<'a, O, T> Not for &'a mut BitSlice<O, T>
where
    O: BitOrder,
    T: 'a + BitStore {
    type Output = Self;

    #[doc= " Inverts all bits in the slice."]
    #[doc= ""]
    #[doc= " This will not affect bits outside the slice in slice storage elements."]
    #[doc= ""]
    #[doc= " # Parameters"]
    #[doc= ""]
    #[doc= " - `self`"]
    #[doc= ""]
    #[doc= " # Examples"]
    #[doc= ""]
    #[doc= " ```rust"]
    #[doc= " use bitvec::prelude::*;"]
    #[doc= ""]
    #[doc= " let mut src = [0u8; 2];"]
    #[doc= " let bits = &mut src.bits_mut::<Msb0>()[2 .. 14];"]
    #[doc= " let _ = !bits;"]
    #[doc= " //  The `bits` binding is consumed by the `!` operator, and a new"]
    #[doc= " //  reference is returned."]
    #[doc= " // assert_eq!(bits.as_ref(), &[!0, !0]);"]
    #[doc= " assert_eq!(src, [0x3F, 0xFC]);"]
    #[doc= " ```"]
    fn not(self) -> Self::Output {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

__bitslice_shift!(u8, u16, u32, u64, i8, i16, i32, i64);

#[doc= " Shifts all bits in the array to the left — **DOWN AND TOWARDS THE FRONT**.\n\nOn fundamentals, the left-shift operator `<<` moves bits away from the origin\nand  towards the ceiling. This is because we label the bits in a primitive with\nthe  minimum on the right and the maximum on the left, which is big-endian bit\norder.  This increases the value of the primitive being shifted.\n\n**THAT IS NOT HOW `BitSlice` WORKS!**\n\n`BitSlice` defines its layout with the minimum on the left and the maximum on\nthe right! Thus, left-shifting moves bits towards the **minimum**.\n\nIn `Msb0` order, the effect in memory will be what you expect the `<<` operator\nto do.\n\n**In `Lsb0` order, the effect will be equivalent to using `>>` on the**\n**fundamentals in memory!**\n\n# Notes\n\nIn order to preserve the effecs in memory that this operator traditionally\nexpects, the bits that are emptied by this operation are zeroed rather than\nleft to their old value.\n\nThe shift amount is modulated against the array length, so it is not an\nerror to pass a shift amount greater than the array length.\n\nA shift amount of zero is a no-op, and returns immediately.\n*"]
impl<O, T> ShlAssign<usize> for BitSlice<O, T>
where
    O: BitOrder,
    T: BitStore {
    #[doc= " Shifts a slice left, in place."]
    #[doc= ""]
    #[doc= " # Parameters"]
    #[doc= ""]
    #[doc= " - `&mut self`"]
    #[doc= " - `shamt`: The shift amount. If this is greater than the length, then"]
    #[doc= "   the slice is zeroed immediately."]
    #[doc= ""]
    #[doc= " # Examples"]
    #[doc= ""]
    #[doc= " ```rust"]
    #[doc= " use bitvec::prelude::*;"]
    #[doc= ""]
    #[doc= " let mut src = [0x4Bu8, 0xA5];"]
    #[doc= " let bits = &mut src.bits_mut::<Msb0>()[2 .. 14];"]
    #[doc= " *bits <<= 3;"]
    #[doc= " assert_eq!(src, [0b01_011_101, 0b001_000_01]);"]
    #[doc= " ```"]
    #[allow(clippy::suspicious_op_assign_impl)]
    fn shl_assign(&mut self, shamt: usize) {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

#[doc= " Shifts all bits in the array to the right — **UP AND TOWARDS THE BACK**.\n\nOn fundamentals, the right-shift operator `>>` moves bits towards the origin and\naway from the ceiling. This is because we label the bits in a primitive with the\nminimum on the right and the maximum on the left, which is big-endian bit order.\nThis decreases the value of the primitive being shifted.\n\n**THAT IS NOT HOW `BitSlice` WORKS!**\n\n`BitSlice` defines its layout with the minimum on the left and the maximum on\nthe right! Thus, right-shifting moves bits towards the **maximum**.\n\nIn `Msb0` order, the effect in memory will be what you expect the `>>` operator\nto do.\n\n**In `Lsb0` order, the effect will be equivalent to using `<<` on the**\n**fundamentals in memory!**\n\n# Notes\n\nIn order to preserve the effects in memory that this operator traditionally\nexpects, the bits that are emptied by this operation are zeroed rather than left\nto their old value.\n\nThe shift amount is modulated against the array length, so it is not an error to\npass a shift amount greater than the array length.\n\nA shift amount of zero is a no-op, and returns immediately.\n*"]
impl<O, T> ShrAssign<usize> for BitSlice<O, T>
where
    O: BitOrder,
    T: BitStore {
    #[doc= " Shifts a slice right, in place."]
    #[doc= ""]
    #[doc= " # Parameters"]
    #[doc= ""]
    #[doc= " - `&mut self`"]
    #[doc= " - `shamt`: The shift amount. If this is greater than the length, then"]
    #[doc= "   the slice is zeroed immediately."]
    #[doc= ""]
    #[doc= " # Examples"]
    #[doc= ""]
    #[doc= " ```rust"]
    #[doc= " use bitvec::prelude::*;"]
    #[doc= ""]
    #[doc= " let mut src = [0x4Bu8, 0xA5];"]
    #[doc= " let bits = &mut src.bits_mut::<Msb0>()[2 .. 14];"]
    #[doc= " *bits >>= 3;"]
    #[doc= " assert_eq!(src, [0b01_000_00_1, 0b011_101_01])"]
    #[doc= " ```"]
    #[allow(clippy::suspicious_op_assign_impl)]
    fn shr_assign(&mut self, shamt: usize) {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}
