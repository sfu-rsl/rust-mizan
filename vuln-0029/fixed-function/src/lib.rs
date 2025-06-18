#![cfg_attr(all(not(test), not(feature = "std")), no_std)]

//! # `pared`
//! Reference-counted pointers that contain projections of data stored in [`std::sync::Arc`]
//! or [`std::rc::Rc`].
//! This is a "self-referential" type in the vein of [ouroboros](https://lib.rs/ouroboros)
//! or [yoke](https://lib.rs/yoke).
//!
//! This crate specializes to only supporting `Arc` and `Rc` and only references to fields
//! obtainable from them, which allows it to provide a much simpler API compared to general
//! self-referential crates.
//!
//! Parc can be useful in situations where we want to expose only a part of data stored
//! in a reference-counted pointer while still retaining the same shared ownership of that data.
//! We project a field from our stored data to store in Parc, allowing us to only expose that data
//! to the receiver.
//!
//! # Example
//! ```
//! use std::sync::Arc;
//! use pared::sync::Parc;
//!
//! fn accepts_parc(parc: Parc<u8>) {}
//!
//! // Parc can be created by projecting references from an Arc
//! let from_tuple = Parc::from_arc(&Arc::new((16usize, 8u8)), |tuple| &tuple.1);
//! // Or by using any T: Into<Arc<_>>
//! let from_u8: Parc<_> = Parc::new(8u8);
//!
//! // Functions accept any Parc<T>, regardless of which Arc<U> it was created from
//! if (true) {
//!     accepts_parc(from_tuple);
//! } else {
//!     accepts_parc(from_u8);
//! }
//! ```

// #![deny(missing_docs)]
#![deny(clippy::std_instead_of_core)]
#![deny(clippy::std_instead_of_alloc)]
#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

extern crate alloc;
extern crate core;

#[cfg(doctest)]
doc_comment::doctest!("../README.md");

pub mod prc;
pub mod sync;

mod erased_ptr;
mod vtable;

use alloc::rc::Rc;
use core::{
    ops::Deref,
    ptr::NonNull,
};

use crate::prc::erased_rc::TypeErasedRc;

pub struct Prc<T: ?Sized> {
    rc: TypeErasedRc,
    projected: NonNull<T>,
}

impl<T: ?Sized> Prc<T> {
    /// Constructs a new `Prc<T>` from an existing `Rc<T>` by projecting a field.
    ///
    /// # Panics
    /// If `f` panics, the panic is propagated to the caller and the rc won't be cloned.
    ///
    /// # Example
    /// ```
    /// # use std::rc::Rc;
    /// use pared::prc::Prc;
    /// let rc = Rc::new((5u64,));
    /// let prc = Prc::from_rc(&rc, |tuple| &tuple.0);
    /// ```
    ///
    /// Note that references to local variables cannot be returned from the `project` function:
    /// ```compile_fail,E0597
    /// # use std::rc::Rc;
    /// use pared::prc::Prc;
    /// let rc = Rc::new((5u64,));
    /// let local = 5;
    /// let prc = Prc::from_rc(&rc, |_| &local);
    /// ```
    ///
    /// Do not allow calling `from_rc` on `Rc`s that don't own their data.
    /// If this is allowed to happen, we could create a `Prc` with a longer lifetime than the
    /// original `Rc`.
    /// ```compile_fail,E0597
    /// use pared::prc::Prc;
    /// use std::rc::Rc;
    ///
    /// struct PrintOnDrop<'a>(&'a str);
    /// impl Drop for PrintOnDrop<'_> {
    ///     fn drop(&mut self) {
    ///         println!("dropping: {:?}", self.0);
    ///     }
    /// }
    ///
    /// let s = "Hello World!".to_owned();
    /// let rc = Rc::new(PrintOnDrop(&s));
    /// let p = Prc::from_rc(&rc, |_| &());
    /// drop(rc);
    /// drop(s);
    /// drop(p); // garbage / use-after-free if `from_rc` compiles
    /// ```
    #[inline]
    pub fn from_rc<U, F>(rc: &Rc<U>, project: F) -> Self
    where
        U: ?Sized + 'static,
        T: 'static,
        F: FnOnce(&U) -> &T,
    {
        let projected = project(rc);
        // SAFETY: fn shouldn't be able to capture any local references
        // which should mean that the projection done by f is safe
        let projected = unsafe { NonNull::new_unchecked(projected as *const T as *mut T) };
        Self {
            rc: TypeErasedRc::new(rc.clone()),
            projected,
        }
    }

    /// Constructs a new `Option<Prc<T>>` from an existing `Rc<T>` by trying to project a field.
    ///
    /// If the function passed into this returns `None`, this method will also return `None`.
    ///
    /// # Panics
    /// If `f` panics, the panic is propagated to the caller and the rc won't be cloned.
    ///
    /// # Example
    /// ```
    /// use std::rc::Rc;
    /// use pared::prc::Prc;
    ///
    /// enum Enum {
    ///     Str(String),
    ///     Int(isize),
    /// }
    ///
    /// let rc = Rc::new(Enum::Int(5));
    /// let prc = Prc::try_from_rc(&rc, |x| match x {
    ///     Enum::Str(s) => Err(()),
    ///     Enum::Int(i) => Ok(i),
    /// });
    ///
    /// assert!(matches!(prc, Ok(prc) if *prc == 5 ));
    /// ```
    ///
    /// Do not allow calling `from_rc` on `Rc`s that don't own their data.
    /// If this is allowed to happen, we could create a `Prc` with a longer lifetime than the
    /// original `Rc`.
    /// ```compile_fail,E0597
    /// use pared::prc::Prc;
    /// use std::rc::Rc;
    ///
    /// struct PrintOnDrop<'a>(&'a str);
    /// impl Drop for PrintOnDrop<'_> {
    ///     fn drop(&mut self) {
    ///         println!("dropping: {:?}", self.0);
    ///     }
    /// }
    ///
    /// let s = "Hello World!".to_owned();
    /// let rc = Rc::new(PrintOnDrop(&s));
    /// let p = Prc::try_from_rc(&rc, |_| Ok::<&'static(), ()>(&()));
    /// drop(rc);
    /// drop(s);
    /// drop(p); // garbage / use-after-free if `try_from_rc` compiles
    /// ```
    #[inline]
    pub fn try_from_rc<U, E, F>(rc: &Rc<U>, project: F) -> Result<Self, E>
    where
        U: ?Sized + 'static,
        T: 'static,
        F: FnOnce(&U) -> Result<&T, E>,
    {
        let projected = project(rc)?;
        // SAFETY: fn shouldn't be able to capture any local references
        // which should mean that the projection done by f is safe
        let projected = unsafe { NonNull::new_unchecked(projected as *const T as *mut T) };
        Ok(Self {
            rc: TypeErasedRc::new(rc.clone()),
            projected,
        })
    }

    /// Constructs a new `Prc<T>` from an existing `Prc<T>` by projecting a field.
    ///
    /// # Panics
    /// If `f` panics, the panic is propagated to the caller and the underlying rc won't be cloned.
    ///
    /// # Example
    /// ```
    /// use pared::prc::Prc;
    /// let prc = Prc::new((5u64,));
    /// let projected = prc.project(|tuple| &tuple.0);
    /// ```
    ///
    /// Note that references to local variables cannot be returned from the `project` function:
    /// ```compile_fail,E0597
    /// use pared::prc::Prc;
    /// let prc = Prc::new((5u64,));
    /// let local = 5;
    /// let projected = prc.project(|_| &local);
    /// ```
    ///
    /// /// Do not allow coercing `Prc<&'static T>` to `Prc<&'short T>` when projecting:
    /// ```compile_fail,E0597
    /// use pared::prc::Prc;
    ///
    /// let s = "Hello World!".to_owned();
    /// // x: Prc<&'static ()>
    /// let x = Prc::new(&());
    /// // This must fail
    /// let x = x.project(|_| s.as_str());
    ///
    /// println!("{:?}", &*x); // "Hello World!"
    /// drop(s);
    /// println!("{:?}", &*x); // garbage / use-after-free if the above doesn't fail
    /// ```
    #[inline]
    pub fn project<U, F>(&self, project: F) -> Prc<U>
    where
        U: ?Sized + 'static,
        T: 'static,
        F: FnOnce(&T) -> &U,
    {
        let projected = project(self);
        // SAFETY: fn shouldn't be able to capture any local references
        // which should mean that the projection done by f is safe
        let projected = unsafe { NonNull::new_unchecked(projected as *const U as *mut U) };
        Prc::<U> {
            rc: self.rc.clone(),
            projected,
        }
    }
}

impl<T: ?Sized> Deref for Prc<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { self.projected.as_ref() }
    }
}

use alloc::sync::Arc;

use crate::sync::erased_arc::TypeErasedArc;

pub struct Parc<T: ?Sized> {
    arc: TypeErasedArc,
    projected: NonNull<T>,
}

impl<T: ?Sized> Parc<T> {
    /// Constructs a new `Parc<T>` from an existing `Arc<T>` by projecting a field.
    ///
    /// # Panics
    /// If `f` panics, the panic is propagated to the caller and the arc won't be cloned.
    ///
    /// # Example
    /// ```
    /// # use std::sync::Arc;
    /// use pared::sync::Parc;
    /// let arc = Arc::new((5u64,));
    /// let parc = Parc::from_arc(&arc, |tuple| &tuple.0);
    /// ```
    ///
    /// Note that references to local variables cannot be returned from the `project` function:
    /// ```compile_fail,E0597
    /// # use std::sync::Arc;
    /// use pared::sync::Parc;
    /// let arc = Arc::new((5u64,));
    /// let local = 5;
    /// let parc = Parc::from_arc(&arc, |tuple| &local);
    /// ```
    ///
    /// Do not allow calling `from_arc` on `Arc`s that don't own their data.
    /// If this is allowed to happen, we could create a `Parc` with a longer lifetime than the
    /// original `Arc`.
    /// ```compile_fail,E0597
    /// use pared::sync::Parc;
    /// use std::sync::Arc;
    ///
    /// struct PrintOnDrop<'a>(&'a str);
    /// impl Drop for PrintOnDrop<'_> {
    ///     fn drop(&mut self) {
    ///         println!("dropping: {:?}", self.0);
    ///     }
    /// }
    ///
    /// let s = "Hello World!".to_owned();
    /// let arc = Arc::new(PrintOnDrop(&s));
    /// let p = Parc::from_arc(&arc, |_| &());
    /// drop(arc);
    /// drop(s);
    /// drop(p); // garbage / use-after-free if `from_arc` compiles
    /// ```
    #[inline]
    pub fn from_arc<U, F>(arc: &Arc<U>, project: F) -> Self
    where
        U: ?Sized + Send + Sync + 'static,
        F: FnOnce(&U) -> &T,
        T: 'static,
    {
        let projected = project(arc);
        // SAFETY: the returned reference always converts to a non-null pointer.
        // It's safe to convert the returned reference to a pointer (and then convert it in `Deref`)
        // because the lifetime of the reference returned by `F` must be either the lifetime
        // of the local reference passed to it, or 'static
        let projected = unsafe { NonNull::new_unchecked(projected as *const T as *mut T) };
        Self {
            arc: TypeErasedArc::new(arc.clone()),
            projected,
        }
    }

    /// Constructs a new `Result<Parc<T>, E>` from an existing `Arc<T>`
    /// by trying to project a field.
    ///
    /// If the function passed into this returns `Err(x)`, this method will also return `Err(x)`.
    ///
    /// # Panics
    /// If `f` panics, the panic is propagated to the caller and the rc won't be cloned.
    ///
    /// # Example
    /// ```
    /// use std::sync::Arc;
    /// use pared::sync::Parc;
    ///
    /// enum Enum {
    ///     Str(String),
    ///     Int(isize),
    /// }
    ///
    /// let arc = Arc::new(Enum::Int(5));
    /// let parc = Parc::try_from_arc(&arc, |x| match x {
    ///     Enum::Str(s) => Err(()),
    ///     Enum::Int(i) => Ok(i),
    /// });
    ///
    /// assert!(matches!(parc, Ok(parc) if *parc == 5 ));
    /// ```
    ///
    /// Do not allow calling `try_from_arc` on `Arc`s that don't own their data.
    /// If this is allowed to happen, we could create a `Parc` with a longer lifetime than the
    /// original `Arc`.
    /// ```compile_fail,E0597
    /// use pared::sync::Parc;
    /// use std::sync::Arc;
    ///
    /// struct PrintOnDrop<'a>(&'a str);
    /// impl Drop for PrintOnDrop<'_> {
    ///     fn drop(&mut self) {
    ///         println!("dropping: {:?}", self.0);
    ///     }
    /// }
    ///
    /// let s = "Hello World!".to_owned();
    /// let arc = Arc::new(PrintOnDrop(&s));
    /// let p = Parc::try_from_arc(&arc, |_| Ok::<&'static (), ()>(&()));
    /// drop(arc);
    /// drop(s);
    /// drop(p); // garbage / use-after-free if `try_from_arc` compiles
    /// ```
    #[inline]
    pub fn try_from_arc<U, E, F>(arc: &Arc<U>, project: F) -> Result<Self, E>
    where
        U: ?Sized + Sync + Send + 'static,
        T: 'static,
        F: FnOnce(&U) -> Result<&T, E>,
    {
        let projected = project(arc)?;
        // SAFETY: fn shouldn't be able to capture any local references
        // which should mean that the projection done by f is safe
        let projected = unsafe { NonNull::new_unchecked(projected as *const T as *mut T) };
        Ok(Self {
            arc: TypeErasedArc::new(arc.clone()),
            projected,
        })
    }

    /// Constructs a new `Parc<T>` from an existing `Parc<T>` by projecting a field.
    ///
    /// # Panics
    /// If `f` panics, the panic is propagated to the caller and the underlying arc won't be cloned.
    ///
    /// # Example
    /// ```
    /// use pared::sync::Parc;
    /// let parc = Parc::new((5u64,));
    /// let projected = parc.project(|tuple| &tuple.0);
    /// ```
    ///
    /// Note that references to local variables cannot be returned from the `project` function:
    /// ```compile_fail,E0597
    /// use pared::sync::Parc;
    /// let parc = Parc::new((5u64,));
    /// let local = 5;
    /// let projected = parc.project(|tuple| &local);
    /// ```
    ///
    /// Do not allow coercing `Parc<&'static T>` to `Parc<&'short T>` when projecting:
    /// ```compile_fail,E0597
    /// use pared::sync::Parc;
    ///
    /// let s = "Hello World!".to_owned();
    /// // x: Parc<&'static ()>
    /// let x = Parc::new(&());
    /// // This must fail
    /// let x = x.project(|_| s.as_str());
    ///
    /// println!("{:?}", &*x); // "Hello World!"
    /// drop(s);
    /// println!("{:?}", &*x); // garbage / use-after-free if the above doesn't fail
    /// ```
    #[inline]
    pub fn project<U, F>(&self, project: F) -> Parc<U>
    where
        T: Send + Sync + 'static,
        U: ?Sized + 'static,
        F: FnOnce(&T) -> &U,
    {
        let projected = project(self);
        // SAFETY: the returned reference always converts to a non-null pointer.
        // It's safe to convert the returned reference to a pointer (and then convert it in `Deref`)
        // because the lifetime of the reference returned by `F` must be either the lifetime
        // of the local reference passed to it, or 'static
        let projected = unsafe { NonNull::new_unchecked(projected as *const U as *mut U) };
        Parc::<U> {
            arc: self.arc.clone(),
            projected,
        }
    }
}

impl<T: ?Sized> Deref for Parc<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        // SAFETY: projected is safely constructed only in `from_arc` or `project`,
        // where we guarantee the pointer will be valid as long as the original `Arc` lives.
        unsafe { self.projected.as_ref() }
    }
}
