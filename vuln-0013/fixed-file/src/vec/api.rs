#![doc= " Reimplementation of the standard library’s `Vec` inherent method API."]

use crate::{
    mem::BitMemory,
    order::BitOrder,
    pointer::BitPtr,
    slice::BitSlice,
    store::BitStore,
    vec::{
        iter::{
            Drain,
            Splice,
        },
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
    ops::RangeBounds,
};
use funty::IsInteger;

impl<O, T> BitVec<O, T>
where
    O: BitOrder,
    T: BitStore {
    #[doc= " Constructs a new, empty `BitVec<O, T>`."]
    #[doc= ""]
    #[doc= " The vector will not allocate until elements are pushed onto it."]
    #[doc= ""]
    #[doc= " ```rust"]
    #[doc= " # use bitvec::prelude::*;"]
    #[doc= " let mut bv: BitVec<Local, usize> = BitVec::new();"]
    #[doc= " ```"]
    #[inline]
    pub(crate) fn new() -> Self {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

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

    #[doc= " Reserves capacity for at least `additional` more bits to be inserted in"]
    #[doc= " the given `BitVec<O, T>`. The collection may reserve more space to avoid"]
    #[doc= " frequent reallocations. After calling `reserve`, the capacity will be"]
    #[doc= " greater than or equal to `self.len() + additional`. Does nothing if the"]
    #[doc= " capacity is already sufficient."]
    #[doc= ""]
    #[doc= " # Panics"]
    #[doc= ""]
    #[doc= " Panics if the new capacity overflows `BitPtr::<T>::MAX_BITS`."]
    #[doc= ""]
    #[doc= " # Examples"]
    #[doc= ""]
    #[doc= " ```rust"]
    #[doc= " # use bitvec::prelude::*;"]
    #[doc= " let mut bv = bitvec![1];"]
    #[doc= " bv.reserve(10);"]
    #[doc= " assert!(bv.capacity() >= 11);"]
    #[doc= " ```"]
    pub(crate) fn reserve(&mut self, additional: usize) {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    #[doc= " Reserves the minimum capacity for exactly `additional` more bits to be"]
    #[doc= " inserted in the given `BitVec<O, T>`. After calling `reserve_exact`,"]
    #[doc= " capacity will be greater than or equal to `self.len() + additional`."]
    #[doc= " Does nothing if the capacity is already sufficient."]
    #[doc= ""]
    #[doc= " Note that the allocator may give the collection more space than it"]
    #[doc= " requests. Therefore, capacity can not be relied upon to be precisely"]
    #[doc= " minimal. Prefer `reserve` if future insertions are expected."]
    #[doc= ""]
    #[doc= " # Panics"]
    #[doc= ""]
    #[doc= " Panics if the new capacity overflows `BitPtr::<T>::MAX_BITS`."]
    #[doc= ""]
    #[doc= " # Examples"]
    #[doc= ""]
    #[doc= " ```rust"]
    #[doc= " # use bitvec::prelude::*;"]
    #[doc= " let mut bv = bitvec![1];"]
    #[doc= " bv.reserve_exact(10);"]
    #[doc= " assert!(bv.capacity() >= 11);"]
    #[doc= " ```"]
    pub(crate) fn reserve_exact(&mut self, additional: usize) {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    #[doc= " Shrinks the capacity of the vector as much as possible."]
    #[doc= ""]
    #[doc= " It will drop down as close as possible to the length but the allocator"]
    #[doc= " may still inform the vector that there is space for a few more elements."]
    #[doc= ""]
    #[doc= " # Examples"]
    #[doc= ""]
    #[doc= " ```rust"]
    #[doc= " # use bitvec::prelude::*;"]
    #[doc= " let mut bv: BitVec<Local, usize> = BitVec::with_capacity(10);"]
    #[doc= " bv.extend([true, false, true].iter().copied());"]
    #[doc= " assert!(bv.capacity() >= 10);"]
    #[doc= " bv.shrink_to_fit();"]
    #[doc= " assert!(bv.capacity() >= 3);"]
    #[doc= " ```"]
    #[inline]
    pub(crate) fn shrink_to_fit(&mut self) {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
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

    #[doc= " Shortens the vector, keeping the first `len` bits and dropping the rest."]
    #[doc= ""]
    #[doc= " If `len` is greater than the vector’s current length, this has no"]
    #[doc= " effect."]
    #[doc= ""]
    #[doc= " The [`drain`] method can emulate `truncate`, but causes the excess bits"]
    #[doc= " to be returned instead of dropped."]
    #[doc= ""]
    #[doc= " Note that this method has no effect on the allocated capacity of the"]
    #[doc= " vector."]
    #[doc= ""]
    #[doc= " # Examples"]
    #[doc= ""]
    #[doc= " Truncating a five-bit vector to two bits:"]
    #[doc= ""]
    #[doc= " ```rust"]
    #[doc= " # use bitvec::prelude::*;"]
    #[doc= " let mut bv = bitvec![1, 0, 1, 0, 1];"]
    #[doc= " bv.truncate(2);"]
    #[doc= " assert_eq!(bv, bitvec![1, 0]);"]
    #[doc= " ```"]
    #[doc= ""]
    #[doc= " No truncation occurs when `len` is greater than the vector’s current"]
    #[doc= " length:"]
    #[doc= ""]
    #[doc= " ```rust"]
    #[doc= " # use bitvec::prelude::*;"]
    #[doc= " let mut bv = bitvec![1; 5];"]
    #[doc= " bv.truncate(10);"]
    #[doc= " assert_eq!(bv, bitvec![1; 5]);"]
    #[doc= " ```"]
    #[doc= ""]
    #[doc= " Truncating to zero is equivalent to calling the [`clear`] method."]
    #[doc= ""]
    #[doc= " ```rust"]
    #[doc= " # use bitvec::prelude::*;"]
    #[doc= " let mut bv = bitvec![0; 5];"]
    #[doc= " bv.truncate(0);"]
    #[doc= " assert!(bv.is_empty());"]
    #[doc= " ```"]
    #[doc= ""]
    #[doc= " [`clear`]: #method.clear"]
    #[doc= " [`drain`]: #method.drain"]
    #[inline]
    pub(crate) fn truncate(&mut self, len: usize) {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    #[doc= " Extracts an element slice containing the entire vector."]
    #[doc= ""]
    #[doc= " Unlike [`BitSlice::as_slice`], this will produce partial edge elements,"]
    #[doc= " as they are known to not be aliased by any other slice handles."]
    #[doc= ""]
    #[doc= " # Examples"]
    #[doc= ""]
    #[doc= " ```rust"]
    #[doc= " # use bitvec::prelude::*;"]
    #[doc= " # #[cfg(feature = \"std\")] {"]
    #[doc= " use std::io::{self, Write};"]
    #[doc= " let buffer = bitvec![Local, u8; 1, 0, 1, 0, 1];"]
    #[doc= " io::sink().write(buffer.as_slice()).unwrap();"]
    #[doc= " # }"]
    #[doc= " ```"]
    #[doc= ""]
    #[doc= " [`BitSlice::as_slice`]: ../slice/struct.BitSlice.html#method.as_slice"]
    #[inline]
    pub(crate) fn as_slice(&self) -> &[T] {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
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

    #[doc= " Removes a bit from the vector and returns it."]
    #[doc= ""]
    #[doc= " The removed bit is replaced by the last bit of the vector."]
    #[doc= ""]
    #[doc= " This does not preserve ordering, but is O(1)."]
    #[doc= ""]
    #[doc= " # Panics"]
    #[doc= ""]
    #[doc= " Panics if `index` is out of bounds."]
    #[doc= ""]
    #[doc= " # Examples"]
    #[doc= ""]
    #[doc= " ```rust"]
    #[doc= " # use bitvec::prelude::*;"]
    #[doc= " let mut bv = bitvec![1, 0, 1, 0, 1];"]
    #[doc= ""]
    #[doc= " assert!(!bv.swap_remove(1));"]
    #[doc= " assert_eq!(bv, bitvec![1, 1, 1, 0]);"]
    #[doc= ""]
    #[doc= " assert!(bv.swap_remove(0));"]
    #[doc= " assert_eq!(bv, bitvec![0, 1, 1]);"]
    #[doc= " ```"]
    pub(crate) fn swap_remove(&mut self, index: usize) -> bool {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    #[doc= " Inserts a bit at position `index` within the vector, shifting all bits"]
    #[doc= " after it to the right."]
    #[doc= ""]
    #[doc= " # Panics"]
    #[doc= ""]
    #[doc= " Panics if `index > len`."]
    #[doc= ""]
    #[doc= " # Examples"]
    #[doc= ""]
    #[doc= " ```rust"]
    #[doc= " # use bitvec::prelude::*;"]
    #[doc= " let mut bv = bitvec![1, 0, 1, 0, 1];"]
    #[doc= " bv.insert(1, false);"]
    #[doc= " assert_eq!(bv, bitvec![1, 0, 0, 1, 0, 1]);"]
    #[doc= " bv.insert(4, true);"]
    #[doc= " assert_eq!(bv, bitvec![1, 0, 0, 1, 1, 0, 1]);"]
    #[doc= " ```"]
    pub(crate) fn insert(&mut self, index: usize, value: bool) {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    #[doc= " Removes and returns the bit at position `index` within the vector,"]
    #[doc= " shifting all bits after it to the left."]
    #[doc= ""]
    #[doc= " # Panics"]
    #[doc= ""]
    #[doc= " Panics if `index` is out of bounds."]
    #[doc= ""]
    #[doc= " # Examples"]
    #[doc= ""]
    #[doc= " ```rust"]
    #[doc= " # use bitvec::prelude::*;"]
    #[doc= " let mut bv = bitvec![1, 0, 1, 0, 1];"]
    #[doc= " assert!(!bv.remove(1));"]
    #[doc= " assert_eq!(bv, bitvec![1, 1, 0, 1]);"]
    #[doc= " ```"]
    pub(crate) fn remove(&mut self, index: usize) -> bool {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    #[doc= " Retains only the bits that pass the predicate."]
    #[doc= ""]
    #[doc= " This removes all bits `b` where `f(e)` returns `false`. This method"]
    #[doc= " operates in place and preserves the order of the retained bits. Because"]
    #[doc= " it is in-place, it operates in `O(n²)` time."]
    #[doc= ""]
    #[doc= " # API Differences"]
    #[doc= ""]
    #[doc= " The [`Vec::retain`] method takes a predicate function with signature"]
    #[doc= " `(&T) -> bool`, whereas this method’s predicate function has signature"]
    #[doc= " `(usize, &T) -> bool`. This difference is in place because `BitSlice` by"]
    #[doc= " definition has only one bit of information per slice item, and including"]
    #[doc= " the index allows the callback function to make more informed choices."]
    #[doc= ""]
    #[doc= " # Examples"]
    #[doc= ""]
    #[doc= " ```rust"]
    #[doc= " # use bitvec::prelude::*;"]
    #[doc= " let mut bv = bitvec![0, 1, 0, 1, 0, 1];"]
    #[doc= " bv.retain(|_, b| b);"]
    #[doc= " assert_eq!(bv, bitvec![1, 1, 1]);"]
    #[doc= " ```"]
    #[doc= ""]
    #[doc= " [`BitSlice::for_each`]: ../slice/struct.BitSlice.html#method.for_each"]
    pub(crate) fn retain<F>(&mut self, mut pred: F)
    where
        F: FnMut(usize, bool) -> bool {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
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

    #[doc= " Removes the last element from a vector and returns it, or `None` if it"]
    #[doc= " is empty."]
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
    #[doc= ""]
    #[doc= " assert!(bv.pop().unwrap());"]
    #[doc= " assert!(bv.is_empty());"]
    #[doc= " assert!(bv.pop().is_none());"]
    #[doc= " ```"]
    pub(crate) fn pop(&mut self) -> Option<bool> {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    #[doc= " Moves all the elements of `other` into `self`, leaving `other` empty."]
    #[doc= ""]
    #[doc= " # Panics"]
    #[doc= ""]
    #[doc= " Panics if the number of bits in the vector overflows"]
    #[doc= " `BitPtr::<T>::MAX_ELTS`."]
    #[doc= ""]
    #[doc= " # Examples"]
    #[doc= ""]
    #[doc= " ```rust"]
    #[doc= " # use bitvec::prelude::*;"]
    #[doc= " let mut bv1 = bitvec![0; 10];"]
    #[doc= " let mut bv2 = bitvec![1; 10];"]
    #[doc= " bv1.append(&mut bv2);"]
    #[doc= " assert_eq!(bv1.len(), 20);"]
    #[doc= " assert!(bv1[10]);"]
    #[doc= " assert!(bv2.is_empty());"]
    #[doc= " ```"]
    #[inline]
    pub(crate) fn append<D, U>(&mut self, other: &mut BitVec<D, U>)
    where
        D: BitOrder,
        U: BitStore {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    #[doc= " Creates a draining iterator that removes the specified range from the"]
    #[doc= " vector and yields the removed bits."]
    #[doc= ""]
    #[doc= " # Notes"]
    #[doc= ""]
    #[doc= " 1. The element range is removed even if the iterator is only partially"]
    #[doc= "    consumed or not consumed at all."]
    #[doc= " 2. It is unspecified how many bits are removed from the vector if the"]
    #[doc= "    `Drain` value is leaked."]
    #[doc= ""]
    #[doc= " # Panics"]
    #[doc= ""]
    #[doc= " Panics if the starting point is greater than the end point or if the end"]
    #[doc= " point is greater than the length of the vector."]
    #[doc= ""]
    #[doc= " # Examples"]
    #[doc= ""]
    #[doc= " ```rust"]
    #[doc= " # use bitvec::prelude::*;"]
    #[doc= " let mut bv = bitvec![0, 0, 1, 1, 1, 0, 0];"]
    #[doc= " assert_eq!(bv.len(), 7);"]
    #[doc= " for bit in bv.drain(2 .. 5) {"]
    #[doc= "   assert!(bit);"]
    #[doc= " }"]
    #[doc= " assert!(bv.not_any());"]
    #[doc= " assert_eq!(bv.len(), 4);"]
    #[doc= " ```"]
    pub(crate) fn drain<R>(&mut self, range: R) -> Drain<O, T>
    where
        R: RangeBounds<usize> {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    #[doc= " Clears the vector, removing all values."]
    #[doc= ""]
    #[doc= " Note that this method has no effect on the allocated capacity of the"]
    #[doc= " vector."]
    #[doc= ""]
    #[doc= " # Examples"]
    #[doc= ""]
    #[doc= " ```rust"]
    #[doc= " # use bitvec::prelude::*;"]
    #[doc= " let mut bv = bitvec![1; 30];"]
    #[doc= " assert_eq!(bv.len(), 30);"]
    #[doc= " assert!(bv.iter().all(|b| *b));"]
    #[doc= " bv.clear();"]
    #[doc= " assert!(bv.is_empty());"]
    #[doc= " ```"]
    #[doc= ""]
    #[doc= " After calling `clear()`, `bv` will no longer show raw memory, so the"]
    #[doc= " above test cannot show that the underlying memory is not altered. This"]
    #[doc= " is also an implementation detail on which you should not rely."]
    pub(crate) fn clear(&mut self) {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    #[doc= " Splits the collection into two at the given index."]
    #[doc= ""]
    #[doc= " Returns a newly allocated `Self`. `self` contains elements `[0, at)`,"]
    #[doc= " and the returned `Self` contains elements `[at, len)`."]
    #[doc= ""]
    #[doc= " Note that the capacity of `self` does not change."]
    #[doc= ""]
    #[doc= " # Panics"]
    #[doc= ""]
    #[doc= " Panics if `at > len`."]
    #[doc= ""]
    #[doc= " # Examples"]
    #[doc= ""]
    #[doc= " ```rust"]
    #[doc= " # use bitvec::prelude::*;"]
    #[doc= " let mut bv1 = bitvec![0, 0, 0, 1, 1, 1];"]
    #[doc= " let bv2 = bv1.split_off(3);"]
    #[doc= " assert_eq!(bv1, bitvec![0, 0, 0]);"]
    #[doc= " assert_eq!(bv2, bitvec![1, 1, 1]);"]
    #[doc= " ```"]
    pub(crate) fn split_off(&mut self, at: usize) -> Self {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    #[doc= " Resizes the `BitVec` in-place so that `len` is equal to `new_len`."]
    #[doc= ""]
    #[doc= " If `new_len` is greater than `len`, the `BitVec` is extended by the"]
    #[doc= " difference, with each additional slot filled with the result of calling"]
    #[doc= " the closure `f`. The return values from `f` will end up in the `BitVec`"]
    #[doc= " in the order they have been generated."]
    #[doc= ""]
    #[doc= " If `new_len` is less than `len`, the `BitVec` is simply truncated."]
    #[doc= ""]
    #[doc= " This method uses a closure to create new values on every push. If you’d"]
    #[doc= " rather [`Clone`] a given value, use [`resize`]. If you want to use the"]
    #[doc= " [`Default`] trait to generate values, you can pass"]
    #[doc= " [`Default::default()`] as the second argument."]
    #[doc= ""]
    #[doc= " # Examples"]
    #[doc= ""]
    #[doc= " ```rust"]
    #[doc= " # use bitvec::prelude::*;"]
    #[doc= " let mut bv = bitvec![1, 0, 1];"]
    #[doc= " bv.resize_with(5, Default::default);"]
    #[doc= " assert_eq!(bv, bitvec![1, 0, 1, 0, 0]);"]
    #[doc= ""]
    #[doc= " let mut bv = bitvec![];"]
    #[doc= " let mut p = 1;"]
    #[doc= " bv.resize_with(4, || {"]
    #[doc= "   p += 1;"]
    #[doc= "   p % 2 == 0"]
    #[doc= " });"]
    #[doc= " assert_eq!(bv, bitvec![1, 0, 1, 0]);"]
    #[doc= " ```"]
    #[doc= ""]
    #[doc= " [`Clone`]: https://doc.rust-lang.org/std/clone/trait.Clone.html"]
    #[doc= " [`Default`]: https://doc.rust-lang.org/std/default/trait.Default.html"]
    #[doc= " [`Default::default()`]: https://doc.rust-lang.org/std/default/trait.Default.html#tymethod.default"]
    #[doc= " [`resize`]: #method.resize"]
    pub(crate) fn resize_with<F>(&mut self, new_len: usize, mut f: F)
    where
        F: FnMut() -> bool {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    #[doc= " Resizes the `BitVec` in place so that `len` is equal to `new_len`."]
    #[doc= ""]
    #[doc= " If `new_len` is greater than `len`, the `BitVec` is extended by the"]
    #[doc= " difference, with each additional slot filled with `value`. If `new_len`"]
    #[doc= " is less than `len`, the `BitVec` is simply truncated."]
    #[doc= ""]
    #[doc= " # Examples"]
    #[doc= ""]
    #[doc= " ```rust"]
    #[doc= " # use bitvec::prelude::*;"]
    #[doc= " let mut bv = bitvec![0; 4];"]
    #[doc= " bv.resize(8, true);"]
    #[doc= " assert_eq!(bv, bitvec![0, 0, 0, 0, 1, 1, 1, 1]);"]
    #[doc= " bv.resize(5, false);"]
    #[doc= " assert_eq!(bv, bitvec![0, 0, 0, 0, 1]);"]
    #[doc= " ```"]
    pub(crate) fn resize(&mut self, new_len: usize, value: bool) {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    #[doc= " Clones and appends all bits in a bit-slice to the `BitVec`."]
    #[doc= ""]
    #[doc= " Iterates over the bit-slice `other`, clones each bit, and then appends"]
    #[doc= " it to this `BitVec`. The `other` slice is traversed in-order."]
    #[doc= ""]
    #[doc= " Note that this function is the same as [`extend`] except that it is"]
    #[doc= " specialized to work with bit-slices instead. If and when Rust gets"]
    #[doc= " specialization this function will likely be deprecated (but still"]
    #[doc= " available)."]
    #[doc= ""]
    #[doc= " # Examples"]
    #[doc= ""]
    #[doc= " ```rust"]
    #[doc= " # use bitvec::prelude::*;"]
    #[doc= " let mut bv = bitvec![1];"]
    #[doc= " bv.extend_from_slice(0xA5u8.bits::<Lsb0>());"]
    #[doc= " assert_eq!(bv, bitvec![1, 1, 0, 1, 0, 0, 1, 0, 1]);"]
    #[doc= " ```"]
    #[doc= ""]
    #[doc= " [`extend`]: #method.extend"]
    pub(crate) fn extend_from_slice<D, U>(&mut self, other: &BitSlice<D, U>)
    where
        D: BitOrder,
        U: BitStore {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }

    #[doc= " Creates a splicing iterator that replaces the specified range in the"]
    #[doc= " vector with the given `replace_with` iterator and yields the removed"]
    #[doc= " bits. `replace_with` does not need to be the same length as `range`."]
    #[doc= ""]
    #[doc= " # Notes"]
    #[doc= ""]
    #[doc= " 1. The element range is removed and replaced even if the iterator"]
    #[doc= "    produced by this method is not consumed until the end."]
    #[doc= " 2. It is unspecified how many bits are removed from the vector if the"]
    #[doc= "    `Splice` value is leaked."]
    #[doc= " 3. The input iterator `replace_with` is only consumed when the `Splice`"]
    #[doc= "    value is dropped."]
    #[doc= " 4. This is optimal if:"]
    #[doc= "    - the tail (elements in the vector after `range`) is empty,"]
    #[doc= "    - or `replace_with` yields fewer bits than `range`’s length,"]
    #[doc= "    - the lower bound of its `size_hint()` is exact."]
    #[doc= ""]
    #[doc= "    Otherwise, a temporary vector is allocated and the tail is moved"]
    #[doc= "    twice."]
    #[doc= ""]
    #[doc= " # Panics"]
    #[doc= ""]
    #[doc= " Panics if the starting point is greater than the end point or if the end"]
    #[doc= " point is greater than the length of the vector."]
    #[doc= ""]
    #[doc= " # Examples"]
    #[doc= ""]
    #[doc= " This example starts with six bits of zero, and then splices out bits 2"]
    #[doc= " and 3 and replaces them with four bits of one."]
    #[doc= ""]
    #[doc= " ```rust"]
    #[doc= " # use bitvec::prelude::*;"]
    #[doc= " let mut bv = bitvec![0; 6];"]
    #[doc= " let bv2 = bitvec![1; 4];"]
    #[doc= ""]
    #[doc= " let s = bv.splice(2 .. 4, bv2).collect::<BitVec>();"]
    #[doc= " assert_eq!(s.len(), 2);"]
    #[doc= " assert!(!s[0]);"]
    #[doc= " assert_eq!(bv, bitvec![0, 0, 1, 1, 1, 1, 0, 0]);"]
    #[doc= " ```"]
    pub(crate) fn splice<R, I>(&mut self, range: R, replace_with: I) -> Splice<O, T, <I as IntoIterator>::IntoIter>
    where
        I: IntoIterator<Item = bool>,
        R: RangeBounds<usize> {
        panic!("CARGO_MINIMIZE_PANIC_FAIL")
    }
}
