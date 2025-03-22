#![doc= " Operator trait implementations."]

use crate::{
    order::BitOrder,
    slice::BitSlice,
    store::BitStore,
};
use core::ops::{
    Index,
    Range,
    RangeFrom,
    RangeTo,
    ShlAssign,
    ShrAssign,
};
use core::slice::SliceIndex;
use crate::slice::api::BitSliceIndex;

impl<O, T> Index<usize> for BitSlice<O, T>
where
    O: BitOrder,
    T: BitStore,
{
    type Output = bool;

    fn index(&self, place: usize) -> &Self::Output {
        place.index(self)
    }
}

impl<O, T> Index<Range<usize>> for BitSlice<O, T>
where
    O: BitOrder,
    T: BitStore,
{
    type Output = Self;

    fn index(&self, range: Range<usize>) -> &Self {
        range.index(self)
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
