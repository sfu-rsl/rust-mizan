//! This module contains the actual, albeit generic, implementaiton of the `Cow`,
//! and the traits that are available to it.

use core::marker::PhantomData;
use core::ptr::NonNull;

#[cfg(target_pointer_width = "64")]
use crate::traits::{Beef, Capacity};

/// A clone-on-write smart pointer, mostly compatible with [`std::borrow::Cow`](https://doc.rust-lang.org/std/borrow/enum.Cow.html).
///
/// This type is using a generic `U: Capacity`. Use either [`beef::Cow`](../type.Cow.html) or [`beef::lean::Cow`](../lean/type.Cow.html) in your code.
pub struct Cow<'a, T: Beef + ?Sized + 'a, U: Capacity> {
    /// Pointer to data
    ptr: NonNull<T::PointerT>,

    /// This usize contains length, but it may contain other
    /// information pending on impl of `Capacity`, and must therefore
    /// always go through `U::len` or `U::unpack`
    fat: usize,

    /// Capacity field. For `beef::lean::Cow` this is 0-sized!
    cap: U::Field,

    /// Lifetime marker
    marker: PhantomData<&'a T>,
}

impl<T, U> Cow<'_, T, U>
where
    T: Beef + ?Sized,
    U: Capacity,
{
    /// Owned data.
    ///
    /// # Example
    ///
    /// ```rust
    /// use beef::Cow;
    ///
    /// let owned: Cow<str> = Cow::owned("I own my content".to_string());
    /// ```
    #[inline]
    pub fn owned(val: T::Owned) -> Self {
        let (ptr, fat, cap) = T::owned_into_parts::<U>(val);

        Cow {
            ptr,
            fat,
            cap,
            marker: PhantomData,
        }
    }
}

impl<'a, T, U> Cow<'a, T, U>
where
    T: Beef + ?Sized,
    U: Capacity,
{
    /// Borrowed data.
    ///
    /// # Example
    ///
    /// ```rust
    /// use beef::Cow;
    ///
    /// let borrowed: Cow<str> = Cow::borrowed("I'm just a borrow");
    /// ```
    #[inline]
    pub fn borrowed(val: &'a T) -> Self {
        let (ptr, fat, cap) = T::ref_into_parts::<U>(val);

        Cow {
            ptr,
            fat,
            cap,
            marker: PhantomData,
        }
    }

    /// Extracts borrowed data.
    ///
    /// Panics: If the data is owned.
    #[inline]
    pub fn unwrap_borrowed(self) -> &'a T {
        if self.capacity().is_some() {
            panic!("Can not turn owned beef::Cow into a borrowed value")
        }
        unsafe { &*T::ref_from_parts::<U>(self.ptr, self.fat) }
    }

    /// Internal convenience method for casting `ptr` into a `&T`
    #[inline]
    fn borrow(&self) -> &T {
        unsafe { &*T::ref_from_parts::<U>(self.ptr, self.fat) }
    }

    #[inline]
    fn capacity(&self) -> Option<U::NonZero> {
        U::maybe(self.fat, self.cap)
    }
}

impl<T, U> Drop for Cow<'_, T, U>
where
    T: Beef + ?Sized,
    U: Capacity,
{
    #[inline]
    fn drop(&mut self) {
        if let Some(capacity) = self.capacity() {
            unsafe { T::owned_from_parts::<U>(self.ptr, self.fat, capacity) };
        }
    }
}

impl<'a, T, U> Clone for Cow<'a, T, U>
where
    T: Beef + ?Sized,
    U: Capacity,
{
    #[inline]
    fn clone(&self) -> Self {
        match self.capacity() {
            Some(_) => Cow::owned(self.borrow().to_owned()),
            None => Cow { ..*self },
        }
    }
}

unsafe impl<T: Beef + Sync + ?Sized, U: Capacity> Sync for Cow<'_, T, U> {}
unsafe impl<T: Beef + Send + ?Sized, U: Capacity> Send for Cow<'_, T, U> {}
