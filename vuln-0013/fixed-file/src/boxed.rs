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
    #[doc= " Constructs an empty boxed bitslice."]
    #[doc= ""]
    #[doc= " # Returns"]
    #[doc= ""]
    #[doc= " An empty `BitBox` at an arbitrary location."]
    #[doc= ""]
    #[doc= " # Examples"]
    #[doc= ""]
    #[doc= " ```rust"]
    #[doc= " use bitvec::prelude::*;"]
    #[doc= ""]
    #[doc= " let bb: BitBox = BitBox::empty();"]
    #[doc= " assert!(bb.is_empty());"]
    #[doc= " ```"]
    pub(crate) fn empty() -> Self {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    #[doc= " Produces a `BitBox` from a single element."]
    #[doc= ""]
    #[doc= " # Parameters"]
    #[doc= ""]
    #[doc= " - `elt`: The source element from which to make the `BitBox`."]
    #[doc= ""]
    #[doc= " # Returns"]
    #[doc= ""]
    #[doc= " A `BitBox` containing the provided element."]
    #[doc= ""]
    #[doc= " # Examples"]
    #[doc= ""]
    #[doc= " ```rust"]
    #[doc= " use bitvec::prelude::*;"]
    #[doc= ""]
    #[doc= " let bb: BitBox<Msb0, u16> = BitBox::from_element(!0);"]
    #[doc= " assert!(bb.all());"]
    #[doc= " ```"]
    pub(crate) fn from_element(elt: T) -> Self {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    #[doc= " Builds a `BitBox` from a borrowed slice of elements."]
    #[doc= ""]
    #[doc= " # Parameters"]
    #[doc= ""]
    #[doc= " - `slice`: The source slice from which to make the `BitBox`."]
    #[doc= ""]
    #[doc= " # Returns"]
    #[doc= ""]
    #[doc= " A `BitBox` containing the (cloned) provided slice."]
    #[doc= ""]
    #[doc= " # Panics"]
    #[doc= ""]
    #[doc= " This function may panic if the provided slice is longer than the"]
    #[doc= " `BitBox` can support."]
    #[doc= ""]
    #[doc= " # Examples"]
    #[doc= ""]
    #[doc= " ```rust"]
    #[doc= " use bitvec::prelude::*;"]
    #[doc= ""]
    #[doc= " let src = [5, 10];"]
    #[doc= " let bb: BitBox<Msb0, u8> = BitBox::from_slice(&src[..]);"]
    #[doc= " assert!(bb[5]);"]
    #[doc= " assert!(bb[7]);"]
    #[doc= " assert!(bb[12]);"]
    #[doc= " assert!(bb[14]);"]
    #[doc= " ```"]
    pub(crate) fn from_slice(slice: &[T]) -> Self {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    #[doc= " Clones a `&BitSlice` into a `BitBox`."]
    #[doc= ""]
    #[doc= " # Parameters"]
    #[doc= ""]
    #[doc= " - `slice`: The bit slice to clone into a bit box."]
    #[doc= ""]
    #[doc= " # Returns"]
    #[doc= ""]
    #[doc= " A `BitBox` containing the same bits as the source slice."]
    #[doc= ""]
    #[doc= " # Examples"]
    #[doc= ""]
    #[doc= " ```rust"]
    #[doc= " use bitvec::prelude::*;"]
    #[doc= ""]
    #[doc= " let src = [0u8, !0];"]
    #[doc= " let bb = BitBox::<Msb0, _>::from_bitslice(src.bits());"]
    #[doc= " assert_eq!(bb.len(), 16);"]
    #[doc= " assert!(bb.some());"]
    #[doc= " ```"]
    pub(crate) fn from_bitslice(slice: &BitSlice<O, T>) -> Self {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    #[doc= " Produces a `BitBox` from an owned slice of elements."]
    #[doc= ""]
    #[doc= " # Parameters"]
    #[doc= ""]
    #[doc= " - `slice`: The source boxed slice from which to make the `BitBox`."]
    #[doc= ""]
    #[doc= " # Returns"]
    #[doc= ""]
    #[doc= " A `BitBox` governing the same slice that was passed in. This function"]
    #[doc= " does not reallocate."]
    #[doc= ""]
    #[doc= " # Panics"]
    #[doc= ""]
    #[doc= " This function may panic if the provided slice is longer than the"]
    #[doc= " `BitBox` can support."]
    #[doc= ""]
    #[doc= " # Examples"]
    #[doc= ""]
    #[doc= " ```rust"]
    #[doc= " use bitvec::prelude::*;"]
    #[doc= ""]
    #[doc= " let slice: Box<[u16]> = vec![0, !0].into_boxed_slice();"]
    #[doc= " let bb = BitBox::<Lsb0, _>::from_boxed_slice(slice);"]
    #[doc= " assert!(bb.some());"]
    #[doc= " assert_eq!(bb.len(), 32);"]
    #[doc= " ```"]
    pub(crate) fn from_boxed_slice(boxed: Box<[T]>) -> Self {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    #[doc= " Removes the `BitBox` wrapper from a `Box<[T]>`."]
    #[doc= ""]
    #[doc= " # Parameters"]
    #[doc= ""]
    #[doc= " - `self`"]
    #[doc= ""]
    #[doc= " # Returns"]
    #[doc= ""]
    #[doc= " The `Box<[T]>` underneath `self`."]
    #[doc= ""]
    #[doc= " # Examples"]
    #[doc= ""]
    #[doc= " ```rust"]
    #[doc= " use bitvec::prelude::*;"]
    #[doc= ""]
    #[doc= " let slice: Box<[u16]> = vec![0, !0].into_boxed_slice();"]
    #[doc= " let bb = BitBox::<Lsb0, _>::from_boxed_slice(slice);"]
    #[doc= " assert_eq!(bb.len(), 32);"]
    #[doc= " let slice = bb.into_boxed_slice();"]
    #[doc= " assert_eq!(slice.len(), 2);"]
    #[doc= " ```"]
    pub(crate) fn into_boxed_slice(self) -> Box<[T]> {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    #[doc= " Changes the order on a box handle, without changing the data it"]
    #[doc= " governs."]
    #[doc= ""]
    #[doc= " # Parameters"]
    #[doc= ""]
    #[doc= " - `self`"]
    #[doc= ""]
    #[doc= " # Returns"]
    #[doc= ""]
    #[doc= " An equivalent handle to the same data, with a new order parameter."]
    pub(crate) fn change_order<P>(self) -> BitBox<P, T>
    where
        P: BitOrder {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    #[doc= " Accesses the `BitSlice<O, T>` to which the `BitBox` refers."]
    #[doc= ""]
    #[doc= " # Parameters"]
    #[doc= ""]
    #[doc= " - `&self`"]
    #[doc= ""]
    #[doc= " # Returns"]
    #[doc= ""]
    #[doc= " The slice of bits behind the box."]
    pub(crate) fn as_bitslice(&self) -> &BitSlice<O, T> {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    #[doc= " Accesses the `BitSlice<O, T>` to which the `BitBox` refers."]
    #[doc= ""]
    #[doc= " # Parameters"]
    #[doc= ""]
    #[doc= " - `&mut self`"]
    #[doc= ""]
    #[doc= " # Returns"]
    #[doc= ""]
    #[doc= " The slice of bits behind the box."]
    pub(crate) fn as_mut_bitslice(&mut self) -> &mut BitSlice<O, T> {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    #[doc= " Accesses the vector’s backing store as an element slice."]
    #[doc= ""]
    #[doc= " Unlike `BitSlice`’s method of the same name, this includes the partial"]
    #[doc= " edges, as `BitBox` forbids fragmentation that leads to contention."]
    #[doc= ""]
    #[doc= " # Parameters"]
    #[doc= ""]
    #[doc= " - `&self`"]
    #[doc= ""]
    #[doc= " # Returns"]
    #[doc= ""]
    #[doc= " The slice of all live elements in the backing storage, including the"]
    #[doc= " partial edges if present."]
    pub(crate) fn as_slice(&self) -> &[T] {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    #[doc= " Accesses the vector’s backing store as an element slice."]
    #[doc= ""]
    #[doc= " Unlike `BitSlice`’s method of the same name, this includes the partial"]
    #[doc= " edges, as `BitBox` forbids fragmentation that leads to contention."]
    #[doc= ""]
    #[doc= " # Parameters"]
    #[doc= ""]
    #[doc= " - `&mut self`"]
    #[doc= ""]
    #[doc= " # Returns"]
    #[doc= ""]
    #[doc= " The slice of all live elements in the backing storage, including the"]
    #[doc= " partial edges if present."]
    pub(crate) fn as_mut_slice(&mut self) -> &mut [T] {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

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

    #[doc= " Allows a function to access the `Box<[T]>` that the `BitBox` is using"]
    #[doc= " under the hood."]
    #[doc= ""]
    #[doc= " # Parameters"]
    #[doc= ""]
    #[doc= " - `&self`"]
    #[doc= " - `func`: A function which works with a borrowed `Box<[T]>` representing"]
    #[doc= "   the actual memory held by the `BitBox`."]
    #[doc= ""]
    #[doc= " # Type Parameters"]
    #[doc= ""]
    #[doc= " - `F: FnOnce(&Box<[T]>) -> R`: A function which borrows a box."]
    #[doc= " - `R`: The return value of the function."]
    #[doc= ""]
    #[doc= " # Returns"]
    #[doc= ""]
    #[doc= " The return value of the provided function."]
    fn do_with_box<F, R>(&self, func: F) -> R
    where
        F: FnOnce(&Box<[T::Mem]>) -> R {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}
mod api;
mod iter;
mod ops;
mod traits;
