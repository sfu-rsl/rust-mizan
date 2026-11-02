//! Provides two types, `CSemiBox` and `DisposeRef`

use libc::{c_char, c_void, free};
use std::ffi::CStr;
use std::marker::PhantomData;
use std::ops::{Deref, Drop};
use std::{fmt, str};
/// Implemented by any type of which its reference represents a C pointer that can be disposed.
pub trait DisposeRef {
    /// What a reference to this type represents as a C pointer.
    type RefTo;
    /// Destroy the contents at the pointer's location.
    ///
    /// This should run some variant of `libc::free(ptr)`
    unsafe fn dispose(ptr: *mut Self::RefTo) {
        free(ptr as *mut c_void);
    }
}

/// A wrapper for pointers made by C that are now partially owned in Rust.
///
/// This is necessary to allow owned and borrowed representations of C types
/// to be represented by the same type as they are in C with little overhead
pub struct CSemiBox<'a, D: ?Sized>
where
    D: DisposeRef + 'a,
{
    ptr: *mut D::RefTo,
    marker: PhantomData<&'a ()>,
}
impl<'a, D: ?Sized> CSemiBox<'a, D>
where
    D: DisposeRef + 'a,
{
    #[inline(always)]
    /// Wrap the pointer in a `CSemiBox`
    pub fn new(ptr: *mut D::RefTo) -> Self {
        CSemiBox {
            ptr: ptr,
            marker: PhantomData,
        }
    }
}

impl<'a, D: ?Sized> Drop for CSemiBox<'a, D>
where
    D: DisposeRef + 'a,
{
    #[inline(always)]
    /// Run the destructor
    fn drop(&mut self) {
        unsafe { <D as DisposeRef>::dispose(self.ptr) }
    }
}

impl DisposeRef for str {
    type RefTo = c_char;
}

/// A wrapper for pointers made by C that are now completely owned by Rust, so
/// they are not limited by any lifetimes.
///
/// This is necessary to allow owned and borrowed representations of C types
/// to be represented by the same type as they are in C with little overhead.
pub struct CBox<D: ?Sized>
where
    D: DisposeRef,
{
    ptr: *mut D::RefTo,
}
impl<D: ?Sized> CBox<D>
where
    D: DisposeRef,
{
    #[inline(always)]
    /// Wrap the pointer in a `CBox`.
    pub fn new(ptr: *mut D::RefTo) -> Self {
        CBox { ptr: ptr }
    }
}

impl<'a> Deref for CBox<str> {
    type Target = str;
    fn deref(&self) -> &str {
        unsafe {
            let text = CStr::from_ptr(self.ptr);
            str::from_utf8_unchecked(text.to_bytes())
        }
    }
}

impl fmt::Display for CBox<str> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(self.deref())
    }
}
impl fmt::Debug for CBox<str> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(self.deref())
    }
}
