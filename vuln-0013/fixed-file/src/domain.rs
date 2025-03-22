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

#[doc= " Representations of the state of the bit domain in its containing elements.\n\n`BitSlice` regions can be described in terms of maybe-aliased and\nknown-unaliased sub-regions. This type produces correctly-marked subslices of a\nsource slice, according to information contained in its pointer.\n\n# Lifetimes\n\n- `'a`: Lifetime of the containing storage\n\n# Type Parameters\n\n- `O`: The ordering type of the parent `BitSlice`.\n- `T`: The storage type of the parent `BitSlice`.\n*"]
#[derive(Debug)]
pub(crate) enum BitDomain<'a, O, T>
where
    O: BitOrder,
    T: 'a + BitStore {
    #[doc= " A `BitSlice` region contained entirely within the interior of one"]
    #[doc= " element."]
    Enclave {
        #[doc= " The index at which the slice region begins."]
        head: BitIdx<T::Mem>,
        #[doc= " The original slice, marked as aliased."]
        body: &'a BitSlice<O, T::Alias>,
        #[doc= " The index at which the slice region ends."]
        tail: BitTail<T::Mem>,
    },
    #[doc= " A `BitSlice` region that touches at least one element edge."]
    Region {
        #[doc= " The subslice that partially-fills the lowest element in the region."]
        head: &'a BitSlice<O, T::Alias>,
        #[doc= " The subslice that wholly-fills elements, precluding any other handle"]
        #[doc= " from aliasing them."]
        body: &'a BitSlice<O, T::NoAlias>,
        #[doc= " The subslice that partially-fills the highest element in the region."]
        tail: &'a BitSlice<O, T::Alias>,
    },
}

impl<'a, O, T> BitDomain<'a, O, T>
where
    O: BitOrder,
    T: 'a + BitStore {
    #[doc= " Constructs a domain over an empty slice."]
    #[doc= ""]
    #[doc= " # Returns"]
    #[doc= ""]
    #[doc= " A `BitDomain::Region` with all subslices set to the empty slice."]
    fn empty() -> Self {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    #[doc= " Constructs a domain with partial elements on both edges."]
    #[doc= ""]
    #[doc= " # Parameters"]
    #[doc= ""]
    #[doc= " - `head`: The element index at which the slice begins."]
    #[doc= " - `slice`: The original `BitSlice` being split."]
    #[doc= " - `tail`: The element index at which the slice ends."]
    #[doc= ""]
    #[doc= " # Returns"]
    #[doc= ""]
    #[doc= " A `BitDomain::Region` with its `head` section set to the live bits in"]
    #[doc= " the low element, its `body` section set to the live bits in the"]
    #[doc= " wholly-filled interior elements, and its `tail` section set to the live"]
    #[doc= " bits in the high element."]
    fn major(head: BitIdx<T::Mem>, slice: &BitSlice<O, T>, tail: BitTail<T::Mem>) -> Self {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    #[doc= " Constructs a domain wholly within a single element."]
    #[doc= ""]
    #[doc= " # Parameters"]
    #[doc= ""]
    #[doc= " - `head`: The element index at which the slice begins."]
    #[doc= " - `slice`: The source slice."]
    #[doc= " - `tail`: The element index at which the slice ends."]
    #[doc= ""]
    #[doc= " # Returns"]
    #[doc= ""]
    #[doc= " A `BitDomain::Enclave` that marks the source slice as aliased, and"]
    #[doc= " carries the `head` and `tail` indices for mask construction."]
    fn minor(head: BitIdx<T::Mem>, slice: &BitSlice<O, T>, tail: BitTail<T::Mem>) -> Self {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    fn partial_head(head: BitIdx<T::Mem>, slice: &BitSlice<O, T>) -> Self {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    fn partial_tail(slice: &BitSlice<O, T>, tail: BitTail<T::Mem>) -> Self {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    fn spanning(slice: &'a BitSlice<O, T>) -> Self {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<'a, O, T> From<&'a BitSlice<O, T>> for BitDomain<'a, O, T>
where
    O: BitOrder,
    T: 'a + BitStore {
    fn from(this: &'a BitSlice<O, T>) -> Self {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

#[doc= " Writable version of [`BitDomain`].\n\n[`BitDomain`]: enum.BitDomain.html\n*"]
pub(crate) enum BitDomainMut<'a, O, T>
where
    O: BitOrder,
    T: 'a + BitStore {
    #[doc= " Writable version of [`BitDomain::Enclave`]."]
    #[doc= ""]
    #[doc= " [`BitDomain::Enclave`]: enum.BitDomain.html#variant.Enclave"]
    Enclave {
        #[doc= " See `BitDomain::Enclave`."]
        head: BitIdx<T::Mem>,
        #[doc= " See `BitDomain::Enclave`."]
        body: &'a mut BitSlice<O, T::Alias>,
        #[doc= " See `BitDomain::Enclave`."]
        tail: BitTail<T::Mem>,
    },
    #[doc= " Writable version of [`BitDomain::Region`]."]
    #[doc= ""]
    #[doc= " [`BitDomain::Region`]: enum.BitDomain.html#variant.Region"]
    Region {
        #[doc= " See `BitDomain::Region`."]
        head: &'a mut BitSlice<O, T::Alias>,
        #[doc= " See `BitDomain::Region`."]
        body: &'a mut BitSlice<O, T::NoAlias>,
        #[doc= " See `BitDomain::Region`."]
        tail: &'a mut BitSlice<O, T::Alias>,
    },
}

impl<'a, O, T> From<&'a mut BitSlice<O, T>> for BitDomainMut<'a, O, T>
where
    O: BitOrder,
    T: 'a + BitStore {
    fn from(this: &'a mut BitSlice<O, T>) -> Self {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

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

impl<'a, T> Domain<'a, T>
where
    T: 'a + BitStore {
    #[doc= " Produces an iterator over each memory value referenced by the domain."]
    #[doc= ""]
    #[doc= " This iterator will perform the appropriate load on each reference"]
    #[doc= " element, yielding the value of the referenced memory."]
    #[doc= ""]
    #[doc= " # Parameters"]
    #[doc= ""]
    #[doc= " - `&self`"]
    #[doc= ""]
    #[doc= " # Returns"]
    #[doc= ""]
    #[doc= " An iterator yielding each referent value."]
    pub(crate) fn iter(&self) -> DomainIter<'a, T> {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    pub(crate) fn is_spanning(&self) -> bool {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    fn empty() -> Self {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    fn major(head: BitIdx<T::Mem>, elts: &'a [T::Alias], tail: BitTail<T::Mem>) -> Self {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    fn minor(head: BitIdx<T::Mem>, elts: &'a [T::Alias], tail: BitTail<T::Mem>) -> Self {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    fn partial_head(head: BitIdx<T::Mem>, elts: &'a [T::Alias]) -> Self {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    fn partial_tail(elts: &'a [T::Alias], tail: BitTail<T::Mem>) -> Self {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    fn spanning(elts: &[T::Alias]) -> Self {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<T> Clone for Domain<'_, T>
where
    T: BitStore {
    fn clone(&self) -> Self {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<'a, O, T> From<&'a BitSlice<O, T>> for Domain<'a, T>
where
    O: BitOrder,
    T: 'a + BitStore {
    fn from(this: &'a BitSlice<O, T>) -> Self {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<T> Default for Domain<'_, T>
where
    T: BitStore {
    fn default() -> Self {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<T> Binary for Domain<'_, T>
where
    T: BitStore,
    T::Mem: Binary {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<T> Debug for Domain<'_, T>
where
    T: BitStore,
    T::Mem: Debug {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<T> LowerHex for Domain<'_, T>
where
    T: BitStore,
    T::Mem: LowerHex {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<T> Octal for Domain<'_, T>
where
    T: BitStore,
    T::Mem: Octal {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<T> UpperHex for Domain<'_, T>
where
    T: BitStore,
    T::Mem: UpperHex {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<'a, T> IntoIterator for Domain<'a, T>
where
    T: 'a + BitStore {
    type IntoIter = DomainIter<'a, T>;
    type Item = <Self::IntoIter as Iterator>::Item;

    fn into_iter(self) -> Self::IntoIter {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<T> Copy for Domain<'_, T>
where
    T: BitStore { }

#[doc= " Writable version of [`Domain`].\n\n[`Domain`]: enum.Domain.html\n*"]
pub(crate) enum DomainMut<'a, T>
where
    T: 'a + BitStore {
    #[doc= " Writable version of [`Domain::Enclave`]."]
    #[doc= ""]
    #[doc= " [`Domain::Enclave`]: enum.Domain.html#variant.Enclave"]
    Enclave {
        #[doc= " See `Domain::Enclave`."]
        head: BitIdx<T::Mem>,
        #[doc= " See `Domain::Enclave`."]
        elem: &'a T::Alias,
        #[doc= " See `Domain::Enclave`."]
        tail: BitTail<T::Mem>,
    },
    #[doc= " Writable version of [`Domain::Region`]."]
    #[doc= ""]
    #[doc= " [`Domain::Region`]: enum.Domain.html#variant.Region"]
    Region {
        #[doc= " See `Domain::Region`."]
        head: Option<(BitIdx<T::Mem>, &'a T::Alias)>,
        #[doc= " See `Domain::Region`."]
        body: &'a mut [T::NoAlias],
        #[doc= " See `Domain::Region`."]
        tail: Option<(&'a T::Alias, BitTail<T::Mem>)>,
    },
}

impl<'a, O, T> From<&'a mut BitSlice<O, T>> for DomainMut<'a, T>
where
    O: BitOrder,
    T: 'a + BitStore {
    fn from(this: &'a mut BitSlice<O, T>) -> Self {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

#[doc= " Element iterator for a `Domain`."]
#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct DomainIter<'a, T>
where
    T: 'a + BitStore {
    domain: Domain<'a, T>,
}

impl<'a, T> Iterator for DomainIter<'a, T>
where
    T: 'a + BitStore {
    type Item = T::Mem;

    fn next(&mut self) -> Option<Self::Item> {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<T> ExactSizeIterator for DomainIter<'_, T>
where
    T: BitStore {
    fn len(&self) -> usize {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<T> FusedIterator for DomainIter<'_, T>
where
    T: BitStore { }

impl<'a, T> DoubleEndedIterator for DomainIter<'a, T>
where
    T: 'a + BitStore {
    fn next_back(&mut self) -> Option<Self::Item> {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}
