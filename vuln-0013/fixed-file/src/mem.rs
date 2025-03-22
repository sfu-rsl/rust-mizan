#![doc= " Memory description.\n\nThis module defines the properties of bare chunks of memory. It deals only with\nregister-type widths, and has no information on the means by which addressed\nmemory is accessed.\n!"]

use crate::{
    index::BitIdx,
    order::BitOrder,
    store::BitStore,
};
use core::mem;
use funty::IsUnsigned;
use radium::marker::BitOps;

#[doc= " Describes properties of register types.\n\nThis trait describes raw memory, without any access modifiers. It provides\ninformation about the width of a memory element and useful constants.\n*"]
pub trait BitMemory: IsUnsigned + BitOps {
    #[doc= " The width, in bits, of the memory element."]
    const BITS: u8 = mem::size_of::<Self>() as u8 * 8;
    #[doc= " The number of bits required to hold a bit index into the element."]
    const INDX: u8 = Self::BITS.trailing_zeros() as u8;
    #[doc= " The maximum value of a bit index for the element."]
    const MASK: u8 = Self::BITS - 1;
    #[doc= " The element value with only the least significant bit high."]
    const ONE: Self;
    #[doc= " The element value with all bits high."]
    const ALL: Self;
    #[doc= " The element’s name."]
    const TYPENAME: &'static str;

    #[doc= " Gets a specific bit in an element."]
    #[doc= ""]
    #[doc= " # Safety"]
    #[doc= ""]
    #[doc= " This method cannot be called from within an `&BitSlice` context; it may"]
    #[doc= " only be called by construction of an `&Self` reference from a `Self`"]
    #[doc= " element directly."]
    #[doc= ""]
    #[doc= " # Parameters"]
    #[doc= ""]
    #[doc= " - `&self`"]
    #[doc= " - `place`: A bit index in the element. The bit under this index, as"]
    #[doc= "   governed by the `O` `BitOrder`, will be retrieved as a `bool`."]
    #[doc= ""]
    #[doc= " # Returns"]
    #[doc= ""]
    #[doc= " The value of the bit under `place`."]
    #[doc= ""]
    #[doc= " # Type Parameters"]
    #[doc= ""]
    #[doc= " - `O`: A `BitOrder` implementation to translate the index into a"]
    #[doc= "   position."]
    fn get<O>(&self, place: BitIdx<Self>) -> bool
    where
        O: BitOrder {
        *self & *O::select(place) != Self::ZERO
    }

    #[doc= " Sets a specific bit in an element to a given value."]
    #[doc= ""]
    #[doc= " # Safety"]
    #[doc= ""]
    #[doc= " This method cannot be called from within an `&mut BitSlice` context; it"]
    #[doc= " may only be called by construction of an `&mut Self` reference from a"]
    #[doc= " `Self` element directly."]
    #[doc= ""]
    #[doc= " # Parameters"]
    #[doc= ""]
    #[doc= " - `place`: A bit index in the element. The bit under this index, as"]
    #[doc= "   governed by the `O` `BitOrder`, will be set according to `value`."]
    #[doc= ""]
    #[doc= " # Type Parameters"]
    #[doc= ""]
    #[doc= " - `O`: A `BitOrder` implementation to translate the index into a"]
    #[doc= "   position."]
    fn set<O>(&mut self, place: BitIdx<Self>, value: bool)
    where
        O: BitOrder {
        let sel = *O::select(place);
        if value {
            *self |= sel;
        } else {
            *self &= !sel;
        }
    }

    #[doc= " Computes the number of elements of `Self` required to hold some bits."]
    #[doc= ""]
    #[doc= " # Parameters"]
    #[doc= ""]
    #[doc= " - `bits`: The number of bits to store in an array of `[Self]`."]
    #[doc= ""]
    #[doc= " # Returns"]
    #[doc= ""]
    #[doc= " The number of elements of `Self` required to hold the requested bits."]
    fn elts(bits: usize) -> usize {
        crate::mem::elts::<Self>(bits)
    }

    #[doc(hidden)]
    fn retype<T>(self) -> T::Mem
    where
        T: BitStore {
        unsafe {
            *(&self as *const _ as *const _)
        }
    }
}

#[doc= " Computes the number of elements required to store a number of bits.\n\n# Parameters\n\n- `bits`: The number of bits to store in an element `T` array.\n\n# Returns\n\nThe number of elements `T` required to store `bits`.\n\nBecause this is a const function, when `bits` is a const-expr, this function can\nbe used in array types `[T; elts(len)]`.\n*"]
#[doc(hidden)]
pub(crate) const fn elts<T>(bits: usize) -> usize {
    let width = mem::size_of::<T>() * 8;
    bits / width + (bits % width != 0) as usize
}

macro_rules! memory{
    ($($t: ty), * $(,) ?) => {
        $(impl BitMemory for $t {
            const ONE: Self = 1;
            const ALL: Self = !0;
            const TYPENAME: &'static str = stringify!($t);
        }) *
    };
}

memory!(u8, u16, u32, usize);

#[cfg(target_pointer_width = "64")]
memory!(u64);
