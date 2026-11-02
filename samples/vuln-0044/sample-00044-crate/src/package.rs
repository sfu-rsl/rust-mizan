use std::os::raw::{c_char, c_void};

use crate::enums::PkgFrom;
use crate::list::{alpm_list_t, AlpmList,AlpmListItem};
use crate::db::{alpm_db_t, AlpmDB,DBList};

#[link(name="alpm")]
extern {
    fn alpm_pkg_find(pkg_list: *mut alpm_list_t, haystack: *const c_char) -> *mut alpm_pkg_t ;

    fn alpm_pkg_free(pkg: *mut alpm_pkg_t) -> i32;
    fn alpm_pkg_vercmp(a: *const c_char, b: *const c_char) -> i32;
    fn alpm_pkg_checkmd5sum(pkg: *mut alpm_pkg_t) -> i32;
    
    fn alpm_pkg_get_filename(pkg: *mut alpm_pkg_t) -> *const c_char;
    fn alpm_pkg_get_base(pkg: *mut alpm_pkg_t) -> *const c_char;
    fn alpm_pkg_get_base64_sig(pkg: *mut alpm_pkg_t) -> *const c_char;
    fn alpm_pkg_get_name(pkg: *mut alpm_pkg_t) -> *const c_char;
    fn alpm_pkg_get_version(pkg: *mut alpm_pkg_t) -> *const c_char;
    fn alpm_pkg_get_origin(pkg: *mut alpm_pkg_t) -> PkgFrom;
    fn alpm_pkg_get_desc(pkg: *mut alpm_pkg_t) -> *const c_char;
    fn alpm_pkg_get_url(pkg: *mut alpm_pkg_t) -> *const c_char;
    fn alpm_pkg_compute_requiredby(pkg: *mut alpm_pkg_t)-> *mut alpm_list_t;
    fn alpm_pkg_compute_optionalfor(pkg: *mut alpm_pkg_t)-> *mut alpm_list_t;
    fn alpm_pkg_get_validation(pkg: *mut alpm_pkg_t) -> i32;
    fn alpm_pkg_get_db(pkg: *mut alpm_pkg_t) -> *mut alpm_db_t;

    fn alpm_sync_newversion(pkg: *mut alpm_pkg_t, db_sync: *mut alpm_list_t) -> *mut alpm_pkg_t;
}

pub type PackageList = AlpmList<Package>;

#[repr(C)]
pub struct alpm_pkg_t{
    pub (crate) __unused: [u8;0],//get rid of warnings
}

impl alpm_pkg_t{
    pub fn none() -> alpm_pkg_t{
        alpm_pkg_t{
            __unused: [],
        }
    }
}

pub struct Package {
    pub (crate) pkg: *mut alpm_pkg_t,
}

impl AlpmListItem<Package> for Package{
    fn new(data_ptr: *mut c_void) -> Package{
        (data_ptr as *mut alpm_pkg_t).into()
    }
    fn to_ptr(&self) -> *mut c_void {
        self.pkg as *mut c_void
    }
}

impl PackageList {
    pub fn find(&self, haystack: &str) -> Package{
        unsafe{
            alpm_pkg_find(self.list, strc!(haystack)).into()
        }
    }
}

impl From<*mut alpm_pkg_t> for Package{
    fn from(c_pkg: *mut alpm_pkg_t) -> Package{
        Package{
            pkg: c_pkg,
        }
    }
}

impl Drop for Package {
    fn drop(&mut self) {
        self.free();
    }
}

impl Package{
    /// Compare two version strings and determine which one is 'newer'. 
    pub fn vercmp(a : &str, b :&str) -> i32 {
        unsafe{
            alpm_pkg_vercmp(
                strc!(a),
                strc!(b)
            )
        }
    }

    /// Returns the method used to validate a package during install.
    /// Flags:
    /// `
    /// alpm_rs::enum::ALPM_PKG_VALIDATION_UNKNOWN
    /// alpm_rs::enum::ALPM_PKG_VALIDATION_NONE 
    /// alpm_rs::enum::ALPM_PKG_VALIDATION_MD5SUM
    /// alpm_rs::enum::ALPM_PKG_VALIDATION_SHA256SUM
    /// alpm_rs::enum::ALPM_PKG_VALIDATION_SIGNATURE
    /// `
    pub fn validation(&self) -> i32{
        unsafe{
            alpm_pkg_get_validation(self.pkg)
        }
    }
    /// Gets the name of the file from which the package was loaded.
    pub fn filename(&self) -> &str {
        unsafe{
            cstr!(alpm_pkg_get_filename(self.pkg))
        }
    }

    /// Returns the package base name.
    pub fn base(&self) -> &str {
        unsafe{
            cstr!(alpm_pkg_get_base(self.pkg))
        }
    }

    /// Returns the package name.
    pub fn base64_sig(&self) -> &str {
        unsafe{
            cstr!(alpm_pkg_get_base64_sig(self.pkg))
        }
    }

    /// Returns the package name.
    pub fn name(&self) -> &str {
        unsafe{
            cstr!(alpm_pkg_get_name(self.pkg))
        }
    }

    /// Returns the package version as a string.
    /// This includes all available epoch, version, and pkgrel components. 
    /// Use vercmp() to compare version strings if necessary.
    pub fn version(&self) -> &str {
        unsafe{
            cstr!(alpm_pkg_get_version(self.pkg)) 
        }
    }

    /// Returns the origin of the package.
    pub fn origin(&self) -> PkgFrom {
        unsafe{
            alpm_pkg_get_origin(self.pkg)
        }
    }

    /// Returns the package description.
    pub fn description(&self) -> &str {
        unsafe{
            cstr!(alpm_pkg_get_desc(self.pkg)) 
        }
    }

    // Returns the source database
    pub fn db(&self) -> AlpmDB{
        unsafe{
            alpm_pkg_get_db(self.pkg).into()
        }
    }

    /// Returns the package URL.
    pub fn url(&self) -> &str {
        unsafe{
            cstr!(alpm_pkg_get_url(self.pkg))
        }
    }

    /// Computes the list of packages requiring a given package.
    pub fn required_by(&self) -> PackageList {
        unsafe{
            alpm_pkg_compute_requiredby(self.pkg).into()
        }
    } 

    /// Computes the list of packages optionally requiring a given package.
    pub fn optional_for(&self) -> PackageList {
        unsafe{
            alpm_pkg_compute_optionalfor(self.pkg).into()
        }
    }    

    /// Check the integrity (with md5) of a package from the sync cache.
    pub fn checkmd5sum(&self) -> bool {
        unsafe{
            to_bool!(alpm_pkg_checkmd5sum(self.pkg))
        }
    }

    pub fn newer_version(&self, dbs: &DBList) -> Option<Package> {
        
        unsafe {
            let p : Package = alpm_sync_newversion(self.pkg, dbs.list).into();
            if p.pkg == std::ptr::null_mut(){
                None
            }else{
                println!(">not null");
                Some(p)
            }
        }
    }

    /// Free package
    pub fn free(&self) -> bool {
        unsafe{
            if self.pkg == std::ptr::null_mut() {
                true
            }else{            
                to_bool!(alpm_pkg_free(self.pkg))
            }   
        }
    }    
}
