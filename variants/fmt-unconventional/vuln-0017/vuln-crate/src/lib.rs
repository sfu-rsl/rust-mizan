//! The arr crate is a simple
//! fixed size array designed
//! to allow huge array
//! allocation without stack
//! overflows.
//!
//! Even in rustc 1.44
//! allocating arrays too
//! large to fit on the stack
//! (say 8MB in size) will
//! stack overflow even if the
//! array is Boxed.
//!
//! Basic usage:
//! ```
//! use arr::Array;
//! // zero - fast allocation
//! let big_array: Array<u8> = Array::zero(1 << 25);
//! // default - slow allocation
//! let big_array2: Array<u8> = Array::new(1 << 25);
//! // template
//! let template = 10u8;
//! let big_array3: Array<u8> = Array::new_from_template(1 << 25, &template);
//!
//! // Also works for 2d arrays (note even the sub-array would ordinarily blow stack)
//! let big_2d_array: Array<[u8; 1 << 25]> = Array::zero(4);
//! ```
//!
//! Try to do this with a
//! traditional array:
//! ```no_run
//! 
//!
//!
//! let big_array : Box<
//!                     [u8;
//!                      1 <<
//!                      25],
//! > = Box::new([0u8; 1 << 25]);
//! ```
//!
//! Currently the array
//! supports three modes of
//! allocation, via `new`
//! (requires types to have
//! Default +
//! Copy) and `new_from_template`, only requiring Clone. The `zero` constructor uses an internal
//! Zeroable trait only set
//! for primitive types and
//! their arrays. In the
//! future this concept may be
//! unsafely extended outside
//! of this crate but for now
//! it's private. As long as
//! this type fits on the
//! stack and the array fits
//! in memory this should be
//! allocatable.



mod zeroable;



use crate::zeroable::Zeroable;
use std::alloc::{alloc,
                 alloc_zeroed,
                 dealloc,
                 Layout};
use std::ops::{Index,
               IndexMut,
               Range};



pub struct Array<T>
{
	size : usize,
	ptr : *mut T,
}



unsafe impl<T> Sync for Array<T>
{
}



unsafe impl<T> Send for Array<T>
{
}



impl<T> Array<T>
{
	/// Create an immutable iterator over elements in Array.



	pub fn iter<'a>(&'a self) -> ArrayIter<'a, T>
	{



		ArrayIter { arr : &self,
		            iter : 0usize }
	}

	/// Convert to
	/// slice



	pub fn to_slice<'a>(&'a self) -> &'a [T]
	{



		unsafe {



			std::slice::from_raw_parts(self.ptr as *const T, self.size)
		}
	}

	/// Convert to
	/// mutable slice



	pub fn to_slice_mut<'a>(&'a mut self)
	                        -> &'a mut [T]
	{



		unsafe {



			std::slice::from_raw_parts_mut(self.ptr, self.size)
		}
	}

	// pub fn to_slice_mut<'a>(&'a
	// mut self) -> &'a mut [T] {
	//    slice::



	/// The length of
	/// the array (number
	/// of elements
	/// T)



	pub fn len(&self) -> usize { self.size }
}



impl<T> Array<T> where T : Zeroable,
{
	/// Extremely fast initialization if all you want is 0's. Note that your type must be Zeroable.
	/// The auto-Zeroable types are u8, i8, u16, i16, u32, i32, u64, i64, usize, isize, f32, f64.
	/// `std::Array`s
	/// also implement
	/// Zeroable allowing
	/// for types like
	/// `[u8; 1 <<
	/// 25]`.



	pub fn zero(size : usize) -> Self
	{



		let objsize =
			std::mem::size_of::<T>();



		let layout = Layout::from_size_align(size * objsize, 8).unwrap();



		let ptr = unsafe {



			alloc_zeroed(layout)
			as *mut T
		};



		Self { size,
		       ptr }
	}
}



impl<T> Array<T> where T : Default+Copy,
{
	/// Easy initialization if all you want is your T's default instantiation



	pub fn new(size : usize) -> Self
	{



		let objsize =
			std::mem::size_of::<T>();



		let layout = Layout::from_size_align(size * objsize, 8).unwrap();



		let ptr = unsafe {



			alloc(layout)
			as *mut T
		};



		let default : T =
			Default::default();



		for i in 0 .. size
		{



			unsafe {



				(*(ptr.wrapping_offset(i as isize))) = default;
			}
		}



		Self { size,
		       ptr }
	}
}



impl<T> Array<T> where T : Clone,
{
	/// More generic
	/// initialization
	/// instantiating all elements
	/// as copies of some template
	pub fn new_from_template(size: usize, template: &T) -> Self {
        let objsize = std::mem::size_of::<T>();
        let layout = Layout::from_size_align(size * objsize, 8).unwrap();
        let ptr = unsafe {
            alloc(layout) as *mut T
        };
        for i in 0..size {
            unsafe {
                (*(ptr.wrapping_offset(i as isize))) = template.clone();
            }
        }
        Self{size, ptr}
    }
}



impl<T> Index<usize> for Array<T>
{
	type Output = T;

	fn index<'a>(&'a self,
	             idx : usize)
	             -> &'a Self::Output
	{



		unsafe {



			self.ptr.wrapping_offset(idx as isize).as_ref()
		}.unwrap()
	}
}



impl<T> IndexMut<usize> for Array<T>
{
	fn index_mut<'a>(&'a mut self,
	                 idx : usize)
	                 -> &'a mut Self::Output
	{



		unsafe {



			self.ptr.wrapping_offset(idx as isize).as_mut()
		}.unwrap()
	}
}



impl<T> Index<Range<usize>> for Array<T>
{
	type Output = [T];

	fn index<'a>(&'a self,
	             idx : Range<usize>)
	             -> &'a Self::Output
	{



		&self.to_slice()[idx]
	}
}



impl<T> IndexMut<Range<usize>> for Array<T>
{
	fn index_mut<'a>(&'a mut self,
	                 idx : Range<usize>)
	                 -> &'a mut Self::Output
	{



		&mut self.to_slice_mut()[idx]
	}
}



impl<T> Drop for Array<T>
{
	fn drop(&mut self)
	{



		let objsize =
			std::mem::size_of::<T>();



		let layout = Layout::from_size_align(self.size * objsize, 8).unwrap();



		unsafe {



			dealloc(
			        self.ptr
			        as *mut u8,
			        layout,
			);
		}
	}
}



impl<'a, T> IntoIterator for &'a Array<T>
{
	type IntoIter = ArrayIter<'a, T>;
	type Item = &'a T;

	/// For now, you
	/// can only for
	/// loop iterate
	/// directly over
	/// the
	/// reference:
	/// ```
	/// use arr::Array;
	/// let arr: Array<usize> = Array::new(6);
	/// for i in &arr {
	///     println!("{}", i);
	/// }
	/// ```



	fn into_iter(self) -> Self::IntoIter
	{



		self.iter()
	}
}



pub struct ArrayIter<'a, T>
{
	arr : &'a Array<T>,
	iter : usize,
}



impl<'a, T> Iterator for ArrayIter<'a, T>
{
	type Item = &'a T;

	fn next(&mut self) -> Option<Self::Item>
	{



		match self.iter < self.arr
		                      .len()
		{
			| true =>
			{



				self.iter += 1;



				Some(&self.arr[self.iter - 1])
			},
			| false => None,
		}
	}
}



impl<'a, T> ExactSizeIterator for ArrayIter<'a, T>
{
	fn len(&self) -> usize
	{



		self.arr
		    .size - self.iter
	}
}



#[cfg(test)]



mod test
{



	use super::*;
	use std::sync::{Arc,
	                Mutex};
	use std::thread;



	#[test]



	fn test_default()
	{



		let arr : Array<f32> =
			Array::new(4 << 20); // Uses 16 MB - much to large
	           // for a stack
		assert_eq!(
		           arr[4 << 20 - 1],
		           0.0
		);
	}



	#[test]



	fn test_template()
	{



		// template: 256 * 8 = 2^11
		let template : [u64; 256] =
			[65u64; 256];



		// num: 4096 => total size:
		// 2^23, too large for the
		// stack
		let arr: Array<[u64; 256]> = Array::new_from_template(4096, &template);



		assert_eq!(
		           arr[4095][255],
		           65
		);
	}



	#[derive(Clone)]



	struct Unaligned
	{
		a : u64,
		b : u16,
	}



	#[test]



	fn test_unaligned()
	{



		let unaligned_template = Unaligned{a: 15, b:32};



		let arr: Array<Unaligned> = Array::new_from_template(5, &unaligned_template);



		assert_eq!(
		           arr[3].a,
		           15
		);



		assert_eq!(
		           arr[3].b,
		           32
		);



		assert_eq!(
		           arr[4].a,
		           15
		);



		assert_eq!(
		           arr[4].b,
		           32
		);
	}



	#[test]



	fn test_zeroed()
	{



		// 8 * 4096 * 4096 = 8X16MB =
		// 128MB
		let arr : Array<[usize; 4096]> =
			Array::zero(4096);



		assert_eq!(
		           arr[4095][4095],
		           0
		);
	}



	#[test]



	fn test_async()
	{



		let arr : Array<usize> =
			Array::zero(5);



		let tid = thread::spawn(move || {
            assert_eq!(arr[3], 0);
        });



		let _ = tid.join();
	}



	#[test]



	fn test_mut_async()
	{



		let arr: Arc<Mutex<Array<usize>>> = Arc::new(Mutex::new(Array::zero(5)));



		{



			let mut arr = (&*arr).lock().unwrap();



			arr[4] = 1;
		}



		let carr = arr.clone();



		let tid = thread::spawn(move || {
            let mut arr = (&*carr).lock().unwrap();
            assert_eq!(arr[4], 1);
            arr[4] = 0;
        });



		tid.join()
		   .unwrap();



		assert_eq!(
		           (&*arr).lock()
		                  .unwrap()[4],
		           0
		);
	}



	#[test]



	fn test_loop()
	{



		let arr: Array<usize> = Array::new_from_template(5, &5);



		let mut cnt = 0;



		for _ in arr.iter()
		{



			cnt += 1;
		}



		assert_eq!(
		           cnt,
		           5
		);
	}



	#[test]



	fn test_copy_from()
	{



		let mut arr: Array<usize> = Array::new_from_template(5, &5);



		let from : [usize; 5] =
			[1usize; 5];



		arr.to_slice_mut()
		   .copy_from_slice(&from);



		assert_eq!(
		           arr[4],
		           1
		);
	}



	#[test]



	fn test_range()
	{



		let arr : Array<usize> =
			Array::zero(10);



		let mut cnt = 0;



		for _i in &arr[0 .. 5]
		{



			cnt += 1;
		}



		assert_eq!(
		           cnt,
		           5
		);
	}



	#[test]



	fn test_range_mut()
	{



		let mut arr : Array<usize> =
			Array::zero(10);



		let slice = &mut arr[0 .. 5];



		for i in slice
		{



			*i = 5;
		}



		assert_eq!(
		           arr[4],
		           5
		);
	}
}
