//! `bitvec` Prelude
//!
//! This collects the general
//! public API into a single
//! spot for inclusion, as
//! `use bitvec::prelude::*;`,
//! without polluting the root
//! namespace of the crate. !



#[cfg(feature = "alloc")]
pub use crate::{boxed::BitBox,
                vec::BitVec};
pub use crate::{mem::BitMemory,
                order::{BitOrder,
                        Local,
                        Lsb0,
                        Msb0},
                slice::BitSlice,
                store::BitStore};
