#![doc= " `BitSlice` Wide Reference\n\nThis module defines semantic operations on `[u1]`, in contrast to the mechanical\noperations defined in `BitPtr`.\n\nThe `&BitSlice` handle has the same size and general layout as the standard Rust\nslice handle `&[T]`. Its binary layout is wholly incompatible with the layout of\nRust slices, and must never be interchanged except through the provided APIs.\n!"]

use crate::{
    access::BitAccess,
    domain::{
        Domain,
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
        self.set_unchecked(to, *self.get_unchecked(from));
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
    proxy::*,
};
