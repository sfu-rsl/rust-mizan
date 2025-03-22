#![doc= " Bit Ordering\n\n`bitvec` structures are parametric over any ordering of bits within an element.\nThe `BitOrder` trait maps a cursor position (indicated by the `BitIdx` type) to an\nelectrical position (indicated by the `BitPos` type) within that element, and\nalso defines the order of traversal over an element.\n\nThe only requirement on implementors of `BitOrder` is that the transform function\nfrom cursor (`BitIdx`) to position (`BitPos`) is *total* (every integer in the\ndomain `0 .. T::BITS` is used) and *unique* (each cursor maps to one and only\none position, and each position is mapped by one and only one cursor).\nContiguity is not required.\n\n`BitOrder` is a stateless trait, and implementors should be zero-sized types.\n!"]

use crate::{
    index::{
        BitIdx,
        BitMask,
        BitPos,
        BitSel,
        BitTail,
    },
    mem::BitMemory,
};

#[doc= " Traverses an element from `MSbit` to `LSbit`."]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Msb0;
#[doc= " Traverses an element from `LSbit` to `MSbit`."]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Lsb0;

#[doc= " An ordering over an element.\n\n# Usage\n\n`bitvec` structures store and operate on semantic counts, not bit positions. The\n`BitOrder::at` function takes a semantic ordering, `BitIdx`, and produces an\nelectrical position, `BitPos`.\n*"]
pub trait BitOrder {
    #[doc= " Name of the ordering type, for use in text display."]
    const TYPENAME: &'static str;

    /// Translate a semantic bit index into an electrical bit mask.
    ///
    /// This is an optional function; a default implementation is provided for
    /// you.
    ///
    /// The default implementation of this function calls `Self::at` to produce
    /// an electrical position, then turns that into a bitmask by setting the
    /// `n`th bit more significant than the least significant bit of the
    /// element. `BitOrder` implementations may choose to provide a faster mask
    /// production here, but they must satisfy the invariants listed below.
    ///
    /// # Parameters
    ///
    /// - `place`: A semantic bit index into a memory element.
    ///
    /// # Returns
    ///
    /// A one-hot encoding of the provided `BitOrder`’s electrical position in
    /// the `M` element.
    ///
    /// # Type Parameters
    ///
    /// - `M`: The storage type for which the mask will be calculated. The mask
    ///   must also be this type, as it will be applied to an element of `M` in
    ///   order to set, clear, or test a single bit.
    ///
    /// # Invariants
    ///
    /// A one-hot encoding means that there is exactly one bit set in the
    /// produced value. It must be equivalent to `1 << *Self::at(place)`.
    ///
    /// As with `at`, this function must produce a unique mapping from each
    /// legal index in the `M` domain to a one-hot value of `M`.
    ///
    /// # Safety
    ///
    /// This function requires that the output is always a one-hot value. It is
    /// illegal to produce a value with more than one bit set, and doing so will
    /// cause uncontrolled side effects.
    fn select<M>(place: BitIdx<M>) -> BitSel<M>
    where M: BitMemory;
}

impl BitOrder for Msb0 {
    const TYPENAME: &'static str = "Msb0";

    fn select<M>(place: BitIdx<M>) -> BitSel<M>
    where
        M: BitMemory {
        unsafe {
            BitSel::new_unchecked((M::ONE << M::MASK) >> *place)
        }
    }
}

impl BitOrder for Lsb0 {
    const TYPENAME: &'static str = "Lsb0";


    fn select<M>(place: BitIdx<M>) -> BitSel<M>
    where
        M: BitMemory {
        unsafe {
            BitSel::new_unchecked(M::ONE << *place)
        }
    }
}

#[doc= " A default bit ordering.\n\nThe target has big-endian byte ordering, so the default bit ordering is set to\nbig-endian as well, as a convenience. These two orderings are not related.\n*"]
#[cfg(target_endian = "big")]
pub(crate) type Local = Msb0;
#[doc= " A default bit ordering.\n\nThe target has little-endian byte ordering, so the default bit ordering is set\nto little-endian as well, as a convenience. These two orderings are not related.\n*"]
#[cfg(target_endian = "little")]
pub type Local = Lsb0;

#[cfg(not(any(target_endian = "big", target_endian = "little")))]
compile_fail!(concat!("This architecture is currently not supported. File an issue at ", env!(CARGO_PKG_REPOSITORY)));
