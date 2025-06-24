//! This crate provides a super-fast decimal number parser from strings into floats.
//!
//! ## Usage
//!
//! There's two top-level functions provided: [`parse`](crate::parse()) and
//! [`parse_partial`](crate::parse_partial()), both taking
//! either a string or a bytes slice and parsing the input into either `f32` or `f64`:
//!
//! - [`parse`](crate::parse()) treats the whole string as a decimal number and returns an
//!   error if there are invalid characters or if the string is empty.
//! - [`parse_partial`](crate::parse_partial()) tries to find the longest substring at the
//! beginning of the given input string that can be parsed as a decimal number and,
//! in the case of success, returns the parsed value along the number of characters processed;
//! an error is returned if the string doesn't start with a decimal number or if it is empty.
//! This function is most useful as a building block when constructing more complex parsers,
//! or when parsing streams of data.
//!
//! ## Examples
//!
//! ```rust
//! // Parse the entire string as a decimal number.
//! let s = "1.23e-02";
//! let x: f32 = fast_float::parse(s).unwrap();
//! assert_eq!(x, 0.0123);
//!
//! // Parse as many characters as possible as a decimal number.
//! let s = "1.23e-02foo";
//! let (x, n) = fast_float::parse_partial::<f32, _>(s).unwrap();
//! assert_eq!(x, 0.0123);
//! assert_eq!(n, 8);
//! assert_eq!(&s[n..], "foo");
//! ```

#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    clippy::cast_lossless,
    clippy::cast_precision_loss,
    clippy::missing_const_for_fn,
    clippy::use_self,
    clippy::module_name_repetitions,
    clippy::cargo_common_metadata
)]

use core::fmt::{self, Display};

mod common;

/// Opaque error type for fast-float parsing functions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Error;

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "error while parsing a float")
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {
    fn description(&self) -> &str {
        "error while parsing a float"
    }
}

/// Result type alias for fast-float parsing functions.
pub type Result<T> = core::result::Result<T, Error>;