#![doc= " Memory management.\n\nThe `BitStore` trait defines the types that can be used in `bitvec` data\nstructures, and describes how those data structures are allowed to access the\nmemory they govern.\n!"]

use crate::{
    access::BitAccess,
    mem::BitMemory,
};
use core::cell::Cell;
#[cfg(feature = "atomic")]
use core::sync::atomic;

#[doc= " Generalize over types which may be used to access memory holding bits.\n\nThis trait is implemented on the fundamental integers, their `Cell<>` wrappers,\nand (if present) their `Atomic` variants. Users provide this type as a parameter\nto their data structures in order to inform the structure of how it may access\nmemory.\n\nSpecifically, this has the advantage that a `BitSlice<_, Cell<_>>` knows that it\nhas a view of memory that will not undergo concurrent modification. As such, it\ncan skip using atomic accesses, and just use ordinary load/store instructions,\nwithout fear of causing observable race conditions.\n\nThe associated types `Mem` and `Alias` allow implementors to know the register\nwidth of the memory they describe (`Mem`) and to change the aliasing status of\na slice.\n\nA universal property of `BitSlice` regions is that for any handle, it may be\ndescribed as a triad of:\n\n- zero or one partially-used, aliased, elements at the head\n- zero or more wholly-used, unaliased, elements in the body\n- zero or one partially-used, aliased, elements at the tail\n\nAs such, a `&BitSlice` reference with any aliasing type can be split into its\n`Self::Alias` variant for the edges, and `Cell<Self::Mem>` for the interior,\nwithout violating memory safety.\n*"]
pub trait BitStore: seal::Sealed + Sized {
    #[doc= " The fundamental integer type of the governed memory."]
    type Mem: BitMemory + Into<Self>;
    #[doc= " The type used for performing memory accesses."]
    type Access: BitAccess<Self::Mem> + BitStore;
    #[doc= " The destination type when marking a region as known-aliased."]
    type Alias: BitStore + BitAccess<Self::Mem>;
    #[doc= " The destination type when marking a region as known-unaliased."]
    type NoAlias: BitStore;
    #[doc= " Mark whether a type is threadsafe when viewed as bits."]
    #[doc= ""]
    #[doc= " This is necessary because `Cell<T: Send>` is `Send`, but `Cell` is *not*"]
    #[doc= " synchronized and thus cannot be used for aliasing, parallel, bit"]
    #[doc= " manipulation."]
    #[doc(hidden)]
    type Threadsafe;

    #[doc= " Gets the memory element behind this reference, mediated through"]
    #[doc= " `Self::Access`."]
    #[doc= ""]
    #[doc= " # Parameters"]
    #[doc= ""]
    #[doc= " - `&self`"]
    #[doc= ""]
    #[doc= " # Returns"]
    #[doc= ""]
    #[doc= " The current value of the referent element."]
    fn get_elem(&self) -> Self::Mem {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    #[doc= " Sets the memory element behind this reference, mediated through"]
    #[doc= " `Self::Access`."]
    #[doc= ""]
    #[doc= " # Parameters"]
    #[doc= ""]
    #[doc= " - `&mut self`: Even when aliased, you must have exclusive control of the"]
    #[doc= "   referent element to set it to a new value."]
    #[doc= " - `value`: The new value to write into the referent element."]
    fn set_elem(&mut self, value: Self::Mem) {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

#[doc= " Batch implementation of `BitStore` for appropriate types."]
macro_rules! bitstore{
    ($($t: ty => $a: ty), * $(,) ?) => {
        $(impl seal:: Sealed for $t {
        } impl BitStore for $t {
            #[doc= " The unsigned integers are only the `BitStore` parameter for"]
            #[doc= " unaliased slices."]
            type Access = Cell<Self>;
            #[
                doc = " Aliases are required to use atomic access, as `BitSlice`s of"
            ] #[doc = " this type are safe to move across threads."] #[cfg(feature = "atomic")] type Alias = $a;
            #[doc= " Aliases are permitted to use `Cell` wrappers and ordinary"]
            #[doc= " access, as `BitSlice`s of this type are forbidden from crossing"]
            #[doc= " threads."]
            #[cfg(not(feature = "atomic"))]
            type Alias = Cell<Self>;
            type Mem = Self;
            type NoAlias = Self;
            #[doc(hidden)]
            type Threadsafe = Self;
        } #[cfg(feature = "atomic")] impl seal:: Sealed for $a {
        } #[cfg(feature = "atomic")] impl BitStore for $a {
            #[doc= " Atomic stores always use atomic accesses."]
            type Access = Self;
            type Alias = Self;
            type Mem = $t;
            type NoAlias = $t;
            #[doc(hidden)]
            type Threadsafe = Self;
        } impl seal:: Sealed for Cell < $t > {
        } impl BitStore for Cell < $t > {
            #[doc= " `Cell`s always use ordinary, unsynchronized, accesses, as the"]
            #[doc= " type system forbids them from creating memory collisions."]
            type Access = Self;
            type Alias = Self;
            type Mem = $t;
            type NoAlias = Self;
            #[doc= " Raw pointers are never threadsafe, so this prevents"]
            #[doc= " `BitSlice<_, Cell<_>>` from crossing threads."]
            #[doc(hidden)]
            type Threadsafe = *const Self;
        }) *
    };
}

bitstore!(
    u8 => atomic:: AtomicU8,
    u16 => atomic:: AtomicU16,
    u32 => atomic:: AtomicU32,
    usize => atomic:: AtomicUsize,
);

#[cfg(target_pointer_width = "64")]
bitstore!(u64 => atomic:: AtomicU64);

#[cfg(not(any(target_pointer_width = "32", target_pointer_width = "64")))]
compile_fail!(
    concat!("This architecture is currently not supported. File an issue at ", env!("CARGO_PKG_REPOSITORY"))
);

#[doc= " Enclose the `Sealed` trait against client use."]
mod seal {
    #[doc= " Marker trait to seal `BitStore` against downstream implementation."]
    #[doc= ""]
    #[doc= " This trait is public in the module, so that other modules in the crate"]
    #[doc= " can use it, but so long as it is not exported by the crate root and this"]
    #[doc= " module is private, this trait effectively forbids downstream"]
    #[doc= " implementation of the `BitStore` trait."]
    #[doc(hidden)]
    pub trait Sealed { }
}

#[cfg(target_endian = "disabled")]
#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use core::cell::Cell;
    use static_assertions::*;

    #[test]
    fn traits() {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}
