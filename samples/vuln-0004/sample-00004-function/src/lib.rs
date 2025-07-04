#![cfg_attr(not(test), no_std)]
extern crate alloc;

use alloc::vec::{IntoIter, Vec as StdVec};
use core::fmt;
use core::iter::FromIterator;
use core::mem::ManuallyDrop;
use core::ops::{Deref, DerefMut, Index, IndexMut};
use core::ptr::{slice_from_raw_parts, slice_from_raw_parts_mut, NonNull};

/// A contiguous growable array type, written `Vec<T>` but pronounced 'vector'.
pub struct Vec<T> {
    ptr: NonNull<[T]>,
}

impl<T> Vec<T> {
    /// Removes and returns the element at position `index` within the vector,
    /// shifting all elements after it to the left.
    pub fn remove(&mut self, index: usize) -> T {
        self.with(move |v| v.remove(index))
    }

    /// Returns a raw pointer to the vector's buffer.
    #[inline]
    pub const fn as_ptr(&self) -> *const T {
        self.ptr.cast().as_ptr()
    }

    /// Returns an unsafe mutable pointer to the vector's buffer.
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.ptr.cast().as_ptr()
    }

    #[inline]
    fn parts(&self) -> (usize, usize) {
        let parts = unsafe { &*(self.ptr.as_ptr() as *const [()]) }.len();

        (parts & MASK_LO, (parts & MASK_HI) >> 32)
    }

    fn with<'a, R: 'a, F: FnOnce(&mut StdVec<T>) -> R>(&mut self, f: F) -> R {
        let (len, cap) = self.parts();

        let mut stdvec = unsafe { StdVec::from_raw_parts(self.as_mut_ptr(), len, cap) };

        let r = f(&mut stdvec);

        ManuallyDrop::new(core::mem::replace(
            self,
            Self::from_stdvec_unchecked(stdvec),
        ));

        r
    }

    fn from_stdvec_unchecked(stdvec: StdVec<T>) -> Self {
        let mut stdvec = ManuallyDrop::new(stdvec);

        let ptr = stdvec.as_mut_ptr();
        let len = stdvec.len();
        let cap = stdvec.capacity();

        let ptr = slice_from_raw_parts_mut(ptr, len & MASK_LO | (cap & MASK_LO) << 32);

        Vec {
            ptr: unsafe { NonNull::new_unchecked(ptr) },
        }
    }
}

unsafe impl<T: Sync> Sync for Vec<T> {}
unsafe impl<T: Send> Send for Vec<T> {}

const MASK_LO: usize = core::u32::MAX as usize;
const MASK_HI: usize = !(core::u32::MAX as usize);
