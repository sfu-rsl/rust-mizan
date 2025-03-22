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

macro_rules! fmt{
    ($trait: ident, $base: expr, $pfx: expr, $blksz: expr) => {
        #[
            doc = " Write out the contents of a `BitSlice` as a numeric format."
        ] #[
            doc = ""
        ] #[
            doc = " These implementations render the bits of memory governed by a"
        ] #[
            doc = " `BitSlice` as one of the three numeric bases the Rust format system"
        ] #[
            doc = " supports:"
        ] #[
            doc = ""
        ] #[
            doc = " - `Binary` renders each bit individually as `0` or `1`,"
        ] #[
            doc = " - `Octal` renders clusters of three bits as the numbers `0` through"
        ] #[
            doc = "   `7`,"
        ] #[
            doc = " - `Hex` renders clusters of four bits as the numbers `[0-9A-F]`."
        ] #[
            doc = ""
        ] #[
            doc = " The formatters produce a word for each `T` element of memory. The"
        ] #[
            doc = " chunked formats (octal and hexadecimal) operate somewhat peculiarly:"
        ] #[
            doc = " they show the semantic value of the memory as interpreted by the"
        ] #[
            doc = " `BitOrder` type parameter’s implementation, and not the raw value of"
        ] #[
            doc = " the memory as you might observe with a debugger."
        ] #[
            doc = ""
        ] #[
            doc = " Specifically, the chunked formats read between zero and three"
        ] #[
            doc = " (octal) or four (hexadecimal) bits in `BitOrder` order out of a"
        ] #[
            doc = " memory element, store those bits in first-high/last-low order, and"
        ] #[
            doc = " then interpret that sequence as a number in their respective bases."
        ] #[
            doc = " This means that, for instance, the byte `3` (bit pattern"
        ] #[
            doc = " `0b0000_0011`), read in `Lsb0` order, will produce the numerals"
        ] #[
            doc = " `\"600\"` (`110 000 00`) in octal, and `\"C0\"` (`1100 0000`) in"
        ] #[
            doc = " hexadecimal."
        ] #[
            doc = ""
        ] #[
            doc = " If the memory element is exhausted before a chunk is filled with"
        ] #[
            doc = " three or four bits, then the number produced will have a lower"
        ] #[
            doc = " value. The byte `0xFFu8` will always produce the octal numeral"
        ] #[
            doc = " `\"773\"` (`111 111 11`)."
        ] #[
            doc = ""
        ] #[
            doc = " The decision to chunk numeral words by memory element, even though"
        ] #[
            doc = " it breaks the octal chunking pattern was made so that the rendered"
        ] #[doc = " text will still show memory boundaries for easier inspection."] impl < O,
        T > $trait for BitSlice < O,
        T > where O: BitOrder,
        T: BitStore,
        {
            fn fmt(&self, fmt: &mut Formatter) -> fmt:: Result {
                let start = if fmt.alternate() {
                    0
                }
                else {
                    2
                };
                let mut dbg = fmt.debug_list();
                let mut w:[
                    u8;
                    (64 / $blksz) + 2
                ] =[
                    b'0';
                    (64 / $blksz) + 2
                ];
                w[1] = $pfx;
                let mut writer =| bits:& BitSlice < O,
                T:: NoAlias >| {
                    let mut end = 2;
                    for(idx, chunk) in bits.chunks($blksz).enumerate() {
                        let mut val = 0u8;
                        for bit in chunk {
                            val <<= 1;
                            val |= *bit as u8;
                        }
                        w[2 + idx] = match val {
                            v @ 0..= 9 => b'0' + v,
                            v @ 10..= 16 => $base +(v - 10),
                            _ => unsafe {
                                unreachable_unchecked()
                            },
                        };
                        end += 1;
                    }
                    dbg.entry(&RenderPart(unsafe {
                        str::from_utf8_unchecked(&w[start .. end])
                    }));
                };
                match self.domain() {
                    Domain::Enclave { head, elem, tail } => {
                        writer(unsafe {
                            Self::from_element(&elem.load().into())[*head as usize .. *tail as usize].noalias()
                        });
                    },
                    Domain::Region { head, body, tail } => {
                        if let Some((h, head)) = head {
                            writer(unsafe {
                                &Self::from_element(&head.load().into())[*h as usize..].noalias()
                            });
                        }
                        for elt in body.iter() {
                            writer(BitSlice::from_element(&elt));
                        }
                        if let Some((tail, t)) = tail {
                            writer(unsafe {
                                &Self::from_element(&tail.load().into())[..*t as usize].noalias()
                            });
                        }
                    },
                }
                dbg.finish()
            }
        }
    };
}

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

fmt![Binary, b'0', b'b', 1];

fmt![Octal, b'0', b'o', 3];

fmt![LowerHex, b'a', b'x', 4];

fmt![UpperHex, b'A', b'x', 4];

#[doc= " Wrapper for inserting pre-rendered text into a formatting stream.\n\nThe numeric formatters write text into a buffer, which a formatter then reads\ndirectly. The formatter only takes `&dyn Debug` objects, so this translates the\ntext buffer into a compatible trait object.\n*"]
struct RenderPart<'a>(&'a str);

impl Debug for RenderPart<'_> {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}
