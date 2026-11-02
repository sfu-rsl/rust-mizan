// This is a part of Chrono.
// Portions Copyright 2013-2014 The Rust Project Developers.
// See README.md and LICENSE.txt for details.

//! Integer division utilities. (Shamelessly copied from [num](https://github.com/rust-lang/num/))

// Algorithm from [Daan Leijen. _Division and Modulus for Computer Scientists_,
// December 2001](http://research.microsoft.com/pubs/151917/divmodnote-letter.pdf)

pub use num_integer::{div_mod_floor, div_rem, mod_floor};
