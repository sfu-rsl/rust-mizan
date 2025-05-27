//! `bitvec` Prelude
//!
//! This collects the general
//! public API into a single
//! spot for inclusion, as
//! `use bitvec::prelude::*;`,
//! without polluting the root
//! namespace of the crate. !



#[cfg(feature = "alloc")]
pub use crate::bitbox;
pub use crate::bits;
#[cfg(feature = "alloc")]
pub use crate::bitvec;
#[cfg(feature = "alloc")]
pub use crate::boxed::BitBox;
pub use crate::domain::BitDomain;
pub use crate::domain::BitDomainMut;
pub use crate::fields::BitField;
pub use crate::mem::BitMemory;
pub use crate::order::BitOrder;
pub use crate::order::Local;
pub use crate::order::Lsb0;
pub use crate::order::Msb0;
pub use crate::slice::AsBits;
pub use crate::slice::BitSlice;
pub use crate::store::BitStore;
#[cfg(feature = "alloc")]
pub use crate::vec::BitVec;
