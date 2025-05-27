//! Operator trait
//! implementations.



use crate::{boxed::BitBox,
            order::BitOrder,
            store::BitStore};
use alloc::vec::Vec;



impl<O, T> Drop for BitBox<O, T>
	where O : BitOrder,
	      T : BitStore,
{
	fn drop(&mut self)
	{



		let ptr = self.as_mut_slice()
		              .as_mut_ptr();



		let len = self.as_slice()
		              .len();



		//  Run the `Box<[T]>`
		// destructor.
		drop(
		     unsafe {



			     Vec::from_raw_parts(ptr, 0, len)
		     }.into_boxed_slice(),
		);
	}
}
