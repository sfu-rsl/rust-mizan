#![doc= " Reïmplementation of the `[T]` API.\n\nThis module tracks the [`slice`] primitive and [`core::slice`] module in the\nversion of Rust specified in the `rust-toolchain` file. It is required to\nprovide an exact or equivalent API surface matching the `Box<[T]>` type, to the\nextent that it is possible in the language. Where differences occur, they must\nbe documented in a section called `API Differences`.\n\n[`core::slice`]: https://doc.rust-lang.org/core/slice\n[`slice`]: https://doc.rust-lang.org/std/primitive.slice.html\n!"]

use crate::{
    access::BitAccess,
    order::BitOrder,
    pointer::BitPtr,
    slice::{
        proxy::BitMut,
        BitSlice,
    },
    store::BitStore,
};

use core::{
    marker::PhantomData,
    ops::{
        Range,
        RangeFrom,
        RangeFull,
        RangeInclusive,
        RangeTo,
        RangeToInclusive,
    },
    ptr::NonNull,
};

#[doc= " Reimplementation of the `[T]` inherent-method API."]
impl<O, T> BitSlice<O, T>
where
    O: BitOrder,
    T: BitStore {
    #[doc= " Returns the number of bits in the slice."]
    #[doc= ""]
    #[doc= " # Original"]
    #[doc= ""]
    #[doc= " [`slice::len`](https://doc.rust-lang.org/std/primitive.slice.html#method.len)"]
    #[doc= ""]
    #[doc= " # Examples"]
    #[doc= ""]
    #[doc= " ```rust"]
    #[doc= " # use bitvec::prelude::*;"]
    #[doc= " let bits = 0u8.bits::<Local>();"]
    #[doc= " assert_eq!(bits.len(), 8);"]
    #[doc= " ```"]
    pub(crate) fn len(&self) -> usize {
        self.bitptr().len()
    }

    #[doc= " Returns `true` if the slice has a length of 0."]
    #[doc= ""]
    #[doc= " # Original"]
    #[doc= ""]
    #[doc= " [`slice::is_empty`](https://doc.rust-lang.org/std/primitive.slice.html#method.is_empty)"]
    #[doc= ""]
    #[doc= " # Examples"]
    #[doc= ""]
    #[doc= " ```rust"]
    #[doc= " # use bitvec::prelude::*;"]
    #[doc= " let bits = 0u8.bits::<Local>();"]
    #[doc= " assert!(!bits.is_empty());"]
    #[doc= ""]
    #[doc= " assert!(BitSlice::<Local, usize>::empty().is_empty())"]
    #[doc= " ```"]
    pub(crate) fn is_empty(&self) -> bool {
        self.bitptr().len() == 0
    }


    /// Returns a reference to a bit or subslice, without doing bounds checking.
    ///
    /// This is generally not recommended; use with caution! For a safe
    /// alternative, see [`get`].
    ///
    /// # Safety
    ///
    /// As this function does not perform boundary checking, the caller must
    /// ensure that `self` is an index within the boundaries of `slice` before
    /// calling in order to avoid boundary escapes and ensuing safety
    /// violations.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use bitvec::prelude::*;
    /// let data = 4u8;
    /// let bits = data.bits::<Lsb0>();
    /// unsafe {
    ///     assert!(bits.get_unchecked(2));
    ///     assert!(!bits.get_unchecked(1));
    /// }
    /// ```
    ///
    /// [`get`]: #method.get
    #[inline]
    pub unsafe fn get_unchecked<'a, I>(&'a self, index: I) -> I::Immut
    where I: BitSliceIndex<'a, O, T> {
        index.get_unchecked(self)
    }
}


/** Replacement for [`slice::SliceIndex`].

This trait is stabilized in definition and `type Output` only, but all methods
are unstable. This makes it unusable in non-`libstd` slice libraries, and so it
must be duplicated here.

There is no tracking issue for `feature(slice_index_methods)`.

[`slice::SliceIndex`]: https://doc.rust-lang.org/stable/core/slice/trait.SliceIndex.html
**/
pub trait BitSliceIndex<'a, O, T>
where
    O: 'a + BitOrder,
    T: 'a + BitStore,
{
    /// Immutable output type.
    type Immut;

    /// Mutable output type. This is necessary because `&mut BitSlice` is
    /// producible for range indices, but `&mut bool` is not producable for
    /// `usize` indices.
    type Mut;

    /// Returns a shared reference to the output at this location, if in bounds.
    ///
    /// # Parameters
    ///
    /// - `self`: The index value.
    /// - `slice`: The slice under index.
    ///
    /// # Returns
    ///
    /// An immutable output, if `self` is in bounds; otherwise `None`.
    fn get(self, slice: &'a BitSlice<O, T>) -> Option<Self::Immut>;

    /// Returns a mutable reference to the output at this location, if in
    /// bounds.
    ///
    /// # Parameters
    ///
    /// - `self`: The index value.
    /// - `slice`: The slice under index.
    ///
    /// # Returns
    ///
    /// A mutable output, if `self` is in bounds; otherwise `None`.
    fn get_mut(self, slice: &'a mut BitSlice<O, T>) -> Option<Self::Mut>;

    /// Returns a shared reference to the output at this location, without
    /// performing any bounds checking.
    ///
    /// # Parameters
    ///
    /// - `self`: The index value.
    /// - `slice`: The slice under index.
    ///
    /// # Returns
    ///
    /// An immutable output.
    ///
    /// # Safety
    ///
    /// As this function does not perform boundary checking, the caller must
    /// ensure that `self` is an index within the boundaries of `slice` before
    /// calling in order to avoid boundary escapes and ensuing safety
    /// violations.
    unsafe fn get_unchecked(self, slice: &'a BitSlice<O, T>) -> Self::Immut;

    /// Returns a mutable reference to the output at this location, without
    /// performing any bounds checking.
    ///
    /// # Parameters
    ///
    /// - `self`: The index value.
    /// - `slice`: The slice under index.
    ///
    /// # Returns
    ///
    /// A mutable output.
    ///
    /// # Safety
    ///
    /// As this function does not perform boundary checking, the caller must
    /// ensure that `self` is an index within the boundaries of `slice` before
    /// calling in order to avoid boundary escapes and ensuing safety
    /// violations.
    unsafe fn get_unchecked_mut(
        self,
        slice: &'a mut BitSlice<O, T>,
    ) -> Self::Mut;

    /// Returns a shared reference to the output at this location, panicking if
    /// out of bounds.
    ///
    /// # Parameters
    ///
    /// - `self`: The index value.
    /// - `slice`: The slice under index.
    ///
    /// # Returns
    ///
    /// An immutable output.
    ///
    /// # Panics
    ///
    /// This panics if `self` is out of bounds of `slice`.
    fn index(self, slice: &'a BitSlice<O, T>) -> Self::Immut;

    /// Returns a mutable reference to the output at this location, panicking if
    /// out of bounds.
    ///
    /// # Parameters
    ///
    /// - `self`: The index value.
    /// - `slice`: The slice under index.
    ///
    /// # Returns
    ///
    /// A mutable output.
    ///
    /// # Panics
    ///
    /// This panics if `self` is out of bounds of `slice`.
    fn index_mut(self, slice: &'a mut BitSlice<O, T>) -> Self::Mut;
}

impl<'a, O, T> BitSliceIndex<'a, O, T> for usize
where
    O: 'a + BitOrder,
    T: 'a + BitStore,
{
    type Immut = &'a bool;
    type Mut = BitMut<'a, O, T>;

    fn get(self, slice: &'a BitSlice<O, T>) -> Option<Self::Immut> {
        if self < slice.len() {
            Some(unsafe { self.get_unchecked(slice) })
        }
        else {
            None
        }
    }

    fn get_mut(self, slice: &'a mut BitSlice<O, T>) -> Option<Self::Mut> {
        if self < slice.len() {
            Some(unsafe { self.get_unchecked_mut(slice) })
        }
        else {
            None
        }
    }

    unsafe fn get_unchecked(self, slice: &'a BitSlice<O, T>) -> Self::Immut {
        let bitptr = slice.bitptr();
        let (elt, bit) = bitptr.head().offset(self as isize);
        let data_ptr = bitptr.pointer().a();

        if (&*data_ptr.offset(elt)).get::<O>(bit) {
            &true
        }
        else {
            &false
        }
    }

    #[inline]
    unsafe fn get_unchecked_mut(
        self,
        slice: &'a mut BitSlice<O, T>,
    ) -> Self::Mut
    {
        let bp = slice.bitptr();
        let (offset, head) = bp.head().offset(self as isize);
        let ptr = bp.pointer().a().offset(offset);
        BitMut {
            _parent: PhantomData,
            data: NonNull::new_unchecked(ptr as *mut T::Access),
            head,
            bit: (*ptr).get::<O>(head),
        }
    }

    fn index(self, slice: &'a BitSlice<O, T>) -> Self::Immut {
        self.get(slice).unwrap_or_else(|| {
            panic!("Index {} out of bounds: {}", self, slice.len())
        })
    }

    fn index_mut(self, slice: &'a mut BitSlice<O, T>) -> Self::Mut {
        let len = slice.len();
        self.get_mut(slice)
            .unwrap_or_else(|| panic!("Index {} out of bounds: {}", self, len))
    }
}

impl<'a, O, T> BitSliceIndex<'a, O, T> for Range<usize>
where
    O: 'a + BitOrder,
    T: 'a + BitStore,
{
    type Immut = &'a BitSlice<O, T>;
    type Mut = &'a mut BitSlice<O, T>;

    fn get(self, slice: Self::Immut) -> Option<Self::Immut> {
        (|Range { start, end }, slice: Self::Immut| {
            let len = slice.len();

            if start > len || end > len || start > end {
                return None;
            }

            Some(unsafe { (start..end).get_unchecked(slice) })
        })(self, slice)
    }

    #[inline]
    fn get_mut(self, slice: Self::Mut) -> Option<Self::Mut> {
        self.get(slice).map(|s| s.bitptr().into_bitslice_mut())
    }

    unsafe fn get_unchecked(self, slice: Self::Immut) -> Self::Immut {
        (|Range { start, end }, slice: Self::Immut| {
            let (data, head, _) = slice.bitptr().raw_parts();

            let (skip, new_head) = head.offset(start as isize);

            BitPtr::new_unchecked(
                data.r().offset(skip),
                new_head,
                end - start,
            ).into_bitslice()
        })(self, slice)
    }

    #[inline]
    unsafe fn get_unchecked_mut(self, slice: Self::Mut) -> Self::Mut {
        self.get_unchecked(slice).bitptr().into_bitslice_mut()
    }

    #[inline]
    fn index(self, slice: Self::Immut) -> Self::Immut {
        let r = self.clone();
        let l = slice.len();
        self.clone()
            .get(slice)
            .unwrap_or_else(|| {
                panic!("Range {:?} out of bounds: {}", r, l)
            })
    }

    #[inline]
    fn index_mut(self, slice: Self::Mut) -> Self::Mut {
        self.index(slice).bitptr().into_bitslice_mut()
    }
}
impl<'a, O, T> BitSliceIndex<'a, O, T> for RangeFrom<usize>
where
    O: 'a + BitOrder,
    T: 'a + BitStore,
{
    type Immut = &'a BitSlice<O, T>;
    type Mut = &'a mut BitSlice<O, T>;

    fn get(self, slice: Self::Immut) -> Option<Self::Immut> {
        (|RangeFrom { start }, slice: Self::Immut| {
            let len = slice.len();
            if start <= len {
                Some(unsafe { (start..).get_unchecked(slice) })
            } else {
                None
            }
        })(self, slice)
    }

    #[inline]
    fn get_mut(self, slice: Self::Mut) -> Option<Self::Mut> {
        self.get(slice).map(|s| s.bitptr().into_bitslice_mut())
    }

    unsafe fn get_unchecked(self, slice: Self::Immut) -> Self::Immut {
        (|RangeFrom { start }, slice: Self::Immut| {
            let (data, head, bits) = slice.bitptr().raw_parts();

            let (skip, new_head) = head.offset(start as isize);

            BitPtr::new_unchecked(
                data.r().offset(skip),
                new_head,
                bits - start,
            ).into_bitslice()
        })(self, slice)
    }

    #[inline]
    unsafe fn get_unchecked_mut(self, slice: Self::Mut) -> Self::Mut {
        self.get_unchecked(slice).bitptr().into_bitslice_mut()
    }

    #[inline]
    fn index(self, slice: Self::Immut) -> Self::Immut {
        let r = self.clone();
        let l = slice.len();
        self.clone()
            .get(slice)
            .unwrap_or_else(|| {
                panic!("Range {:?} out of bounds: {}", r, l)
            })
    }

    #[inline]
    fn index_mut(self, slice: Self::Mut) -> Self::Mut {
        self.index(slice).bitptr().into_bitslice_mut()
    }
}
impl<'a, O, T> BitSliceIndex<'a, O, T> for RangeTo<usize>
where
    O: 'a + BitOrder,
    T: 'a + BitStore,
{
    type Immut = &'a BitSlice<O, T>;
    type Mut = &'a mut BitSlice<O, T>;

    fn get(self, slice: Self::Immut) -> Option<Self::Immut> {
        (|RangeTo { end }, slice: Self::Immut| {
            let len = slice.len();
            if end <= len {
                Some(unsafe { (..end).get_unchecked(slice) })
            } else {
                None
            }
        })(self, slice)
    }

    #[inline]
    fn get_mut(self, slice: Self::Mut) -> Option<Self::Mut> {
        self.get(slice).map(|s| s.bitptr().into_bitslice_mut())
    }

    unsafe fn get_unchecked(self, slice: Self::Immut) -> Self::Immut {
        (|RangeTo { end }, slice: Self::Immut| {
            let mut bp = slice.bitptr();
            bp.set_len(end);
            bp.into_bitslice()
        })(self, slice)
    }

    #[inline]
    unsafe fn get_unchecked_mut(self, slice: Self::Mut) -> Self::Mut {
        self.get_unchecked(slice).bitptr().into_bitslice_mut()
    }

    #[inline]
    fn index(self, slice: Self::Immut) -> Self::Immut {
        let r = self.clone();
        let l = slice.len();
        self.clone()
            .get(slice)
            .unwrap_or_else(|| {
                panic!("Range {:?} out of bounds: {}", r, l)
            })
    }

    #[inline]
    fn index_mut(self, slice: Self::Mut) -> Self::Mut {
        self.index(slice).bitptr().into_bitslice_mut()
    }
}

impl<'a, O, T> BitSliceIndex<'a, O, T> for RangeInclusive<usize>
where
    O: 'a + BitOrder,
    T: 'a + BitStore,
{
    type Immut = &'a BitSlice<O, T>;
    type Mut = &'a mut BitSlice<O, T>;

    #[inline]
    fn get(self, slice: Self::Immut) -> Option<Self::Immut> {
        (|this: Self| {
            #[allow(clippy::range_plus_one)]
            (*this.start()..*this.end() + 1)
        })(self).get(slice)
    }

    #[inline]
    fn get_mut(self, slice: Self::Mut) -> Option<Self::Mut> {
        (|this: Self| {
            #[allow(clippy::range_plus_one)]
            (*this.start()..*this.end() + 1)
        })(self).get_mut(slice)
    }

    #[inline]
    unsafe fn get_unchecked(self, slice: Self::Immut) -> Self::Immut {
        (|this: Self| {
            #[allow(clippy::range_plus_one)]
            (*this.start()..*this.end() + 1)
        })(self).get_unchecked(slice)
    }

    #[inline]
    unsafe fn get_unchecked_mut(self, slice: Self::Mut) -> Self::Mut {
        (|this: Self| {
            #[allow(clippy::range_plus_one)]
            (*this.start()..*this.end() + 1)
        })(self).get_unchecked_mut(slice)
    }

    #[inline]
    fn index(self, slice: Self::Immut) -> Self::Immut {
        (|this: Self| {
            #[allow(clippy::range_plus_one)]
            (*this.start()..*this.end() + 1)
        })(self).index(slice)
    }

    #[inline]
    fn index_mut(self, slice: Self::Mut) -> Self::Mut {
        (|this: Self| {
            #[allow(clippy::range_plus_one)]
            (*this.start()..*this.end() + 1)
        })(self).index_mut(slice)
    }
}
impl<'a, O, T> BitSliceIndex<'a, O, T> for RangeToInclusive<usize>
where
    O: 'a + BitOrder,
    T: 'a + BitStore,
{
    type Immut = &'a BitSlice<O, T>;
    type Mut = &'a mut BitSlice<O, T>;

    #[inline]
    fn get(self, slice: Self::Immut) -> Option<Self::Immut> {
        (|RangeToInclusive { end }| {
            #[allow(clippy::range_plus_one)]
            (..end + 1)
        })(self).get(slice)
    }

    #[inline]
    fn get_mut(self, slice: Self::Mut) -> Option<Self::Mut> {
        (|RangeToInclusive { end }| {
            #[allow(clippy::range_plus_one)]
            (..end + 1)
        })(self).get_mut(slice)
    }

    #[inline]
    unsafe fn get_unchecked(self, slice: Self::Immut) -> Self::Immut {
        (|RangeToInclusive { end }| {
            #[allow(clippy::range_plus_one)]
            (..end + 1)
        })(self).get_unchecked(slice)
    }

    #[inline]
    unsafe fn get_unchecked_mut(self, slice: Self::Mut) -> Self::Mut {
        (|RangeToInclusive { end }| {
            #[allow(clippy::range_plus_one)]
            (..end + 1)
        })(self).get_unchecked_mut(slice)
    }

    #[inline]
    fn index(self, slice: Self::Immut) -> Self::Immut {
        (|RangeToInclusive { end }| {
            #[allow(clippy::range_plus_one)]
            (..end + 1)
        })(self).index(slice)
    }

    #[inline]
    fn index_mut(self, slice: Self::Mut) -> Self::Mut {
        (|RangeToInclusive { end }| {
            #[allow(clippy::range_plus_one)]
            (..end + 1)
        })(self).index_mut(slice)
    }
}

/// `RangeFull` is the identity function.
impl<'a, O, T> BitSliceIndex<'a, O, T> for RangeFull
where
    O: 'a + BitOrder,
    T: 'a + BitStore,
{
    type Immut = &'a BitSlice<O, T>;
    type Mut = &'a mut BitSlice<O, T>;

    #[inline]
    fn get(self, slice: Self::Immut) -> Option<Self::Immut> {
        Some(slice)
    }

    #[inline]
    fn get_mut(self, slice: Self::Mut) -> Option<Self::Mut> {
        Some(slice)
    }

    #[inline]
    unsafe fn get_unchecked(self, slice: Self::Immut) -> Self::Immut {
        slice
    }

    #[inline]
    unsafe fn get_unchecked_mut(self, slice: Self::Mut) -> Self::Mut {
        slice
    }

    #[inline]
    fn index(self, slice: Self::Immut) -> Self::Immut {
        slice
    }

    #[inline]
    fn index_mut(self, slice: Self::Mut) -> Self::Mut {
        slice
    }
}