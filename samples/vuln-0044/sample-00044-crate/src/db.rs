extern crate libc;

use std::os::raw::{c_char,c_void};

use crate::package::{PackageList,alpm_pkg_t,Package};
use crate::list::{alpm_list_t, AlpmList, AlpmListItem, StringList };

//alpm_list_t *alpm_db_get_servers(const alpm_db_t *db);
//int alpm_db_set_servers(alpm_db_t *db, alpm_list_t *servers);
//int alpm_db_add_server(alpm_db_t *db, const char *url);
//int alpm_db_remove_server(alpm_db_t *db, const char *url);

#[link(name="alpm")]
extern {
    fn alpm_db_get_pkgcache(db: *mut alpm_db_t)-> *mut alpm_list_t;
    fn alpm_db_unregister(db: *mut alpm_db_t) -> i32;
    fn alpm_db_get_name(db: *mut alpm_db_t) -> *const c_char;
    fn alpm_db_search(db: *mut alpm_db_t, needles: *mut alpm_list_t) -> *mut alpm_list_t;
    fn alpm_db_get_pkg(db: *mut alpm_db_t, name: *const c_char) -> *mut alpm_pkg_t;

    fn alpm_db_update(force: i32, db: *mut alpm_db_t) -> i32;
    fn alpm_db_get_servers(db: *mut alpm_db_t) -> *mut alpm_list_t;
    fn alpm_db_set_servers(db: *mut alpm_db_t, servers: *mut alpm_list_t) -> i32;
    fn alpm_db_add_server(db: *mut alpm_db_t,server: *mut c_char) -> i32;
    fn alpm_db_remove_server(db: *mut alpm_db_t, server: *mut c_char) -> i32;

}

#[repr(C)]
pub (crate) struct alpm_db_t{
    __unused: [u8;0],//get rid of warnings

}
pub type DBList = AlpmList<AlpmDB>;

impl AlpmListItem<AlpmDB> for AlpmDB{
    fn new(data_ptr: *mut c_void) -> AlpmDB{
        AlpmDB{
            db: data_ptr as *mut alpm_db_t,
        }
    }

    fn to_ptr(&self) -> *mut c_void {
        self.db as *mut c_void
    }
}

/// Alpm database
pub struct AlpmDB {
    pub (crate) db: *mut alpm_db_t,
}


impl From<*mut alpm_db_t> for AlpmDB {
    fn from(db :  *mut alpm_db_t) -> AlpmDB{
        AlpmDB{
            db: db,
        }
    }
}

impl AlpmDB{

    /// Get the name of the package database.
    pub fn name(&self) -> &str {
        unsafe{
            cstr!(alpm_db_get_name(self.db))
        }   
    }

    pub fn search<'a, T, S>(&self, needles: T) -> PackageList 
    where
        T: IntoIterator<Item=S>,
        S: AsRef<str> + 'a{
        unsafe{
            let mut needle_list = StringList::empty();
            for n in needles{
                needle_list.add(n.into());
            }
            alpm_db_search(self.db, needle_list.to_ptr()).into()
        }
    }

    pub fn search_one<'a, S: AsRef<str> + 'a>(&self, search: S) -> PackageList {
        self.search(&[search])
    }

    /// Get the package cache of the package database.
    pub fn pkgcache(&self) -> PackageList {
        unsafe{
            alpm_db_get_pkgcache(self.db).into()
        }
    }

    /// Unregister the package database.
    pub fn unregister(&self) -> bool{
        unsafe{
            to_bool!(alpm_db_unregister(self.db))
        }
    }

    pub fn get_pkg(&self, name: &str) -> Option<Package> {
        unsafe{
            let pkg = alpm_db_get_pkg(self.db, strc!(name));
            if pkg != std::ptr::null_mut() {
                Some(pkg.into())
            }else{
                None
            }
        }
    }

    /// Requires root
    pub fn update(&self, force: bool) -> i32{
        let force_i = if force {
            1
        }else{
            0
        };

        unsafe{
            alpm_db_update(force_i, self.db)
        }
    } 

    pub fn set_servers(&self, servers: &[&str]) -> bool{
        unsafe{
            let lst : StringList = servers.into();
            to_bool!(alpm_db_set_servers(self.db, lst.to_ptr()))
        }
    }

    pub fn get_servers(&self) -> StringList {
        unsafe{
            alpm_db_get_servers(self.db).into()
        }
    }

    pub fn add_server(&self, server: &str) -> bool {
        unsafe{
            to_bool!(alpm_db_add_server(self.db, strc!(server)))
        }
    }

    pub fn remove_server(&self, server: &str) -> bool {
        unsafe{
            to_bool!(alpm_db_remove_server(self.db, strc!(server)))
        }
    }
}

impl Drop for AlpmDB {
    fn drop(&mut self) {
    }
}

