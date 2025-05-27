//! Operator trait
//! implementations.



use super::*;
use crate::order::BitOrder;
use crate::store::BitStore;
use core::ops::Deref;
use core::ops::DerefMut;



/// Reborrows the `BitVec` as
/// a `BitSlice`.
///
/// This mimics the separation
/// between `Vec<T>` and
/// `[T]`.



impl<O, T> Deref for BitVec<O, T>
	where O : BitOrder,
	      T : BitStore,
{
	type Target = BitSlice<O, T>;

	/// Dereferences
	/// `&BitVec` down
	/// to `&BitSlice`.
	///
	///
	/// # Examples
	///
	/// ```rust
	/// use bitvec::prelude::*;
	///
	/// let bv: BitVec = bitvec![1; 4];
	/// let bref: &BitSlice = &bv;
	/// assert!(bref[2]);
	/// ```



	fn deref(&self) -> &Self::Target
	{



		self.as_bitslice()
	}
}



/// Mutably reborrows the
/// `BitVec` as a `BitSlice`.
///
/// This mimics the separation
/// between `Vec<T>` and
/// `[T]`.



impl<O, T> DerefMut for BitVec<O, T>
	where O : BitOrder,
	      T : BitStore,
{
	/// Dereferences
	/// `&mut BitVec`
	/// down to `&mut
	/// BitSlice`.
	///
	/// # Examples
	///
	/// ```rust
	/// use bitvec::prelude::*;
	///
	/// let mut bv: BitVec = bitvec![0; 6];
	/// let bref: &mut BitSlice = &mut bv;
	/// assert!(!bref[5]);
	/// bref.set(5, true);
	/// assert!(bref[5]);
	/// ```



	fn deref_mut(&mut self) -> &mut Self::Target
	{



		self.as_mut_bitslice()
	}
}
