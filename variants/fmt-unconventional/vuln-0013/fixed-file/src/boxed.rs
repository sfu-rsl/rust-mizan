//! `BitBox` structure
//!
//! This module holds the type
//! for an owned but
//! ungrowable bit sequence.
//! `BitVec` is
//! the more appropriate and
//! useful type for most
//! collections. !

#![cfg(feature = "alloc")]



use crate::{order::{BitOrder,
                    Local},
            pointer::BitPtr,
            store::BitStore};
use core::marker::PhantomData;



/// A pointer type for owned
/// bit sequences.
///
/// This type is essentially a
/// `&BitSlice` that owns its
/// own memory. It can change
/// the contents of its
/// domain, but it cannot
/// change its own domain like
/// `BitVec` can. It is useful
/// for fixed-size collections
/// without lifetime tracking.
///
/// # Type Parameters
///
/// - `O: BitOrder`: An
///   implementor of the
///   [`BitOrder`] trait. This
///   type is used to
/// convert semantic indices
/// into concrete bit
/// positions in elements, and
/// store or retrieve bit
/// values from the storage
/// type.
/// - `T: BitStore`: An
///   implementor of the
///   [`BitStore`] trait:
///   `u8`, `u16`, `u32`,
/// or `u64` (64-bit systems
/// only). This is the actual
/// type in memory that the
/// box will use to store
/// data.
///
/// # Safety
///
/// The `BitBox` handle has
/// the same *size* as
/// standard Rust `Box<[T]>`
/// handles, but
/// it is ***extremely binary
/// incompatible*** with them.
/// Attempting to treat
/// `BitBox<_, T>` as
/// `Box<[T]>` in any manner
/// except through the
/// provided APIs is
/// *catastrophically***
/// unsafe and unsound.
///
/// # Trait Implementations
///
/// `BitBox<O, T>` implements
/// all the traits that
/// `BitSlice<O, T>` does, by
/// deferring to the
/// `BitSlice` implementation.
/// It also implements
/// conversion traits
/// to and from `BitSlice`,
/// and to/from `BitVec`.
#[repr(C)]



pub struct BitBox<O=Local, T=usize>
	where O : BitOrder,
	      T : BitStore,
{
	_order : PhantomData<O>,
	pointer : BitPtr<T>,
}



impl<O, T> BitBox<O, T>
	where O : BitOrder,
	      T : BitStore,
{
	/// Accesses the
	/// vector’s backing
	/// store as an
	/// element slice.
	///
	///
	/// Unlike `BitSlice`’s method of the same name, this includes the partial
	/// edges, as `BitBox` forbids fragmentation that leads to contention.
	///
	/// # Parameters
	///
	/// - `&self`
	///
	/// # Returns
	///
	/// The slice of
	/// all live elements
	/// in the backing
	/// storage, including
	/// the
	/// partial edges
	/// if present.



	pub fn as_slice(&self) -> &[T]
	{



		self.bitptr()
		    .as_slice()
	}

	/// Accesses the
	/// vector’s backing
	/// store as an
	/// element slice.
	///
	///
	/// Unlike `BitSlice`’s method of the same name, this includes the partial
	/// edges, as `BitBox` forbids fragmentation that leads to contention.
	///
	/// # Parameters
	///
	/// - `&mut self`
	///
	/// # Returns
	///
	/// The slice of
	/// all live elements
	/// in the backing
	/// storage, including
	/// the
	/// partial edges
	/// if present.



	pub fn as_mut_slice(&mut self) -> &mut [T]
	{



		self.bitptr()
		    .as_mut_slice()
	}

	/// Gives read
	/// access to the
	/// `BitPtr<T>`
	/// structure powering
	/// the box.
	///
	/// # Parameters
	///
	/// - `&self`
	///
	/// # Returns
	///
	/// A copy of the
	/// interior `BitPtr<T>`.
	///



	pub(crate) fn bitptr(&self) -> BitPtr<T>
	{



		self.pointer
	}
}



mod api;
mod ops;
mod traits;
