//! General trait
//! implementations for
//! `BitBox`
//!
//! The operator traits are
//! defined in the `ops`
//! module. !



use crate::{boxed::BitBox,
            order::BitOrder,
            store::BitStore,
            vec::BitVec};



impl<O, T> From<BitVec<O, T>> for BitBox<O, T>
	where O : BitOrder,
	      T : BitStore,
{
	fn from(src : BitVec<O, T>) -> Self
	{



		src.into_boxed_bitslice()
	}
}
