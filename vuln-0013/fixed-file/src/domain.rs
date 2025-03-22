#![doc= " Data Model for Bit Sequence Domains\n\nThe domains governed by `BitSlice` and owned-variant handles have different\nrepresentative states depending on the span of governed elements and live bits.\n\nThis module provides representations of the domain states for ease of use by\nhandle operations.\n!"]

use crate::{
    index::{
        BitIdx,
        BitTail,
    },
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
    iter::FusedIterator,
};

#[doc= " Representations of the raw memory domain for a `BitSlice`.\n\nThis structure is produced by [`BitSlice::domain`], and describes the region of\nmemory the `BitSlice` covers in terms of its raw memory elements, rather than\nits bits.\n\nThe aliased references contained in this structure permit the mutation of memory\nobserved by other immutable `&BitSlice` handles. You are responsible for\nmaintaining memory correctness by not using mutating methods on these\nreferences.\n\n# `[T::Mem]` replacement\n\nAs it is unsafe to produce a reference to raw memory, due to runtime alias\nconditions, this type also functions as a replacement for a slice view of the\nbacking memory. It is capable of iterating over the values of the memory store,\nand implements the [`core::fmt`] traits to render the backing store.\n\n[`BitSlice::domain`]: ../slice/struct.BitSlice.html#method.domain\n[`core::fmt`]: //doc.rust-lang.org/corce/fmt\n*"]
pub(crate) enum Domain<'a, T>
where
    T: 'a + BitStore {
    #[doc= " The source `BitSlice` region is in the interior of one element."]
    Enclave {
        #[doc= " Index, according to the `BitSlice`’s `BitOrder` parameter, at which"]
        #[doc= " the slice begins."]
        head: BitIdx<T::Mem>,
        #[doc= " The memory address of the element containing the `BitSlice`."]
        elem: &'a T::Alias,
        #[doc= " Index, according to the `BitSlice`’s `BitOrder` parameter, at which"]
        #[doc= " the slice ends."]
        tail: BitTail<T::Mem>,
    },
    #[doc= " The source `BitSlice` region touches at least one edge of an element."]
    Region {
        #[doc= " If the first element is partially-filled, its address and starting"]
        #[doc= " bit index according to the `BitOrder` parameter."]
        head: Option<(BitIdx<T::Mem>, &'a T::Alias)>,
        #[doc= " Any fully-spanned elements in the source `BitSlice`."]
        body: &'a [T::NoAlias],
        #[doc= " If the last element is partially-filled, its address and ending bit"]
        #[doc= " index according to the `BitOrder` parameter."]
        tail: Option<(&'a T::Alias, BitTail<T::Mem>)>,
    },
}
