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

    #[doc= " Translate a semantic bit index into an electrical bit position."]
    #[doc= ""]
    #[doc= " # Parameters"]
    #[doc= ""]
    #[doc= " - `place`: The semantic bit value."]
    #[doc= ""]
    #[doc= " # Returns"]
    #[doc= ""]
    #[doc= " - A concrete position. This value can be used for shifting and masking"]
    #[doc= "   to extract a bit from an element. This must be in the domain `0 .."]
    #[doc= "   T::BITS`."]
    #[doc= ""]
    #[doc= " # Type Parameters"]
    #[doc= ""]
    #[doc= " - `M`: The storage type for which the position will be calculated."]
    #[doc= ""]
    #[doc= " # Invariants"]
    #[doc= ""]
    #[doc= " The function **must** be *total* for the domain `.. M::BITS`. All values"]
    #[doc= " in this domain are valid indices that the library will pass to it, and"]
    #[doc= " which this function must satisfy."]
    #[doc= ""]
    #[doc= " The function **must** be *bijective* over the domain `.. M::BITS`. All"]
    #[doc= " input values in this domain must have one and only one correpsonding"]
    #[doc= " output, which must also be in this domain."]
    #[doc= ""]
    #[doc= " The function *may* support input in the domain `M::BITS ..`. The library"]
    #[doc= " will not produce any values in this domain as input indices. The"]
    #[doc= " function **must not** produce output in the domain `M::BITS ..`. It must"]
    #[doc= " choose between panicking, or producing an output in `.. M::BITS`. The"]
    #[doc= " reduction in domain from `M::BITS ..` to `.. M::BITS` removes the"]
    #[doc= " requirement for inputs in `M::BITS ..` to have unique outputs in"]
    #[doc= " `.. M::BITS`."]
    #[doc= ""]
    #[doc= " This function **must** be *pure*. Calls which have the same input must"]
    #[doc= " produce the same output. This invariant is only required to be upheld"]
    #[doc= " for the lifetime of all data structures which use an implementor. The"]
    #[doc= " behavior of the function *may* be modified after all existing dependent"]
    #[doc= " data structures are destroyed and before any new dependent data"]
    #[doc= " structures are created."]
    #[doc= ""]
    #[doc= " # Non-Invariants"]
    #[doc= ""]
    #[doc= " This function is *not* required to be stateless. It *may* refer to"]
    #[doc= " immutable global state, subject to the purity requirement on lifetimes."]
    #[doc= ""]
    #[doc= " # Safety"]
    #[doc= ""]
    #[doc= " This function requires that the output be in the domain `.. M::BITS`."]
    #[doc= " Implementors must uphold this themselves. Outputs in the domain"]
    #[doc= " `M::BITS ..` will induce panics elsewhere in the library."]
    fn at<M>(place: BitIdx<M>) -> BitPos<M>
    where
        M: BitMemory;

    #[doc= " Translate a semantic bit index into an electrical bit mask."]
    #[doc= ""]
    #[doc= " This is an optional function; a default implementation is provided for"]
    #[doc= " you."]
    #[doc= ""]
    #[doc= " The default implementation of this function calls `Self::at` to produce"]
    #[doc= " an electrical position, then turns that into a bitmask by setting the"]
    #[doc= " `n`th bit more significant than the least significant bit of the"]
    #[doc= " element. `BitOrder` implementations may choose to provide a faster mask"]
    #[doc= " production here, but they must satisfy the invariants listed below."]
    #[doc= ""]
    #[doc= " # Parameters"]
    #[doc= ""]
    #[doc= " - `place`: A semantic bit index into a memory element."]
    #[doc= ""]
    #[doc= " # Returns"]
    #[doc= ""]
    #[doc= " A one-hot encoding of the provided `BitOrder`’s electrical position in"]
    #[doc= " the `M` element."]
    #[doc= ""]
    #[doc= " # Type Parameters"]
    #[doc= ""]
    #[doc= " - `M`: The storage type for which the mask will be calculated. The mask"]
    #[doc= "   must also be this type, as it will be applied to an element of `M` in"]
    #[doc= "   order to set, clear, or test a single bit."]
    #[doc= ""]
    #[doc= " # Invariants"]
    #[doc= ""]
    #[doc= " A one-hot encoding means that there is exactly one bit set in the"]
    #[doc= " produced value. It must be equivalent to `1 << *Self::at(place)`."]
    #[doc= ""]
    #[doc= " As with `at`, this function must produce a unique mapping from each"]
    #[doc= " legal index in the `M` domain to a one-hot value of `M`."]
    #[doc= ""]
    #[doc= " # Safety"]
    #[doc= ""]
    #[doc= " This function requires that the output is always a one-hot value. It is"]
    #[doc= " illegal to produce a value with more than one bit set, and doing so will"]
    #[doc= " cause uncontrolled side effects."]
    fn select<M>(place: BitIdx<M>) -> BitSel<M>
    where
        M: BitMemory {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    #[doc= " Produce a bitmask with each position in the provided range selected."]
    #[doc= ""]
    #[doc= " # Parameters"]
    #[doc= ""]
    #[doc= " - `from`: An optional starting index in the element. If this is `None`,"]
    #[doc= "   then the range begins at zero."]
    #[doc= " - `to`: An optional ending index in the element. If this is `None`, then"]
    #[doc= "   the range ends at `M::BITS`."]
    #[doc= ""]
    #[doc= " # Returns"]
    #[doc= ""]
    #[doc= " A mask with each *position* specified by the input range set high."]
    fn mask<M>(from: impl Into<Option<BitIdx<M>>>, to: impl Into<Option<BitTail<M>>>) -> BitMask<M>
    where
        M: BitMemory {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl BitOrder for Msb0 {
    const TYPENAME: &'static str = "Msb0";

    #[doc= " Maps a semantic count to a concrete position."]
    #[doc= ""]
    #[doc= " `Msb0` order moves from `MSbit` first to `LSbit` last."]
    fn at<M>(place: BitIdx<M>) -> BitPos<M>
    where
        M: BitMemory {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    fn select<M>(place: BitIdx<M>) -> BitSel<M>
    where
        M: BitMemory {
        unsafe {
            BitSel::new_unchecked((M::ONE << M::MASK) >> *place)
        }
    }

    fn mask<M>(from: impl Into<Option<BitIdx<M>>>, to: impl Into<Option<BitTail<M>>>) -> BitMask<M>
    where
        M: BitMemory {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl BitOrder for Lsb0 {
    const TYPENAME: &'static str = "Lsb0";

    #[doc= " Maps a semantic count to a concrete position."]
    #[doc= ""]
    #[doc= " `Lsb0` order moves from `LSbit` first to `MSbit` last."]
    fn at<M>(place: BitIdx<M>) -> BitPos<M>
    where
        M: BitMemory {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    fn select<M>(place: BitIdx<M>) -> BitSel<M>
    where
        M: BitMemory {
        unsafe {
            BitSel::new_unchecked(M::ONE << *place)
        }
    }

    fn mask<M>(from: impl Into<Option<BitIdx<M>>>, to: impl Into<Option<BitTail<M>>>) -> BitMask<M>
    where
        M: BitMemory {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
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

#[cfg(test)]
#[allow(clippy::cognitive_complexity)]
mod tests {
    use super::*;

    #[test]
    fn be_u8_range() {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    #[test]
    fn be_u16_range() {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    #[test]
    fn be_u32_range() {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    #[cfg(target_pointer_width = "64")]
    #[test]
    fn be_u64_range() {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    #[test]
    fn le_u8_range() {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    #[test]
    fn le_u16_range() {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    #[test]
    fn le_u32_range() {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    #[cfg(target_pointer_width = "64")]
    #[test]
    fn le_u64_range() {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    #[test]
    fn lsb0_mask() {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    #[test]
    fn msb0_mask() {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}
