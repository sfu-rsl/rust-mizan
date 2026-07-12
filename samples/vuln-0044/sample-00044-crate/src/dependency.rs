extern crate libc;

use std::os::raw::{c_char,c_void};

use crate::list::{AlpmList, AlpmListItem};
use crate::enums;

#[link(name="alpm")]
extern {

}

pub type DepMissingList = AlpmList<DepMissing>;

impl AlpmListItem<DepMissing> for DepMissing{
    fn new(ptr: *mut c_void) -> Self {
        (ptr as *mut alpm_depmissing_t).into()
    }

    fn to_ptr(&self) -> *mut c_void{
        self.ptr
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub (crate) struct alpm_depmissing_t{
    target: *mut c_char,
    depend: *mut alpm_depend_t,
    causingpkg: *mut c_char,
} 

pub struct DepMissing{
    ptr: *mut c_void,
    pub target: String,
    pub depend: Depend,
    pub causingpkg: String,
}

impl From<*mut alpm_depmissing_t> for DepMissing{
    fn from(depm: *mut alpm_depmissing_t) -> Self{
        let d = unsafe { *depm };
        DepMissing{
            ptr: depm as *mut c_void,
            target: cstring!(d.target),
            depend: d.depend.into(),
            causingpkg: cstring!(d.causingpkg),
        }
    }
}


#[repr(C)]
#[derive(Copy, Clone)]
pub (crate) struct alpm_conflict_t{
    package1_hash: u64,
    package2_hash: u64,
    package1: *const c_char,
    package2: *const c_char,
    reason: *mut alpm_depend_t,
}


pub struct Conflict {
    pub package1_hash: u64,
    pub package2_hash: u64,
    pub package1: String,
    pub package2: String,
    pub reason: Depend,
}

impl From<*mut alpm_conflict_t> for Conflict{
    fn from(cflt: *mut alpm_conflict_t) -> Self{
        let d= unsafe { *cflt };
        Conflict{
            package1_hash: d.package1_hash,
            package2_hash: d.package2_hash,
            package1: cstring!(d.package1),
            package2: cstring!(d.package2),
            reason: d.reason.into(),
        }
    }
}



#[repr(C)]
#[derive(Copy, Clone)]
pub (crate) struct alpm_depend_t{
    name: *const c_char,
    version: *const c_char,
    desc: *const c_char,
    name_hash: u64,
    dep_mod: enums::alpm_depmod,
}

pub struct Depend {
    pub name: String,
    pub version: String,
    pub desc: String,
    pub name_hash: u64,
    pub dep_mod: enums::alpm_depmod,
}


impl From<*mut alpm_depend_t> for Depend{
    fn from(dpnd: *mut alpm_depend_t) -> Self{
        let d = unsafe { *dpnd };
        Depend{
            name: cstring!(d.name),
            version: cstring!(d.name),
            desc: cstring!(d.name),
            name_hash: d.name_hash,
            dep_mod: d.dep_mod,
        }
    }
}