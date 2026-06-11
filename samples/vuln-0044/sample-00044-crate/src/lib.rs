#[macro_use]
pub mod macros;
pub mod callbacks;
pub mod clib;
pub mod db;
pub mod dependency;
pub mod enums;
pub mod error;
pub mod list;
pub mod package;
pub mod question;

pub use crate::list::AlpmListItem;

extern crate libc;

use std::error::Error;
use std::os::raw::c_char;

use crate::db::{alpm_db_t, AlpmDB, DBList};
use crate::dependency::DepMissingList;
use crate::enums::ErrorNo;
use crate::list::{alpm_list_t, AlpmList, AnyList};
use crate::package::{alpm_pkg_t, Package};

#[link(name = "alpm")]
extern "C" {
    fn alpm_initialize(
        root: *const c_char,
        dbpath: *const c_char,
        error: *mut ErrorNo,
    ) -> *mut alpm_handle_t;
    fn alpm_release(handle: *mut alpm_handle_t) -> i32;
    fn alpm_errno(handle: *mut alpm_handle_t) -> ErrorNo;
    fn alpm_pkg_should_ignore(handle: *mut alpm_handle_t, pkg_handle: *mut alpm_pkg_t) -> i32;
    fn alpm_pkg_load(
        handle: *mut alpm_handle_t,
        filename: *const c_char,
        full: i32,
        level: i32,
        pkg: *mut alpm_pkg_t,
    ) -> i32;

    fn alpm_get_localdb(handle: *mut alpm_handle_t) -> *mut alpm_db_t;
    fn alpm_get_syncdbs(handle: *mut alpm_handle_t) -> *mut alpm_list_t;

    fn alpm_unregister_all_syncdbs(handle: *mut alpm_handle_t) -> i32;

    fn alpm_register_syncdb(
        handle: *mut alpm_handle_t,
        treename: *const c_char,
        level: i32,
    ) -> *mut alpm_db_t;
    fn alpm_sync_sysupgrade(handle: *mut alpm_handle_t, enable_downgrade: i32) -> i32;

    fn alpm_trans_init(handle: *mut alpm_handle_t, flags: i32) -> i32;
    fn alpm_trans_get_add(handle: *mut alpm_handle_t) -> *mut alpm_list_t;
    fn alpm_trans_commit(handle: *mut alpm_handle_t, list: *mut *mut alpm_list_t) -> i32;
    fn alpm_trans_release(handle: *mut alpm_handle_t) -> i32;
    fn alpm_trans_prepare(handle: *mut alpm_handle_t, data: *mut *mut alpm_list_t) -> i32;

    fn alpm_add_pkg(handle: *mut alpm_handle_t, pkg: *mut alpm_pkg_t) -> i32;
    fn alpm_remove_pkg(handle: *mut alpm_handle_t, pkg: *mut alpm_pkg_t) -> i32;
}

#[repr(C)]
struct alpm_handle_t {
    __unused: [u8; 0], //get rid of warnings
}

/// Alpm handle
pub struct Handle {
    alpm_handle: *mut alpm_handle_t,
}

/// Initializes the alpm library and retuns a handle.
/// ///
/// /// # Arguments
/// ///
/// /// * `root` - the root path for all filesystem operations
/// /// * `dbpath` - the absolute path to the libalpm database
/// ///
/// /// # Example
/// ///
/// /// ```
/// /// use alpm;
/// /// let handle = alpm::initialize("/", "/var/lib/pacman").unwrap();
/// /// ```
pub fn initialize(root: &str, dbpath: &str) -> Result<Handle, Box<dyn Error>> {
    let mut err = enums::ErrorNo::ALPM_ERR_OK;
    let handle: *mut alpm_handle_t;

    unsafe {
        handle = alpm_initialize(strc!(root), strc!(dbpath), &mut err as *mut enums::ErrorNo);
    }

    if err as i32 != 0 {
        Err(error::AlpmError::new(err).into())
    } else {
        Ok(Handle::new(handle))
    }
}

impl Handle {
    fn new(handle: *mut alpm_handle_t) -> Self {
        Handle {
            alpm_handle: handle,
        }
    }

    /// Get the database of locally installed packages.
    pub fn local_db(&self) -> AlpmDB {
        unsafe { alpm_get_localdb(self.alpm_handle).into() }
    }

    /// Get the list of sync databases.
    pub fn sync_dbs(&self) -> DBList {
        unsafe { alpm_get_syncdbs(self.alpm_handle).into() }
    }

    ///  Register a sync database of packages.
    pub fn register_syncdb(&self, tree_name: &str, level: i32) -> AlpmDB {
        unsafe { alpm_register_syncdb(self.alpm_handle, strc!(tree_name), level).into() }
    }

    /// Unregister all package databases.
    pub fn unregister_all_syncdbs(&self) -> bool {
        unsafe { to_bool!(alpm_unregister_all_syncdbs(self.alpm_handle)) }
    }

    /// Release handle
    pub fn release(&self) -> bool {
        unsafe { to_bool!(alpm_release(self.alpm_handle)) }
    }

    /// Returns last error code
    pub fn error_no(&self) -> ErrorNo {
        unsafe { alpm_errno(self.alpm_handle) }
    }

    /// Test if a package should be ignored.
    /// Checks if the package is ignored via IgnorePkg, or if the package is
    /// in a group ignored via IgnoreGroup.
    pub fn should_ignore(&self, pkg: &Package) -> bool {
        unsafe { to_bool!(alpm_pkg_should_ignore(self.alpm_handle, pkg.pkg)) }
    }

    /// Create a package from a file.
    /// If full is false, the archive is read only until all necessary
    /// metadata is found. If it is true, the entire archive is read, which
    /// serves as a verification of integrity and the filelist can be created.
    ///  /// /// # Arguments
    /// ///
    /// /// * `filename` - location of the package tarball
    /// /// * `full` - whether to stop the load after metadata is read or continue through the full archive
    /// /// * `level` - what level of package signature checking to perform on the package; note that this must be a '.sig' file type verification
    /// ///
    pub fn load_package(&self, filename: &str, full: bool, level: i32) -> Option<Package> {
        let mut lm_pkg = Box::new(alpm_pkg_t::none());
        let pkg_ptr = &mut *lm_pkg as *mut alpm_pkg_t;

        let err = unsafe {
            alpm_pkg_load(
                self.alpm_handle,
                strc!(filename),
                full.into(),
                level,
                pkg_ptr, // as alpm_pkg_t,
            )
        };
        if err != -1 {
            Some(pkg_ptr.into())
        } else {
            None
        }
    }

    /// Search for packages to upgrade and add them to the transaction.
    pub fn sys_upgrade(&self, enable_downgrade: bool) -> bool {
        let en_dg = if enable_downgrade { 1 } else { 0 };
        unsafe { to_bool!(alpm_sync_sysupgrade(self.alpm_handle, en_dg)) }
    }

    pub fn trans_init(&self, flags: i32) -> bool {
        unsafe { to_bool!(alpm_trans_init(self.alpm_handle, flags)) }
    }

    /// Add a package to the transaction.
    /// If the package was loaded by alpm_pkg_load(), it will be freed upon
    /// alpm_trans_release() invocation.

    pub fn add_pkg(&self, pkg: &Package) -> bool {
        unsafe { to_bool!(alpm_add_pkg(self.alpm_handle, pkg.pkg)) }
    }

    /// Add a package removal action to the transaction.
    pub fn remove_pkg(&self, pkg: &Package) -> bool {
        unsafe { to_bool!(alpm_remove_pkg(self.alpm_handle, pkg.pkg)) }
    }

    pub fn trans_get_add(&self) -> package::PackageList {
        unsafe { alpm_trans_get_add(self.alpm_handle).into() }
    }

    pub fn trans_release(&self) -> bool {
        unsafe { to_bool!(alpm_trans_release(self.alpm_handle)) }
    }

    pub fn trans_prepare(&self, deps: *mut DepMissingList) -> bool {
        unsafe {
            let mut list = AlpmList::empty();
            if alpm_trans_prepare(self.alpm_handle, &mut list.list as *mut *mut alpm_list_t) < 0 {
                false
            } else {
                *deps = list;
                true
            }
        }
    }

    pub fn trans_commit(&self, list: *mut AnyList) -> bool {
        unsafe {
            let mut t_list = AlpmList::empty();
            if alpm_trans_commit(self.alpm_handle, &mut t_list.list as *mut *mut alpm_list_t) < 0 {
                false
            } else {
                *list = t_list;
                true
            }
        }
    }
}
