#![no_std]

#[doc(hidden)]
pub extern crate alloc;

#[allow(deprecated)]
use alloc::alloc::{Layout, LayoutErr};
use core::hint::unreachable_unchecked;
use core::iter::{repeat, FromIterator, FusedIterator, IntoIterator};
use core::mem;
use core::mem::MaybeUninit;
use core::ops::{self, Range, RangeBounds};
use core::ptr::{self, NonNull};
use core::slice::{self, SliceIndex};

/// `panic!()` in debug builds, optimization hint in release.
macro_rules! debug_unreachable {
    () => {
        debug_unreachable!("entered unreachable code")
    };
    ($e:expr) => {
        if cfg!(not(debug_assertions)) {
            unreachable_unchecked();
        } else {
            panic!($e);
        }
    };
}

/// Error type for APIs with fallible heap allocation
#[derive(Debug)]
pub enum CollectionAllocErr {
    /// Overflow `usize::MAX` or other error during size computation
    CapacityOverflow,
    /// The allocator return an error
    AllocErr {
        /// The layout that was passed to the allocator
        layout: Layout,
    },
}

fn infallible<T>(result: Result<T, CollectionAllocErr>) -> T {
    match result {
        Ok(x) => x,
        Err(CollectionAllocErr::CapacityOverflow) => panic!("capacity overflow"),
        Err(CollectionAllocErr::AllocErr { layout }) => alloc::alloc::handle_alloc_error(layout),
    }
}

/// FIXME: use `Layout::array` when we require a Rust version where it’s stable
/// https://github.com/rust-lang/rust/issues/55724
fn layout_array<T>(n: usize) -> Result<Layout, CollectionAllocErr> {
    let size = mem::size_of::<T>()
        .checked_mul(n)
        .ok_or(CollectionAllocErr::CapacityOverflow)?;
    let align = mem::align_of::<T>();
    Layout::from_size_align(size, align).map_err(|_| CollectionAllocErr::CapacityOverflow)
}

unsafe fn deallocate<T>(ptr: *mut T, capacity: usize) {
    // This unwrap should succeed since the same did when allocating.
    let layout = layout_array::<T>(capacity).unwrap();
    alloc::alloc::dealloc(ptr as *mut u8, layout)
}

enum SmallVecData<A: Array> {
    Inline(MaybeUninit<A>),
    Heap((*mut A::Item, usize)),
}

impl<A: Array> SmallVecData<A> {
    #[inline]
    unsafe fn inline(&self) -> *const A::Item {
        match self {
            SmallVecData::Inline(a) => a.as_ptr() as *const A::Item,
            _ => debug_unreachable!(),
        }
    }
    #[inline]
    unsafe fn inline_mut(&mut self) -> *mut A::Item {
        match self {
            SmallVecData::Inline(a) => a.as_mut_ptr() as *mut A::Item,
            _ => debug_unreachable!(),
        }
    }
    #[inline]
    fn from_inline(inline: MaybeUninit<A>) -> SmallVecData<A> {
        SmallVecData::Inline(inline)
    }
    #[inline]
    unsafe fn heap(&self) -> (*mut A::Item, usize) {
        match self {
            SmallVecData::Heap(data) => *data,
            _ => debug_unreachable!(),
        }
    }
    #[inline]
    unsafe fn heap_mut(&mut self) -> &mut (*mut A::Item, usize) {
        match self {
            SmallVecData::Heap(data) => data,
            _ => debug_unreachable!(),
        }
    }
    #[inline]
    fn from_heap(ptr: *mut A::Item, len: usize) -> SmallVecData<A> {
        SmallVecData::Heap((ptr, len))
    }
}

unsafe impl<A: Array + Send> Send for SmallVecData<A> {}
unsafe impl<A: Array + Sync> Sync for SmallVecData<A> {}

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
    // If capacity <= Self::inline_capacity() then the inline variant is used and capacity holds the current length of the vector (number of elements actually in use).
    // If capacity > Self::inline_capacity() then the heap variant is used and capacity holds the size of the memory allocation.
    capacity: usize,
    data: SmallVecData<A>,
}

impl<A: Array> SmallVec<A> {
    /// Sets the length of a vector.
    ///
    /// This will explicitly set the size of the vector, without actually
    /// modifying its buffers, so it is up to the caller to ensure that the
    /// vector is actually the specified size.
    pub unsafe fn set_len(&mut self, new_len: usize) {
        let (_, len_ptr, _) = self.triple_mut();
        *len_ptr = new_len;
    }

    /// The maximum number of elements this vector can hold inline
    #[inline]
    fn inline_capacity() -> usize {
        if mem::size_of::<A::Item>() > 0 {
            A::size()
        } else {
            // For zero-size items code like `ptr.add(offset)` always returns the same pointer.
            // Therefore all items are at the same address,
            // and any array size has capacity for infinitely many items.
            // The capacity is limited by the bit width of the length field.
            //
            // `Vec` also does this:
            // https://github.com/rust-lang/rust/blob/1.44.0/src/liballoc/raw_vec.rs#L186
            //
            // In our case, this also ensures that a smallvec of zero-size items never spills,
            // and we never try to allocate zero bytes which `std::alloc::alloc` disallows.
            core::usize::MAX
        }
    }

    /// The maximum number of elements this vector can hold inline
    #[inline]
    pub fn inline_size(&self) -> usize {
        Self::inline_capacity()
    }

    /// The number of items the vector can hold without reallocating
    #[inline]
    pub fn capacity(&self) -> usize {
        self.triple().2
    }

    /// Returns a tuple with (data ptr, len, capacity)
    /// Useful to get all SmallVec properties with a single check of the current storage variant.
    #[inline]
    fn triple(&self) -> (*const A::Item, usize, usize) {
        unsafe {
            if self.spilled() {
                let (ptr, len) = self.data.heap();
                (ptr, len, self.capacity)
            } else {
                (self.data.inline(), self.capacity, Self::inline_capacity())
            }
        }
    }

    /// Returns a tuple with (data ptr, len ptr, capacity)
    #[inline]
    fn triple_mut(&mut self) -> (*mut A::Item, &mut usize, usize) {
        unsafe {
            if self.spilled() {
                let &mut (ptr, ref mut len_ptr) = self.data.heap_mut();
                (ptr, len_ptr, self.capacity)
            } else {
                (
                    self.data.inline_mut(),
                    &mut self.capacity,
                    Self::inline_capacity(),
                )
            }
        }
    }

    /// Returns `true` if the data has spilled into a separate heap-allocated buffer.
    #[inline]
    pub fn spilled(&self) -> bool {
        self.capacity > Self::inline_capacity()
    }

    /// Append an item to the vector.
    #[inline]
    pub fn push(&mut self, value: A::Item) {
        unsafe {
            let (mut ptr, mut len, cap) = self.triple_mut();
            if *len == cap {
                self.reserve(1);
                let &mut (heap_ptr, ref mut heap_len) = self.data.heap_mut();
                ptr = heap_ptr;
                len = heap_len;
            }
            ptr::write(ptr.add(*len), value);
            *len += 1;
        }
    }

    /// Re-allocate to set the capacity to `max(new_cap, inline_size())`.
    ///
    /// Panics if `new_cap` is less than the vector's length
    pub fn try_grow(&mut self, new_cap: usize) -> Result<(), CollectionAllocErr> {
        unsafe {
            let (ptr, &mut len, cap) = self.triple_mut();
            let unspilled = !self.spilled();
            assert!(new_cap >= len);
            if new_cap <= self.inline_size() {
                if unspilled {
                    return Ok(());
                }
                self.data = SmallVecData::from_inline(MaybeUninit::uninit());
                ptr::copy_nonoverlapping(ptr, self.data.inline_mut(), len);
                self.capacity = len;
                deallocate(ptr, cap);
            } else if new_cap != cap {
                let layout = layout_array::<A::Item>(new_cap)?;
                debug_assert!(layout.size() > 0);
                let new_alloc;
                if unspilled {
                    new_alloc = NonNull::new(alloc::alloc::alloc(layout))
                        .ok_or(CollectionAllocErr::AllocErr { layout })?
                        .cast()
                        .as_ptr();
                    ptr::copy_nonoverlapping(ptr, new_alloc, len);
                } else {
                    // This should never fail since the same succeeded
                    // when previously allocating `ptr`.
                    let old_layout = layout_array::<A::Item>(cap)?;

                    let new_ptr = alloc::alloc::realloc(ptr as *mut u8, old_layout, layout.size());
                    new_alloc = NonNull::new(new_ptr)
                        .ok_or(CollectionAllocErr::AllocErr { layout })?
                        .cast()
                        .as_ptr();
                }
                self.data = SmallVecData::from_heap(new_alloc, len);
                self.capacity = new_cap;
            }
            Ok(())
        }
    }

    /// Reserve capacity for `additional` more elements to be inserted.
    ///
    /// May reserve more space to avoid frequent reallocations.
    ///
    /// Panics if the capacity computation overflows `usize`.
    #[inline]
    pub fn reserve(&mut self, additional: usize) {
        infallible(self.try_reserve(additional))
    }

    /// Reserve capacity for `additional` more elements to be inserted.
    ///
    /// May reserve more space to avoid frequent reallocations.
    pub fn try_reserve(&mut self, additional: usize) -> Result<(), CollectionAllocErr> {
        // prefer triple_mut() even if triple() would work
        // so that the optimizer removes duplicated calls to it
        // from callers like insert()
        let (_, &mut len, cap) = self.triple_mut();
        if cap - len >= additional {
            return Ok(());
        }
        let new_cap = len
            .checked_add(additional)
            .and_then(usize::checked_next_power_of_two)
            .ok_or(CollectionAllocErr::CapacityOverflow)?;
        self.try_grow(new_cap)
    }

    /// Insert an element at position `index`, shifting all elements after it to the right.
    ///
    /// Panics if `index` is out of bounds.
    pub fn insert(&mut self, index: usize, element: A::Item) {
        self.reserve(1);

        unsafe {
            let (mut ptr, len_ptr, _) = self.triple_mut();
            let len = *len_ptr;
            assert!(index <= len);
            *len_ptr = len + 1;
            ptr = ptr.add(index);
            ptr::copy(ptr, ptr.add(1), len - index);
            ptr::write(ptr, element);
        }
    }

    /// Insert multiple elements at position `index`, shifting all following elements toward the
    /// back.
    pub fn insert_many<I: IntoIterator<Item = A::Item>>(&mut self, index: usize, iterable: I) {
        let mut iter = iterable.into_iter();
        if index == self.len() {
            return self.extend(iter);
        }

        let (lower_size_bound, _) = iter.size_hint();
        assert!(lower_size_bound <= core::isize::MAX as usize); // Ensure offset is indexable
        assert!(index + lower_size_bound >= index); // Protect against overflow

        let mut num_added = 0;
        let old_len = self.len();
        assert!(index <= old_len);

        unsafe {
            // Reserve space for `lower_size_bound` elements.
            self.reserve(lower_size_bound);
            let start = self.as_mut_ptr();
            let ptr = start.add(index);

            // Move the trailing elements.
            ptr::copy(ptr, ptr.add(lower_size_bound), old_len - index);

            // In case the iterator panics, don't double-drop the items we just copied above.
            self.set_len(0);
            let mut guard = DropOnPanic {
                start,
                skip: index..(index + lower_size_bound),
                len: old_len + lower_size_bound,
            };

            while num_added < lower_size_bound {
                let element = match iter.next() {
                    Some(x) => x,
                    None => break,
                };
                let cur = ptr.add(num_added);
                ptr::write(cur, element);
                guard.skip.start += 1;
                num_added += 1;
            }

            if num_added < lower_size_bound {
                // Iterator provided fewer elements than the hint. Move the tail backward.
                ptr::copy(
                    ptr.add(lower_size_bound),
                    ptr.add(num_added),
                    old_len - index,
                );
            }
            // There are no more duplicate or uninitialized slots, so the guard is not needed.
            self.set_len(old_len + num_added);
            mem::forget(guard);
        }

        // Insert any remaining elements one-by-one.
        for element in iter {
            self.insert(index + num_added, element);
            num_added += 1;
        }

        struct DropOnPanic<T> {
            start: *mut T,
            skip: Range<usize>, // Space we copied-out-of, but haven't written-to yet.
            len: usize,
        }

        impl<T> Drop for DropOnPanic<T> {
            fn drop(&mut self) {
                for i in 0..self.len {
                    if !self.skip.contains(&i) {
                        unsafe {
                            ptr::drop_in_place(self.start.add(i));
                        }
                    }
                }
            }
        }
    }
}

impl<A: Array> ops::Deref for SmallVec<A> {
    type Target = [A::Item];
    #[inline]
    fn deref(&self) -> &[A::Item] {
        unsafe {
            let (ptr, len, _) = self.triple();
            slice::from_raw_parts(ptr, len)
        }
    }
}

impl<A: Array> ops::DerefMut for SmallVec<A> {
    #[inline]
    fn deref_mut(&mut self) -> &mut [A::Item] {
        unsafe {
            let (ptr, &mut len, _) = self.triple_mut();
            slice::from_raw_parts_mut(ptr, len)
        }
    }
}

impl<A: Array> Extend<A::Item> for SmallVec<A> {
    fn extend<I: IntoIterator<Item = A::Item>>(&mut self, iterable: I) {
        let mut iter = iterable.into_iter();
        let (lower_size_bound, _) = iter.size_hint();
        self.reserve(lower_size_bound);

        unsafe {
            let (ptr, len_ptr, cap) = self.triple_mut();
            let mut len = SetLenOnDrop::new(len_ptr);
            while len.get() < cap {
                if let Some(out) = iter.next() {
                    ptr::write(ptr.add(len.get()), out);
                    len.increment_len(1);
                } else {
                    return;
                }
            }
        }

        for elem in iter {
            self.push(elem);
        }
    }
}

unsafe impl<A: Array> Send for SmallVec<A> where A::Item: Send {}

/// Types that can be used as the backing store for a SmallVec
pub unsafe trait Array {
    /// The type of the array's elements.
    type Item;
    /// Returns the number of items the array can hold.
    fn size() -> usize;
}

/// Set the length of the vec when the `SetLenOnDrop` value goes out of scope.
///
/// Copied from https://github.com/rust-lang/rust/pull/36355
struct SetLenOnDrop<'a> {
    len: &'a mut usize,
    local_len: usize,
}

impl<'a> SetLenOnDrop<'a> {
    #[inline]
    fn new(len: &'a mut usize) -> Self {
        SetLenOnDrop {
            local_len: *len,
            len,
        }
    }

    #[inline]
    fn get(&self) -> usize {
        self.local_len
    }

    #[inline]
    fn increment_len(&mut self, increment: usize) {
        self.local_len += increment;
    }
}
