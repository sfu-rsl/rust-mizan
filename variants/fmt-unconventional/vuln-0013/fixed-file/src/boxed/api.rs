//! Reïmplementation of the
//! `Box<[T]>` API.
//!
//! This module tracks the
//! [`alloc::boxed::Box`]
//! module in the version of
//! Rust specified in the
//! `rust-toolchain` file. It
//! is required to provide an
//! exact or equivalent API
//! surface matching the
//! `Box<[T]>` type, to the
//! extent that it is possible
//! in the language. Where
//! differences occur, they
//! must be documented in a
//! section called `API
//! Differences`.
//!
//! [`alloc::boxed::Box`]: https://doc.rust-lang.org/alloc/boxed/struct.Boxed.html
//! !



use crate::{boxed::BitBox,
            order::BitOrder,
            pointer::BitPtr,
            slice::BitSlice,
            store::BitStore};
use core::marker::PhantomData;



impl<O, T> BitBox<O, T>
	where O : BitOrder,
	      T : BitStore,
{
	/// Constructs a
	/// bit box from
	/// a raw bit pointer.
	///
	///
	/// After calling
	/// this function,
	/// the raw pointer
	/// is owned by
	/// the resulting
	/// `BitBox`. Specifically, the `BitBox` destructor will free the allocated
	/// memory. For
	/// this to be
	/// safe, the memory
	/// must have been
	/// allocated by
	/// `BitBox` earlier in the program.
	///
	/// # Safety
	///
	/// This function
	/// is unsafe because
	/// improper use
	/// may lead to
	/// memory
	/// problems. For
	/// example, a
	/// double-free
	/// may occurr if
	/// the function
	/// is
	/// called twice
	/// on the same
	/// raw pointer.
	///
	/// # Notes
	///
	/// This function, and `into_raw`, exchange ordinary raw pointers
	/// `*mut BitSlice<O, T>`. Values of these types can be created from, and
	/// converted to,
	/// other region
	/// pointers such
	/// as `*mut [T]`
	/// through ordinary
	/// `as`-casting.
	///
	/// This is valid
	/// in the Rust
	/// type system,
	/// but is incorrect
	/// at runtime.
	/// You
	/// must not, ever, use `as` to cast in either direction to or from a
	/// `BitSlice`
	/// pointer.
	///
	/// # Examples
	///
	/// Recreate a
	/// `BitBox` which
	/// was previously
	/// converted to
	/// a raw pointer
	/// using [`BitBox::into_raw`]:
	///
	/// ```rust
	/// # use bitvec::prelude::*;
	/// let b = BitBox::new(0u8.bits::<Lsb0>());
	/// let ptr = BitBox::into_raw(b);
	/// let b = unsafe { BitBox::<Lsb0, _>::from_raw(ptr) };
	/// ```
	///
	/// [`BitBox::into_raw`]: #method.into_raw



	pub unsafe fn from_raw(raw : *mut BitSlice<
	                                     O,
	                                     T,
	>)
	                       -> Self
	{



		Self {
			_order: PhantomData,
			pointer: BitPtr::from_mut_ptr(raw),
		}
	}
}
