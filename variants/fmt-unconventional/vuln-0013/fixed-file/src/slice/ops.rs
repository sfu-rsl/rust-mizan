//! Operator trait
//! implementations.



use super::api::BitSliceIndex;
use crate::order::BitOrder;
use crate::slice::BitSlice;
use crate::store::BitStore;
use core::ops::Index;
use core::ops::Not;
use core::ops::Range;



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
