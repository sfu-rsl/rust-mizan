/*! Reïmplementation of the `[T]` API.

This module tracks the [`slice`] primitive and [`core::slice`] module in the
version of Rust specified in the `rust-toolchain` file. It is required to
provide an exact or equivalent API surface matching the `Box<[T]>` type, to the
extent that it is possible in the language. Where differences occur, they must
be documented in a section called `API Differences`.

[`core::slice`]: https://doc.rust-lang.org/core/slice
[`slice`]: https://doc.rust-lang.org/std/primitive.slice.html
!*/

use crate::{
	order::BitOrder,
	slice::{
		BitSlice,
	},
	store::BitStore,
};

/// Reimplementation of the `[T]` inherent-method API.
impl<O, T> BitSlice<O, T>
where
	O: BitOrder,
	T: BitStore,
{
	/// Returns the number of bits in the slice.
	///
	/// # Original
	///
	/// [`slice::len`](https://doc.rust-lang.org/std/primitive.slice.html#method.len)
	///
	/// # Examples
	///
	/// ```rust
	/// # use bitvec::prelude::*;
	/// let bits = 0u8.bits::<Local>();
	/// assert_eq!(bits.len(), 8);
	/// ```
	pub fn len(&self) -> usize {
		self.bitptr().len()
	}

	/// Returns `true` if the slice has a length of 0.
	///
	/// # Original
	///
	/// [`slice::is_empty`](https://doc.rust-lang.org/std/primitive.slice.html#method.is_empty)
	///
	/// # Examples
	///
	/// ```rust
	/// # use bitvec::prelude::*;
	/// let bits = 0u8.bits::<Local>();
	/// assert!(!bits.is_empty());
	///
	/// assert!(BitSlice::<Local, usize>::empty().is_empty())
	/// ```
	pub fn is_empty(&self) -> bool {
		self.bitptr().len() == 0
	}
}
