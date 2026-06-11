


use std::os::raw::{c_char, c_void};

#[link(name="alpm")]
extern {
    fn vsnprintf(s: *mut c_char, num: usize,fmt: *mut c_char, args: VaList) -> i32;
}

#[repr(C)]
#[derive(Debug,Copy,Clone)]
pub struct VaListInner {
    gp_offset: u32,
    fp_offset: u32,
    overflow_arg_area: *const c_void,
    reg_save_area: *const c_void,
}

#[derive(Debug,Copy,Clone)]
#[repr(C)]
pub struct VaList{
    innter: *mut VaListInner,
}


pub unsafe fn vsn_printf(fmt: *mut c_char, args: VaList) -> String{
    let mut buf = vec![0;255];
    vsnprintf(buf.as_mut_ptr(), buf.len(), fmt, args) as usize;
    cstr!(buf.as_mut_ptr() as *mut c_char).to_string()
}