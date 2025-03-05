#![no_std]

pub extern crate alloc;

use alloc::alloc::{Layout, LayoutErr};
use core::fmt;
use core::hint::unreachable_unchecked;
use core::iter::IntoIterator;
use core::mem;
use core::mem::MaybeUninit;
use core::ops::Range;
use core::ptr;

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

#[derive(Debug)]
pub enum CollectionAllocErr {
    CapacityOverflow,
    AllocErr { layout: Layout },
}

impl fmt::Display for CollectionAllocErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Allocation error: {:?}", self)
    }
}

impl From<LayoutErr> for CollectionAllocErr {
    fn from(_: LayoutErr) -> Self {
        CollectionAllocErr::CapacityOverflow
    }
}

fn infallible<T>(result: Result<T, CollectionAllocErr>) -> T {
    match result {
        Ok(x) => x,
        Err(CollectionAllocErr::CapacityOverflow) => panic!("capacity overflow"),
        Err(CollectionAllocErr::AllocErr { layout }) => alloc::alloc::handle_alloc_error(layout),
    }
}

fn layout_array<T>(n: usize) -> Result<Layout, CollectionAllocErr> {
    let size = mem::size_of::<T>()
        .checked_mul(n)
        .ok_or(CollectionAllocErr::CapacityOverflow)?;
    let align = mem::align_of::<T>();
    Layout::from_size_align(size, align).map_err(|_| CollectionAllocErr::CapacityOverflow)
}

unsafe fn deallocate<T>(ptr: *mut T, capacity: usize) {
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

pub unsafe trait Array {
    type Item;
    fn size() -> usize;
}

pub struct SmallVec<A: Array> {
    capacity: usize,
    data: SmallVecData<A>,
}

impl<A: Array> SmallVec<A> {
    #[inline]
    pub fn new() -> SmallVec<A> {
        assert!(
            mem::size_of::<A>() == A::size() * mem::size_of::<A::Item>()
                && mem::align_of::<A>() >= mem::align_of::<A::Item>()
        );
        SmallVec {
            capacity: 0,
            data: SmallVecData::from_inline(MaybeUninit::uninit()),
        }
    }

    fn inline_capacity() -> usize {
        if mem::size_of::<A::Item>() > 0 {
            A::size()
        } else {
            core::usize::MAX
        }
    }

    pub fn inline_size(&self) -> usize {
        Self::inline_capacity()
    }

    pub fn len(&self) -> usize {
        self.triple().1
    }

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

    pub fn spilled(&self) -> bool {
        self.capacity > Self::inline_capacity()
    }

    unsafe fn set_len(&mut self, new_len: usize) {
        let (_, len_ptr, _) = self.triple_mut();
        *len_ptr = new_len;
    }

    pub fn as_mut_ptr(&mut self) -> *mut A::Item {
        self.triple_mut().0
    }

    pub fn reserve(&mut self, additional: usize) {
        infallible(self.try_reserve(additional))
    }

    pub fn try_reserve(&mut self, additional: usize) -> Result<(), CollectionAllocErr> {
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
                let new_alloc;
                if unspilled {
                    new_alloc = ptr::NonNull::new(alloc::alloc::alloc(layout))
                        .ok_or(CollectionAllocErr::AllocErr { layout })?
                        .cast()
                        .as_ptr();
                    ptr::copy_nonoverlapping(ptr, new_alloc, len);
                } else {
                    let old_layout = layout_array::<A::Item>(cap)?;
                    let new_ptr = alloc::alloc::realloc(ptr as *mut u8, old_layout, layout.size());
                    new_alloc = ptr::NonNull::new(new_ptr)
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

    pub fn extend<I: IntoIterator<Item = A::Item>>(&mut self, iterable: I) {
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

    pub fn insert_many<I: IntoIterator<Item = A::Item>>(&mut self, index: usize, iterable: I) {
        let mut iter = iterable.into_iter();
        if index == self.len() {
            return self.extend(iter);
        }

        let (lower_size_bound, _) = iter.size_hint();
        assert!(lower_size_bound <= core::isize::MAX as usize);
        assert!(index + lower_size_bound >= index);

        let mut num_added = 0;
        let old_len = self.len();
        assert!(index <= old_len);

        unsafe {
            // Reserve space for `lower_size_bound` elements.
            self.reserve(lower_size_bound);
            let start = self.as_mut_ptr();
            let ptr = start.add(index);

            ptr::copy(ptr, ptr.add(lower_size_bound), old_len - index);

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
            mem::forget(guard);

            if num_added < lower_size_bound {
                ptr::copy(
                    ptr.add(lower_size_bound),
                    ptr.add(num_added),
                    old_len - index,
                );
            }

            self.set_len(old_len + num_added);
        }

        struct DropOnPanic<T> {
            start: *mut T,
            skip: Range<usize>,
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

impl<A: Array> Default for SmallVec<A> {
    fn default() -> SmallVec<A> {
        SmallVec::new()
    }
}

struct SetLenOnDrop<'a> {
    len: &'a mut usize,
    local_len: usize,
}

impl<'a> SetLenOnDrop<'a> {
    fn new(len: &'a mut usize) -> Self {
        SetLenOnDrop {
            local_len: *len,
            len,
        }
    }

    fn get(&self) -> usize {
        self.local_len
    }

    fn increment_len(&mut self, increment: usize) {
        self.local_len += increment;
    }
}

impl<'a> Drop for SetLenOnDrop<'a> {
    fn drop(&mut self) {
        *self.len = self.local_len;
    }
}

unsafe impl<T> Array for [T; 1] {
    type Item = T;
    fn size() -> usize {
        1
    }
}
