#![doc= " General trait implementations for `BitSlice`.\n\nThe operator traits are defined in the `ops` module.\n!"]

use crate::{
    access::BitAccess,
    domain::Domain,
    order::BitOrder,
    slice::BitSlice,
    store::BitStore,
};
use core::{
    fmt::{
        self,
        Binary,
        Debug,
        Formatter,
        LowerHex,
        Octal,
        UpperHex,
    },
    hash::{
        Hasher,
    },
    hint::unreachable_unchecked,
    str,
};

#[doc= " Prints the `BitSlice` for debugging.\n\nThe output is of the form `BitSlice<O, T> [ELT, *]` where `<O, T>` is the order\nand element type, with square brackets on each end of the bits and all the\nelements of the array printed in binary. The printout is always in semantic\norder, and may not reflect the underlying buffer. To see the underlying buffer,\nuse `.as_total_slice()`.\n\nThe alternate character `{:#?}` prints each element on its own line, rather than\nhaving all elements on the same line.\n*"]
impl<O, T> Debug for BitSlice<O, T>
where
    O: BitOrder,
    T: BitStore {
    #[doc= " Renders the `BitSlice` type header and contents for debug."]
    #[doc= ""]
    #[doc= " # Examples"]
    #[doc= ""]
    #[doc= " ```rust"]
    #[doc= " # #[cfg(feature = \"alloc\")] {"]
    #[doc= " use bitvec::prelude::*;"]
    #[doc= ""]
    #[doc= " let src = [0b0101_0000_1111_0101u16, 0b00000000_0000_0010];"]
    #[doc= " let bits = &src.bits::<Lsb0>()[.. 18];"]
    #[doc= " assert_eq!("]
    #[doc= "     \"BitSlice<Lsb0, u16> [1010111100001010, 01]\","]
    #[doc= "     &format!(\"{:?}\", bits),"]
    #[doc= " );"]
    #[doc= " # }"]
    #[doc= " ```"]
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}
