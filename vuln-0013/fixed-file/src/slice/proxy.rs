#![doc= " Heavy bit reference.\n\nRegrettably, while producing a read reference to a bit inside a `BitSlice` is\nrelatively easy to do, Rust’s rules make it impossible to produce a write\nreference to one. This is because references must be addresses that the holder\ncan derefence without type consideration. Read references inspect the `BitSlice`\ndata sequence, and then produce references to static `true` and `false` values\nas appropriate; the returned address does not need to be actually within the\nreferent memory region.\n\nA write reference, however, is required to be the address of a `bool` within the\n`BitSlice`, which can have `0u8` or `1u8` written into it. This rule makes\nproduction of any `&mut bool` from any `&mut BitSlice` impossible. Instead, the\n`BitMut` structure serves as a heavy-weight referential object, that cannot be\nused in the `&mut` write reference system, as a good-enough substitute.\n!"]

use crate::{
    index::BitIdx,
    order::BitOrder,
    slice::BitSlice,
    store::BitStore,
};
use core::{
    marker::PhantomData,
    ops::{
        Deref,
        DerefMut,
    },
    ptr::NonNull,
};

#[doc= " Proxy referential type, equivalent to `&mut bool`.\n\nThis structure is three words wide, and cannot ever fit into the existing Rust\nlanguage and library infrastructure in the way `&BitSlice` does. While `&mut`\nwrite references are themselves an affine type, with a guaranteed single point\nof destruction and no duplication, the language forbids writing finalization\nlogic for them.\n\nThis means that a custom reference type which implements `Deref` and `DerefMut`\nto a location within the canonical handle, and on `Drop` writes the `Deref`\nlocation into referent memory, is impossible. Short of that, a C++-style thick\nreference-like type is as close as Rust will allow.\n*"]
pub struct BitMut<'a, O, T>
where
    O: BitOrder,
    T: 'a + BitStore {
    #[doc= " Inform the compiler that this has an exclusive borrow of a `BitSlice`"]
    pub(super) _parent: PhantomData<&'a mut BitSlice<O, T>>,
    #[doc= " Typed pointer to the memory element containing the proxied bit."]
    pub(super) data: NonNull<T::Access>,
    #[doc= " Index of the proxied bit inside the targeted memory element."]
    pub(super) head: BitIdx<T::Mem>,
    #[doc= " A local cache for `Deref` usage."]
    pub(super) bit: bool,
}

impl<O, T> Deref for BitMut<'_, O, T>
where
    O: BitOrder,
    T: BitStore {
    type Target = bool;

    fn deref(&self) -> &Self::Target {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<O, T> DerefMut for BitMut<'_, O, T>
where
    O: BitOrder,
    T: BitStore {
    fn deref_mut(&mut self) -> &mut Self::Target {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<O, T> Drop for BitMut<'_, O, T>
where
    O: BitOrder,
    T: BitStore {
    fn drop(&mut self) {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}
