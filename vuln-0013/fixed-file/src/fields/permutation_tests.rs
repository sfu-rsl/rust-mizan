#![doc= " Permutation testing\n\nThis module contains tests to check boundary conditions on all permutations of\nthe six variables present in `BitField` functions:\n\n1. `Lsb0` and `Msb0` slice orderings\n2. `u8`, `u16`, `u32`, `u64` slice storage types\n3. `load` and `store` trait behaviors\n4. `_le` and `_be` element orderings\n5. `u8`, `u16`, `u32`, `u64` value transfer types\n6. Empty slice and too-wide slice conditions\n!"]
#![cfg(test)]

use super::*;
use crate::prelude::*;

#[test]
fn check_mask() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
fn check_resize() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsl08_ll08_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsl08_ll16_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsl08_ll32_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[cfg(target_pointer_width = "64")]
#[test]
#[should_panic]
fn bsl08_ll64_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsl08_lb08_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsl08_lb16_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsl08_lb32_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[cfg(target_pointer_width = "64")]
#[test]
#[should_panic]
fn bsl08_lb64_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsl16_ll08_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsl16_ll16_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsl16_ll32_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[cfg(target_pointer_width = "64")]
#[test]
#[should_panic]
fn bsl16_ll64_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsl16_lb08_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsl16_lb16_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsl16_lb32_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[cfg(target_pointer_width = "64")]
#[test]
#[should_panic]
fn bsl16_lb64_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsl32_ll08_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsl32_ll16_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsl32_ll32_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[cfg(target_pointer_width = "64")]
#[test]
#[should_panic]
fn bsl32_ll64_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsl32_lb08_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsl32_lb16_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsl32_lb32_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[cfg(target_pointer_width = "64")]
#[test]
#[should_panic]
fn bsl32_lb64_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[cfg(target_pointer_width = "64")]
#[test]
#[should_panic]
fn bsl64_ll08_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[cfg(target_pointer_width = "64")]
#[test]
#[should_panic]
fn bsl64_ll16_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[cfg(target_pointer_width = "64")]
#[test]
#[should_panic]
fn bsl64_ll32_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[cfg(target_pointer_width = "64")]
#[test]
#[should_panic]
fn bsl64_ll64_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[cfg(target_pointer_width = "64")]
#[test]
#[should_panic]
fn bsl64_lb08_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[cfg(target_pointer_width = "64")]
#[test]
#[should_panic]
fn bsl64_lb16_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[cfg(target_pointer_width = "64")]
#[test]
#[should_panic]
fn bsl64_lb32_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[cfg(target_pointer_width = "64")]
#[test]
#[should_panic]
fn bsl64_lb64_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsm08_ll08_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsm08_ll16_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsm08_ll32_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[cfg(target_pointer_width = "64")]
#[test]
#[should_panic]
fn bsm08_ll64_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsm08_lb08_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsm08_lb16_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsm08_lb32_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[cfg(target_pointer_width = "64")]
#[test]
#[should_panic]
fn bsm08_lb64_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsm16_ll08_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsm16_ll16_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsm16_ll32_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[cfg(target_pointer_width = "64")]
#[test]
#[should_panic]
fn bsm16_ll64_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsm16_lb08_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsm16_lb16_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsm16_lb32_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[cfg(target_pointer_width = "64")]
#[test]
#[should_panic]
fn bsm16_lb64_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsm32_ll08_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsm32_ll16_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsm32_ll32_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[cfg(target_pointer_width = "64")]
#[test]
#[should_panic]
fn bsm32_ll64_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsm32_lb08_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsm32_lb16_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsm32_lb32_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[cfg(target_pointer_width = "64")]
#[test]
#[should_panic]
fn bsm32_lb64_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[cfg(target_pointer_width = "64")]
#[test]
#[should_panic]
fn bsm64_ll08_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[cfg(target_pointer_width = "64")]
#[test]
#[should_panic]
fn bsm64_ll16_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[cfg(target_pointer_width = "64")]
#[test]
#[should_panic]
fn bsm64_ll32_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[cfg(target_pointer_width = "64")]
#[test]
#[should_panic]
fn bsm64_ll64_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[cfg(target_pointer_width = "64")]
#[test]
#[should_panic]
fn bsm64_lb08_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[cfg(target_pointer_width = "64")]
#[test]
#[should_panic]
fn bsm64_lb16_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[cfg(target_pointer_width = "64")]
#[test]
#[should_panic]
fn bsm64_lb32_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[cfg(target_pointer_width = "64")]
#[test]
#[should_panic]
fn bsm64_lb64_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsl08_ll08_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsl08_ll16_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsl08_ll32_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[cfg(target_pointer_width = "64")]
#[test]
#[should_panic]
fn bsl08_ll64_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsl08_lb08_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsl08_lb16_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsl08_lb32_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[cfg(target_pointer_width = "64")]
#[test]
#[should_panic]
fn bsl08_lb64_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsl16_ll08_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsl16_ll16_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsl16_ll32_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[cfg(target_pointer_width = "64")]
#[test]
#[should_panic]
fn bsl16_ll64_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsl16_lb08_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsl16_lb16_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsl16_lb32_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[cfg(target_pointer_width = "64")]
#[test]
#[should_panic]
fn bsl16_lb64_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsl32_ll08_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsl32_ll16_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsl32_ll32_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[cfg(target_pointer_width = "64")]
#[test]
#[should_panic]
fn bsl32_ll64_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsl32_lb08_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsl32_lb16_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsl32_lb32_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[cfg(target_pointer_width = "64")]
#[test]
#[should_panic]
fn bsl32_lb64_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[cfg(target_pointer_width = "64")]
#[test]
#[should_panic]
fn bsl64_ll08_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[cfg(target_pointer_width = "64")]
#[test]
#[should_panic]
fn bsl64_ll16_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[cfg(target_pointer_width = "64")]
#[test]
#[should_panic]
fn bsl64_ll32_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[cfg(target_pointer_width = "64")]
#[test]
#[should_panic]
fn bsl64_ll64_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[cfg(target_pointer_width = "64")]
#[test]
#[should_panic]
fn bsl64_lb08_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[cfg(target_pointer_width = "64")]
#[test]
#[should_panic]
fn bsl64_lb16_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[cfg(target_pointer_width = "64")]
#[test]
#[should_panic]
fn bsl64_lb32_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[cfg(target_pointer_width = "64")]
#[test]
#[should_panic]
fn bsl64_lb64_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsm08_ll08_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsm08_ll16_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsm08_ll32_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[cfg(target_pointer_width = "64")]
#[test]
#[should_panic]
fn bsm08_ll64_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsm08_lb08_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsm08_lb16_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsm08_lb32_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[cfg(target_pointer_width = "64")]
#[test]
#[should_panic]
fn bsm08_lb64_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsm16_ll08_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsm16_ll16_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsm16_ll32_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[cfg(target_pointer_width = "64")]
#[test]
#[should_panic]
fn bsm16_ll64_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsm16_lb08_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsm16_lb16_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsm16_lb32_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[cfg(target_pointer_width = "64")]
#[test]
#[should_panic]
fn bsm16_lb64_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsm32_ll08_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsm32_ll16_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsm32_ll32_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[cfg(target_pointer_width = "64")]
#[test]
#[should_panic]
fn bsm32_ll64_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsm32_lb08_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsm32_lb16_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsm32_lb32_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[cfg(target_pointer_width = "64")]
#[test]
#[should_panic]
fn bsm32_lb64_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[cfg(target_pointer_width = "64")]
#[test]
#[should_panic]
fn bsm64_ll08_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[cfg(target_pointer_width = "64")]
#[test]
#[should_panic]
fn bsm64_ll16_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[cfg(target_pointer_width = "64")]
#[test]
#[should_panic]
fn bsm64_ll32_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[cfg(target_pointer_width = "64")]
#[test]
#[should_panic]
fn bsm64_ll64_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[cfg(target_pointer_width = "64")]
#[test]
#[should_panic]
fn bsm64_lb08_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[cfg(target_pointer_width = "64")]
#[test]
#[should_panic]
fn bsm64_lb16_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[cfg(target_pointer_width = "64")]
#[test]
#[should_panic]
fn bsm64_lb32_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[cfg(target_pointer_width = "64")]
#[test]
#[should_panic]
fn bsm64_lb64_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsl08_sl08_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsl08_sl16_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsl08_sl32_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[cfg(target_pointer_width = "64")]
#[test]
#[should_panic]
fn bsl08_sl64_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsl08_sb08_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsl08_sb16_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsl08_sb32_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[cfg(target_pointer_width = "64")]
#[test]
#[should_panic]
fn bsl08_sb64_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsl16_sl08_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsl16_sl16_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsl16_sl32_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[cfg(target_pointer_width = "64")]
#[test]
#[should_panic]
fn bsl16_sl64_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsl16_sb08_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsl16_sb16_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsl16_sb32_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[cfg(target_pointer_width = "64")]
#[test]
#[should_panic]
fn bsl16_sb64_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsl32_sl08_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsl32_sl16_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsl32_sl32_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[cfg(target_pointer_width = "64")]
#[test]
#[should_panic]
fn bsl32_sl64_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsl32_sb08_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsl32_sb16_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsl32_sb32_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[cfg(target_pointer_width = "64")]
#[test]
#[should_panic]
fn bsl32_sb64_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[cfg(target_pointer_width = "64")]
#[test]
#[should_panic]
fn bsl64_sl08_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[cfg(target_pointer_width = "64")]
#[test]
#[should_panic]
fn bsl64_sl16_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[cfg(target_pointer_width = "64")]
#[test]
#[should_panic]
fn bsl64_sl32_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[cfg(target_pointer_width = "64")]
#[test]
#[should_panic]
fn bsl64_sl64_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[cfg(target_pointer_width = "64")]
#[test]
#[should_panic]
fn bsl64_sb08_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[cfg(target_pointer_width = "64")]
#[test]
#[should_panic]
fn bsl64_sb16_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[cfg(target_pointer_width = "64")]
#[test]
#[should_panic]
fn bsl64_sb32_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[cfg(target_pointer_width = "64")]
#[test]
#[should_panic]
fn bsl64_sb64_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsm08_sl08_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsm08_sl16_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsm08_sl32_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[cfg(target_pointer_width = "64")]
#[test]
#[should_panic]
fn bsm08_sl64_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsm08_sb08_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsm08_sb16_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsm08_sb32_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[cfg(target_pointer_width = "64")]
#[test]
#[should_panic]
fn bsm08_sb64_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsm16_sl08_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsm16_sl16_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsm16_sl32_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[cfg(target_pointer_width = "64")]
#[test]
#[should_panic]
fn bsm16_sl64_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsm16_sb08_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsm16_sb16_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsm16_sb32_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[cfg(target_pointer_width = "64")]
#[test]
#[should_panic]
fn bsm16_sb64_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsm32_sl08_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsm32_sl16_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsm32_sl32_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[cfg(target_pointer_width = "64")]
#[test]
#[should_panic]
fn bsm32_sl64_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsm32_sb08_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsm32_sb16_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsm32_sb32_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[cfg(target_pointer_width = "64")]
#[test]
#[should_panic]
fn bsm32_sb64_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[cfg(target_pointer_width = "64")]
#[test]
#[should_panic]
fn bsm64_sl08_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[cfg(target_pointer_width = "64")]
#[test]
#[should_panic]
fn bsm64_sl16_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[cfg(target_pointer_width = "64")]
#[test]
#[should_panic]
fn bsm64_sl32_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[cfg(target_pointer_width = "64")]
#[test]
#[should_panic]
fn bsm64_sl64_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[cfg(target_pointer_width = "64")]
#[test]
#[should_panic]
fn bsm64_sb08_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[cfg(target_pointer_width = "64")]
#[test]
#[should_panic]
fn bsm64_sb16_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[cfg(target_pointer_width = "64")]
#[test]
#[should_panic]
fn bsm64_sb32_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[cfg(target_pointer_width = "64")]
#[test]
#[should_panic]
fn bsm64_sb64_empty() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsl08_sl08_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsl08_sl16_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsl08_sl32_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[cfg(target_pointer_width = "64")]
#[test]
#[should_panic]
fn bsl08_sl64_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsl08_sb08_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsl08_sb16_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsl08_sb32_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[cfg(target_pointer_width = "64")]
#[test]
#[should_panic]
fn bsl08_sb64_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsl16_sl08_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsl16_sl16_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsl16_sl32_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[cfg(target_pointer_width = "64")]
#[test]
#[should_panic]
fn bsl16_sl64_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsl16_sb08_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsl16_sb16_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsl16_sb32_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[cfg(target_pointer_width = "64")]
#[test]
#[should_panic]
fn bsl16_sb64_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsl32_sl08_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsl32_sl16_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsl32_sl32_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[cfg(target_pointer_width = "64")]
#[test]
#[should_panic]
fn bsl32_sl64_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsl32_sb08_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsl32_sb16_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsl32_sb32_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[cfg(target_pointer_width = "64")]
#[test]
#[should_panic]
fn bsl32_sb64_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[cfg(target_pointer_width = "64")]
#[test]
#[should_panic]
fn bsl64_sl08_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[cfg(target_pointer_width = "64")]
#[test]
#[should_panic]
fn bsl64_sl16_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[cfg(target_pointer_width = "64")]
#[test]
#[should_panic]
fn bsl64_sl32_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[cfg(target_pointer_width = "64")]
#[test]
#[should_panic]
fn bsl64_sl64_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[cfg(target_pointer_width = "64")]
#[test]
#[should_panic]
fn bsl64_sb08_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[cfg(target_pointer_width = "64")]
#[test]
#[should_panic]
fn bsl64_sb16_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[cfg(target_pointer_width = "64")]
#[test]
#[should_panic]
fn bsl64_sb32_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[cfg(target_pointer_width = "64")]
#[test]
#[should_panic]
fn bsl64_sb64_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsm08_sl08_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsm08_sl16_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsm08_sl32_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[cfg(target_pointer_width = "64")]
#[test]
#[should_panic]
fn bsm08_sl64_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsm08_sb08_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsm08_sb16_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsm08_sb32_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[cfg(target_pointer_width = "64")]
#[test]
#[should_panic]
fn bsm08_sb64_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsm16_sl08_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsm16_sl16_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsm16_sl32_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[cfg(target_pointer_width = "64")]
#[test]
#[should_panic]
fn bsm16_sl64_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsm16_sb08_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsm16_sb16_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsm16_sb32_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[cfg(target_pointer_width = "64")]
#[test]
#[should_panic]
fn bsm16_sb64_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsm32_sl08_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsm32_sl16_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsm32_sl32_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[cfg(target_pointer_width = "64")]
#[test]
#[should_panic]
fn bsm32_sl64_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsm32_sb08_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsm32_sb16_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[test]
#[should_panic]
fn bsm32_sb32_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[cfg(target_pointer_width = "64")]
#[test]
#[should_panic]
fn bsm32_sb64_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[cfg(target_pointer_width = "64")]
#[test]
#[should_panic]
fn bsm64_sl08_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[cfg(target_pointer_width = "64")]
#[test]
#[should_panic]
fn bsm64_sl16_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[cfg(target_pointer_width = "64")]
#[test]
#[should_panic]
fn bsm64_sl32_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[cfg(target_pointer_width = "64")]
#[test]
#[should_panic]
fn bsm64_sl64_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[cfg(target_pointer_width = "64")]
#[test]
#[should_panic]
fn bsm64_sb08_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[cfg(target_pointer_width = "64")]
#[test]
#[should_panic]
fn bsm64_sb16_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[cfg(target_pointer_width = "64")]
#[test]
#[should_panic]
fn bsm64_sb32_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}

#[cfg(target_pointer_width = "64")]
#[test]
#[should_panic]
fn bsm64_sb64_full() {
    panic!("CARGO_MINIMIZE_PANIC_FAIL")
}
