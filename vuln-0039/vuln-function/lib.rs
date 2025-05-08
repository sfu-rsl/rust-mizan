// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Small vectors in various sizes. These store a certain number of elements inline, and fall back
//! to the heap for larger allocations.  This can be a useful optimization for improving cache
//! locality and reducing allocator traffic for workloads that fit within the inline buffer.
//!
//! ## no_std support
//!
//! By default, `smallvec` depends on `libstd`. However, it can be configured to use the unstable
//! `liballoc` API instead, for use on platforms that have `liballoc` but not `libstd`.  This
//! configuration is currently unstable and is not guaranteed to work on all versions of Rust.
//!
//! To depend on `smallvec` without `libstd`, use `default-features = false` in the `smallvec`
//! section of Cargo.toml to disable its `"std"` feature.
//!
//! ## `union` feature
//!
//! When the `union` feature is enabled `smallvec` will track its state (inline or spilled)
//! without the use of an enum tag, reducing the size of the `smallvec` by one machine word.
//! This means that there is potentially no space overhead compared to `Vec`.
//! Note that `smallvec` can still be larger than `Vec` if the inline buffer is larger than two
//! machine words.
//!
//! To use this feature add `features = ["union"]` in the `smallvec` section of Cargo.toml.
//! Note that this feature requires a nightly compiler (for now).

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(not(feature = "std"), feature(alloc))]
#![cfg_attr(feature = "union", feature(untagged_unions))]
#![cfg_attr(feature = "specialization", feature(specialization))]
#![cfg_attr(feature = "may_dangle", feature(dropck_eyepatch))]
#![deny(missing_docs)]


#[cfg(not(feature = "std"))]
#[macro_use]
extern crate alloc;

#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

#[cfg(not(feature = "std"))]
mod std {
    pub use core::*;
}

use std::mem;
use std::mem::{ManuallyDrop, MaybeUninit};
use std::ptr;

/// Hint to the optimizer that any code path which calls this function is
/// statically unreachable and can be removed.
///
/// Equivalent to `std::hint::unreachable_unchecked` but works in older versions of Rust.
#[inline]
pub unsafe fn unreachable() -> ! {
    enum Void {}
    let x: &Void = mem::transmute(1usize);
    match *x {}
}

/// `panic!()` in debug builds, optimization hint in release.
#[cfg(not(feature = "union"))]
macro_rules! debug_unreachable {
    () => { debug_unreachable!("entered unreachable code") };
    ($e:expr) => {
        if cfg!(not(debug_assertions)) {
            unreachable();
        } else {
            panic!($e);
        }
    }
}

unsafe fn deallocate<T>(ptr: *mut T, capacity: usize) {
    let _vec: Vec<T> = Vec::from_raw_parts(ptr, 0, capacity);
    // Let it drop.
}

#[cfg(feature = "union")]
union SmallVecData<A: Array> {
    inline: ManuallyDrop<A>,
    heap: (*mut A::Item, usize),
}

#[cfg(feature = "union")]
impl<A: Array> SmallVecData<A> {
    #[inline]
    unsafe fn inline(&self) -> &A {
        &self.inline
    }
    #[inline]
    unsafe fn inline_mut(&mut self) -> &mut A {
        &mut self.inline
    }
    #[inline]
    fn from_inline(inline: A) -> SmallVecData<A> {
        SmallVecData { inline: ManuallyDrop::new(inline) }
    }
    #[inline]
    unsafe fn into_inline(self) -> A { ManuallyDrop::into_inner(self.inline) }
    #[inline]
    unsafe fn heap(&self) -> (*mut A::Item, usize) {
        self.heap
    }
    #[inline]
    unsafe fn heap_mut(&mut self) -> &mut (*mut A::Item, usize) {
        &mut self.heap
    }
    #[inline]
    fn from_heap(ptr: *mut A::Item, len: usize) -> SmallVecData<A> {
        SmallVecData { heap: (ptr, len) }
    }
}

#[cfg(not(feature = "union"))]
enum SmallVecData<A: Array> {
    Inline(ManuallyDrop<A>),
    Heap((*mut A::Item, usize)),
}

#[cfg(not(feature = "union"))]
impl<A: Array> SmallVecData<A> {
    #[inline]
    unsafe fn inline_mut(&mut self) -> &mut A {
        match *self {
            SmallVecData::Inline(ref mut a) => a,
            _ => debug_unreachable!(),
        }
    }
    #[inline]
    fn from_inline(inline: A) -> SmallVecData<A> {
        SmallVecData::Inline(ManuallyDrop::new(inline))
    }
    #[inline]
    unsafe fn heap_mut(&mut self) -> &mut (*mut A::Item, usize) {
        match *self {
            SmallVecData::Heap(ref mut data) => data,
            _ => debug_unreachable!(),
        }
    }
    #[inline]
    fn from_heap(ptr: *mut A::Item, len: usize) -> SmallVecData<A> {
        SmallVecData::Heap((ptr, len))
    }
}

/// A `Vec`-like container that can store a small number of elements inline.
///
/// `SmallVec` acts like a vector, but can store a limited amount of data inline within the
/// `SmallVec` struct rather than in a separate allocation.  If the data exceeds this limit, the
/// `SmallVec` will "spill" its data onto the heap, allocating a new buffer to hold it.
///
/// The amount of data that a `SmallVec` can store inline depends on its backing store. The backing
/// store can be any type that implements the `Array` trait; usually it is a small fixed-sized
/// array.  For example a `SmallVec<[u64; 8]>` can hold up to eight 64-bit integers inline.
///
/// ## Example
///
/// ```rust
/// use smallvec::SmallVec;
/// let mut v = SmallVec::<[u8; 4]>::new(); // initialize an empty vector
///
/// // The vector can hold up to 4 items without spilling onto the heap.
/// v.extend(0..4);
/// assert_eq!(v.len(), 4);
/// assert!(!v.spilled());
///
/// // Pushing another element will force the buffer to spill:
/// v.push(4);
/// assert_eq!(v.len(), 5);
/// assert!(v.spilled());
/// ```
pub struct SmallVec<A: Array> {
    // The capacity field is used to determine which of the storage variants is active:
    // If capacity <= A::size() then the inline variant is used and capacity holds the current length of the vector (number of elements actually in use).
    // If capacity > A::size() then the heap variant is used and capacity holds the size of the memory allocation.
    capacity: usize,
    data: SmallVecData<A>,
}

impl<A: Array> SmallVec<A> {

    /// The maximum number of elements this vector can hold inline
    #[inline]
    pub fn inline_size(&self) -> usize {
        A::size()
    }

    /// Returns a tuple with (data ptr, len ptr, capacity)
    #[inline]
    fn triple_mut(&mut self) -> (*mut A::Item, &mut usize, usize) {
        unsafe {
            if self.spilled() {
                let &mut (ptr, ref mut len_ptr) = self.data.heap_mut();
                (ptr, len_ptr, self.capacity)
            } else {
                (self.data.inline_mut().ptr_mut(), &mut self.capacity, A::size())
            }
        }
    }

    /// Returns `true` if the data has spilled into a separate heap-allocated buffer.
    #[inline]
    pub fn spilled(&self) -> bool {
        self.capacity > A::size()
    }

    /// Re-allocate to set the capacity to `max(new_cap, inline_size())`.
    ///
    /// Panics if `new_cap` is less than the vector's length.
    pub fn grow(&mut self, new_cap: usize) {
        unsafe {
            let (ptr, &mut len, cap) = self.triple_mut();
            let unspilled = !self.spilled();
            assert!(new_cap >= len);
            if new_cap <= self.inline_size() {
                if unspilled {
                    return;
                }
                self.data = SmallVecData::from_inline(MaybeUninit::uninit().assume_init());
                ptr::copy_nonoverlapping(ptr, self.data.inline_mut().ptr_mut(), len);
            } else if new_cap != cap {
                let mut vec = Vec::with_capacity(new_cap);
                let new_alloc = vec.as_mut_ptr();
                mem::forget(vec);
                ptr::copy_nonoverlapping(ptr, new_alloc, len);
                self.data = SmallVecData::from_heap(new_alloc, len);
                self.capacity = new_cap;
                if unspilled {
                    return;
                }
            }
            deallocate(ptr, cap);
        }
    }
}

/// Types that can be used as the backing store for a SmallVec
pub unsafe trait Array {
    /// The type of the array's elements.
    type Item;
    /// Returns the number of items the array can hold.
    fn size() -> usize;
    /// Returns a pointer to the first element of the array.
    fn ptr(&self) -> *const Self::Item;
    /// Returns a mutable pointer to the first element of the array.
    fn ptr_mut(&mut self) -> *mut Self::Item;
}
