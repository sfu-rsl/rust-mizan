#![doc= " `BitSlice` Wide Reference\n\nThis module defines semantic operations on `[u1]`, in contrast to the mechanical\noperations defined in `BitPtr`.\n\nThe `&BitSlice` handle has the same size and general layout as the standard Rust\nslice handle `&[T]`. Its binary layout is wholly incompatible with the layout of\nRust slices, and must never be interchanged except through the provided APIs.\n!"]

use crate::{
    access::BitAccess,
    domain::{
        BitDomain,
        BitDomainMut,
        Domain,
        DomainMut,
    },
    index::Indexable,
    mem::BitMemory,
    order::{
        BitOrder,
        Local,
    },
    pointer::BitPtr,
    store::BitStore,
};
use core::marker::PhantomData;
use funty::IsInteger;

#[doc= " A compact slice of bits, whose order and storage types can be customized.\n\n`BitSlice` is a specialized slice type, which can only ever be held by\nreference or specialized owning pointers provided by this crate. The value\npatterns of its handles are opaque binary structures, which cannot be\nmeaningfully inspected by user code.\n\n`BitSlice` can only be dynamically allocated by this library. Creation of any\nother `BitSlice` collections will result in severely incorrect behavior.\n\nA `BitSlice` reference can be created through the [`bitvec!`] macro, from a\n[`BitVec`] collection, or from most common Rust types (fundamentals, slices of\nthem, and small arrays) using the [`Bits`] and [`BitsMut`] traits.\n\n`BitSlice`s are a view into a block of memory at bit-level resolution. They are\nrepresented by a crate-internal pointer structure that ***cannot*** be used with\nother Rust code except through the provided conversion APIs.\n\n```rust\nuse bitvec::prelude::*;\n\n# #[cfg(feature = \"alloc\")] {\nlet bv = bitvec![0, 1, 0, 1];\n//  slicing a bitvec\nlet bslice: &BitSlice = &bv[..];\n# }\n\n//  coercing an array to a bitslice\nlet bslice: &BitSlice<_, _> = [1u8, 254u8].bits::<Msb0>();\n```\n\nBit slices are either mutable or shared. The shared slice type is\n`&BitSlice<O, T>`, while the mutable slice type is `&mut BitSlice<O, T>`. For\nexample, you can mutate bits in the memory to which a mutable `BitSlice` points:\n\n```rust\nuse bitvec::prelude::*;\n\nlet mut base = [0u8, 0, 0, 0];\nlet bs: &mut BitSlice<_, _> = base.bits_mut::<Msb0>();\nbs.set(13, true);\neprintln!(\"{:?}\", bs.as_slice());\nassert!(bs[13]);\nassert_eq!(base[1], 4);\n```\n\n# Type Parameters\n\n- `O`: An implementor of the `BitOrder` trait. This type is used to convert\n  semantic indices into concrete bit positions in elements, and store or\n  retrieve bit values from the storage type.\n- `T`: An implementor of the `BitStore` trait: `u8`, `u16`, `u32`, or `u64`\n  (64-bit systems only). This is the actual type in memory that the slice will\n  use to store data.\n\n# Safety\n\nThe `&BitSlice` reference handle has the same *size* as standard Rust slice\nhandles, but it is ***extremely value-incompatible*** with them. Attempting to\ntreat `&BitSlice<_, T>` as `&[T]` in any manner except through the provided APIs\nis ***catastrophically*** unsafe and unsound.\n\n[`BitVec`]: ../vec/struct.BitVec.html\n[`Bits`]: ../bits/trait.Bits.html\n[`BitsMut`]: ../bits/trait.BitsMut.html\n[`From`]: https://doc.rust-lang.org/stable/std/convert/trait.From.html\n[`bitvec!`]: ../macro.bitvec.html\n*"]
#[repr(transparent)]
pub struct BitSlice<O = Local, T = usize>
where
    O: BitOrder,
    T: BitStore {
    #[doc= " BitOrder type for selecting bits inside an element."]
    _kind: PhantomData<O>,
    #[doc= " Element type of the slice."]
    #[doc= ""]
    #[doc= " eddyb recommends using `PhantomData<T>` and `[()]` instead of `[T]`"]
    #[doc= " alone."]
    _type: PhantomData<T>,
    #[doc= " Slice of elements `T` over which the `BitSlice` has usage."]
    _elts: [()],
}

impl<O, T> BitSlice<O, T>
where
    O: BitOrder,
    T: BitStore {
    #[doc= " Produces the empty slice. This is equivalent to `&[]` for Rust slices."]
    #[doc= ""]
    #[doc= " # Returns"]
    #[doc= ""]
    #[doc= " An empty `&BitSlice` handle."]
    #[doc= ""]
    #[doc= " # Examples"]
    #[doc= ""]
    #[doc= " ```rust"]
    #[doc= " use bitvec::prelude::*;"]
    #[doc= ""]
    #[doc= " let bits: &BitSlice = BitSlice::empty();"]
    #[doc= " ```"]
    #[inline]
    pub(crate) fn empty<'a>() -> &'a Self {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    #[doc= " Produces the empty mutable slice. This is equivalent to `&mut []` for"]
    #[doc= " Rust slices."]
    #[doc= ""]
    #[doc= " # Returns"]
    #[doc= ""]
    #[doc= " An empty `&mut BitSlice` handle."]
    #[doc= ""]
    #[doc= " # Examples"]
    #[doc= ""]
    #[doc= " ```rust"]
    #[doc= " use bitvec::prelude::*;"]
    #[doc= ""]
    #[doc= " let bits: &mut BitSlice = BitSlice::empty_mut();"]
    #[doc= " ```"]
    #[inline]
    pub(crate) fn empty_mut<'a>() -> &'a mut Self {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    #[doc= " Produces an immutable `BitSlice` over a single element."]
    #[doc= ""]
    #[doc= " # Parameters"]
    #[doc= ""]
    #[doc= " - `elt`: A reference to an element over which the `BitSlice` will be"]
    #[doc= "   created."]
    #[doc= ""]
    #[doc= " # Returns"]
    #[doc= ""]
    #[doc= " A `BitSlice` over the provided element."]
    #[doc= ""]
    #[doc= " # Examples"]
    #[doc= ""]
    #[doc= " ```rust"]
    #[doc= " use bitvec::prelude::*;"]
    #[doc= ""]
    #[doc= " let elt: u8 = !0;"]
    #[doc= " let bs: &BitSlice<Local, _> = BitSlice::from_element(&elt);"]
    #[doc= " assert!(bs.all());"]
    #[doc= " ```"]
    #[inline]
    pub(crate) fn from_element(elt: &T) -> &Self {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    #[doc= " Produces a mutable `BitSlice` over a single element."]
    #[doc= ""]
    #[doc= " # Parameters"]
    #[doc= ""]
    #[doc= " - `elt`: A reference to an element over which the `BitSlice` will be"]
    #[doc= "   created."]
    #[doc= ""]
    #[doc= " # Returns"]
    #[doc= ""]
    #[doc= " A `BitSlice` over the provided element."]
    #[doc= ""]
    #[doc= " # Examples"]
    #[doc= ""]
    #[doc= " ```rust"]
    #[doc= " use bitvec::prelude::*;"]
    #[doc= ""]
    #[doc= " let mut elt: u8 = !0;"]
    #[doc= " let bs: &mut BitSlice<Local, _> = BitSlice::from_element_mut(&mut elt);"]
    #[doc= " bs.set(0, false);"]
    #[doc= " assert!(!bs.all());"]
    #[doc= " ```"]
    #[inline]
    pub(crate) fn from_element_mut(elt: &mut T) -> &mut Self {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    #[doc= " Wraps a `&[T: BitStore]` in a `&BitSlice<O: BitOrder, T>`. The order"]
    #[doc= " must be specified at the call site. The element type cannot be changed."]
    #[doc= ""]
    #[doc= " # Parameters"]
    #[doc= ""]
    #[doc= " - `src`: The elements over which the new `BitSlice` will operate."]
    #[doc= ""]
    #[doc= " # Returns"]
    #[doc= ""]
    #[doc= " A `BitSlice` representing the original element slice."]
    #[doc= ""]
    #[doc= " # Panics"]
    #[doc= ""]
    #[doc= " The source slice must not exceed the maximum number of elements that a"]
    #[doc= " `BitSlice` can contain. This value is documented in [`BitPtr`]."]
    #[doc= ""]
    #[doc= " # Examples"]
    #[doc= ""]
    #[doc= " ```rust"]
    #[doc= " use bitvec::prelude::*;"]
    #[doc= ""]
    #[doc= " let src = [1, 2, 3];"]
    #[doc= " let bits = BitSlice::<Msb0, u8>::from_slice(&src[..]);"]
    #[doc= " assert_eq!(bits.len(), 24);"]
    #[doc= " assert_eq!(bits.as_slice().len(), 3);"]
    #[doc= " assert!(bits[7]); // src[0] == 0b0000_0001"]
    #[doc= " assert!(bits[14]); // src[1] == 0b0000_0010"]
    #[doc= " assert!(bits[22]); // src[2] == 0b0000_0011"]
    #[doc= " assert!(bits[23]);"]
    #[doc= " ```"]
    #[doc= ""]
    #[doc= " [`BitPtr`]: ../pointer/struct.BitPtr.html"]
    pub(crate) fn from_slice(slice: &[T]) -> &Self {
        let len = slice.len();
        assert!(len <= BitPtr::<T>::MAX_ELTS, "BitSlice cannot address {} elements", len);
        let bits = len.checked_mul(T::Mem::BITS as usize).expect("Bit length out of range");
        BitPtr::new(slice.as_ptr(), 0u8.idx(), bits).into_bitslice()
    }

    #[doc= " Wraps a `&mut [T: BitStore]` in a `&mut BitSlice<O: BitOrder, T>`. The"]
    #[doc= " order must be specified by the call site. The element type cannot"]
    #[doc= " be changed."]
    #[doc= ""]
    #[doc= " # Parameters"]
    #[doc= ""]
    #[doc= " - `src`: The elements over which the new `BitSlice` will operate."]
    #[doc= ""]
    #[doc= " # Returns"]
    #[doc= ""]
    #[doc= " A `BitSlice` representing the original element slice."]
    #[doc= ""]
    #[doc= " # Panics"]
    #[doc= ""]
    #[doc= " The source slice must not exceed the maximum number of elements that a"]
    #[doc= " `BitSlice` can contain. This value is documented in [`BitPtr`]."]
    #[doc= ""]
    #[doc= " # Examples"]
    #[doc= ""]
    #[doc= " ```rust"]
    #[doc= " use bitvec::prelude::*;"]
    #[doc= ""]
    #[doc= " let mut src = [1, 2, 3];"]
    #[doc= " let bits = BitSlice::<Lsb0, u8>::from_slice_mut(&mut src[..]);"]
    #[doc= " //  The first bit is the LSb of the first element."]
    #[doc= " assert!(bits[0]);"]
    #[doc= " bits.set(0, false);"]
    #[doc= " assert!(!bits[0]);"]
    #[doc= " assert_eq!(bits.as_slice(), &[0, 2, 3]);"]
    #[doc= " ```"]
    #[doc= ""]
    #[doc= " [`BitPtr`]: ../pointer/struct.BitPtr.html"]
    #[inline]
    pub(crate) fn from_slice_mut(slice: &mut [T]) -> &mut Self {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    #[doc= " Sets the bit value at the given position."]
    #[doc= ""]
    #[doc= " # Parameters"]
    #[doc= ""]
    #[doc= " - `&mut self`"]
    #[doc= " - `index`: The bit index to set. It must be in the domain `0 .."]
    #[doc= "   self.len()`."]
    #[doc= " - `value`: The value to be set, `true` for `1` and `false` for `0`."]
    #[doc= ""]
    #[doc= " # Panics"]
    #[doc= ""]
    #[doc= " This method panics if `index` is outside the slice domain."]
    #[doc= ""]
    #[doc= " # Examples"]
    #[doc= ""]
    #[doc= " ```rust"]
    #[doc= " use bitvec::prelude::*;"]
    #[doc= ""]
    #[doc= " let mut store = 8u8;"]
    #[doc= " let bits = store.bits_mut::<Msb0>();"]
    #[doc= " assert!(!bits[3]);"]
    #[doc= " bits.set(3, true);"]
    #[doc= " assert!(bits[3]);"]
    #[doc= " ```"]
    pub(crate) fn set(&mut self, index: usize, value: bool) {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    #[doc= " Sets a bit at an index, without doing bounds checking."]
    #[doc= ""]
    #[doc= " This is generally not recommended; use with caution! For a safe"]
    #[doc= " alternative, see [`set`]."]
    #[doc= ""]
    #[doc= " # Parameters"]
    #[doc= ""]
    #[doc= " - `&mut self`"]
    #[doc= " - `index`: The bit index to retrieve. This index is *not* checked"]
    #[doc= "   against the length of `self`."]
    #[doc= ""]
    #[doc= " # Effects"]
    #[doc= ""]
    #[doc= " The bit at `index` is set to `value`."]
    #[doc= ""]
    #[doc= " # Safety"]
    #[doc= ""]
    #[doc= " This method is **not** safe. It performs raw pointer arithmetic to seek"]
    #[doc= " from the start of the slice to the requested index, and set the bit"]
    #[doc= " there. It does not inspect the length of `self`, and it is free to"]
    #[doc= " perform out-of-bounds memory *write* access."]
    #[doc= ""]
    #[doc= " Use this method **only** when you have already performed the bounds"]
    #[doc= " check, and can guarantee that the call occurs with a safely in-bounds"]
    #[doc= " index."]
    #[doc= ""]
    #[doc= " # Examples"]
    #[doc= ""]
    #[doc= " This example uses a bit slice of length 2, and demonstrates"]
    #[doc= " out-of-bounds access to the last bit in the element."]
    #[doc= ""]
    #[doc= " ```rust"]
    #[doc= " use bitvec::prelude::*;"]
    #[doc= ""]
    #[doc= " let mut src = 0u8;"]
    #[doc= " let bits = &mut src.bits_mut::<Msb0>()[2 .. 4];"]
    #[doc= " assert_eq!(bits.len(), 2);"]
    #[doc= " unsafe {"]
    #[doc= "     bits.set_unchecked(5, true);"]
    #[doc= " }"]
    #[doc= " assert_eq!(src, 1);"]
    #[doc= " ```"]
    #[doc= ""]
    #[doc= " [`set`]: #method.set"]
    pub(crate) unsafe fn set_unchecked(&mut self, index: usize, value: bool) {
        let bitptr = self.bitptr();
        let (elt, bit) = bitptr.head().offset(index as isize);
        let data_ptr = bitptr.pointer().a();
        (*data_ptr.offset(elt)).set::<O>(bit, value);
    }

    #[doc= " Produces a write reference to a region of the slice."]
    #[doc= ""]
    #[doc= " This method corresponds to [`Index::index`], except that it produces a"]
    #[doc= " writable reference rather than a read-only reference. See"]
    #[doc= " [`BitSliceIndex`] for the possible types of the produced reference."]
    #[doc= ""]
    #[doc= " Use of this method locks the `&mut BitSlice` for the duration of the"]
    #[doc= " produced reference’s lifetime. If you need multiple **non-overlapping**"]
    #[doc= " write references into a single source `&mut BitSlice`, see the"]
    #[doc= " [`::split_at_mut`] method."]
    #[doc= ""]
    #[doc= " # Lifetimes"]
    #[doc= ""]
    #[doc= " - `'a`: Propagates the lifetime of the referent slice to the interior"]
    #[doc= "   reference produced."]
    #[doc= ""]
    #[doc= " # Parameters"]
    #[doc= ""]
    #[doc= " - `&mut self`"]
    #[doc= " - `index`: Some value whose type can be used to index `BitSlice`s."]
    #[doc= ""]
    #[doc= " # Returns"]
    #[doc= ""]
    #[doc= " A writable reference into `self`, whose exact type is determined by"]
    #[doc= " `index`’s implementation of [`BitSliceIndex`]. This may be either a"]
    #[doc= " smaller `&mut BitSlice` when `index` is a range, or a [`BitMut`] proxy"]
    #[doc= " type when `index` is a `usize`. See the [`BitMut`] documentation for"]
    #[doc= " information on how to use it."]
    #[doc= ""]
    #[doc= " # Panics"]
    #[doc= ""]
    #[doc= " This panics if `index` is out of bounds of `self`."]
    #[doc= ""]
    #[doc= " # Examples"]
    #[doc= ""]
    #[doc= " ```rust"]
    #[doc= " use bitvec::prelude::*;"]
    #[doc= ""]
    #[doc= " let mut src = 0u8;"]
    #[doc= " let bits = src.bits_mut::<Msb0>();"]
    #[doc= ""]
    #[doc= " assert!(!bits[0]);"]
    #[doc= " *bits.at(0) = true;"]
    #[doc= " //  note the leading dereference."]
    #[doc= " assert!(bits[0]);"]
    #[doc= " ```"]
    #[doc= ""]
    #[doc= " This example shows multiple usage by using `split_at_mut`."]
    #[doc= ""]
    #[doc= " ```rust"]
    #[doc= " use bitvec::prelude::*;"]
    #[doc= ""]
    #[doc= " let mut src = 0u8;"]
    #[doc= " let bits = src.bits_mut::<Msb0>();"]
    #[doc= ""]
    #[doc= " let (mut a, rest) = bits.split_at_mut(2);"]
    #[doc= " let (mut b, rest) = rest.split_at_mut(3);"]
    #[doc= " *a.at(0) = true;"]
    #[doc= " *b.at(0) = true;"]
    #[doc= " *rest.at(0) = true;"]
    #[doc= ""]
    #[doc= " assert_eq!(bits.as_slice()[0], 0b1010_0100);"]
    #[doc= " //                               a b   rest"]
    #[doc= " ```"]
    #[doc= ""]
    #[doc= " The above example splits the slice into three (the first, the second,"]
    #[doc= " and the rest) in order to hold multiple write references into the slice."]
    #[doc= ""]
    #[doc= " [`BitSliceIndex`]: trait.BitSliceIndex.html"]
    #[doc= " [`Index::index`]: https://doc.rust-lang.org/core/ops/trait.Index.html#method.index"]
    #[doc= " [`::get`]: #method.get"]
    #[doc= " [`::split_at_mut`]: #method.split_at_mut"]
    #[deprecated(since = "0.18.0", note = "Use `.get_mut()` instead")]
    #[inline]
    pub(crate) fn at<'a, I>(&'a mut self, index: I) -> I::Mut
    where
        I: BitSliceIndex<'a, O, T> {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    #[doc= " Version of [`at`](#method.at) that does not perform boundary checking."]
    #[doc= ""]
    #[doc= " # Safety"]
    #[doc= ""]
    #[doc= " If `index` is outside the boundaries of `self`, then this function will"]
    #[doc= " induce safety violations. The caller must ensure that `index` is within"]
    #[doc= " the boundaries of `self` before calling."]
    #[deprecated(since = "0.18.0", note = "Use `.get_unchecked_mut()` instead")]
    #[inline]
    pub(crate) unsafe fn at_unchecked<'a, I>(&'a mut self, index: I) -> I::Mut
    where
        I: BitSliceIndex<'a, O, T> {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    #[doc= " Version of [`split_at`](#method.split_at) that does not perform boundary"]
    #[doc= " checking."]
    #[doc= ""]
    #[doc= " # Safety"]
    #[doc= ""]
    #[doc= " If `mid` is outside the boundaries of `self`, then this function will"]
    #[doc= " induce safety violations. The caller must ensure that `mid` is within"]
    #[doc= " the boundaries of `self` before calling."]
    pub(crate) unsafe fn split_at_unchecked(&self, mid: usize) -> (&Self, &Self) {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    #[doc= " Version of [`split_at_mut`](#method.split_at_mut) that does not perform"]
    #[doc= " boundary checking."]
    #[doc= ""]
    #[doc= " # Aliasing"]
    #[doc= ""]
    #[doc= " Mutable splits mark their returned slices as aliased, as it is permitted"]
    #[doc= " for the split to occur in the middle of a memory element. To undo this"]
    #[doc= " aliasing, use `.bit_domain_mut()` on the returned slices."]
    #[doc= ""]
    #[doc= " # Safety"]
    #[doc= ""]
    #[doc= " If `mid` is outside the boundaries of `self`, then this function will"]
    #[doc= " induce safety violations. The caller must ensure that `mid` is within"]
    #[doc= " the boundaries of `self` before calling."]
    #[inline]
    #[allow(clippy::type_complexity)]
    pub(crate) unsafe fn split_at_mut_unchecked(
        &mut self,
        mid: usize,
    ) -> (&mut BitSlice<O, T::Alias>, &mut BitSlice<O, T::Alias>) {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    #[doc= " Version of [`swap`](#method.swap) that does not perform boundary checks."]
    #[doc= ""]
    #[doc= " # Safety"]
    #[doc= ""]
    #[doc= " `a` and `b` must be within the bounds of `self`, otherwise, the memory"]
    #[doc= " access is unsound and may induce undefined behavior."]
    #[inline]
    pub(crate) unsafe fn swap_unchecked(&mut self, a: usize, b: usize) {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    #[doc= " Tests if *all* bits in the slice domain are set (logical `∧`)."]
    #[doc= ""]
    #[doc= " # Truth Table"]
    #[doc= ""]
    #[doc= " ```text"]
    #[doc= " 0 0 => 0"]
    #[doc= " 0 1 => 0"]
    #[doc= " 1 0 => 0"]
    #[doc= " 1 1 => 1"]
    #[doc= " ```"]
    #[doc= ""]
    #[doc= " # Parameters"]
    #[doc= ""]
    #[doc= " - `&self`"]
    #[doc= ""]
    #[doc= " # Returns"]
    #[doc= ""]
    #[doc= " Whether all bits in the slice domain are set. The empty slice returns"]
    #[doc= " `true`."]
    #[doc= ""]
    #[doc= " # Examples"]
    #[doc= ""]
    #[doc= " ```rust"]
    #[doc= " use bitvec::prelude::*;"]
    #[doc= ""]
    #[doc= " let bits = 0xFDu8.bits::<Msb0>();"]
    #[doc= " assert!(bits[.. 4].all());"]
    #[doc= " assert!(!bits[4 ..].all());"]
    #[doc= " ```"]
    pub(crate) fn all(&self) -> bool {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    #[doc= " Tests if *any* bit in the slice is set (logical `∨`)."]
    #[doc= ""]
    #[doc= " # Truth Table"]
    #[doc= ""]
    #[doc= " ```text"]
    #[doc= " 0 0 => 0"]
    #[doc= " 0 1 => 1"]
    #[doc= " 1 0 => 1"]
    #[doc= " 1 1 => 1"]
    #[doc= " ```"]
    #[doc= ""]
    #[doc= " # Parameters"]
    #[doc= ""]
    #[doc= " - `&self`"]
    #[doc= ""]
    #[doc= " # Returns"]
    #[doc= ""]
    #[doc= " Whether any bit in the slice domain is set. The empty slice returns"]
    #[doc= " `false`."]
    #[doc= ""]
    #[doc= " # Examples"]
    #[doc= ""]
    #[doc= " ```rust"]
    #[doc= " use bitvec::prelude::*;"]
    #[doc= ""]
    #[doc= " let bits = 0x40u8.bits::<Msb0>();"]
    #[doc= " assert!(bits[.. 4].any());"]
    #[doc= " assert!(!bits[4 ..].any());"]
    #[doc= " ```"]
    pub(crate) fn any(&self) -> bool {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    #[doc= " Tests if *any* bit in the slice is unset (logical `¬∧`)."]
    #[doc= ""]
    #[doc= " # Truth Table"]
    #[doc= ""]
    #[doc= " ```text"]
    #[doc= " 0 0 => 1"]
    #[doc= " 0 1 => 1"]
    #[doc= " 1 0 => 1"]
    #[doc= " 1 1 => 0"]
    #[doc= " ```"]
    #[doc= ""]
    #[doc= " # Parameters"]
    #[doc= ""]
    #[doc= " - `&self"]
    #[doc= ""]
    #[doc= " # Returns"]
    #[doc= ""]
    #[doc= " Whether any bit in the slice domain is unset."]
    #[doc= ""]
    #[doc= " # Examples"]
    #[doc= ""]
    #[doc= " ```rust"]
    #[doc= " use bitvec::prelude::*;"]
    #[doc= ""]
    #[doc= " let bits = 0xFDu8.bits::<Msb0>();"]
    #[doc= " assert!(!bits[.. 4].not_all());"]
    #[doc= " assert!(bits[4 ..].not_all());"]
    #[doc= " ```"]
    #[inline]
    pub(crate) fn not_all(&self) -> bool {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    #[doc= " Tests if *all* bits in the slice are unset (logical `¬∨`)."]
    #[doc= ""]
    #[doc= " # Truth Table"]
    #[doc= ""]
    #[doc= " ```text"]
    #[doc= " 0 0 => 1"]
    #[doc= " 0 1 => 0"]
    #[doc= " 1 0 => 0"]
    #[doc= " 1 1 => 0"]
    #[doc= " ```"]
    #[doc= ""]
    #[doc= " # Parameters"]
    #[doc= ""]
    #[doc= " - `&self`"]
    #[doc= ""]
    #[doc= " # Returns"]
    #[doc= ""]
    #[doc= " Whether all bits in the slice domain are unset."]
    #[doc= ""]
    #[doc= " # Examples"]
    #[doc= ""]
    #[doc= " ```rust"]
    #[doc= " use bitvec::prelude::*;"]
    #[doc= ""]
    #[doc= " let bits = 0x40u8.bits::<Msb0>();"]
    #[doc= " assert!(!bits[.. 4].not_any());"]
    #[doc= " assert!(bits[4 ..].not_any());"]
    #[doc= " ```"]
    #[inline]
    pub(crate) fn not_any(&self) -> bool {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    #[doc= " Tests whether the slice has some, but not all, bits set and some, but"]
    #[doc= " not all, bits unset."]
    #[doc= ""]
    #[doc= " This is `false` if either `all()` or `not_any()` are `true`."]
    #[doc= ""]
    #[doc= " # Truth Table"]
    #[doc= ""]
    #[doc= " ```text"]
    #[doc= " 0 0 => 0"]
    #[doc= " 0 1 => 1"]
    #[doc= " 1 0 => 1"]
    #[doc= " 1 1 => 0"]
    #[doc= " ```"]
    #[doc= ""]
    #[doc= " # Parameters"]
    #[doc= ""]
    #[doc= " - `&self`"]
    #[doc= ""]
    #[doc= " # Returns"]
    #[doc= ""]
    #[doc= " Whether the slice domain has mixed content. The empty slice returns"]
    #[doc= " `false`."]
    #[doc= ""]
    #[doc= " # Examples"]
    #[doc= ""]
    #[doc= " ```rust"]
    #[doc= " use bitvec::prelude::*;"]
    #[doc= ""]
    #[doc= " let bits = 0b111_000_10u8.bits::<Msb0>();"]
    #[doc= " assert!(!bits[0 .. 3].some());"]
    #[doc= " assert!(!bits[3 .. 6].some());"]
    #[doc= " assert!(bits[6 ..].some());"]
    #[doc= " ```"]
    #[inline]
    pub(crate) fn some(&self) -> bool {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    #[doc= " Counts how many bits are set high."]
    #[doc= ""]
    #[doc= " # Parameters"]
    #[doc= ""]
    #[doc= " - `&self`"]
    #[doc= ""]
    #[doc= " # Returns"]
    #[doc= ""]
    #[doc= " The number of high bits in the slice domain."]
    #[doc= ""]
    #[doc= " # Examples"]
    #[doc= ""]
    #[doc= " ```rust"]
    #[doc= " use bitvec::prelude::*;"]
    #[doc= ""]
    #[doc= " let bits = [0xFDu8, 0x25].bits::<Msb0>();"]
    #[doc= " assert_eq!(bits.count_ones(), 10);"]
    #[doc= " ```"]
    pub(crate) fn count_ones(&self) -> usize {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    #[doc= " Counts how many bits are set low."]
    #[doc= ""]
    #[doc= " # Parameters"]
    #[doc= ""]
    #[doc= " - `&self`"]
    #[doc= ""]
    #[doc= " # Returns"]
    #[doc= ""]
    #[doc= " The number of low bits in the slice domain."]
    #[doc= ""]
    #[doc= " # Examples"]
    #[doc= ""]
    #[doc= " ```rust"]
    #[doc= " use bitvec::prelude::*;"]
    #[doc= ""]
    #[doc= " let bits = [0xFDu8, 0x25].bits::<Msb0>();"]
    #[doc= " assert_eq!(bits.count_zeros(), 6);"]
    #[doc= " ```"]
    pub(crate) fn count_zeros(&self) -> usize {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    #[doc= " Set all bits in the slice to a value."]
    #[doc= ""]
    #[doc= " # Parameters"]
    #[doc= ""]
    #[doc= " - `&mut self`"]
    #[doc= " - `value`: The bit value to which all bits in the slice will be set."]
    #[doc= ""]
    #[doc= " # Examples"]
    #[doc= ""]
    #[doc= " ```rust"]
    #[doc= " use bitvec::prelude::*;"]
    #[doc= ""]
    #[doc= " let mut src = 0u8;"]
    #[doc= " let bits = src.bits_mut::<Msb0>();"]
    #[doc= " bits[2 .. 6].set_all(true);"]
    #[doc= " assert_eq!(bits.as_slice(), &[0b0011_1100]);"]
    #[doc= " bits[3 .. 5].set_all(false);"]
    #[doc= " assert_eq!(bits.as_slice(), &[0b0010_0100]);"]
    #[doc= " bits[.. 1].set_all(true);"]
    #[doc= " assert_eq!(bits.as_slice(), &[0b1010_0100]);"]
    #[doc= " ```"]
    pub(crate) fn set_all(&mut self, value: bool) {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    #[doc= " Provides mutable traversal of the collection."]
    #[doc= ""]
    #[doc= " It is impossible to implement `IndexMut` on `BitSlice`, because bits do"]
    #[doc= " not have addresses, so there can be no `&mut u1`. This method allows the"]
    #[doc= " client to receive an enumerated bit, and provide a new bit to set at"]
    #[doc= " each index."]
    #[doc= ""]
    #[doc= " # Parameters"]
    #[doc= ""]
    #[doc= " - `&mut self`"]
    #[doc= " - `func`: A function which receives a `(usize, bool)` pair of index and"]
    #[doc= "   value, and returns a bool. It receives the bit at each position, and"]
    #[doc= "   the return value is written back at that position."]
    #[doc= ""]
    #[doc= " # Examples"]
    #[doc= ""]
    #[doc= " ```rust"]
    #[doc= " use bitvec::prelude::*;"]
    #[doc= ""]
    #[doc= " let mut src = 0u8;"]
    #[doc= " let bits = src.bits_mut::<Msb0>();"]
    #[doc= " bits.for_each(|idx, _bit| idx % 3 == 0);"]
    #[doc= " assert_eq!(src, 0b1001_0010);"]
    #[doc= " ```"]
    pub(crate) fn for_each<F>(&mut self, func: F)
    where
        F: Fn(usize, bool) -> bool {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    #[doc= " Accesses the total backing stoarge of the `BitSlice`, as a slice of its"]
    #[doc= " aliased elements."]
    #[doc= ""]
    #[doc= " Because `BitSlice` is permitted to create aliasing views to memory at"]
    #[doc= " runtime, this method is required to mark the entire slice as aliased in"]
    #[doc= " order to include the maybe-aliased edge elements."]
    #[doc= ""]
    #[doc= " You should prefer using [`.domain()`] to produce a fine-grained"]
    #[doc= " view that only aliases when necessary. This method is only appropriate"]
    #[doc= " when you require a single, contiguous, slice for some API."]
    #[doc= ""]
    #[doc= " # Parameters"]
    #[doc= ""]
    #[doc= " - `&self`"]
    #[doc= ""]
    #[doc= " # Returns"]
    #[doc= ""]
    #[doc= " An aliased view of the entire memory region this slice covers, including"]
    #[doc= " contended edge elements."]
    #[doc= ""]
    #[doc= " [`.domain()`]: #method.domain"]
    pub(crate) fn as_aliased_slice(&self) -> &[T::Alias] {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    #[doc= " Accesses the backing storage of the `BitSlice` as a slice of its"]
    #[doc= " elements."]
    #[doc= ""]
    #[doc= " This will not include partially-owned edge elements, as they may be"]
    #[doc= " contended by other slice handles."]
    #[doc= ""]
    #[doc= " # Parameters"]
    #[doc= ""]
    #[doc= " - `&self`"]
    #[doc= ""]
    #[doc= " # Returns"]
    #[doc= ""]
    #[doc= " A slice of all the elements that the `BitSlice` uses for storage."]
    #[doc= ""]
    #[doc= " # Examples"]
    #[doc= ""]
    #[doc= " ```rust"]
    #[doc= " use bitvec::prelude::*;"]
    #[doc= ""]
    #[doc= " let src = [1u8, 66];"]
    #[doc= " let bits = src.bits::<Msb0>();"]
    #[doc= ""]
    #[doc= " let accum = bits"]
    #[doc= "     .as_slice()"]
    #[doc= "     .iter()"]
    #[doc= "     .map(|elt| elt.count_ones())"]
    #[doc= "     .sum::<u32>();"]
    #[doc= " assert_eq!(accum, 3);"]
    #[doc= " ```"]
    pub(crate) fn as_slice(&self) -> &[T] {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    #[doc= " Accesses the underlying store."]
    #[doc= ""]
    #[doc= " This will not include partially-owned edge elements, as they may be"]
    #[doc= " contended by other slice handles."]
    #[doc= ""]
    #[doc= " # Examples"]
    #[doc= ""]
    #[doc= " ```rust"]
    #[doc= " use bitvec::prelude::*;"]
    #[doc= ""]
    #[doc= " let mut src = [1u8, 64];"]
    #[doc= " let bits = src.bits_mut::<Msb0>();"]
    #[doc= " for elt in bits.as_mut_slice() {"]
    #[doc= "     *elt |= 2;"]
    #[doc= " }"]
    #[doc= " assert_eq!(&[3, 66], bits.as_slice());"]
    #[doc= " ```"]
    pub(crate) fn as_mut_slice(&mut self) -> &mut [T] {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    #[doc= " Splits the slice into the components of its memory domain."]
    #[doc= ""]
    #[doc= " This produces a set of read-only aliased and unaliased subslices,"]
    #[doc= " according to its pointer information. See the `BitDomain` documentation"]
    #[doc= " for more information about the returned descriptor."]
    pub(crate) fn bit_domain(&self) -> BitDomain<O, T> {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    #[doc= " Splits the slice into the components of its memory domain."]
    #[doc= ""]
    #[doc= " This produces a set of writable aliased and unaliased subslices,"]
    #[doc= " according to its pointer information. See the `BitDomainMut`"]
    #[doc= " documentation for more information about the returned descriptor."]
    pub(crate) fn bit_domain_mut(&mut self) -> BitDomainMut<O, T> {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    #[doc= " Splits the slice into references to its underlying memory elements."]
    #[doc= ""]
    #[doc= " Unlike `.bit_domain()` and `.bit_domain_mut()`, this does not return"]
    #[doc= " smaller, specialized, `BitSlice` handles, but rather appropriately"]
    #[doc= " un/aliased references to memory elements."]
    #[doc= ""]
    #[doc= " The aliased references allow mutation of these elements. You are"]
    #[doc= " required to not mutate through these references. This function is not"]
    #[doc= " marked `unsafe`, but this is a contract you must uphold. Use"]
    #[doc= " [`.domain_mut()`] for mutation."]
    #[doc= ""]
    #[doc= " # Parameters"]
    #[doc= ""]
    #[doc= " - `&self`"]
    #[doc= ""]
    #[doc= " # Returns"]
    #[doc= ""]
    #[doc= " A read-only descriptor of the memory elements underneath `*self`."]
    #[doc= ""]
    #[doc= " [`.domain_mut()`]: #method.domain_mut"]
    pub(crate) fn domain(&self) -> Domain<T> {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    #[doc= " Splits the slice into mutable references to its underlying memory"]
    #[doc= " elements."]
    #[doc= ""]
    #[doc= " Like `.domain()`, this returns appropriately un/aliased references to"]
    #[doc= " memory elements. These references are all writable."]
    #[doc= ""]
    #[doc= " The aliased edge references permit modifying memory beyond their bit"]
    #[doc= " marker. You are required to only mutate the region of these edge"]
    #[doc= " elements that you currently govern. This function is not marked"]
    #[doc= " `unsafe`, but this is a contract you must uphold."]
    #[doc= ""]
    #[doc= " # Parameters"]
    #[doc= ""]
    #[doc= " - `&mut self`"]
    #[doc= ""]
    #[doc= " # Returns"]
    #[doc= ""]
    #[doc= " A descriptor of the memory elements underneath `*self`, permitting"]
    #[doc= " mutation."]
    pub(crate) fn domain_mut(&mut self) -> DomainMut<T> {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    #[doc= " Accesses the underlying pointer structure."]
    #[doc= ""]
    #[doc= " # Parameters"]
    #[doc= ""]
    #[doc= " - `&self`"]
    #[doc= ""]
    #[doc= " # Returns"]
    #[doc= ""]
    #[doc= " The [`BitPtr`] structure of the slice handle."]
    #[doc= ""]
    #[doc= " [`BitPtr`]: ../pointer/struct.BitPtr.html"]
    #[inline]
    pub(crate) fn bitptr(&self) -> BitPtr<T> {
        BitPtr::from_bitslice(self)
    }

    #[doc= " Copy a bit from one location in a slice to another."]
    #[doc= ""]
    #[doc= " # Parameters"]
    #[doc= ""]
    #[doc= " - `&mut self`"]
    #[doc= " - `from`: The index of the bit to be copied."]
    #[doc= " - `to`: The index at which the copied bit will be written."]
    #[doc= ""]
    #[doc= " # Safety"]
    #[doc= ""]
    #[doc= " `from` and `to` must be within the bounds of `self`. This is not"]
    #[doc= " checked."]
    #[inline]
    pub(crate) unsafe fn copy_unchecked(&mut self, from: usize, to: usize) {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    #[doc= " Mark an immutable slice as referring to aliased memory."]
    pub(crate) fn alias(&self) -> &BitSlice<O, T::Alias> {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    #[doc= " Mark a mutable slice as referring to aliased memory."]
    pub(crate) fn alias_mut(&mut self) -> &mut BitSlice<O, T::Alias> {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    #[doc= " Mark a slice as referring to known-unaliased memory."]
    #[doc= ""]
    #[doc= " # Safety"]
    #[doc= ""]
    #[doc= " This function requires that the unaliasing condition is correct,"]
    #[doc= " otherwise it will introduce race conditions."]
    pub(crate) unsafe fn noalias(&self) -> &BitSlice<O, T::NoAlias> {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    #[doc= " Remove the aliasing marker from a mutable slice."]
    #[doc= ""]
    #[doc= " # Safety"]
    #[doc= ""]
    #[doc= " This may only be done when the slice is known to refer to unaliased"]
    #[doc= " memory, or when the marker is about to be reäpplied."]
    pub(crate) unsafe fn unalias_mut(this: &mut BitSlice<O, T::Alias>) -> &mut Self {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

#[doc= " Allows a type to be used as a sequence of immutable bits.\n\n# Requirements\n\nThis trait can only be implemented by contiguous structures: individual\nfundamentals, and sequences (arrays or slices) of them.\n*"]
pub(crate) trait AsBits {
    #[doc= " The underlying fundamental type of the implementor."]
    type Store: BitStore;

    #[doc= " Constructs a `BitSlice` reference over data."]
    #[doc= ""]
    #[doc= " # Type Parameters"]
    #[doc= ""]
    #[doc= " - `O: BitOrder`: The `BitOrder` type used to index within the slice."]
    #[doc= ""]
    #[doc= " # Parameters"]
    #[doc= ""]
    #[doc= " - `&self`"]
    #[doc= ""]
    #[doc= " # Returns"]
    #[doc= ""]
    #[doc= " A `BitSlice` handle over `self`’s data, using the provided `BitOrder`"]
    #[doc= " type and using `Self::Store` as the data type."]
    #[doc= ""]
    #[doc= " # Examples"]
    #[doc= ""]
    #[doc= " ```rust"]
    #[doc= " use bitvec::prelude::*;"]
    #[doc= ""]
    #[doc= " let src = 8u8;"]
    #[doc= " let bits = src.bits::<Msb0>();"]
    #[doc= " assert!(bits[4]);"]
    #[doc= " ```"]
    fn bits<O>(&self) -> &BitSlice<O, Self::Store>
    where
        O: BitOrder;
    #[doc= " Constructs a mutable `BitSlice` reference over data."]
    #[doc= ""]
    #[doc= " # Type Parameters"]
    #[doc= ""]
    #[doc= " - `O: BitOrder`: The `BitOrder` type used to index within the slice."]
    #[doc= ""]
    #[doc= " # Parameters"]
    #[doc= ""]
    #[doc= " - `&mut self`"]
    #[doc= ""]
    #[doc= " # Returns"]
    #[doc= ""]
    #[doc= " A `BitSlice` handle over `self`’s data, using the provided `BitOrder`"]
    #[doc= " type and using `Self::Store` as the data type."]
    #[doc= ""]
    #[doc= " # Examples"]
    #[doc= ""]
    #[doc= " ```rust"]
    #[doc= " use bitvec::prelude::*;"]
    #[doc= ""]
    #[doc= " let mut src = 8u8;"]
    #[doc= " let bits = src.bits_mut::<Lsb0>();"]
    #[doc= " assert!(bits[3]);"]
    #[doc= " *bits.at(3) = false;"]
    #[doc= " assert!(!bits[3]);"]
    #[doc= " ```"]
    fn bits_mut<O>(&mut self) -> &mut BitSlice<O, Self::Store>
    where
        O: BitOrder;
}

impl<T> AsBits for T
where
    T: BitStore {
    type Store = T;

    fn bits<O>(&self) -> &BitSlice<O, T>
    where
        O: BitOrder {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    fn bits_mut<O>(&mut self) -> &mut BitSlice<O, T>
    where
        O: BitOrder {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<T> AsBits for [T]
where
    T: BitStore {
    type Store = T;

    fn bits<O>(&self) -> &BitSlice<O, T>
    where
        O: BitOrder {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    fn bits_mut<O>(&mut self) -> &mut BitSlice<O, T>
    where
        O: BitOrder {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

macro_rules! impl_bits_for{
    ($($n: expr), *) => {
        $(impl < T > AsBits for[
            T;
            $n
        ] where T: BitStore {
            type Store = T;

            fn bits<O>(&self) -> &BitSlice<O, T>
            where
                O: BitOrder {
                BitSlice::from_slice(self)
            }

            fn bits_mut<O>(&mut self) -> &mut BitSlice<O, T>
            where
                O: BitOrder {
                BitSlice::from_slice_mut(self)
            }
        }) *
    };
}

impl_bits_for![
    0,
    1,
    2,
    3,
    4,
    5,
    6,
    7,
    8,
    9,
    10,
    11,
    12,
    13,
    14,
    15,
    16,
    17,
    18,
    19,
    20,
    21,
    22,
    23,
    24,
    25,
    26,
    27,
    28,
    29,
    30,
    31,
    32
];
mod api;
pub(crate) mod iter;
mod ops;
mod proxy;
mod traits;

pub(crate) use self::{
    api::*,
    iter::*,
    proxy::*,
};

#[cfg(test)]
mod tests;
