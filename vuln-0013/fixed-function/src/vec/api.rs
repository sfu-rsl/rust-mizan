//! Reimplementation of the standard library’s `Vec` inherent method API.

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
	T: BitStore,
{
	/// Constructs a new, empty `BitVec<O, T>` with the specified capacity.
	///
	/// The vector will be able to hold at least `capacity` bits without
	/// reallocating. If `capacity` is 0, the vector will not allocate.
	///
	/// It is important to note that although the returned vector has the
	/// *capacity* specified, the vector will have a zero *length*. For an
	/// explanation of the difference between length and capacity, see
	/// [*Capacity and reallocation*].
	///
	/// [*Capacity and reallocation*]: #capacity-and-reallocation
	pub fn with_capacity(capacity: usize) -> Self {
		//  Get the number of `T` elements needed to store the requested bit
		//  capacity.
		let elts = T::Mem::elts(capacity);
		//  Allocate a buffer that can hold that many elements.
		let v = Vec::with_capacity(elts);
		let (ptr, cap) = (v.as_ptr(), v.capacity());
		//  Disarm the `Vec` destructor.
		mem::forget(v);
		Self {
			_order: PhantomData,
			pointer: BitPtr::uninhabited(ptr),
			capacity: cap,
		}
	}

	/// Appends a bit to the back of the vector.
	///
	/// If the vector is at capacity, this may cause a reallocation.
	///
	/// # Panics
	///
	/// This will panic if the push will cause the vector to allocate above
	/// `BitPtr<T>::MAX_ELTS` or machine capacity.
	///
	/// # Examples
	///
	/// ```rust
	/// # use bitvec::prelude::*;
	/// let mut bv: BitVec = BitVec::new();
	/// assert!(bv.is_empty());
	/// bv.push(true);
	/// assert_eq!(bv.len(), 1);
	/// assert!(bv[0]);
	/// ```
	pub fn push(&mut self, value: bool) {
		let len = self.len();
		assert!(
			len <= BitPtr::<T>::MAX_BITS,
			"Capacity overflow: {} >= {}",
			len,
			BitPtr::<T>::MAX_BITS,
		);
		//  If self is empty *or* tail is at the back edge of an element, push
		//  an element onto the vector.
		if self.is_empty() || *self.pointer.tail() == T::Mem::BITS {
			self.with_vec(|v| v.push(T::Mem::ZERO.into()));
		}
		//  At this point, it is always safe to increment the tail, and then
		//  write to the newly live bit.
		unsafe {
			self.pointer.set_len(len + 1);
			self.set_unchecked(len, value);
		}
	}
}
