#![doc= " Permutation testing\n\nThis module contains tests to check boundary conditions on all permutations of\nthe six variables present in `BitField` functions:\n\n1. `Lsb0` and `Msb0` slice orderings\n2. `u8`, `u16`, `u32`, `u64` slice storage types\n3. `load` and `store` trait behaviors\n4. `_le` and `_be` element orderings\n5. `u8`, `u16`, `u32`, `u64` value transfer types\n6. Empty slice and too-wide slice conditions\n!"]
#![cfg(test)]

use super::*;
use crate::prelude::*;
