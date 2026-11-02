#![no_std]
#![cfg_attr(test, deny(warnings))]
#![deny(missing_docs)]

//! Wrappers for total order on Floats.

#[cfg(feature = "std")]
extern crate std;
#[cfg(feature = "std")]
use std::error::Error;

use core::cmp::Ordering;
use core::fmt;
use core::hint::unreachable_unchecked;
use core::ops::{AddAssign, DivAssign, MulAssign, RemAssign, SubAssign};
#[cfg(not(feature = "std"))]
use num_traits::float::FloatCore as Float;
#[cfg(feature = "std")]
use num_traits::Float;

/// A wrapper around Floats providing an implementation of Ord and Hash.
///
/// A NaN value cannot be stored in this type.
#[derive(PartialOrd, PartialEq, Debug, Default, Clone, Copy)]
#[repr(transparent)]
pub struct NotNan<T>(T);

impl<T> NotNan<T> {
    /// Create a NotNan value from a value that is guaranteed to not be NaN
    ///
    /// # Safety
    ///
    /// Behaviour is undefined if `val` is NaN
    pub const unsafe fn unchecked_new(val: T) -> Self {
        NotNan(val)
    }
}

impl<T: Float> NotNan<T> {
    /// Create a NotNan value.
    ///
    /// Returns Err if val is NaN
    pub fn new(val: T) -> Result<Self, FloatIsNan> {
        match val {
            ref val if val.is_nan() => Err(FloatIsNan),
            val => Ok(NotNan(val)),
        }
    }

    /// Get the value out.
    pub fn into_inner(self) -> T {
        self.0
    }
}

/// Adds a float directly.
///
/// Panics if the provided value is NaN.
impl<T: Float + AddAssign> AddAssign<T> for NotNan<T> {
    fn add_assign(&mut self, other: T) {
        self.0 += other;
        assert!(!self.0.is_nan(), "Addition resulted in NaN");
    }
}

/// Subtracts a float directly.
///
/// Panics if the provided value is NaN or the computation results in NaN
impl<T: Float + SubAssign> SubAssign<T> for NotNan<T> {
    fn sub_assign(&mut self, other: T) {
        self.0 -= other;
        assert!(!self.0.is_nan(), "Subtraction resulted in NaN");
    }
}

/// Multiplies a float directly.
///
/// Panics if the provided value is NaN.
impl<T: Float + MulAssign> MulAssign<T> for NotNan<T> {
    fn mul_assign(&mut self, other: T) {
        self.0 *= other;
        assert!(!self.0.is_nan(), "Multiplication resulted in NaN");
    }
}

/// Divides a float directly.
///
/// Panics if the provided value is NaN or the computation results in NaN
impl<T: Float + DivAssign> DivAssign<T> for NotNan<T> {
    fn div_assign(&mut self, other: T) {
        self.0 /= other;
        assert!(!self.0.is_nan(), "Division resulted in NaN");
    }
}

/// Calculates `%=` with a float directly.
///
/// Panics if the provided value is NaN or the computation results in NaN
impl<T: Float + RemAssign> RemAssign<T> for NotNan<T> {
    fn rem_assign(&mut self, other: T) {
        self.0 %= other;
        assert!(!self.0.is_nan(), "Rem resulted in NaN");
    }
}

impl<T: Float> Ord for NotNan<T> {
    fn cmp(&self, other: &NotNan<T>) -> Ordering {
        match self.partial_cmp(&other) {
            Some(ord) => ord,
            None => unsafe { unreachable_unchecked() },
        }
    }
}

impl<T: Float + PartialEq> Eq for NotNan<T> {}

impl<T: Float> PartialEq<T> for NotNan<T> {
    fn eq(&self, other: &T) -> bool {
        self.0 == *other
    }
}

/// An error indicating an attempt to construct NotNan from a NaN
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct FloatIsNan;

#[cfg(feature = "std")]
impl Error for FloatIsNan {
    fn description(&self) -> &str {
        "NotNan constructed with NaN"
    }
}

impl fmt::Display for FloatIsNan {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "NotNan constructed with NaN")
    }
}
