/*! Internal implementation macros for the public exports.

The macros in this module are required to be exported from the crate, as the
public macros will call them from client contexts (`macro_rules!` expansion
bodies are not in source crate scope, as they are token expansion rather than
symbolic calls). However, they are not part of the public *API* of the crate,
and are not intended for use anywhere but in the expansion bodies of the
public-API constructor macros.
!*/

#![doc(hidden)]

#[doc(hidden)]
#[cfg(target_endian = "big")]
pub(crate) use self::u8_from_be_bits as u8_from_ne_bits;
#[doc(hidden)]
#[cfg(target_endian = "big")]
pub(crate) use u8_from_be_bytes as u8_from_ne_bytes;
#[doc(hidden)]
#[cfg(target_endian = "big")]
pub(crate) use u16_from_be_bytes as u16_from_ne_bytes;
#[doc(hidden)]
#[cfg(target_endian = "big")]
pub(crate) use u32_from_be_bytes as u32_from_ne_bytes;
#[doc(hidden)]
#[cfg(target_endian = "big")]
pub(crate) use u64_from_be_bytes as u64_from_ne_bytes;
#[doc(hidden)]
#[cfg(target_endian = "big")]
pub(crate) use usize_from_be_bytes as usize_from_ne_bytes;
