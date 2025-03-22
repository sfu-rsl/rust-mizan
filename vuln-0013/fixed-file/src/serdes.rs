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
