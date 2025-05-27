//! Fast and lightweight Slab
//! Allocator.



use std::ops::Index;



pub struct Slab<T>
{
	capacity : usize,
	len : usize,
	mem : *mut T,
}



impl<T> Index<usize> for Slab<T>
{
	type Output = T;

	fn index(&self, index: usize) -> &Self::Output {
        unsafe { &(*(self.mem.offset(index as isize))) }
    }
}
