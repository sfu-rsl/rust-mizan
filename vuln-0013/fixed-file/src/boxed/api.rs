#![doc= " Reïmplementation of the `Box<[T]>` API.\n\nThis module tracks the [`alloc::boxed::Box`] module in the version of Rust\nspecified in the `rust-toolchain` file. It is required to provide an exact or\nequivalent API surface matching the `Box<[T]>` type, to the extent that it is\npossible in the language. Where differences occur, they must be documented in a\nsection called `API Differences`.\n\n[`alloc::boxed::Box`]: https://doc.rust-lang.org/alloc/boxed/struct.Boxed.html\n!"]

use crate::{
    boxed::BitBox,
    order::BitOrder,
    pointer::BitPtr,
    slice::BitSlice,
    store::BitStore,
};
use core::{
    marker::{
        PhantomData,
        Unpin,
    },
    pin::Pin,
};

impl<O, T> BitBox<O, T>
where
    O: BitOrder,
    T: BitStore {
    #[doc= " Constructs a bit box from a raw bit pointer."]
    #[doc= ""]
    #[doc= " After calling this function, the raw pointer is owned by the resulting"]
    #[doc= " `BitBox`. Specifically, the `BitBox` destructor will free the allocated"]
    #[doc= " memory. For this to be safe, the memory must have been allocated by"]
    #[doc= " `BitBox` earlier in the program."]
    #[doc= ""]
    #[doc= " # Safety"]
    #[doc= ""]
    #[doc= " This function is unsafe because improper use may lead to memory"]
    #[doc= " problems. For example, a double-free may occurr if the function is"]
    #[doc= " called twice on the same raw pointer."]
    #[doc= ""]
    #[doc= " # Notes"]
    #[doc= ""]
    #[doc= " This function, and `into_raw`, exchange ordinary raw pointers"]
    #[doc= " `*mut BitSlice<O, T>`. Values of these types can be created from, and"]
    #[doc= " converted to, other region pointers such as `*mut [T]` through ordinary"]
    #[doc= " `as`-casting."]
    #[doc= ""]
    #[doc= " This is valid in the Rust type system, but is incorrect at runtime. You"]
    #[doc= " must not, ever, use `as` to cast in either direction to or from a"]
    #[doc= " `BitSlice` pointer."]
    #[doc= ""]
    #[doc= " # Examples"]
    #[doc= ""]
    #[doc= " Recreate a `BitBox` which was previously converted to a raw pointer"]
    #[doc= " using [`BitBox::into_raw`]:"]
    #[doc= ""]
    #[doc= " ```rust"]
    #[doc= " # use bitvec::prelude::*;"]
    #[doc= " let b = BitBox::new(0u8.bits::<Lsb0>());"]
    #[doc= " let ptr = BitBox::into_raw(b);"]
    #[doc= " let b = unsafe { BitBox::<Lsb0, _>::from_raw(ptr) };"]
    #[doc= " ```"]
    #[doc= ""]
    #[doc= " [`BitBox::into_raw`]: #method.into_raw"]
    pub(crate) unsafe fn from_raw(raw: *mut BitSlice<O, T>) -> Self {
        Self {
            _order: PhantomData,
            pointer: BitPtr::from_mut_ptr(raw),
        }
    }
}
