#![doc= " Operator trait implementations."]

use super::*;
use crate::{
    order::BitOrder,
    store::BitStore,
};
use core::ops::{
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
};

#[doc= " Performs the Boolean `AND` operation between each element of a `BitVec` and\nanything that can provide a stream of `bool` values (such as another `BitVec`,\nor any `bool` generator of your choice). The `BitVec` emitted will have the\nlength of the shorter sequence of bits -- if one is longer than the other, the\nextra bits will be ignored.\n*"]
impl<O, T, I> BitAnd<I> for BitVec<O, T>
where
    O: BitOrder,
    T: BitStore,
    I: IntoIterator<Item = bool> {
    type Output = Self;

    #[doc= " `AND`s a vector and a bitstream, producing a new vector."]
    #[doc= ""]
    #[doc= " # Examples"]
    #[doc= ""]
    #[doc= " ```rust"]
    #[doc= " use bitvec::prelude::*;"]
    #[doc= ""]
    #[doc= " let lhs = bitvec![Msb0, u8; 0, 1, 0, 1];"]
    #[doc= " let rhs = bitvec![Msb0, u8; 0, 0, 1, 1];"]
    #[doc= " let and = lhs & rhs;"]
    #[doc= " assert_eq!(\"[0001]\", &format!(\"{}\", and));"]
    #[doc= " ```"]
    fn bitand(mut self, rhs: I) -> Self::Output {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

#[doc= " Performs the Boolean `AND` operation in place on a `BitVec`, using a stream\nof `bool` values as the other bit for each operation. If the other stream is\nshorter than `self`, `self` will be truncated when the other stream expires.\n*"]
impl<O, T, I> BitAndAssign<I> for BitVec<O, T>
where
    O: BitOrder,
    T: BitStore,
    I: IntoIterator<Item = bool> {
    #[doc= " `AND`s another bitstream into a vector."]
    #[doc= ""]
    #[doc= " # Examples"]
    #[doc= ""]
    #[doc= " ```rust"]
    #[doc= " use bitvec::prelude::*;"]
    #[doc= ""]
    #[doc= " let mut src  = bitvec![Msb0, u8; 0, 1, 0, 1];"]
    #[doc= "         src &= bitvec![Msb0, u8; 0, 0, 1, 1];"]
    #[doc= " assert_eq!(\"[0001]\", &format!(\"{}\", src));"]
    #[doc= " ```"]
    fn bitand_assign(&mut self, rhs: I) {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

#[doc= " Performs the Boolean `OR` operation between each element of a `BitVec` and\nanything that can provide a stream of `bool` values (such as another `BitVec`,\nor any `bool` generator of your choice). The `BitVec` emitted will have the\nlength of the shorter sequence of bits -- if one is longer than the other, the\nextra bits will be ignored.\n*"]
impl<O, T, I> BitOr<I> for BitVec<O, T>
where
    O: BitOrder,
    T: BitStore,
    I: IntoIterator<Item = bool> {
    type Output = Self;

    #[doc= " `OR`s a vector and a bitstream, producing a new vector."]
    #[doc= ""]
    #[doc= " # Examples"]
    #[doc= ""]
    #[doc= " ```rust"]
    #[doc= " use bitvec::prelude::*;"]
    #[doc= ""]
    #[doc= " let lhs = bitvec![0, 1, 0, 1];"]
    #[doc= " let rhs = bitvec![0, 0, 1, 1];"]
    #[doc= " let or = lhs | rhs;"]
    #[doc= " assert_eq!(\"[0111]\", &format!(\"{}\", or));"]
    #[doc= " ```"]
    fn bitor(mut self, rhs: I) -> Self::Output {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

#[doc= " Performs the Boolean `OR` operation in place on a `BitVec`, using a stream\nof `bool` values as the other bit for each operation. If the other stream is\nshorter than `self`, `self` will be truncated when the other stream expires.\n*"]
impl<O, T, I> BitOrAssign<I> for BitVec<O, T>
where
    O: BitOrder,
    T: BitStore,
    I: IntoIterator<Item = bool> {
    #[doc= " `OR`s another bitstream into a vector."]
    #[doc= ""]
    #[doc= " # Examples"]
    #[doc= ""]
    #[doc= " ```rust"]
    #[doc= " use bitvec::prelude::*;"]
    #[doc= ""]
    #[doc= " let mut src  = bitvec![0, 1, 0, 1];"]
    #[doc= "         src |= bitvec![0, 0, 1, 1];"]
    #[doc= " assert_eq!(\"[0111]\", &format!(\"{}\", src));"]
    #[doc= " ```"]
    fn bitor_assign(&mut self, rhs: I) {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

#[doc= " Performs the Boolean `XOR` operation between each element of a `BitVec` and\nanything that can provide a stream of `bool` values (such as another `BitVec`,\nor any `bool` generator of your choice). The `BitVec` emitted will have the\nlength of the shorter sequence of bits -- if one is longer than the other, the\nextra bits will be ignored.\n*"]
impl<O, T, I> BitXor<I> for BitVec<O, T>
where
    O: BitOrder,
    T: BitStore,
    I: IntoIterator<Item = bool> {
    type Output = Self;

    #[doc= " `XOR`s a vector and a bitstream, producing a new vector."]
    #[doc= ""]
    #[doc= " # Examples"]
    #[doc= ""]
    #[doc= " ```rust"]
    #[doc= " use bitvec::prelude::*;"]
    #[doc= ""]
    #[doc= " let lhs = bitvec![0, 1, 0, 1];"]
    #[doc= " let rhs = bitvec![0, 0, 1, 1];"]
    #[doc= " let xor = lhs ^ rhs;"]
    #[doc= " assert_eq!(\"[0110]\", &format!(\"{}\", xor));"]
    #[doc= " ```"]
    fn bitxor(mut self, rhs: I) -> Self::Output {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

#[doc= " Performs the Boolean `XOR` operation in place on a `BitVec`, using a stream\nof `bool` values as the other bit for each operation. If the other stream is\nshorter than `self`, `self` will be truncated when the other stream expires.\n*"]
impl<O, T, I> BitXorAssign<I> for BitVec<O, T>
where
    O: BitOrder,
    T: BitStore,
    I: IntoIterator<Item = bool> {
    #[doc= " `XOR`s another bitstream into a vector."]
    #[doc= ""]
    #[doc= " # Examples"]
    #[doc= ""]
    #[doc= " ```rust"]
    #[doc= " use bitvec::prelude::*;"]
    #[doc= ""]
    #[doc= " let mut src  = bitvec![0, 1, 0, 1];"]
    #[doc= "         src ^= bitvec![0, 0, 1, 1];"]
    #[doc= " assert_eq!(\"[0110]\", &format!(\"{}\", src));"]
    #[doc= " ```"]
    fn bitxor_assign(&mut self, rhs: I) {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

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

#[doc= " Readies the underlying storage for Drop."]
impl<O, T> Drop for BitVec<O, T>
where
    O: BitOrder,
    T: BitStore {
    #[doc= " Rebuild the interior `Vec` and let it run the deallocator."]
    fn drop(&mut self) {
        let bp = mem::replace(&mut self.pointer, BitPtr::empty());
        let (ptr, cap) = (bp.pointer(), self.capacity);
        drop(unsafe {
            Vec::from_raw_parts(ptr.w(), 0, cap)
        });
    }
}

#[doc= " Gets the bit at a specific index. The index must be less than the length of"]
#[doc= " the `BitVec`."]
impl<O, T> Index<usize> for BitVec<O, T>
where
    O: BitOrder,
    T: BitStore {
    type Output = bool;

    #[doc= " Looks up a single bit by semantic count."]
    #[doc= ""]
    #[doc= " # Examples"]
    #[doc= ""]
    #[doc= " ```rust"]
    #[doc= " use bitvec::prelude::*;"]
    #[doc= ""]
    #[doc= " let bv = bitvec![Msb0, u8; 0, 0, 0, 0, 0, 0, 0, 0, 1, 0];"]
    #[doc= " assert!(!bv[7]); // ---------------------------------^  |  |"]
    #[doc= " assert!( bv[8]); // ------------------------------------^  |"]
    #[doc= " assert!(!bv[9]); // ---------------------------------------^"]
    #[doc= " ```"]
    #[doc= ""]
    #[doc= " If the index is greater than or equal to the length, indexing will"]
    #[doc= " panic."]
    #[doc= ""]
    #[doc= " The below test will panic when accessing index 1, as only index 0 is"]
    #[doc= " valid."]
    #[doc= ""]
    #[doc= " ```rust,should_panic"]
    #[doc= " use bitvec::prelude::*;"]
    #[doc= ""]
    #[doc= " let mut bv: BitVec = BitVec::new();"]
    #[doc= " bv.push(true);"]
    #[doc= " bv[1];"]
    #[doc= " ```"]
    fn index(&self, cursor: usize) -> &Self::Output {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<O, T> Index<Range<usize>> for BitVec<O, T>
where
    O: BitOrder,
    T: BitStore {
    type Output = BitSlice<O, T>;

    fn index(&self, range: Range<usize>) -> &Self::Output {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<O, T> IndexMut<Range<usize>> for BitVec<O, T>
where
    O: BitOrder,
    T: BitStore {
    fn index_mut(&mut self, range: Range<usize>) -> &mut Self::Output {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<O, T> Index<RangeFrom<usize>> for BitVec<O, T>
where
    O: BitOrder,
    T: BitStore {
    type Output = BitSlice<O, T>;

    fn index(&self, range: RangeFrom<usize>) -> &Self::Output {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<O, T> IndexMut<RangeFrom<usize>> for BitVec<O, T>
where
    O: BitOrder,
    T: BitStore {
    fn index_mut(&mut self, range: RangeFrom<usize>) -> &mut Self::Output {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<O, T> Index<RangeFull> for BitVec<O, T>
where
    O: BitOrder,
    T: BitStore {
    type Output = BitSlice<O, T>;

    fn index(&self, _: RangeFull) -> &Self::Output {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<O, T> IndexMut<RangeFull> for BitVec<O, T>
where
    O: BitOrder,
    T: BitStore {
    fn index_mut(&mut self, _: RangeFull) -> &mut Self::Output {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<O, T> Index<RangeInclusive<usize>> for BitVec<O, T>
where
    O: BitOrder,
    T: BitStore {
    type Output = BitSlice<O, T>;

    fn index(&self, range: RangeInclusive<usize>) -> &Self::Output {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<O, T> IndexMut<RangeInclusive<usize>> for BitVec<O, T>
where
    O: BitOrder,
    T: BitStore {
    fn index_mut(&mut self, range: RangeInclusive<usize>) -> &mut Self::Output {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<O, T> Index<RangeTo<usize>> for BitVec<O, T>
where
    O: BitOrder,
    T: BitStore {
    type Output = BitSlice<O, T>;

    fn index(&self, range: RangeTo<usize>) -> &Self::Output {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<O, T> IndexMut<RangeTo<usize>> for BitVec<O, T>
where
    O: BitOrder,
    T: BitStore {
    fn index_mut(&mut self, range: RangeTo<usize>) -> &mut Self::Output {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<O, T> Index<RangeToInclusive<usize>> for BitVec<O, T>
where
    O: BitOrder,
    T: BitStore {
    type Output = BitSlice<O, T>;

    fn index(&self, range: RangeToInclusive<usize>) -> &Self::Output {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<O, T> IndexMut<RangeToInclusive<usize>> for BitVec<O, T>
where
    O: BitOrder,
    T: BitStore {
    fn index_mut(&mut self, range: RangeToInclusive<usize>) -> &mut Self::Output {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

#[doc= " Flips all bits in the vector."]
impl<O, T> Not for BitVec<O, T>
where
    O: BitOrder,
    T: BitStore {
    type Output = Self;

    #[doc= " Inverts all bits in the vector."]
    #[doc= ""]
    #[doc= " # Examples"]
    #[doc= ""]
    #[doc= " ```rust"]
    #[doc= " use bitvec::prelude::*;"]
    #[doc= ""]
    #[doc= " let bv: BitVec<Msb0, u32> = BitVec::from(&[0u32] as &[u32]);"]
    #[doc= " let flip = !bv;"]
    #[doc= " assert_eq!(!0u32, flip.as_slice()[0]);"]
    #[doc= " ```"]
    fn not(mut self) -> Self::Output {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
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
