use serde::de::{self,
                Deserialize,
                Deserializer,
                SeqAccess,
                Visitor};
use std::fmt;
use std::marker::PhantomData;
use std::mem::{self,
               size_of};



/// Errors associated with the
/// [`FamStructWrapper`](struct.
/// FamStructWrapper.html)
/// struct.
#[derive(Clone, Debug, PartialEq, Eq)]



pub enum Error
{
	/// The max size
	/// has been exceeded
	SizeLimitExceeded,
}



impl std::error::Error for Error
{
}



impl fmt::Display for Error
{
	fn fmt(&self,
	       f : &mut fmt::Formatter)
	       -> fmt::Result
	{



		match self {
            Self::SizeLimitExceeded => write!(f, "The max size has been exceeded"),
        }
	}
}



/// Trait for accessing
/// properties of C defined
/// FAM structures.
///
/// # Safety
///
/// This is unsafe due to the
/// number of constraints that
/// aren't checked:
/// * the implementer should
///   be a POD
/// * the implementor should
///   contain a flexible array
///   member of elements of
///   type `Entry`
/// * `Entry` should be a POD
///
/// Violating these may cause
/// problems.
///
/// # Example
///
/// ```
/// use vmm_sys_util::fam::*;
///
/// #[repr(C)]
/// #[derive(Default)]
/// pub struct __IncompleteArrayField<T>(::std::marker::PhantomData<T>, [T; 0]);
/// impl<T> __IncompleteArrayField<T> {
///     #[inline]
///     pub fn new() -> Self {
///         __IncompleteArrayField(::std::marker::PhantomData, [])
///     }
///     #[inline]
///     pub unsafe fn as_ptr(&self) -> *const T {
///         ::std::mem::transmute(self)
///     }
///     #[inline]
///     pub unsafe fn as_mut_ptr(&mut self) -> *mut T {
///         ::std::mem::transmute(self)
///     }
///     #[inline]
///     pub unsafe fn as_slice(&self, len: usize) -> &[T] {
///         ::std::slice::from_raw_parts(self.as_ptr(), len)
///     }
///     #[inline]
///     pub unsafe fn as_mut_slice(&mut self, len: usize) -> &mut [T] {
///         ::std::slice::from_raw_parts_mut(self.as_mut_ptr(), len)
///     }
/// }
///
/// #[repr(C)]
/// #[derive(Default)]
/// struct MockFamStruct {
///     pub len: u32,
///     pub padding: u32,
///     pub entries: __IncompleteArrayField<u32>,
/// }
///
/// unsafe impl FamStruct for MockFamStruct {
///     type Entry = u32;
///
///     fn len(&self) -> usize {
///         self.len as usize
///     }
///
///     fn set_len(&mut self, len: usize) {
///         self.len = len as u32
///     }
///
///     fn max_len() -> usize {
///         100
///     }
///
///     fn as_slice(&self) -> &[u32] {
///         let len = self.len();
///         unsafe { self.entries.as_slice(len) }
///     }
///
///     fn as_mut_slice(&mut self) -> &mut [u32] {
///         let len = self.len();
///         unsafe { self.entries.as_mut_slice(len) }
///     }
/// }
///
/// type MockFamStructWrapper = FamStructWrapper<MockFamStruct>;
/// ```
#[allow(clippy::len_without_is_empty)]



pub unsafe trait FamStruct
{
	/// The type of
	/// the FAM entries



	type Entry: PartialEq+Copy;



	/// Get the FAM
	/// length
	///
	/// These type of
	/// structures
	/// contain a member
	/// that holds
	/// the FAM length.
	/// This method
	/// will return
	/// the value of
	/// that member.



	fn len(&self) -> usize;



	/// Set the FAM
	/// length
	///
	/// These type of
	/// structures
	/// contain a member
	/// that holds
	/// the FAM length.
	/// This method
	/// will set the
	/// value of that
	/// member.



	fn set_len(&mut self,
	           len : usize);



	/// Get max allowed FAM length
	///
	/// This depends
	/// on each structure.
	/// For example
	/// a structure
	/// representing
	/// the cpuid can
	/// contain at
	/// most 80 entries.
	///



	fn max_len() -> usize;



	/// Get the FAM
	/// entries as
	/// slice



	fn as_slice(&self) -> &[Self::Entry];



	/// Get the FAM
	/// entries as
	/// mut slice



	fn as_mut_slice(&mut self)
	                -> &mut [Self::Entry];
}



/// A wrapper for
/// [`FamStruct`](trait.
/// FamStruct.html).
///
/// It helps in treating a
/// [`FamStruct`](trait.
/// FamStruct.html) similarly
/// to an actual `Vec`.
#[derive(Debug)]



pub struct FamStructWrapper<T : Default+FamStruct>
{
	// This variable holds the FamStruct
	// structure. We use a `Vec<T>` to make the
	// allocation large enough while still being
	// aligned for `T`. Only the first element of
	// `Vec<T>` will actually be used as a `T`.
	// The remaining memory in the `Vec<T>` is for
	// `entries`, which must be contiguous. Since
	// the entries are of type `FamStruct::Entry`
	// we must be careful to convert the desired
	// capacity of the `FamStructWrapper`
	// from `FamStruct::Entry` to `T` when
	// reserving or releasing memory.
	mem_allocator : Vec<T>,
}



impl<T : Default+FamStruct> FamStructWrapper<T>
{
	/// Convert FAM
	/// len to `mem_allocator`
	/// len.
	///
	/// Get the capacity required by mem_allocator in order to hold
	/// the provided
	/// number of [`FamStruct::Entry`](trait.
	/// FamStruct.html#
	/// associatedtype.
	/// Entry).
	/// Returns `None` if the required length would overflow usize.



	fn mem_allocator_len(fam_len : usize)
	                     -> Option<usize>
	{



		let wrapper_size_in_bytes =
            size_of::<T>().checked_add(fam_len.checked_mul(size_of::<T::Entry>())?)?;



		wrapper_size_in_bytes
            .checked_add(size_of::<T>().checked_sub(1)?)?
            .checked_div(size_of::<T>())
	}

	/// Convert `mem_allocator` len to FAM len.
	///
	/// Get the number of elements of type
	/// [`FamStruct::Entry`](trait.FamStruct.html#associatedtype.Entry)
	/// that fit in a
	/// mem_allocator
	/// of provided
	/// len.



	fn fam_len(mem_allocator_len : usize)
	           -> usize
	{



		if mem_allocator_len == 0
		{



			return 0;
		}



		let array_size_in_bytes =
			(mem_allocator_len -
			 1) * size_of::<T>();



		array_size_in_bytes /
		size_of::<T::Entry>()
	}

	/// Create a new
	/// FamStructWrapper
	/// with `num_elements`
	/// elements.
	///
	/// The elements
	/// will be zero-initialized.
	/// The type of
	/// the elements
	/// will be
	/// [`FamStruct::Entry`](trait.FamStruct.html#associatedtype.Entry).
	///
	/// # Arguments
	///
	/// * `num_elements` - The number of elements in the FamStructWrapper.
	///
	/// # Errors
	///
	/// When `num_elements` is greater than the max possible len, it returns
	/// `Error::SizeLimitExceeded`.



	pub fn new(
		num_elements : usize)
		-> Result<
		          FamStructWrapper<T,>,
		          Error,
		>
	{



		if num_elements > T::max_len()
		{



			return Err(Error::SizeLimitExceeded);
		}



		let required_mem_allocator_capacity =
            FamStructWrapper::<T>::mem_allocator_len(num_elements)
                .ok_or(Error::SizeLimitExceeded)?;



		let mut mem_allocator = Vec::with_capacity(required_mem_allocator_capacity);



		mem_allocator.push(T::default());



		for _ in 1..required_mem_allocator_capacity {
            // SAFETY: Safe as long T follows the requirements of being POD.
            mem_allocator.push(unsafe { mem::zeroed() })
        }



		mem_allocator[0].set_len(num_elements);



		Ok(FamStructWrapper { mem_allocator })
	}

	/// Create a new
	/// FamStructWrapper
	/// from a slice
	/// of elements.
	///
	/// # Arguments
	///
	/// * `entries` -
	///   The slice of
	///   [`FamStruct::Entry`](trait.
	///   FamStruct.html#
	///   associatedtype.
	///   Entry) entries.
	///
	/// # Errors
	///
	/// When the size
	/// of `entries`
	/// is greater
	/// than the max
	/// possible len,
	/// it returns
	/// `Error::SizeLimitExceeded`.



	pub fn from_entries(
		entries : &[T::Entry])
		-> Result<
		          FamStructWrapper<T,>,
		          Error,
		>
	{



		let mut adapter = FamStructWrapper::<T>::new(entries.len())?;



		{



			let wrapper_entries = adapter.as_mut_fam_struct().as_mut_slice();



			wrapper_entries.copy_from_slice(entries);
		}



		Ok(adapter)
	}

	/// Get a reference to the actual [`FamStruct`](trait.FamStruct.html) instance.



	pub fn as_fam_struct_ref(&self) -> &T
	{



		&self.mem_allocator[0]
	}

	/// Get a mut reference to the actual [`FamStruct`](trait.FamStruct.html) instance.



	pub fn as_mut_fam_struct(&mut self) -> &mut T
	{



		&mut self.mem_allocator[0]
	}

	/// Get the number of elements of type `FamStruct::Entry` currently in the vec.



	fn len(&self) -> usize
	{



		self.as_fam_struct_ref()
		    .len()
	}

	/// Get the capacity of the `FamStructWrapper`
	///
	/// The capacity
	/// is measured
	/// in elements
	/// of type `FamStruct::Entry`.
	///



	fn capacity(&self) -> usize
	{



		FamStructWrapper::<T>::fam_len(self.mem_allocator.capacity())
	}

	/// Reserve additional capacity.
	///
	/// Reserve capacity for at least `additional` more
	/// [`FamStruct::Entry`](trait.FamStruct.html#associatedtype.Entry) elements.
	///
	/// If the capacity is already reserved, this method doesn't do anything.
	/// If not this
	/// will trigger
	/// a reallocation
	/// of the underlying
	/// buffer.



	fn reserve(&mut self,
	           additional : usize)
	           -> Result<(), Error>
	{



		let desired_capacity =
			self.len() +
			additional;



		if desired_capacity <=
		   self.capacity()
		{



			return Ok(());
		}



		let current_mem_allocator_len =
			self.mem_allocator
			    .len();



		let required_mem_allocator_len = FamStructWrapper::<T>::mem_allocator_len(desired_capacity)
            .ok_or(Error::SizeLimitExceeded)?;



		let additional_mem_allocator_len = required_mem_allocator_len - current_mem_allocator_len;



		self.mem_allocator.reserve(additional_mem_allocator_len);



		Ok(())
	}

	/// Update the length of the
	/// FamStructWrapper.
	///
	/// The length of `self` will
	/// be updated to the
	/// specified value.
	/// The length of the `T`
	/// structure and of
	/// `self.mem_allocator` will
	/// be updated accordingly.
	/// If the len is increased
	/// additional capacity will
	/// be reserved. If the len
	/// is decreased the
	/// unnecessary memory will be
	/// deallocated.
	///
	/// This method might trigger
	/// reallocations of the
	/// underlying buffer.
	///
	/// # Errors
	///
	/// When len is greater than
	/// the max possible len it
	/// returns Error::SizeLimitExceeded.
	///
	fn set_len(&mut self, len: usize) -> Result<(), Error> {
        let additional_elements = isize::try_from(len)
            .and_then(|len| isize::try_from(self.len()).map(|self_len| len - self_len))
            .map_err(|_| Error::SizeLimitExceeded)?;

        // If len == self.len there's nothing to do.
        if additional_elements == 0 {
            return Ok(());
        }

        // If the len needs to be increased:
        if additional_elements > 0 {
            // Check if the new len is valid.
            if len > T::max_len() {
                return Err(Error::SizeLimitExceeded);
            }
            // Reserve additional capacity.
            self.reserve(additional_elements as usize)?;
        }

        let current_mem_allocator_len = self.mem_allocator.len();
        let required_mem_allocator_len =
            FamStructWrapper::<T>::mem_allocator_len(len).ok_or(Error::SizeLimitExceeded)?;
        // Update the len of the `mem_allocator`.
        // SAFETY: This is safe since enough capacity has been reserved.
        unsafe {
            self.mem_allocator.set_len(required_mem_allocator_len);
        }
        // Zero-initialize the additional elements if any.
        for i in current_mem_allocator_len..required_mem_allocator_len {
            // SAFETY: Safe as long as the trait is only implemented for POD. This is a requirement
            // for the trait implementation.
            self.mem_allocator[i] = unsafe { mem::zeroed() }
        }
        // Update the len of the underlying `FamStruct`.
        self.as_mut_fam_struct().set_len(len);

        // If the len needs to be decreased, deallocate unnecessary memory
        if additional_elements < 0 {
            self.mem_allocator.shrink_to_fit();
        }

        Ok(())
    }
}



impl<'de, T: Default + FamStruct + Deserialize<'de>> Deserialize<'de> for FamStructWrapper<T>
where
    <T as FamStruct>::Entry: std::marker::Copy + serde::Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FamStructWrapperVisitor<X> {
            dummy: PhantomData<X>,
        }

        impl<'de, X: Default + FamStruct + Deserialize<'de>> Visitor<'de> for FamStructWrapperVisitor<X>
        where
            <X as FamStruct>::Entry: std::marker::Copy + serde::Deserialize<'de>,
        {
            type Value = FamStructWrapper<X>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("FamStructWrapper")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<FamStructWrapper<X>, V::Error>
            where
                V: SeqAccess<'de>,
            {
                use serde::de::Error;

                let header = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let entries: Vec<X::Entry> = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;

                let mut result: Self::Value = FamStructWrapper::from_entries(entries.as_slice())
                    .map_err(|e| V::Error::custom(format!("{:?}", e)))?;
                result.mem_allocator[0] = header;
                Ok(result)
            }
        }

        deserializer.deserialize_tuple(2, FamStructWrapperVisitor { dummy: PhantomData })
    }
}
