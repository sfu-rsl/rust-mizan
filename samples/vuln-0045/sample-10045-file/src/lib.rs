//! Faster, more compact implementation of `Cow`.
//!
//! **[Changelog](https://github.com/maciejhirsz/beef/releases) -**
//! **[Cargo](https://crates.io/crates/beef) -**
//! **[Repository](https://github.com/maciejhirsz/beef)**
//!
//! ```rust
//! use beef::Cow;
//!
//! let borrowed: Cow<str> = Cow::borrowed("Hello");
//! let owned: Cow<str> = Cow::owned(String::from("World"));
//!
//! assert_eq!(
//!     format!("{} {}!", borrowed, owned),
//!     "Hello World!",
//! );
//! ```
//!
//! There are two versions of `Cow` exposed by this crate:
//!
//! + `beef::Cow` is 3 words wide: pointer, length, and capacity. It stores the ownership tag in capacity.
//! + `beef::lean::Cow` is 2 words wide, storing length, capacity, and the ownership tag all in one word.
//!
//! Both versions are leaner than the `std::borrow::Cow`:
//!
//! ```rust
//! use std::mem::size_of;
//!
//! const WORD: usize = size_of::<usize>();
//!
//! assert_eq!(size_of::<std::borrow::Cow<str>>(), 4 * WORD);
//! assert_eq!(size_of::<beef::Cow<str>>(), 3 * WORD);
//! assert_eq!(size_of::<beef::lean::Cow<str>>(), 2 * WORD);
//! ```
#![cfg_attr(feature = "const_fn", feature(const_fn))]
#![warn(missing_docs)]
#![cfg_attr(not(test), no_std)]
extern crate alloc;

mod traits;
mod wide;

pub mod generic;
#[cfg(target_pointer_width = "64")]
pub mod lean;

#[cfg(not(target_pointer_width = "64"))]
pub mod lean {
    /// Re-exports 3-word Cow for non-64-bit targets
    pub use super::wide::Cow;
}

pub use wide::Cow;
