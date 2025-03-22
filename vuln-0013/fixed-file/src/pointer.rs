#![doc= " Raw Pointer Representation\n\nThis module defines the binary representation of the handle to a `BitSlice`\nregion. This structure is crate-internal, and defines the methods required to\nstore a `BitSlice` pointer in memory and retrieve values from it suitable for\nwork.\n!"]

use crate::{
    index::{
        BitIdx,
        Indexable,
    },
    mem::BitMemory,
    order::BitOrder,
    slice::BitSlice,
    store::BitStore,
};
use core::{
    fmt::{
        self,
        Debug,
        Formatter,
        Pointer,
    },
    marker::PhantomData,
    mem::size_of,
    ptr::NonNull,
    slice,
};
#[cfg(any(test, feature = "alloc"))]
use crate::index::BitTail;

#[doc= " Width in bits of a pointer on the target machine."]
const PTR_BITS: usize = size_of::<*const u8>() * 8;

#[doc= " Union to permit reinterpreting a pointer-shaped value as a read pointer,\nwrite pointer, or bare numeric address.\n\n# Safety\n\nAbsolutely none whatsoever. This is probably flirting with undefined\nbehavior, and should be presumed to be the origin site of failure if the\ncrate ever breaks in the future.\n\n# Type Parameters\n\n- `T`: The referent data type.\n*"]
#[doc(hidden)]
pub(crate) union Address<T>
where
    T: BitStore {
    #[doc= " A shareable pointer to some contended mutable data."]
    a: *const <T as BitStore>::Access,
    #[doc= " A read pointer to some data."]
    r: *const T,
    #[doc= " A write pointer to some data."]
    w: *mut T,
    #[doc= " The pointer address as a bare integer."]
    u: usize,
}

impl<T> Address<T>
where
    T: BitStore {
    #[doc= " Accesses the address as a shared mutable pointer."]
    #[doc= ""]
    #[doc= " # Parameters"]
    #[doc= ""]
    #[doc= " - `&self`"]
    #[doc= ""]
    #[doc= " # Returns"]
    #[doc= ""]
    #[doc= " The stored address, interpreted as a shared pointer to a mutable memory"]
    #[doc= " location."]
    #[inline]
    pub(crate) fn a(self) -> *const <T as BitStore>::Access {
        unsafe {
            self.a
        }
    }

    #[doc= " Accesses the address as a read pointer."]
    #[doc= ""]
    #[doc= " # Parameters"]
    #[doc= ""]
    #[doc= " - `&self`"]
    #[doc= ""]
    #[doc= " # Returns"]
    #[doc= ""]
    #[doc= " The stored address, as a read pointer."]
    #[inline]
    pub(crate) fn r(self) -> *const T {
        unsafe {
            self.r
        }
    }

    #[doc= " Accesses the address as a write pointer."]
    #[doc= ""]
    #[doc= " # Parameters"]
    #[doc= ""]
    #[doc= " - `&self`"]
    #[doc= ""]
    #[doc= " # Returns"]
    #[doc= ""]
    #[doc= " The stored address, as a write pointer."]
    #[inline]
    pub(crate) fn w(self) -> *mut T {
        unsafe {
            self.w
        }
    }

    #[doc= " Accesses the address as a bare integral value."]
    #[doc= ""]
    #[doc= " # Parameters"]
    #[doc= ""]
    #[doc= " - `&self`"]
    #[doc= ""]
    #[doc= " # Returns"]
    #[doc= ""]
    #[doc= " The stored address, as a bare integer."]
    #[inline]
    pub(crate) fn u(self) -> usize {
        unsafe {
            self.u
        }
    }

    pub(crate) fn to_alias(self) -> *const T::Alias {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<T> Clone for Address<T>
where
    T: BitStore {
    fn clone(&self) -> Self {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<T> From<&T> for Address<T>
where
    T: BitStore {
    fn from(r: &T) -> Self {
        Self { r }
    }
}

impl<T> From<*const T> for Address<T>
where
    T: BitStore {
    fn from(r: *const T) -> Self {
        Self { r }
    }
}

impl<T> From<&mut T> for Address<T>
where
    T: BitStore {
    fn from(w: &mut T) -> Self {
        Self { w }
    }
}

impl<T> From<*mut T> for Address<T>
where
    T: BitStore {
    fn from(w: *mut T) -> Self {
        Self { w }
    }
}

impl<T> From<usize> for Address<T>
where
    T: BitStore {
    fn from(u: usize) -> Self {
        Self { u }
    }
}

impl<T> Pointer for Address<T>
where
    T: BitStore {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<T> Copy for Address<T>
where
    T: BitStore { }

#[doc= " In-memory representation of `&BitSlice` handles.\n\n# Layout\n\nThis structure is a more complex version of the `*const T`/`usize` tuple that\nRust uses to represent slices throughout the language. It breaks the pointer and\ncounter fundamentals into sub-field components. Rust does not have bitfield\nsyntax, so the below description of the element layout is in C++.\n\n```cpp\ntemplate <typename T>\nstruct BitPtr {\n  size_t ptr_head : __builtin_ctzll(alignof(T));\n  size_t ptr_data : sizeof(uintptr_t) * 8 - __builtin_ctzll(alignof(T));\n\n  size_t len_head : 3;\n  size_t len_bits : sizeof(size_t) * 8 - 3;\n};\n```\n\nThis means that the `BitPtr<T>` structure has three *logical* fields, stored in\nfour segments across the two *structural* fields of the type. The widths and\nplacements of each segment are functions of the size of `*const T` and `usize`,\nand the alignment of `T`.\n\n# Fields\n\nThis section describes the purpose, meaning, and layout of the four logical\nfields.\n\n## Data Pointer\n\nAligned pointers to `T` always have low bits available for use to refine the\naddress of a `T` to the address of a `u8`. It is stored in the high bits of the\n`ptr` field, running from MSb down to (inclusive)\n`core::mem::align_of::<T>().trailing_zeros()`.\n\n## Bit Counter\n\nThe memory representation stores a counter of the live bits contained in the\nslice, starting at the head index. This counter occupies all but the lowest\nthree bits of the `len` structural field.\n\n## Head Bit Index\n\nFor any fundamental type `T`, `core::mem::align_of::<T>().trailing_zeros() + 3`\nbits are required to count the bit positions inside it.\n\n|Type |Alignment|Trailing Zeros|Count Bits|\n|:----|--------:|-------------:|---------:|\n|`u8` |        1|             0|         3|\n|`u16`|        2|             1|         4|\n|`u32`|        4|             2|         5|\n|`u64`|        8|             3|         6|\n\nThe head bit counter is split such that its bottom three bits are stored in the\nlow bits of the `len` field and the remaining high bits are stored in the low\nbits of `ptr`.\n\nThe counter is a value in the range `0 .. (1 << Count)` that serves as a cursor\ninto the zeroth storage element to find the first live bit.\n\n# Edge Cases\n\nThe following value sets are edge cases of valid `BitPtr` structures.\n\n## Null Slice\n\nThe fully zeroed slot is not a valid member of the `BitPtr<T>` type; it is the\nsentinel for `Option::<BitPtr<T>>::None`.\n\n## Empty Slice\n\nAll empty slices have `0` in their `bits` logical field, and do not constrain\ntheir `data` or `head` logical fields. The canonical empty slice structure uses\n`NonNull::<T>::dangling()` as its `data` pointer, and `0` as its `head` index,\nbut any slice structure with `0` as `bits` is considered to be empty, and all\nempty slices are equivalent to each other.\n\n## Uninhabited Slice\n\nThe subset of empty slices with non-dangling pointers are considered\nuninhabited. All `BitPtr` structures preserve their pointer information, even\nwhen empty, because they may be the owners of the memory region at the pointer.\nUninhabited slices are also unconstrained in their `head` index value.\n\n# Type Parameters\n\n- `T: BitStore` is the storage type over which the pointer governs.\n\n# Safety\n\nA `BitPtr` must never be constructed such that the element addressed by\n`self.pointer().offset(self.elements())` causes an addition overflow. This will\nbe checked in `new()`.\n\n# Undefined Behavior\n\nUsing values of this type directly as pointers or counters will result in\nundefined behavior. The pointer value will be invalid for the type, and both the\npointer and length values will be invalid for the memory model and allocation\nregime.\n*"]
#[repr(C)]
#[derive(Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct BitPtr<T = u8>
where
    T: BitStore {
    _ty: PhantomData<T>,
    #[doc= " Two-element bitfield structure, holding pointer and head information."]
    #[doc= ""]
    #[doc= " This stores a pointer to the zeroth element of the slice, and the high"]
    #[doc= " bits of the head bit cursor. It is typed as a `NonNull<u8>` in order to"]
    #[doc= " provide null-value optimizations to `Option<BitPtr<T>>`, and because the"]
    #[doc= " presence of head-bit cursor information in the lowest bits means the"]
    #[doc= " bit pattern will not uphold alignment properties assumed by"]
    #[doc= " `NonNull<T>`."]
    #[doc= ""]
    #[doc= " This field cannot be treated as an address of the zeroth byte of the"]
    #[doc= " slice domain, because the owning handle’s [`BitOrder`] implementation"]
    #[doc= " governs the bit pattern of the head cursor."]
    #[doc= ""]
    #[doc= " [`BitOrder`]: ../order/trait.BitOrder.html"]
    ptr: NonNull<u8>,
    #[doc= " Two-element bitfield structure, holding bit-count and head-index"]
    #[doc= " information."]
    #[doc= ""]
    #[doc= " This stores the bit count in its highest bits and the low three bits of"]
    #[doc= " the head `BitIdx` in the lowest three bits."]
    #[doc= ""]
    #[doc= " [`BitIdx`]: ../struct.BitIdx.html"]
    len: usize,
}

impl<T> BitPtr<T>
where
    T: BitStore {
    #[doc= " The number of low bits in `self.len` that are the low bits of the head"]
    #[doc= " `BitIdx` cursor."]
    #[doc= ""]
    #[doc= " This is always `3`, until Rust tries to target a machine whose bytes are"]
    #[doc= " not eight bits wide."]
    pub(crate) const LEN_HEAD_BITS: usize = 3;
    #[doc= " Marks the bits of `self.len` that are the `head` section."]
    pub(crate) const LEN_HEAD_MASK: usize = 0b0111;
    #[doc= " The inclusive maximum bit index."]
    pub(crate) const MAX_BITS: usize = !0 >> Self::LEN_HEAD_BITS;
    #[doc= " The inclusive maximum number of elements that can be stored in a"]
    #[doc= " `BitPtr` domain."]
    pub(crate) const MAX_ELTS: usize = (Self::MAX_BITS >> 3) + 1;
    #[doc= " Marks the bits of `self.ptr` that are the `data` section."]
    pub(crate) const PTR_DATA_MASK: usize = !Self::PTR_HEAD_MASK;
    #[doc= " The number of low bits in `self.ptr` that are the high bits of the head"]
    #[doc= " `BitIdx` cursor."]
    pub(crate) const PTR_HEAD_BITS: usize = T::Mem::INDX as usize - Self::LEN_HEAD_BITS;
    #[doc= " Marks the bits of `self.ptr` that are the `head` section."]
    pub(crate) const PTR_HEAD_MASK: usize = T::Mem::MASK as usize >> Self::LEN_HEAD_BITS;

    #[doc= " Produces an empty-slice representation."]
    #[doc= ""]
    #[doc= " This has no live bits, and has a dangling pointer. It is useful as a"]
    #[doc= " default value (and is the function used by `Default`) to indicate"]
    #[doc= " arbitrary empty slices."]
    #[doc= ""]
    #[doc= " # Returns"]
    #[doc= ""]
    #[doc= " An uninhabited, uninhabitable, empty slice."]
    #[doc= ""]
    #[doc= " # Safety"]
    #[doc= ""]
    #[doc= " The `BitPtr` returned by this function must never be dereferenced."]
    pub(crate) fn empty() -> Self {
        Self {
            _ty: PhantomData,
            ptr: NonNull::dangling(),
            len: 0,
        }
    }

    #[doc= " Produces an uninhabited slice from a bare pointer."]
    #[doc= ""]
    #[doc= " # Parameters"]
    #[doc= ""]
    #[doc= " - `ptr`: Some kind of pointer to `T`."]
    #[doc= ""]
    #[doc= " # Returns"]
    #[doc= ""]
    #[doc= " If `ptr` is null, then this returns the empty slice; otherwise, the"]
    #[doc= " returned slice is uninhabited and points to the given address."]
    #[doc= ""]
    #[doc= " # Panics"]
    #[doc= ""]
    #[doc= " This function panics if the given pointer is not well aligned to its"]
    #[doc= " type."]
    #[doc= ""]
    #[doc= " # Safety"]
    #[doc= ""]
    #[doc= " The provided pointer must be either null, or valid in the caller’s"]
    #[doc= " memory model and allocation regime."]
    #[cfg(feature = "alloc")]
    pub(crate) fn uninhabited(ptr: impl Into<Address<T>>) -> Self {
        let ptr = ptr.into();
        assert!(
            (ptr.u()).trailing_zeros() as usize >= Self::PTR_HEAD_BITS,
            "Pointer {:p} does not satisfy minimum alignment requirements {}",
            ptr.r(),
            Self::PTR_HEAD_BITS
        );
        Self {
            _ty: PhantomData,
            ptr: NonNull::new(ptr.w() as *mut u8).unwrap_or_else(NonNull::dangling),
            len: 0,
        }
    }

    #[doc= " Creates a new `BitPtr` from its components."]
    #[doc= ""]
    #[doc= " # Parameters"]
    #[doc= ""]
    #[doc= " - `data`: A well-aligned pointer to a storage element."]
    #[doc= " - `head`: The bit index of the first live bit in the element under"]
    #[doc= "   `*data`."]
    #[doc= " - `bits`: The number of live bits in the region the produced `BitPtr<T>`"]
    #[doc= "   describes."]
    #[doc= ""]
    #[doc= " # Returns"]
    #[doc= ""]
    #[doc= " If `data` is the null pointer, then this function produces the canonical"]
    #[doc= " empty slice. If `bits` is `0`, then this function produces an"]
    #[doc= " uninhabited slice at `data`. Otherwise, this produces a `BitPtr<T>`"]
    #[doc= " structure of the region described by the arguments."]
    #[doc= ""]
    #[doc= " # Panics"]
    #[doc= ""]
    #[doc= " This function panics in the following events:"]
    #[doc= ""]
    #[doc= " - `data` is not well aligned to `T`’s requirements."]
    #[doc= " - `bits` is larger than `Self::MAX_BITS`."]
    #[doc= " - `data` and `bits` describe a `[T]` slice which wraps around the edge"]
    #[doc= "   of the memory space."]
    #[doc= ""]
    #[doc= " # Safety"]
    #[doc= ""]
    #[doc= " The caller must provide a `data` pointer and a `bits` counter which"]
    #[doc= " describe a `[T]` region which is correctly aligned and validly allocated"]
    #[doc= " in the caller’s memory space. The caller is responsible for ensuring"]
    #[doc= " that the slice of memory the produced `BitPtr<T>` describes is all"]
    #[doc= " governable in the caller’s context."]
    pub(crate) fn new(data: impl Into<Address<T>>, head: BitIdx<T::Mem>, bits: usize) -> Self {
        let data = data.into();
        if data.r().is_null() {
            return Self::empty();
        }
        assert!(
            data.u().trailing_zeros() as usize >= Self::PTR_HEAD_BITS,
            "BitPtr domain pointer ({:p}) to {} must be aligned to at least {}",
            data.r(),
            T::Mem::TYPENAME,
            Self::PTR_HEAD_BITS
        );
        assert!(bits <= Self::MAX_BITS, "BitPtr cannot address {} bits; the maximum is {}", bits, Self::MAX_BITS);
        let elts = head.span(bits).0;
        let tail = data.r().wrapping_add(elts);
        assert!(
            tail >= data.r(),
            "BitPtr region cannot wrap the address space: {:p} + {:02X} = {:p}",
            data.r(),
            elts,
            tail
        );
        unsafe {
            Self::new_unchecked(data, head, bits)
        }
    }

    #[doc= " Creates a new `BitPtr<T>` from its components, without any validity"]
    #[doc= " checks."]
    #[doc= ""]
    #[doc= " # Safety"]
    #[doc= ""]
    #[doc= " ***ABSOLUTELY NONE.*** This function *only* packs its arguments into the"]
    #[doc= " bit pattern of the `BitPtr<T>` type. It should only be used in contexts"]
    #[doc= " where a previously extant `BitPTR<T>` was constructed with ancestry"]
    #[doc= " known to have survived [`::new`], and any manipulations of its raw"]
    #[doc= " components are known to be valid for reconstruction."]
    #[doc= ""]
    #[doc= " # Parameters"]
    #[doc= ""]
    #[doc= " See [`::new`]."]
    #[doc= ""]
    #[doc= " # Returns"]
    #[doc= ""]
    #[doc= " See [`::new`]."]
    #[doc= ""]
    #[doc= " [`::new`]: #method.new"]
    pub(crate) unsafe fn new_unchecked(data: impl Into<Address<T>>, head: BitIdx<T::Mem>, bits: usize) -> Self {
        let (data, head) = (data.into(), *head as usize);
        let ptr_data = data.u() & Self::PTR_DATA_MASK;
        let ptr_head = head >> Self::LEN_HEAD_BITS;
        let len_head = head & Self::LEN_HEAD_MASK;
        let len_bits = bits << Self::LEN_HEAD_BITS;
        let ptr: Address<u8> = (ptr_data | ptr_head).into();
        Self {
            _ty: PhantomData,
            ptr: NonNull::new_unchecked(ptr.w()),
            len: len_bits | len_head,
        }
    }

    #[doc= " Extracts the pointer to the first storage element."]
    #[doc= ""]
    #[doc= " # Parameters"]
    #[doc= ""]
    #[doc= " - `&self`"]
    #[doc= ""]
    #[doc= " # Returns"]
    #[doc= ""]
    #[doc= " A pointer to the first storage element in the slice domain. The concrete"]
    #[doc= " type returned is opaque, and unusable outside this library."]
    #[doc= ""]
    #[doc= " # Safety"]
    #[doc= ""]
    #[doc= " This pointer must be valid in the user’s memory model and allocation"]
    #[doc= " regime in order for the caller to dereference it."]
    #[inline]
    pub(crate) fn pointer(&self) -> Address<T> {
        (self.ptr.as_ptr() as usize & Self::PTR_DATA_MASK).into()
    }

    #[doc= " Overwrites the data pointer with a new address. This method does not"]
    #[doc= " perform safety checks on the new pointer."]
    #[doc= ""]
    #[doc= " # Parameters"]
    #[doc= ""]
    #[doc= " - `&mut self`"]
    #[doc= " - `ptr`: The new address of the `BitPtr<T>`’s domain."]
    #[doc= ""]
    #[doc= " # Safety"]
    #[doc= ""]
    #[doc= " None. The invariants of `::new` must be checked at the caller."]
    #[inline]
    #[cfg(feature = "alloc")]
    pub(crate) unsafe fn set_pointer(&mut self, ptr: impl Into<Address<T>>) {
        let mut data = ptr.into();
        if data.r().is_null() {
            *self = Self::empty();
            return;
        }
        data.u &= Self::PTR_DATA_MASK;
        data.u |= self.ptr.as_ptr() as usize & Self::PTR_HEAD_MASK;
        self.ptr = NonNull::new_unchecked(data.w() as *mut u8);
    }

    #[doc= " Extracts the element cursor of the head bit."]
    #[doc= ""]
    #[doc= " # Parameters"]
    #[doc= ""]
    #[doc= " - `&self`"]
    #[doc= ""]
    #[doc= " # Returns"]
    #[doc= ""]
    #[doc= " A `BitIdx` that is the index of the first live bit in the first element."]
    #[doc= " This will be in the domain `0 .. T::Mem::BITS`."]
    #[inline]
    pub(crate) fn head(&self) -> BitIdx<T::Mem> {
        let ptr = self.ptr.as_ptr() as usize;
        let ptr_head = (ptr & Self::PTR_HEAD_MASK) << Self::LEN_HEAD_BITS;
        let len_head = self.len & Self::LEN_HEAD_MASK;
        ((ptr_head | len_head) as u8).idx()
    }

    #[cfg(feature = "alloc")]
    pub(crate) unsafe fn set_head(&mut self, head: BitIdx<T::Mem>) {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    #[doc= " Counts how many bits are in the domain of a `BitPtr` slice."]
    #[doc= ""]
    #[doc= " # Parameters"]
    #[doc= ""]
    #[doc= " - `&self`"]
    #[doc= ""]
    #[doc= " # Returns"]
    #[doc= ""]
    #[doc= " A count of the live bits in the slice."]
    #[inline]
    pub(crate) fn len(&self) -> usize {
        self.len >> Self::LEN_HEAD_BITS
    }

    #[doc= " Overwrites the bit count with a new counter. This does not perform any"]
    #[doc= " safety checks."]
    #[doc= ""]
    #[doc= " # Parameters"]
    #[doc= ""]
    #[doc= " - `&mut self`"]
    #[doc= " - `len: usize`: A new bit length for the `BitPtr<T>`’s domain."]
    #[doc= ""]
    #[doc= " # Safety"]
    #[doc= ""]
    #[doc= " None. The caller must ensure that the invariants of `::new` are upheld."]
    #[inline]
    pub(crate) unsafe fn set_len(&mut self, len: usize) {
        let n = (len << Self::LEN_HEAD_BITS) | (self.len & Self::LEN_HEAD_MASK);
        self.len = n;
    }

    #[doc= " Produces the raw components of the pointer structure."]
    #[doc= ""]
    #[doc= " # Parameters"]
    #[doc= ""]
    #[doc= " - `&self`"]
    #[doc= ""]
    #[doc= " # Returns"]
    #[doc= ""]
    #[doc= " - `.0`: An opaque pointer to the `BitPtr<T>`’s memory region."]
    #[doc= " - `.1`: The index of the first live bit in the bit region."]
    #[doc= " - `.2`: The number of live bits in the bit region."]
    #[inline]
    pub(crate) fn raw_parts(&self) -> (Address<T>, BitIdx<T::Mem>, usize) {
        (self.pointer(), self.head(), self.len())
    }

    #[doc= " Produces the count of all elements in the slice domain."]
    #[doc= ""]
    #[doc= " # Parameters"]
    #[doc= ""]
    #[doc= " - `&self`"]
    #[doc= ""]
    #[doc= " # Returns"]
    #[doc= ""]
    #[doc= " The number of `T` elements in the slice domain."]
    #[doc= ""]
    #[doc= " # Safety"]
    #[doc= ""]
    #[doc= " This size must be valid in the user’s memory model and allocation"]
    #[doc= " regime."]
    pub(crate) fn elements(&self) -> usize {
        self.head().span(self.len()).0
    }

    #[doc= " Extracts the element cursor of the first dead bit *after* the tail bit."]
    #[doc= ""]
    #[doc= " # Parameters"]
    #[doc= ""]
    #[doc= " - `&self`"]
    #[doc= ""]
    #[doc= " # Returns"]
    #[doc= ""]
    #[doc= " A `BitTail` that is the index of the first dead bit after the last live"]
    #[doc= " bit in the last element. This will almost always be in the domain"]
    #[doc= " `1 ..= T::BITS`."]
    #[cfg(any(test, feature = "alloc"))]
    #[inline]
    pub(crate) fn tail(&self) -> BitTail<T::Mem> {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    pub(crate) fn aliased_slice<'a>(&self) -> &'a [T::Alias] {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    #[doc= " Accesses the element slice behind the pointer as a Rust slice."]
    #[doc= ""]
    #[doc= " # Parameters"]
    #[doc= ""]
    #[doc= " - `&self`"]
    #[doc= ""]
    #[doc= " # Returns"]
    #[doc= ""]
    #[doc= " Standard Rust slice handle over the data governed by this pointer."]
    #[doc= ""]
    #[doc= " # Lifetimes"]
    #[doc= ""]
    #[doc= " - `'a`: Lifetime for which the data behind the pointer is live."]
    #[inline]
    pub(crate) fn as_slice<'a>(&self) -> &'a [T] {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    #[doc= " Accesses the element slice behind the pointer as a Rust mutable slice."]
    #[doc= ""]
    #[doc= " # Parameters"]
    #[doc= ""]
    #[doc= " - `&self`"]
    #[doc= ""]
    #[doc= " # Returns"]
    #[doc= ""]
    #[doc= " Standard Rust slice handle over the data governed by this pointer."]
    #[doc= ""]
    #[doc= " # Lifetimes"]
    #[doc= ""]
    #[doc= " - `'a`: Lifetime for which the data behind the pointer is live."]
    #[inline]
    #[cfg(feature = "alloc")]
    pub(crate) fn as_mut_slice<'a>(&self) -> &'a mut [T] {
        unsafe {
            slice::from_raw_parts_mut(self.pointer().w, self.elements())
        }
    }

    #[doc= " Accesses the element slice behind the pointer as a shared-mutable slice."]
    #[doc= ""]
    #[doc= " # Parameters"]
    #[doc= ""]
    #[doc= " - `&self`"]
    #[doc= ""]
    #[doc= " # Returns"]
    #[doc= ""]
    #[doc= " Standard Rust slice handle over the data governed by this pointer."]
    #[doc= ""]
    #[doc= " # Lifetimes"]
    #[doc= ""]
    #[doc= " - `'a`: Lifetime for which the data behind the pointer is live."]
    #[inline]
    #[cfg(feature = "alloc")]
    pub(crate) fn as_access_slice<'a>(&self) -> &'a [T::Access] {
        unsafe {
            slice::from_raw_parts(self.pointer().a, self.elements())
        }
    }

    #[doc= " Converts a `BitSlice` handle into its `BitPtr` representation."]
    #[doc= ""]
    #[doc= " # Parameters"]
    #[doc= ""]
    #[doc= " - `bs`: a `BitSlice` handle"]
    #[doc= ""]
    #[doc= " # Returns"]
    #[doc= ""]
    #[doc= " The `BitPtr<T>` structure composing the handle."]
    pub(crate) fn from_bitslice<O>(bs: &BitSlice<O, T>) -> Self
    where
        O: BitOrder {
        let src = unsafe {
            &*(bs as *const BitSlice<O, T> as *const [()])
        };
        let ptr = Address::from(src.as_ptr() as *const u8);
        let (ptr, len) = match (ptr.w(), src.len()) {
            (_, 0) => (NonNull::dangling(), 0),
            (p, _) if p.is_null() => unreachable!("Rust forbids null refs"),
            (p, l) => (unsafe {
                NonNull::new_unchecked(p)
            }, l),
        };
        Self {
            ptr,
            len,
            _ty: PhantomData,
        }
    }

    #[doc= " Converts a `BitPtr` structure into an immutable `BitSlice` handle."]
    #[doc= ""]
    #[doc= " # Parameters"]
    #[doc= ""]
    #[doc= " - `self`"]
    #[doc= ""]
    #[doc= " # Returns"]
    #[doc= ""]
    #[doc= " A `BitSlice` handle composed of the `BitPtr` structure."]
    pub(crate) fn into_bitslice<'a, O>(self) -> &'a BitSlice<O, T>
    where
        O: BitOrder {
        unsafe {
            &*(slice::from_raw_parts(Address::from(self.ptr.as_ptr()).r() as *const (), self.len) as *const [()] as
                *const BitSlice<O, T>)
        }
    }

    #[doc= " Converts a `BitPtr` structure into a mutable `BitSlice` handle."]
    #[doc= ""]
    #[doc= " # Parameters"]
    #[doc= ""]
    #[doc= " - `self`"]
    #[doc= ""]
    #[doc= " # Returns"]
    #[doc= ""]
    #[doc= " A `BitSlice` handle composed of the `BitPtr` structure."]
    pub(crate) fn into_bitslice_mut<'a, O>(self) -> &'a mut BitSlice<O, T>
    where
        O: BitOrder {
        unsafe {
            &mut *(slice::from_raw_parts_mut(Address::from(self.ptr.as_ptr()).w() as *mut (), self.len) as
                *mut [()] as
                *mut BitSlice<O, T>)
        }
    }

    #[doc= " Cast a `BitPtr<T>` into an equivalent `*mut BitSlice<O, T>`."]
    #[cfg(feature = "alloc")]
    pub(crate) fn as_mut_ptr<O>(self) -> *mut BitSlice<O, T>
    where
        O: BitOrder {
        self.into_bitslice_mut() as *mut BitSlice<O, T>
    }

    #[doc= " Cast a `*mut BitSlice<O, T>` raw pointer into an equivalent `BitPtr<T>`."]
    #[cfg(feature = "alloc")]
    pub(crate) fn from_mut_ptr<O>(ptr: *mut BitSlice<O, T>) -> Self
    where
        O: BitOrder {
        unsafe {
            &*ptr
        }.bitptr()
    }
}

impl<T> Clone for BitPtr<T>
where
    T: BitStore {
    fn clone(&self) -> Self {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<'a, O, T> From<&'a BitSlice<O, T>> for BitPtr<T>
where
    O: BitOrder,
    T: 'a + BitStore {
    fn from(src: &'a BitSlice<O, T>) -> Self {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<'a, O, T> From<&'a mut BitSlice<O, T>> for BitPtr<T>
where
    O: BitOrder,
    T: 'a + BitStore {
    fn from(src: &'a mut BitSlice<O, T>) -> Self {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

#[doc= " Produces the empty-slice representation."]
impl<T> Default for BitPtr<T>
where
    T: BitStore {
    #[doc= " Produces an empty-slice representation."]
    #[doc= ""]
    #[doc= " The empty slice has no size or cursors, and its pointer is the alignment"]
    #[doc= " of the type. The non-null pointer allows this structure to be null-value"]
    #[doc= " optimized."]
    fn default() -> Self {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

#[doc= " Prints the `BitPtr` data structure for debugging."]
impl<T> Debug for BitPtr<T>
where
    T: BitStore {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}

impl<T> Copy for BitPtr<T>
where
    T: BitStore { }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn associated_consts_u8() {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    #[test]
    fn associated_consts_u16() {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    #[test]
    fn associated_consts_u32() {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    #[cfg(target_pointer_width = "64")]
    #[test]
    fn associated_consts_u64() {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    #[test]
    fn ctors() {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    #[test]
    fn empty() {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    #[cfg(not(miri))]
    #[test]
    #[should_panic]
    fn overfull() {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}
