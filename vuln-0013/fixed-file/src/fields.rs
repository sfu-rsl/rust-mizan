#![doc= " Parallel bitfield access.\n\nThis module provides parallel, multiple-bit, access to a `BitSlice`. This\nfunctionality permits the use of `BitSlice` as a library-level implementation of\nthe bitfield language feature found in C and C++.\n\nThe `BitField` trait is not sealed against client implementation, as there is no\nuseful way to automatically use a `BitOrder` implementation to provide a\nuniversal behavior. As such, the trait has some requirements that the compiler\ncannot enforce for client implementations.\n\n# Batch Behavior\n\nThe purpose of this trait is to provide access to arbitrary bit regions as if\nthey were an ordinary memory location. As such, it is important for\nimplementations of this trait to provide shift/mask register transfer behavior\nwhere possible, for as wide a span as possible in each action. Implementations\nof this trait should *not* use bit-by-bit iteration.\n\n# Register Bit Order Preservation\n\nAs a default assumption – user orderings *may* violate this, but *should* not –\neach element of slice memory used to store part of a value should not reorder\nthe value bits. Transfer between slice memory and a CPU register should solely\nbe an ordinary value load or store between memory and the register, and a\nshift/mask operation to select the part of the value that is live.\n\n# Endianness\n\nThe `_le` and `_be` methods of `BitField` refer to the order in which\n`T: BitStore` elements of the slice are assigned significance when containing\nfragments of a stored data value. Within any `T` element, the order of its\nconstituent bytes is *not* governed by the `BitField` trait method.\n\nThe provided `BitOrder` implementors `Lsb0` and `Msb0` use the local machine’s\nbyte ordering. Other cursors *may* implement ordering of bytes within `T`\nelements differently, for instance by calling `.to_be_bytes` before store and\n`from_be_bytes` after load.\n!"]

use crate::{
    mem::BitMemory,
    order::{
        BitOrder,
        Lsb0,
        Msb0,
    },
    slice::BitSlice,
    store::BitStore,
};
#[cfg(feature = "alloc")]
use crate::{
    boxed::BitBox,
    vec::BitVec,
};

#[doc= " Permit a specific `BitSlice` to be used for C-style bitfield access.\n\nOrders that permit batched access to regions of memory are enabled to load data\nfrom a `BitSlice` and store data to a `BitSlice` with faster behavior than the\ndefault bit-by-bit traversal.\n\nThis trait transfers data between a `BitSlice` and an element. The trait\nfunctions always place the live bit region against the least significant bit\nedge of the transfer element (return value for `load`, argument for `store`).\n\nImplementations are encouraged to preserve in-memory bit ordering, so that call\nsites can provide a value pattern that the user can clearly see matches what\nthey expect for memory ordering. These methods merely move data from a fixed\nlocation in an element to a variable location in the slice.\n\nMethods should be called as `bits[start .. end].load_or_store()`, where the\nrange subslice selects up to but no more than the `U::BITS` element width.\n*"]
pub(crate) trait BitField {
    #[doc= " Load the sequence of bits from `self` into the least-significant bits of"]
    #[doc= " an element."]
    #[doc= ""]
    #[doc= " This can load any fundamental type which implements `BitStore`. Other"]
    #[doc= " Rust fundamental types which do not implement it must be recast"]
    #[doc= " appropriately by the user."]
    #[doc= ""]
    #[doc= " The default implementation of this function calls [`load_le`] on"]
    #[doc= " little-endian byte-ordered CPUs, and [`load_be`] on big-endian"]
    #[doc= " byte-ordered CPUs."]
    #[doc= ""]
    #[doc= " # Parameters"]
    #[doc= ""]
    #[doc= " - `&self`: A read reference to some bits in memory. This slice must be"]
    #[doc= "   trimmed to have a width no more than the `U::BITS` width of the type"]
    #[doc= "   being loaded. This can be accomplished with range indexing on a larger"]
    #[doc= "   slice."]
    #[doc= ""]
    #[doc= " # Returns"]
    #[doc= ""]
    #[doc= " A `U` value whose least `self.len()` significant bits are filled with"]
    #[doc= " the bits of `self`."]
    #[doc= ""]
    #[doc= " # Panics"]
    #[doc= ""]
    #[doc= " If `self` is empty, or wider than a single `U` element, this panics."]
    #[doc= ""]
    #[doc= " [`load_be`]: #tymethod.load_be"]
    #[doc= " [`load_le`]: #tymethod.load_le"]
    fn load<U>(&self) -> U
    where
        U: BitMemory {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
    #[doc= " Load from `self`, using little-endian element ordering."]
    #[doc= ""]
    #[doc= " This function interprets a multi-element slice as having its least"]
    #[doc= " significant chunk in the low memory address, and its most significant"]
    #[doc= " chunk in the high memory address. Each element `T` is still interpreted"]
    #[doc= " from individual bytes according to the local CPU ordering."]
    #[doc= ""]
    #[doc= " # Parameters"]
    #[doc= ""]
    #[doc= " - `&self`: A read reference to some bits in memory. This slice must be"]
    #[doc= "   trimmed to have a width no more than the `U::BITS` width of the type"]
    #[doc= "   being loaded. This can be accomplished with range indexing on a larger"]
    #[doc= "   slice."]
    #[doc= ""]
    #[doc= " # Returns"]
    #[doc= ""]
    #[doc= " A `U` value whose least `self.len()` significant bits are filled with"]
    #[doc= " the bits of `self`. If `self` spans multiple `T` elements, then the"]
    #[doc= " lowest-address `T` is interpreted as containing the least significant"]
    #[doc= " bits of the `U` return value, and the highest-address `T` is interpreted"]
    #[doc= " as containing its most significant bits."]
    #[doc= ""]
    #[doc= " # Panics"]
    #[doc= ""]
    #[doc= " If `self` is empty, or wider than a single `U` element, this panics."]
    fn load_le<U>(&self) -> U
    where
        U: BitMemory;
    #[doc= " Load from `self`, using big-endian element ordering."]
    #[doc= ""]
    #[doc= " This function interprets a multi-element slice as having its most"]
    #[doc= " significant chunk in the low memory address, and its least significant"]
    #[doc= " chunk in the high memory address. Each element `T` is still interpreted"]
    #[doc= " from individual bytes according to the local CPU ordering."]
    #[doc= ""]
    #[doc= " # Parameters"]
    #[doc= ""]
    #[doc= " - `&self`: A read reference to some bits in memory. This slice must be"]
    #[doc= "   trimmed to have a width no more than the `U::BITS` width of the type"]
    #[doc= "   being loaded. This can be accomplished with range indexing on a larger"]
    #[doc= "   slice."]
    #[doc= ""]
    #[doc= " # Returns"]
    #[doc= ""]
    #[doc= " A `U` value whose least `self.len()` significant bits are filled with"]
    #[doc= " the bits of `self`. If `self` spans multiple `T` elements, then the"]
    #[doc= " lowest-address `T` is interpreted as containing the most significant"]
    #[doc= " bits of the `U` return value, and the highest-address `T` is interpreted"]
    #[doc= " as containing its least significant bits."]
    fn load_be<U>(&self) -> U
    where
        U: BitMemory;

    #[doc= " Stores a sequence of bits from the user into the domain of `self`."]
    #[doc= ""]
    #[doc= " This can store any fundamental type which implements `BitStore`. Other"]
    #[doc= " Rust fundamental types which do not implement it must be recast"]
    #[doc= " appropriately by the user."]
    #[doc= ""]
    #[doc= " The default implementation of this function calls [`store_le`] on"]
    #[doc= " little-endian byte-ordered CPUs, and [`store_be`] on big-endian"]
    #[doc= " byte-ordered CPUs."]
    #[doc= ""]
    #[doc= " # Parameters"]
    #[doc= ""]
    #[doc= " - `&mut self`: A write reference to some bits in memory. This slice must"]
    #[doc= "   be trimmed to have a width no more than the `U::BITS` width of the"]
    #[doc= "   type being stored. This can be accomplished with range indexing on a"]
    #[doc= "   larger slice."]
    #[doc= " - `value`: A value, whose `self.len()` least significant bits will be"]
    #[doc= "   stored into `self`."]
    #[doc= ""]
    #[doc= " # Behavior"]
    #[doc= ""]
    #[doc= " The `self.len()` least significant bits of `value` are written into the"]
    #[doc= " domain of `self`."]
    #[doc= ""]
    #[doc= " # Panics"]
    #[doc= ""]
    #[doc= " If `self` is empty, or wider than a single `U` element, this panics."]
    #[doc= ""]
    #[doc= " [`store_be`]: #tymethod.store_be"]
    #[doc= " [`store_le`]: #tymethod.store_le"]
    fn store<U>(&mut self, value: U)
    where
        U: BitMemory {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
    #[doc= " Store into `self`, using little-endian element ordering."]
    #[doc= ""]
    #[doc= " This function interprets a multi-element slice as having its least"]
    #[doc= " significant chunk in the low memory address, and its most significant"]
    #[doc= " chunk in the high memory address. Each element `T` is still interpreted"]
    #[doc= " from individual bytes according to the local CPU ordering."]
    #[doc= ""]
    #[doc= " # Parameters"]
    #[doc= ""]
    #[doc= " - `&mut self`: A write reference to some bits in memory. This slice must"]
    #[doc= "   be trimmed to have a width no more than the `U::BITS` width of the"]
    #[doc= "   type being stored. This can be accomplished with range indexing on a"]
    #[doc= "   larger slice."]
    #[doc= " - `value`: A value, whose `self.len()` least significant bits will be"]
    #[doc= "   stored into `self`."]
    #[doc= ""]
    #[doc= " # Behavior"]
    #[doc= ""]
    #[doc= " The `self.len()` least significant bits of `value` are written into the"]
    #[doc= " domain of `self`. If `self` spans multiple `T` elements, then the"]
    #[doc= " lowest-address `T` is interpreted as containing the least significant"]
    #[doc= " bits of the `U` return value, and the highest-address `T` is interpreted"]
    #[doc= " as containing its most significant bits."]
    #[doc= ""]
    #[doc= " # Panics"]
    #[doc= ""]
    #[doc= " If `self` is empty, or wider than a single `U` element, this panics."]
    fn store_le<U>(&mut self, value: U)
    where
        U: BitMemory;
    #[doc= " Store into `self`, using big-endian element ordering."]
    #[doc= ""]
    #[doc= " This function interprets a multi-element slice as having its most"]
    #[doc= " significant chunk in the low memory address, and its least significant"]
    #[doc= " chunk in the high memory address. Each element `T` is still interpreted"]
    #[doc= " from individual bytes according to the local CPU ordering."]
    #[doc= ""]
    #[doc= " # Parameters"]
    #[doc= ""]
    #[doc= " - `&mut self`: A write reference to some bits in memory. This slice must"]
    #[doc= "   be trimmed to have a width no more than the `U::BITS` width of the"]
    #[doc= "   type being stored. This can be accomplished with range indexing on a"]
    #[doc= "   larger slice."]
    #[doc= " - `value`: A value, whose `self.len()` least significant bits will be"]
    #[doc= "   stored into `self`."]
    #[doc= ""]
    #[doc= " # Behavior"]
    #[doc= ""]
    #[doc= " The `self.len()` least significant bits of `value` are written into the"]
    #[doc= " domain of `self`. If `self` spans multiple `T` elements, then the"]
    #[doc= " lowest-address `T` is interpreted as containing the most significant"]
    #[doc= " bits of the `U` return value, and the highest-address `T` is interpreted"]
    #[doc= " as containing its least significant bits."]
    #[doc= ""]
    #[doc= " # Panics"]
    #[doc= ""]
    #[doc= " If `self` is empty, or wider than a single `U` element, this panics."]
    fn store_be<U>(&mut self, value: U)
    where
        U: BitMemory;
}

impl<T> BitField for BitSlice<Lsb0, T>
where
    T: BitStore {
    fn load_le<U>(&self) -> U
    where
        U: BitMemory {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    fn load_be<U>(&self) -> U
    where
        U: BitMemory {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    fn store_le<U>(&mut self, value: U)
    where
        U: BitMemory {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    fn store_be<U>(&mut self, value: U)
    where
        U: BitMemory {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<T> BitField for BitSlice<Msb0, T>
where
    T: BitStore {
    fn load_le<U>(&self) -> U
    where
        U: BitMemory {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    fn load_be<U>(&self) -> U
    where
        U: BitMemory {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    fn store_le<U>(&mut self, value: U)
    where
        U: BitMemory {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    fn store_be<U>(&mut self, value: U)
    where
        U: BitMemory {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

#[cfg(feature = "alloc")]
impl<O, T> BitField for BitBox<O, T>
where
    O: BitOrder,
    T: BitStore,
    BitSlice<O, T>: BitField {
    fn load_le<U>(&self) -> U
    where
        U: BitMemory {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    fn load_be<U>(&self) -> U
    where
        U: BitMemory {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    fn store_le<U>(&mut self, value: U)
    where
        U: BitMemory {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    fn store_be<U>(&mut self, value: U)
    where
        U: BitMemory {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

#[cfg(feature = "alloc")]
impl<O, T> BitField for BitVec<O, T>
where
    O: BitOrder,
    T: BitStore,
    BitSlice<O, T>: BitField {
    fn load_le<U>(&self) -> U
    where
        U: BitMemory {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    fn load_be<U>(&self) -> U
    where
        U: BitMemory {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    fn store_le<U>(&mut self, value: U)
    where
        U: BitMemory {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    fn store_be<U>(&mut self, value: U)
    where
        U: BitMemory {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

#[doc= " Safely computes an LS-edge bitmask for a value of some length.\n\nThe shift operators panic when the shift amount equals or exceeds the type\nwidth, but this module must be able to produce a mask for exactly the type\nwidth. This function correctly handles that case.\n\n# Parameters\n\n- `len`: The width in bits of the value stored in an element `M`.\n\n# Type Parameters\n\n- `M`: The element type for which the mask is computed.\n\n# Returns\n\nAn LS-edge-aligned bitmask of `len` bits. All bits higher than the `len`th are\nzero.\n*"]
#[inline]
fn mask_for<M>(len: usize) -> M
where
    M: BitMemory {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[doc= " Resizes a value from one fundamental type to another.\n\nThis function uses `usize` as the intermediate type (as it is the largest\n`BitStore` implementor on all supported targets), and either zero-extends or\ntruncates the source value to be valid as the destination type. This is\nessentially a generic-aware version of the `as` operator.\n\n# Parameters\n\n- `value`: Any value to be resized.\n\n# Type Parameters\n\n- `T`: The source type of the value to be resized.\n- `U`: The destination type to which the value will be resized.\n\n# Returns\n\nThe result of transforming `value as U`. Where `U` is wider than `T`, this\nzero-extends; where `U` is narrower, it truncates.\n*"]
fn resize<T, U>(value: T) -> U
where
    T: BitMemory,
    U: BitMemory {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[allow(clippy::inconsistent_digit_grouping)]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;

    #[test]
    fn lsb0() {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    #[test]
    fn msb0() {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}
#[cfg(test)]
mod permutation_tests;
