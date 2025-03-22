#![doc= " Governs access to underlying memory.\n\n`bitvec` allows the logical capability of a program to produce aliased\nreferences to memory elements which may have contended mutation. While `bitvec`\noperations are guaranteed to not observe mutation outside the logical borders of\ntheir domains, the production of aliased mutating access to memory is still\nundefined behavior in the compiler.\n\nAs such, the crate must never produce references, either shared or unique,\nreferences to memory as the bare fundamental types. Instead, this module\ntranslates references to `BitSlice` into references to shared-mutable types as\nappropriate for the crate build configuration: either `Cell` in non-atomic\nbuilds, or `AtomicT` in atomic builds.\n!"]

use crate::{
    index::{
        BitIdx,
        BitMask,
    },
    mem::BitMemory,
    order::BitOrder,
};
use core::{
    fmt::Debug,
    sync::atomic::Ordering,
};
use radium::Radium;

#[doc= " Access interface for shared/mutable memory access.\n\n`&BitSlice` and `&mut BitSlice` contexts must route through their `Access`\nassociated type, which implements this trait, in order to perform *any* access\nto underlying memory. This trait extends the `Radium` element-wise shared\nmutable access with single-bit operations suited for use by `BitSlice`.\n*"]
pub trait BitAccess<M>: Debug + Radium<M> + Sized
where
    M: BitMemory {
    #[doc= " Set a single bit in an element low."]
    #[doc= ""]
    #[doc= " `BitAccess::set` calls this when its `value` is `false`; it"]
    #[doc= " unconditionally writes a `0` bit into the electrical position that"]
    #[doc= " `place` controls according to the `BitOrder` parameter `O`."]
    #[doc= ""]
    #[doc= " # Type Parameters"]
    #[doc= ""]
    #[doc= " - `O`: A `BitOrder` implementation which translates `place` into a"]
    #[doc= "   usable bit-mask."]
    #[doc= ""]
    #[doc= " # Parameters"]
    #[doc= ""]
    #[doc= " - `&self`: A shared reference to underlying memory."]
    #[doc= " - `place`: A semantic bit index in the `self` element."]
    fn clear_bit<O>(&self, place: BitIdx<M>)
    where
        O: BitOrder {
        self.fetch_and(!*O::select(place), Ordering::Relaxed);
    }

    #[doc= " Writes the low bits of the mask into the underlying element."]
    #[doc= ""]
    #[doc= " # Parameters"]
    #[doc= ""]
    #[doc= " - `&self`"]
    #[doc= " - `mask`: Any value. The **high** bits of the mask will erase their"]
    #[doc= "   corresponding bits in `*self`; the **low** bits will preserve their"]
    #[doc= "   value."]
    fn clear_bits(&self, mask: BitMask<M>) {
        self.fetch_and(!*mask, Ordering::Relaxed);
    }

    #[doc= " Set a single bit in an element high."]
    #[doc= ""]
    #[doc= " `BitAccess::set` calls this when its `value` is `true`; it"]
    #[doc= " unconditionally writes a `1` bit into the electrical position that"]
    #[doc= " `place` controls according to the `BitOrder` parameter `O`."]
    #[doc= ""]
    #[doc= " # Type Parameters"]
    #[doc= ""]
    #[doc= " - `O`: A `BitOrder` implementation which translates `place` into a"]
    #[doc= "   usable bit-mask."]
    #[doc= ""]
    #[doc= " # Parameters"]
    #[doc= ""]
    #[doc= " - `&self`: A shared reference to underlying memory."]
    #[doc= " - `place`: A semantic bit index in the `self` element."]
    fn set_bit<O>(&self, place: BitIdx<M>)
    where
        O: BitOrder {
        self.fetch_or(*O::select(place), Ordering::Relaxed);
    }

    #[doc= " Writes the high bits of the mask into the underlying element."]
    #[doc= ""]
    #[doc= " # Parameters"]
    #[doc= ""]
    #[doc= " - `&self`"]
    #[doc= " - `mask`: Any value. The high bits of the mask will be written into"]
    #[doc= "   `*self`; the low bits will preserve their value in `*self`."]
    fn set_bits(&self, mask: BitMask<M>) {
        self.fetch_or(*mask, Ordering::Relaxed);
    }

    #[doc= " Invert a single bit in an element."]
    #[doc= ""]
    #[doc= " # Type Parameters"]
    #[doc= ""]
    #[doc= " - `O`: A `BitOrder` implementation which translates `place` into a"]
    #[doc= "   usable bit-mask."]
    #[doc= ""]
    #[doc= " # Parameters"]
    #[doc= ""]
    #[doc= " - `&self`: A shared reference to underlying memory."]
    #[doc= " - `place`: A semantic bit index in the `self` element."]
    fn invert_bit<O>(&self, place: BitIdx<M>)
    where
        O: BitOrder {
        self.fetch_xor(*O::select(place), Ordering::Relaxed);
    }

    #[doc= " Inverts the bits in an element specified by a mask."]
    #[doc= ""]
    #[doc= " # Parameters"]
    #[doc= ""]
    #[doc= " - `&self`"]
    #[doc= " - `mask`: Any value. The high bits of the mask will invert their"]
    #[doc= "   corresponding bits of `*self`; the low bits will have no effect."]
    fn invert_bits(&self, mask: BitMask<M>) {
        self.fetch_xor(*mask, Ordering::Relaxed);
    }

    #[doc= " Retrieve a single bit from an element."]
    #[doc= ""]
    #[doc= " # Type Parameters"]
    #[doc= ""]
    #[doc= " - `O`: A `BitOrder` implementation which translates `place` into a"]
    #[doc= "   usable bit-mask."]
    #[doc= ""]
    #[doc= " # Parameters"]
    #[doc= ""]
    #[doc= " - `&self`: A shared reference to underlying memory."]
    #[doc= " - `place`: A semantic bit index in the `self` element."]
    #[inline]
    fn get<O>(&self, place: BitIdx<M>) -> bool
    where
        O: BitOrder {
        BitAccess::load(self) & *O::select(place) != M::ZERO
    }

    #[doc= " Set a single bit in an element to some value."]
    #[doc= ""]
    #[doc= " # Type Parameters"]
    #[doc= ""]
    #[doc= " - `O`: A `BitOrder` implementation which translates `place` into a"]
    #[doc= "   usable bit-mask."]
    #[doc= ""]
    #[doc= " # Parameters"]
    #[doc= ""]
    #[doc= " - `&self`: A shared reference to underlying memory."]
    #[doc= " - `place`: A semantic bit index in the `self` element."]
    #[doc= " - `value`: The value to which the bit controlled by `place` shall be"]
    #[doc= "   set."]
    #[inline]
    fn set<O>(&self, place: BitIdx<M>, value: bool)
    where
        O: BitOrder {
        if value {
            self.set_bit::<O>(place);
        } else {
            self.clear_bit::<O>(place);
        }
    }

    #[doc= " Read a value out of a contended memory element and into a local scope."]
    #[doc= ""]
    #[doc= " # Parameters"]
    #[doc= ""]
    #[doc= " - `&self`: A shared reference to underlying memory."]
    #[doc= ""]
    #[doc= " # Returns"]
    #[doc= ""]
    #[doc= " The value of `*self`. This value is only useful when access is"]
    #[doc= " uncontended by multiple `BitSlice` regions."]
    fn load(&self) -> M {
        Radium::load(self, Ordering::Relaxed)
    }

    #[doc= " Stores a value into a contended memory element."]
    #[doc= ""]
    #[doc= " # Parameters"]
    #[doc= ""]
    #[doc= " - `&self`: A shared reference to underlying memory."]
    #[doc= " - `value`: The new value to write into `*self`."]
    fn store(&self, value: M) {
        Radium::store(self, value, Ordering::Relaxed)
    }
}

impl<M, R> BitAccess<M> for R
where
    M: BitMemory,
    R: Debug + Radium<M> { }
