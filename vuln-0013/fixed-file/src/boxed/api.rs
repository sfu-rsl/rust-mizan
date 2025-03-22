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
    #[doc= " Allocates memory on the heap and then places `bits` into it."]
    #[doc= ""]
    #[doc= " # API Differences"]
    #[doc= ""]
    #[doc= " [`Box::new`] takes a `T` by direct value, and is not implemented as a"]
    #[doc= " means of cloning slices. As `BitSlice` cannot be held by value, this"]
    #[doc= " function clones the referent slice region into a new fixed-size heap"]
    #[doc= " buffer."]
    #[doc= ""]
    #[doc= " # Examples"]
    #[doc= ""]
    #[doc= " ```rust"]
    #[doc= " # use bitvec::prelude::*;"]
    #[doc= " let boxed = BitBox::new(0u8.bits::<Lsb0>());"]
    #[doc= " ```"]
    pub(crate) fn new(bits: &BitSlice<O, T>) -> Self {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    #[doc= " Constructs a new `Pin<BitBox<O, T>>`."]
    #[doc= ""]
    #[doc= " `BitSlice` is always `Unpin`, so this has no actual immobility effect."]
    pub(crate) fn pin(bits: &BitSlice<O, T>) -> Pin<Self>
    where
        O: Unpin,
        T: Unpin {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

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

    #[doc= " Consumes the `BitBox`, returning a wrapped raw pointer."]
    #[doc= ""]
    #[doc= " The pointer will be properly aligned and non-null."]
    #[doc= ""]
    #[doc= " After calling this function, the caller is responsible for the memory"]
    #[doc= " previously managed by the `BitBox`. In particular, the caller should"]
    #[doc= " properly release the memory by converting the pointer back into a"]
    #[doc= " `BitBox` with the [`BitBox::from_raw`] function, allowing the `BitBox`"]
    #[doc= " destructor to perform the cleanup."]
    #[doc= ""]
    #[doc= " Note: this is an associated function, which means that you have to call"]
    #[doc= " it as `BitBox::into_raw(b)` instead of `b.into_raw()`. This is to match"]
    #[doc= " layout with the standard library’s `Box` API; there will never be a name"]
    #[doc= " conflict with `BitSlice`."]
    #[doc= ""]
    #[doc= " # Notes"]
    #[doc= ""]
    #[doc= " As with `::from_raw`, the pointer returned by this function must not"]
    #[doc= " ever have its type or value changed or inspected in any way. It may only"]
    #[doc= " be held and then passed into `::from_raw` in the future."]
    #[doc= ""]
    #[doc= " # Examples"]
    #[doc= ""]
    #[doc= " Converting the raw pointer back into a `BitBox` with"]
    #[doc= " [`BitBox::from_raw`] for automatic cleanup:"]
    #[doc= ""]
    #[doc= " ```rust"]
    #[doc= " # use bitvec::prelude::*;"]
    #[doc= " let b = BitBox::new(0u64.bits::<Msb0>());"]
    #[doc= " let ptr = BitBox::into_raw(b);"]
    #[doc= " let b = unsafe { BitBox::<Msb0, _>::from_raw(ptr) };"]
    #[doc= " ```"]
    #[doc= ""]
    #[doc= " [`BitBox::from_raw`]: #method.from_raw"]
    pub(crate) fn into_raw(b: Self) -> *mut BitSlice<O, T> {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    #[doc= " Consumes and leaks the `BitBox`, returning a mutable reference,"]
    #[doc= " `&'a mut BitSlice<O, T>`. Note that the memory region `[T]` must outlive"]
    #[doc= " the chosen lifetime `'a`."]
    #[doc= ""]
    #[doc= " This function is mainly useful for bit regions that live for the"]
    #[doc= " remainder of the program’s life. Dropping the returned reference will"]
    #[doc= " cause a memory leak. If this is not acceptable, the reference should"]
    #[doc= " first be wrapped with the [`BitBox::from_raw`] function, producing a"]
    #[doc= " `BitBox`. This `BitBox` can then be dropped which will properly"]
    #[doc= " deallocate the memory."]
    #[doc= ""]
    #[doc= " Note: this is an associated function, which means that you have to call"]
    #[doc= " it as `BitBox::leak(b)` instead of `b.leak()`. This is to match layout"]
    #[doc= " with the standard library’s `Box` API; there will never be a name"]
    #[doc= " conflict with `BitSlice`."]
    #[doc= ""]
    #[doc= " # Examples"]
    #[doc= ""]
    #[doc= " Simple usage:"]
    #[doc= ""]
    #[doc= " ```rust"]
    #[doc= " # use bitvec::prelude::*;"]
    #[doc= " let b = BitBox::new(0u64.bits::<Local>());"]
    #[doc= " let static_ref: &'static mut BitSlice<Local, u64> = BitBox::leak(b);"]
    #[doc= " static_ref.set(0, true);"]
    #[doc= " assert_eq!(static_ref.count_ones(), 1);"]
    #[doc= " ```"]
    #[doc= ""]
    #[doc= " [`BitBox::from_raw`]: #method.from_raw"]
    pub(crate) fn leak<'a>(self) -> &'a mut BitSlice<O, T> {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}
