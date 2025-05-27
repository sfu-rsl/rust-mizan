use std::{alloc,
          mem};



pub struct Matrix<'a, T>
	where T : Default+Clone,
{
	cols : usize,
	buffer : &'a mut [T],
}



impl<'a, T> Matrix<'a, T> where T : Default+Clone,
{
	/// Memory allocation for data
	/// buffer.
	///
	fn alloc(rows: usize, cols: usize) -> &'a mut [T] {
        unsafe {
            let buf = alloc::alloc(layout::<T>(rows * cols).unwrap()) as *mut T;
            let slice = std::slice::from_raw_parts_mut(buf, rows * cols);
            Self::fill_with(slice, T::default());
            slice
        }
    }

	/// Fills data buffer with a
	/// `value`.
	///
	fn fill_with(buf: &mut [T], value: T) {
        for e in buf {
            *e = value.clone();
        }
    }
}



fn layout<T>(size : usize)
             -> Result<alloc::Layout, alloc::LayoutError>
{



	alloc::Layout::from_size_align(size * mem::size_of::<T>(), mem::align_of::<T>())
}
