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

__bitvec_shift!(u8, u16, u32, u64, i8, i16, i32, i64);

#[doc= " Shifts all bits in the vector to the left – **DOWN AND TOWARDS THE FRONT**.\n\nOn fundamentals, the left-shift operator `<<` moves bits away from origin and\ntowards the ceiling. This is because we label the bits in a primitive with the\nminimum on the right and the maximum on the left, which is big-endian bit order.\nThis increases the value of the primitive being shifted.\n\n**THAT IS NOT HOW `BITVEC` WORKS!**\n\n`BitVec` defines its layout with the minimum on the left and the maximum on the\nright! Thus, left-shifting moves bits towards the **minimum**.\n\nIn `Msb0` order, the effect in memory will be what you expect the `<<` operator\nto do.\n\n**In `Lsb0` order, the effect will be equivalent to using `>>` on the**\n**fundamentals in memory!**\n\n# Notes\n\nIn order to preserve the effects in memory that this operator traditionally\nexpects, the bits that are emptied by this operation are zeroed rather than\nleft to their old value.\n\nThe length of the vector is decreased by the shift amount.\n\nIf the shift amount is greater than the length, the vector calls `clear()` and\nzeroes its memory. This is *not* an error.\n*"]
impl<O, T> Shl<usize> for BitVec<O, T>
where
    O: BitOrder,
    T: BitStore {
    type Output = Self;

    #[doc= " Shifts a `BitVec` to the left, shortening it."]
    #[doc= ""]
    #[doc= " # Examples"]
    #[doc= ""]
    #[doc= " ```rust"]
    #[doc= " use bitvec::prelude::*;"]
    #[doc= ""]
    #[doc= " let bv = bitvec![Msb0, u8; 0, 0, 0, 1, 1, 1];"]
    #[doc= " assert_eq!(\"[000111]\", &format!(\"{}\", bv));"]
    #[doc= " assert_eq!(0b0001_1100, bv.as_slice()[0]);"]
    #[doc= " assert_eq!(bv.len(), 6);"]
    #[doc= " let ls = bv << 2usize;"]
    #[doc= " assert_eq!(\"[0111]\", &format!(\"{}\", ls));"]
    #[doc= " assert_eq!(0b0111_0000, ls.as_slice()[0]);"]
    #[doc= " assert_eq!(ls.len(), 4);"]
    #[doc= " ```"]
    fn shl(mut self, shamt: usize) -> Self::Output {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

#[doc= " Shifts all bits in the vector to the left – **DOWN AND TOWARDS THE FRONT**.\n\nOn fundamentals, the left-shift operator `<<` moves bits away from origin and\ntowards the ceiling. This is because we label the bits in a primitive with the\nminimum on the right and the maximum on the left, which is big-endian bit order.\nThis increases the value of the primitive being shifted.\n\n**THAT IS NOT HOW `BITVEC` WORKS!**\n\n`BitVec` defines its layout with the minimum on the left and the maximum on the\nright! Thus, left-shifting moves bits towards the **minimum**.\n\nIn `Msb0` order, the effect in memory will be what you expect the `<<` operator\nto do.\n\n**In `Lsb0` order, the effect will be equivalent to using `>>` on the**\n**fundamentals in memory!**\n\n# Notes\n\nIn order to preserve the effects in memory that this operator traditionally\nexpects, the bits that are emptied by this operation are zeroed rather than left\nto their old value.\n\nThe length of the vector is decreased by the shift amount.\n\nIf the shift amount is greater than the length, the vector calls `clear()` and\nzeroes its memory. This is *not* an error.\n*"]
impl<O, T> ShlAssign<usize> for BitVec<O, T>
where
    O: BitOrder,
    T: BitStore {
    #[doc= " Shifts a `BitVec` to the left in place, shortening it."]
    #[doc= ""]
    #[doc= " # Examples"]
    #[doc= ""]
    #[doc= " ```rust"]
    #[doc= " use bitvec::prelude::*;"]
    #[doc= ""]
    #[doc= " let mut bv = bitvec![Lsb0, u8; 0, 0, 0, 1, 1, 1];"]
    #[doc= " assert_eq!(\"[000111]\", &format!(\"{}\", bv));"]
    #[doc= " assert_eq!(0b0011_1000, bv.as_slice()[0]);"]
    #[doc= " assert_eq!(bv.len(), 6);"]
    #[doc= " bv <<= 2;"]
    #[doc= " assert_eq!(\"[0111]\", &format!(\"{}\", bv));"]
    #[doc= " assert_eq!(0b0000_1110, bv.as_slice()[0]);"]
    #[doc= " assert_eq!(bv.len(), 4);"]
    #[doc= " ```"]
    fn shl_assign(&mut self, shamt: usize) {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

#[doc= " Shifts all bits in the vector to the right – **UP AND TOWARDS THE BACK**.\n\nOn fundamentals, the right-shift operator `>>` moves bits towards the origin and\naway from the ceiling. This is because we label the bits in a primitive with the\nminimum on the right and the maximum on the left, which is big-endian bit order.\nThis decreases the value of the primitive being shifted.\n\n**THAT IS NOT HOW `BITVEC` WORKS!**\n\n`BitVec` defines its layout with the minimum on the left and the maximum on the\nright! Thus, right-shifting moves bits towards the **maximum**.\n\nIn `Msb0` order, the effect in memory will be what you expect the `>>` operator\nto do.\n\n**In `Lsb0` order, the effect will be equivalent to using `<<` on the**\n**fundamentals in memory!**\n\n# Notes\n\nIn order to preserve the effects in memory that this operator traditionally\nexpects, the bits that are emptied by this operation are zeroed rather than left\nto their old value.\n\nThe length of the vector is increased by the shift amount.\n\nIf the new length of the vector would overflow, a panic occurs. This *is* an\nerror.\n*"]
impl<O, T> Shr<usize> for BitVec<O, T>
where
    O: BitOrder,
    T: BitStore {
    type Output = Self;

    #[doc= " Shifts a `BitVec` to the right, lengthening it and filling the front"]
    #[doc= " with 0."]
    #[doc= ""]
    #[doc= " # Examples"]
    #[doc= ""]
    #[doc= " ```rust"]
    #[doc= " use bitvec::prelude::*;"]
    #[doc= ""]
    #[doc= " let bv = bitvec![Msb0, u8; 0, 0, 0, 1, 1, 1];"]
    #[doc= " assert_eq!(\"[000111]\", &format!(\"{}\", bv));"]
    #[doc= " assert_eq!(0b0001_1100, bv.as_slice()[0]);"]
    #[doc= " assert_eq!(bv.len(), 6);"]
    #[doc= " let rs = bv >> 2usize;"]
    #[doc= " assert_eq!(\"[00000111]\", &format!(\"{}\", rs));"]
    #[doc= " assert_eq!(0b0000_0111, rs.as_slice()[0]);"]
    #[doc= " assert_eq!(rs.len(), 8);"]
    #[doc= " ```"]
    fn shr(mut self, shamt: usize) -> Self::Output {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

#[doc= " Shifts all bits in the vector to the right – **UP AND TOWARDS THE BACK**.\n\nOn fundamentals, the right-shift operator `>>` moves bits towards the origin and\naway from the ceiling. This is because we label the bits in a primitive with the\nminimum on the right and the maximum on the left, which is big-endian bit order.\nThis decreases the value of the primitive being shifted.\n\n**THAT IS NOT HOW `BITVEC` WORKS!**\n\n`BitVec` defines its layout with the minimum on the left and the maximum on the\nright! Thus, right-shifting moves bits towards the **maximum**.\n\nIn `Msb0` order, the effect in memory will be what you expect the `>>` operator\nto do.\n\n**In `Lsb0` order, the effect will be equivalent to using `<<` on the**\n**fundamentals in memory!**\n\n# Notes\n\nIn order to preserve the effects in memory that this operator traditionally\nexpects, the bits that are emptied by this operation are zeroed rather than left\nto their old value.\n\nThe length of the vector is increased by the shift amount.\n\nIf the new length of the vector would overflow, a panic occurs. This *is* an\nerror.\n*"]
impl<O, T> ShrAssign<usize> for BitVec<O, T>
where
    O: BitOrder,
    T: BitStore {
    #[doc= " Shifts a `BitVec` to the right in place, lengthening it and filling the"]
    #[doc= " front with 0."]
    #[doc= ""]
    #[doc= " # Examples"]
    #[doc= ""]
    #[doc= " ```rust"]
    #[doc= " use bitvec::prelude::*;"]
    #[doc= ""]
    #[doc= " let mut bv = bitvec![Lsb0, u8; 0, 0, 0, 1, 1, 1];"]
    #[doc= " assert_eq!(\"[000111]\", &format!(\"{}\", bv));"]
    #[doc= " assert_eq!(0b0011_1000, bv.as_slice()[0]);"]
    #[doc= " assert_eq!(bv.len(), 6);"]
    #[doc= " bv >>= 2;"]
    #[doc= " assert_eq!(\"[00000111]\", &format!(\"{}\", bv));"]
    #[doc= " assert_eq!(0b1110_0000, bv.as_slice()[0]);"]
    #[doc= " assert_eq!(bv.len(), 8);"]
    #[doc= " ```"]
    fn shr_assign(&mut self, shamt: usize) {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}
