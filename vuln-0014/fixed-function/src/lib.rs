#![no_std]

extern crate alloc as alloc_crate;

// Re-export uefi and uefi_alloc
// pub use uefi;
// pub use uefi_alloc;

/// These two lines were added to make the crate compile
pub extern crate uefi;
pub extern crate uefi_alloc;

// Public modules
pub mod prelude;

use crate::prelude::*;

pub unsafe fn nstr(wstring: *const u16) -> String {
    let mut string = String::new();

    let mut i = 0;
    loop {
        let w = *wstring.offset(i);
        i += 1;
        if w == 0 {
            break;
        }
        let c = char::from_u32_unchecked(w as u32);
        string.push(c);
    }

    string
}
