//! Reimplementation of the
//! standard library’s `Vec`
//! inherent method API.



use crate::{mem::BitMemory,
            order::BitOrder,
            pointer::BitPtr,
            store::BitStore,
            vec::BitVec};
use alloc::{boxed::Box,
            vec::Vec};
use core::{marker::PhantomData,
           mem};
use funty::IsInteger;



impl<O, T> BitVec<O, T>
	where O : BitOrder,
	      T : BitStore,
{
	/// Constructs a
	/// new, empty
	/// `BitVec<O,
	/// T>` with the
	/// specified capacity.
	///
	///
	/// The vector
	/// will be able
	/// to hold at
	/// least `capacity`
	/// bits without
	/// reallocating.
	/// If `capacity`
	/// is 0, the vector
	/// will not allocate.
	///
	///
	/// It is important to note that although the returned vector has the
	/// *capacity*
	/// specified,
	/// the vector
	/// will have a
	/// zero *length*
	/// . For an
	/// explanation
	/// of the difference
	/// between length
	/// and capacity,
	/// see
	/// [*Capacity
	/// and reallocation*
	/// ].
	///
	/// [*Capacity and reallocation*]: #capacity-and-reallocation



	pub fn with_capacity(capacity : usize)
	                     -> Self
	{



		//  Get the number of `T`
		// elements needed to store
		// the requested bit
		//  capacity.
		let elts = T::Mem::elts(capacity);



		//  Allocate a buffer that can
		// hold that many elements.
		let v = Vec::with_capacity(elts);



		let (ptr, cap) = (v.as_ptr(),
		                  v.capacity());



		//  Disarm the `Vec`
		// destructor.
		mem::forget(v);



		Self {
			_order: PhantomData,
			pointer: BitPtr::uninhabited(ptr),
			capacity: cap,
		}
	}

	/// Returns the
	/// number of bits
	/// the vector
	/// can hold without
	/// reallocating.
	///
	/// # Examples
	///
	/// ```rust
	/// # use bitvec::prelude::*;
	/// let bv: BitVec<Local, usize> = BitVec::with_capacity(100);
	/// assert!(bv.capacity() >= 100);
	#[inline]



	pub fn capacity(&self) -> usize
	{



		self.capacity
		    .checked_mul(
		                 T::Mem::BITS
		                 as usize,
		)
		    .expect(
		            "Vector capacity \
		             overflow",
		)
	}

	/// Converts the
	/// bit-vector
	/// into [`Box<[T]>`].
	///
	///
	/// Note that this will drop any excess capacity.
	///
	/// For the vec-to-box equivalent that produces a [`BitBox<O, T>`], see
	/// [`into_boxed_bitslice`].
	///
	/// # Examples
	///
	/// ```rust
	/// 
	///
	///
	/// # use bitvec::prelude::*;
	/// let bv = bitvec![1, 0, 1];
	///
	///
	///
	/// let slice = bv.into_boxed_slice();
	/// ```
	///
	/// Any excess
	/// capacity is
	/// removed:
	///
	/// ```rust
	/// # use bitvec::prelude::*;
	/// let mut bv = BitVec::<Local, usize>::with_capacity(100);
	/// bv.extend([true, false, true].iter().copied());
	///
	/// assert!(bv.capacity() >= 100);
	/// let slice = bv.into_boxed_slice();
	/// let boxed_bitslice = BitBox::<Local, usize>::from_boxed_slice(slice);
	/// let bv = BitVec::from_boxed_bitslice(boxed_bitslice);
	/// assert!(bv.capacity() >= 3);
	/// ```
	///
	/// [`BitBox<O, T>`]: ../boxed/struct.BitBox.html
	/// [`Box<[T]>`]: https://doc.rust-lang.org/std/boxed/struct.Box.html
	/// [`into_boxed_bitslice`]: #method.into_boxed_bitslice
	#[inline]



	pub fn into_boxed_slice(self) -> Box<[T]>
	{



		self.into_vec()
		    .into_boxed_slice()
	}

	/// Extracts a
	/// mutable slice
	/// of the entire
	/// vector.
	///
	/// Unlike [`BitSlice::as_mut_slice`], this will produce partial edge
	/// elements, as
	/// they are known
	/// to not be aliased
	/// by any other
	/// slice
	/// handles.
	///
	/// # Examples
	///
	/// ```rust
	/// # use bitvec::prelude::*;
	/// # #[cfg(feature = "std")] {
	/// use std::io::{self, Read};
	/// let mut buffer = bitvec![Local, u8; 0; 24];
	/// io::repeat(0xA5u8).read_exact(buffer.as_mut_slice()).unwrap();
	/// # }
	/// ```
	///
	/// [`BitSlice::as_mut_slice`]:
	/// ../slice/struct.BitSlice.html#method.as_mut_slice
	#[inline]



	pub fn as_mut_slice(&mut self) -> &mut [T]
	{



		self.pointer
		    .as_mut_slice()
	}

	/// Forces the
	/// length of the
	/// vector to `new_len`.
	///
	///
	/// This is a low-level operation that maintains none of the normal
	/// invariants of
	/// the type. Normally
	/// changing the
	/// length of a
	/// vector is done
	/// using one of
	/// the safe operations
	/// instead, such
	/// as [`truncate`],
	/// [`resize`],
	/// [`extend`],
	/// or [`clear`].
	///
	/// # Safety
	///
	/// - `new_len` must be less than or equal to [`capacity()`].
	/// - The underlying elements at `old_len ..new_len` must be initialized.
	///
	/// # Examples
	///
	/// This method
	/// can be useful
	/// for situations
	/// in which the
	/// vector is serving
	/// as a buffer
	/// for other code,
	/// particularly
	/// over FFI.
	///
	/// ```rust
	/// 
	///
	///
	/// # use bitvec::prelude::*;
	/// let mut bv = BitVec::<
	///                     Local,
	///                     usize,
	/// >::with_capacity(17);
	///
	///
	///
	/// assert!(bv.is_empty());
	///
	///
	///
	/// unsafe {
	///
	///
	///
	/// 	bv.set_len(23)
	/// };
	///
	///
	///
	/// assert_eq!(
	///            bv.len(),
	///            23
	/// );
	/// ```
	///
	/// This example
	/// executes correctly,
	/// because the
	/// allocator can
	/// only reserve
	/// even multiples of bytes, and so rounds up from the `with_capacity`
	/// argument.
	///
	/// [`capacity()`]: #method.capacity
	/// [`clear`]: #method.clear
	/// [`extend`]: #method.extend
	/// [`resize`]: #method.resize
	/// [`truncate`]: #method.truncate



	pub unsafe fn set_len(&mut self,
	                      new_len : usize)
	{



		assert!(
		        new_len <=
		        BitPtr::<T,>::MAX_BITS,
		        "Capacity overflow: {} \
		         overflows maximum \
		         length {}",
		        new_len,
		        BitPtr::<T,>::MAX_BITS,
		);



		let cap = self.capacity();



		assert!(
		        new_len <= cap,
		        "Capacity overflow: {} \
		         overflows allocation \
		         size {}",
		        new_len,
		        cap,
		);



		self.pointer
		    .set_len(new_len);
	}

	/// Appends a bit
	/// to the back
	/// of the vector.
	///
	///
	/// If the vector
	/// is at capacity,
	/// this may cause
	/// a reallocation.
	///
	///
	/// # Panics
	///
	/// This will panic if the push will cause the vector to allocate above
	/// `BitPtr<T>::MAX_ELTS` or machine capacity.
	///
	/// # Examples
	///
	/// ```rust
	/// 
	///
	///
	/// # use bitvec::prelude::*;
	/// let mut bv : BitVec =
	/// 	BitVec::new();
	///
	///
	///
	/// assert!(bv.is_empty());
	///
	///
	///
	/// bv.push(true);
	///
	///
	///
	/// assert_eq!(
	///            bv.len(),
	///            1
	/// );
	///
	///
	///
	/// assert!(bv[0]);
	/// ```



	pub fn push(&mut self,
	            value : bool)
	{



		let len = self.len();



		assert!(
		        len <=
		        BitPtr::<T,>::MAX_BITS,
		        "Capacity overflow: {} \
		         >= {}",
		        len,
		        BitPtr::<T,>::MAX_BITS,
		);



		//  If self is empty *or* tail
		// is at the back edge of an
		// element, push  an element
		// onto the vector.
		if self.is_empty() ||
		   *self.pointer
		        .tail() == T::Mem::BITS
		{



			self.with_vec(
			              |v| {
				              v.push(T::Mem::ZERO.into())
			              },
			);
		}



		//  At this point, it is
		// always safe to increment
		// the tail, and then  write
		// to the newly live bit.
		unsafe {



			self.pointer
			    .set_len(
			             len +
			             1,
			);



			self.set_unchecked(len, value);
		}
	}
}
