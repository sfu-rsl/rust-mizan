#![doc= " Reimplementation of the standard library’s `Vec` inherent method API."]

use crate::{
    mem::BitMemory,
    order::BitOrder,
    pointer::BitPtr,
    store::BitStore,
    vec::{
        BitVec,
    },
};
use alloc::{
    boxed::Box,
    vec::Vec,
};
use core::{
    marker::PhantomData,
    mem,
};
use funty::IsInteger;

impl<O, T> BitVec<O, T>
where
    O: BitOrder,
    T: BitStore {
    #[doc= " Constructs a new, empty `BitVec<O, T>` with the specified capacity."]
    #[doc= ""]
    #[doc= " The vector will be able to hold at least `capacity` bits without"]
    #[doc= " reallocating. If `capacity` is 0, the vector will not allocate."]
    #[doc= ""]
    #[doc= " It is important to note that although the returned vector has the"]
    #[doc= " *capacity* specified, the vector will have a zero *length*. For an"]
    #[doc= " explanation of the difference between length and capacity, see"]
    #[doc= " [*Capacity and reallocation*]."]
    #[doc= ""]
    #[doc= " [*Capacity and reallocation*]: #capacity-and-reallocation"]
    pub fn with_capacity(capacity: usize) -> Self {
        let elts = T::Mem::elts(capacity);
        let v = Vec::with_capacity(elts);
        let (ptr, cap) = (v.as_ptr(), v.capacity());
        mem::forget(v);
        Self {
            _order: PhantomData,
            pointer: BitPtr::uninhabited(ptr),
            capacity: cap,
        }
    }

    #[doc= " Returns the number of bits the vector can hold without reallocating."]
    #[doc= ""]
    #[doc= " # Examples"]
    #[doc= ""]
    #[doc= " ```rust"]
    #[doc= " # use bitvec::prelude::*;"]
    #[doc= " let bv: BitVec<Local, usize> = BitVec::with_capacity(100);"]
    #[doc= " assert!(bv.capacity() >= 100);"]
    #[inline]
    pub(crate) fn capacity(&self) -> usize {
        self.capacity.checked_mul(T::Mem::BITS as usize).expect("Vector capacity overflow")
    }

    #[doc= " Converts the bit-vector into [`Box<[T]>`]."]
    #[doc= ""]
    #[doc= " Note that this will drop any excess capacity."]
    #[doc= ""]
    #[doc= " For the vec-to-box equivalent that produces a [`BitBox<O, T>`], see"]
    #[doc= " [`into_boxed_bitslice`]."]
    #[doc= ""]
    #[doc= " # Examples"]
    #[doc= ""]
    #[doc= " ```rust"]
    #[doc= " # use bitvec::prelude::*;"]
    #[doc= " let bv = bitvec![1, 0, 1];"]
    #[doc= ""]
    #[doc= " let slice = bv.into_boxed_slice();"]
    #[doc= " ```"]
    #[doc= ""]
    #[doc= " Any excess capacity is removed:"]
    #[doc= ""]
    #[doc= " ```rust"]
    #[doc= " # use bitvec::prelude::*;"]
    #[doc= " let mut bv = BitVec::<Local, usize>::with_capacity(100);"]
    #[doc= " bv.extend([true, false, true].iter().copied());"]
    #[doc= ""]
    #[doc= " assert!(bv.capacity() >= 100);"]
    #[doc= " let slice = bv.into_boxed_slice();"]
    #[doc= " let boxed_bitslice = BitBox::<Local, usize>::from_boxed_slice(slice);"]
    #[doc= " let bv = BitVec::from_boxed_bitslice(boxed_bitslice);"]
    #[doc= " assert!(bv.capacity() >= 3);"]
    #[doc= " ```"]
    #[doc= ""]
    #[doc= " [`BitBox<O, T>`]: ../boxed/struct.BitBox.html"]
    #[doc= " [`Box<[T]>`]: https://doc.rust-lang.org/std/boxed/struct.Box.html"]
    #[doc= " [`into_boxed_bitslice`]: #method.into_boxed_bitslice"]
    #[inline]
    pub(crate) fn into_boxed_slice(self) -> Box<[T]> {
        self.into_vec().into_boxed_slice()
    }

    #[doc= " Extracts a mutable slice of the entire vector."]
    #[doc= ""]
    #[doc= " Unlike [`BitSlice::as_mut_slice`], this will produce partial edge"]
    #[doc= " elements, as they are known to not be aliased by any other slice"]
    #[doc= " handles."]
    #[doc= ""]
    #[doc= " # Examples"]
    #[doc= ""]
    #[doc= " ```rust"]
    #[doc= " # use bitvec::prelude::*;"]
    #[doc= " # #[cfg(feature = \"std\")] {"]
    #[doc= " use std::io::{self, Read};"]
    #[doc= " let mut buffer = bitvec![Local, u8; 0; 24];"]
    #[doc= " io::repeat(0xA5u8).read_exact(buffer.as_mut_slice()).unwrap();"]
    #[doc= " # }"]
    #[doc= " ```"]
    #[doc= ""]
    #[doc= " [`BitSlice::as_mut_slice`]:"]
    #[doc= " ../slice/struct.BitSlice.html#method.as_mut_slice"]
    #[inline]
    pub(crate) fn as_mut_slice(&mut self) -> &mut [T] {
        self.pointer.as_mut_slice()
    }

    #[doc= " Forces the length of the vector to `new_len`."]
    #[doc= ""]
    #[doc= " This is a low-level operation that maintains none of the normal"]
    #[doc= " invariants of the type. Normally changing the length of a vector is done"]
    #[doc= " using one of the safe operations instead, such as [`truncate`],"]
    #[doc= " [`resize`], [`extend`], or [`clear`]."]
    #[doc= ""]
    #[doc= " # Safety"]
    #[doc= ""]
    #[doc= " - `new_len` must be less than or equal to [`capacity()`]."]
    #[doc= " - The underlying elements at `old_len ..new_len` must be initialized."]
    #[doc= ""]
    #[doc= " # Examples"]
    #[doc= ""]
    #[doc= " This method can be useful for situations in which the vector is serving"]
    #[doc= " as a buffer for other code, particularly over FFI."]
    #[doc= ""]
    #[doc= " ```rust"]
    #[doc= " # use bitvec::prelude::*;"]
    #[doc= " let mut bv = BitVec::<Local, usize>::with_capacity(17);"]
    #[doc= " assert!(bv.is_empty());"]
    #[doc= " unsafe { bv.set_len(23) };"]
    #[doc= " assert_eq!(bv.len(), 23);"]
    #[doc= " ```"]
    #[doc= ""]
    #[doc= " This example executes correctly, because the allocator can only reserve"]
    #[doc= " even multiples of bytes, and so rounds up from the `with_capacity`"]
    #[doc= " argument."]
    #[doc= ""]
    #[doc= " [`capacity()`]: #method.capacity"]
    #[doc= " [`clear`]: #method.clear"]
    #[doc= " [`extend`]: #method.extend"]
    #[doc= " [`resize`]: #method.resize"]
    #[doc= " [`truncate`]: #method.truncate"]
    pub(crate) unsafe fn set_len(&mut self, new_len: usize) {
        assert!(
            new_len <= BitPtr::<T>::MAX_BITS,
            "Capacity overflow: {} overflows maximum length {}",
            new_len,
            BitPtr::<T>::MAX_BITS
        );
        let cap = self.capacity();
        assert!(new_len <= cap, "Capacity overflow: {} overflows allocation size {}", new_len, cap);
        self.pointer.set_len(new_len);
    }

    #[doc= " Appends a bit to the back of the vector."]
    #[doc= ""]
    #[doc= " If the vector is at capacity, this may cause a reallocation."]
    #[doc= ""]
    #[doc= " # Panics"]
    #[doc= ""]
    #[doc= " This will panic if the push will cause the vector to allocate above"]
    #[doc= " `BitPtr<T>::MAX_ELTS` or machine capacity."]
    #[doc= ""]
    #[doc= " # Examples"]
    #[doc= ""]
    #[doc= " ```rust"]
    #[doc= " # use bitvec::prelude::*;"]
    #[doc= " let mut bv: BitVec = BitVec::new();"]
    #[doc= " assert!(bv.is_empty());"]
    #[doc= " bv.push(true);"]
    #[doc= " assert_eq!(bv.len(), 1);"]
    #[doc= " assert!(bv[0]);"]
    #[doc= " ```"]
    pub fn push(&mut self, value: bool) {
        let len = self.len();
        assert!(len <= BitPtr::<T>::MAX_BITS, "Capacity overflow: {} >= {}", len, BitPtr::<T>::MAX_BITS);
        if self.is_empty() || *self.pointer.tail() == T::Mem::BITS {
            self.with_vec(|v| v.push(T::Mem::ZERO.into()));
        }
        unsafe {
            self.pointer.set_len(len + 1);
            self.set_unchecked(len, value);
        }
    }
}
