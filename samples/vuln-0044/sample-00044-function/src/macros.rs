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

use std::ffi::CString;
use std::os::raw::c_char;
