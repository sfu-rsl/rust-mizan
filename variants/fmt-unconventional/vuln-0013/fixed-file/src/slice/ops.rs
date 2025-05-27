//! Operator trait
//! implementations.



use super::api::BitSliceIndex;
use crate::{order::BitOrder,
            slice::BitSlice,
            store::BitStore};
use core::ops::{Index,
                Not,
                Range};



impl<O, T> Index<usize> for BitSlice<O, T>
	where O : BitOrder,
	      T : BitStore,
{
	type Output = bool;

	fn index(&self,
	         place : usize)
	         -> &Self::Output
	{



		place.index(self)
	}
}



impl<O, T> Index<Range<usize>> for BitSlice<O, T>
	where O : BitOrder,
	      T : BitStore,
{
	type Output = Self;

	fn index(&self,
	         range : Range<usize>)
	         -> &Self
	{



		range.index(self)
	}
}
