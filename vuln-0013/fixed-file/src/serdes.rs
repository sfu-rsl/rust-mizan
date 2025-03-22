#![doc= " `serde`-powered de/serialization\n\nThis module implements the Serde traits for the `bitvec` types, as possible.\n\nWithout an allocator, only `BitSlice` exists, and can only implement\n`Serialize`. With an allocator, the `BitBox` and `BitVec` types exist, and are\nable to implement `Deserialize` as well.\n!"]
#![cfg(all(feature = "serde"))]

use crate::{
    domain::Domain,
    mem::BitMemory,
    order::BitOrder,
    slice::BitSlice,
    store::BitStore,
};
#[cfg(feature = "alloc")]
use crate::{
    boxed::BitBox,
    pointer::BitPtr,
    vec::BitVec,
};
#[cfg(feature = "alloc")]
use core::{
    cmp,
    convert::TryInto,
    fmt::{
        self,
        Formatter,
    },
    marker::PhantomData,
    mem,
};
use serde::{
    ser::{
        SerializeSeq,
        SerializeStruct,
        Serializer,
    },
    Serialize,
};
#[cfg(feature = "alloc")]
use serde::{
    de::{
        self,
        Deserializer,
        Error,
        MapAccess,
        SeqAccess,
        Unexpected,
        Visitor,
    },
    Deserialize,
};

#[doc= " A Serde visitor to pull `BitBox` data out of a serialized stream"]
#[cfg(feature = "alloc")]
#[derive(Clone, Copy, Default, Debug)]
pub(crate) struct BitBoxVisitor<'de, O, T>
where
    O: BitOrder,
    T: BitStore + Deserialize<'de> {}

#[cfg(feature = "alloc")]
impl<'de, O, T> BitBoxVisitor<'de, O, T>
where
    O: BitOrder,
    T: BitStore + Deserialize<'de> {
    fn new() -> Self {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

#[cfg(feature = "alloc")]
impl<'de, O, T> Visitor<'de> for BitBoxVisitor<'de, O, T>
where
    O: BitOrder,
    T: BitStore + Deserialize<'de> {
    type Value = BitBox<O, T>;

    fn expecting(&self, fmt: &mut Formatter) -> fmt::Result {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    #[doc= " Visit a sequence of anonymous data elements. These must be in the order"]
    #[doc= " `usize', `u8`, `u8`, `[T]`."]
    fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
    where
        V: SeqAccess<'de> {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    #[doc= " Visit a map of named data elements. These may be in any order, and must"]
    #[doc= " be the pairs `head: u8`, `bits: usize`, and `data: [T]`."]
    fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
    where
        V: MapAccess<'de> {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

#[cfg(feature = "alloc")]
impl<'de, O, T> Deserialize<'de> for BitBox<O, T>
where
    O: BitOrder,
    T: 'de + BitStore + Deserialize<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de> {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

#[cfg(feature = "alloc")]
impl<'de, O, T> Deserialize<'de> for BitVec<O, T>
where
    O: BitOrder,
    T: 'de + BitStore + Deserialize<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de> {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<O, T> Serialize for BitSlice<O, T>
where
    O: BitOrder,
    T: BitStore,
    T::Mem: Serialize {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<T> Serialize for Domain<'_, T>
where
    T: BitStore,
    T::Mem: Serialize {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

#[cfg(feature = "alloc")]
impl<O, T> Serialize for BitBox<O, T>
where
    O: BitOrder,
    T: BitStore + Serialize,
    T::Mem: Serialize {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

#[cfg(feature = "alloc")]
impl<O, T> Serialize for BitVec<O, T>
where
    O: BitOrder,
    T: BitStore + Serialize,
    T::Mem: Serialize {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    #[cfg(feature = "alloc")]
    use serde_test::assert_de_tokens;
    use serde_test::{
        assert_ser_tokens,
        Token,
    };

    macro_rules! bvtok{
        (s $elts: expr, $head: expr, $bits: expr, $ty: ident $(, $data: expr) *) => {
            &[
                Token::Struct {
                    name: "BitSet",
                    len: 3,
                },
                Token::Str("head"),
                Token:: U8($head),
                Token::Str("bits"),
                Token:: U64($bits),
                Token::Str("data"),
                Token:: Seq {
                    len: Some($elts)
                },
                $(Token:: $ty($data),) * Token:: SeqEnd,
                Token::StructEnd,
            ]
        };
        (d $elts: expr, $head: expr, $bits: expr, $ty: ident $(, $data: expr) *) => {
            &[
                Token::Struct {
                    name: "BitSet",
                    len: 3,
                },
                Token::BorrowedStr("head"),
                Token:: U8($head),
                Token::BorrowedStr("bits"),
                Token:: U64($bits),
                Token::BorrowedStr("data"),
                Token:: Seq {
                    len: Some($elts)
                },
                $(Token:: $ty($data),) * Token:: SeqEnd,
                Token::StructEnd,
            ]
        };
    }

    #[test]
    fn empty() {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    #[test]
    fn small() {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn wide() {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn deser() {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}
