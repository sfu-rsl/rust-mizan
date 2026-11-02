/// string to c-string
/// required to be converted back to rust string after use
/// to avoid memory leaking
macro_rules! strc_noctx {
    ($e:expr) => {
        std::ffi::CString::new($e)
            .expect("CString::new failed")
            .into_raw()
    };
}

macro_rules! str_fromraw {
    ($e:expr) => {
        std::ffi::CString::from_raw($e as *mut std::os::raw::c_char)
    };
}

// createates a raw c-string
// and deallocates it in the deconstructor
pub struct StrcCtx {
    pub ptr: *mut c_char,
}

impl StrcCtx {
    pub fn new(s: &str) -> StrcCtx {
        StrcCtx {
            ptr: strc_noctx!(s),
        }
    }
}

impl Drop for StrcCtx {
    fn drop(&mut self) {
        unsafe {
            let _ = str_fromraw!(self.ptr);
        }
    }
}

/// string to c-string
/// using ontext
macro_rules! strc {
    ($e:expr) => {
        crate::macros::StrcCtx::new($e).ptr
    };
}

/// c-string to string
macro_rules! cstr {
    ($e:expr) => {
        if $e == std::ptr::null_mut() {
            ""
        } else {
            std::ffi::CStr::from_ptr($e).to_str().unwrap_or("")
        }
    };
}

macro_rules! cstring {
    ($e:expr) => {
        if $e == std::ptr::null_mut() {
            "".to_string()
        } else {
            unsafe { std::ffi::CStr::from_ptr($e).to_str().unwrap().to_string() }
        }
    };
}

macro_rules! to_bool {
    ($e:expr) => {
        if $e == -1 {
            false
        } else {
            true
        }
    };
}

use std::ffi::CString;
use std::os::raw::c_char;

// Converts a string into a raw pointer, calls the context then
// after execution, back into a rust string so it can be disposed
pub unsafe fn strc_context1(s: &str, context: fn(raw_str: *mut std::os::raw::c_char)) {
    let raw_str = strc_noctx!(s);
    context(raw_str);
    let _ = CString::from_raw(raw_str);
}

pub unsafe fn strc_context2(
    s1: &str,
    s2: &str,
    context: fn(raw_str1: *mut c_char, raw_str2: *mut c_char),
) {
    let raw_str1 = strc_noctx!(s1);
    let raw_str2 = strc_noctx!(s2);
    context(raw_str1, raw_str2);
    let _ = CString::from_raw(raw_str1);
    let _ = CString::from_raw(raw_str2);
}
