#![doc= " `BitBox` structure\n\nThis module holds the type for an owned but ungrowable bit sequence. `BitVec` is\nthe more appropriate and useful type for most collections.\n!"]
#![cfg(feature = "alloc")]

use crate::{
    order::{
        BitOrder,
        Local,
    },
    pointer::BitPtr,
    slice::BitSlice,
    store::BitStore,
};
use alloc::boxed::Box;
use core::marker::PhantomData;

#[doc= " A pointer type for owned bit sequences.\n\nThis type is essentially a `&BitSlice` that owns its own memory. It can change\nthe contents of its domain, but it cannot change its own domain like `BitVec`\ncan. It is useful for fixed-size collections without lifetime tracking.\n\n# Type Parameters\n\n- `O: BitOrder`: An implementor of the [`BitOrder`] trait. This type is used to\n  convert semantic indices into concrete bit positions in elements, and store or\n  retrieve bit values from the storage type.\n- `T: BitStore`: An implementor of the [`BitStore`] trait: `u8`, `u16`, `u32`,\n  or `u64` (64-bit systems only). This is the actual type in memory that the box\n  will use to store data.\n\n# Safety\n\nThe `BitBox` handle has the same *size* as standard Rust `Box<[T]>` handles, but\nit is ***extremely binary incompatible*** with them. Attempting to treat\n`BitBox<_, T>` as `Box<[T]>` in any manner except through the provided APIs is\n***catastrophically*** unsafe and unsound.\n\n# Trait Implementations\n\n`BitBox<O, T>` implements all the traits that `BitSlice<O, T>` does, by\ndeferring to the `BitSlice` implementation. It also implements conversion traits\nto and from `BitSlice`, and to/from `BitVec`.\n*"]
#[repr(C)]
pub struct BitBox<O = Local, T = usize>
where
    O: BitOrder,
    T: BitStore {
    _order: PhantomData<O>,
    pointer: BitPtr<T>,
}

impl<O, T> BitBox<O, T>
where
    O: BitOrder,
    T: BitStore {
    #[doc= " Gives read access to the `BitPtr<T>` structure powering the box."]
    #[doc= ""]
    #[doc= " # Parameters"]
    #[doc= ""]
    #[doc= " - `&self`"]
    #[doc= ""]
    #[doc= " # Returns"]
    #[doc= ""]
    #[doc= " A copy of the interior `BitPtr<T>`."]
    pub(crate) fn bitptr(&self) -> BitPtr<T> {
        self.pointer
    }
}
mod api;
mod traits;
