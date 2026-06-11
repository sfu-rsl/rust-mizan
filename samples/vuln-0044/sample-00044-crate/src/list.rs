use std::marker::PhantomData;
use std::os::raw::{c_char, c_void};
use std::ptr;

#[link(name = "alpm")]
extern "C" {
    //fn alpm_list_free(list: *mut alpm_list_t);
    fn alpm_list_add(list: *mut alpm_list_t, data: *mut c_void) -> *mut alpm_list_t;
}

#[repr(C)]
pub struct alpm_list_t {
    pub(crate) data: *mut c_void,
    pub(crate) prev: *mut alpm_list_t,
    pub(crate) next: *mut alpm_list_t,
}
/// Trait to pass each item into a custom struct's constructor.
/// Workaround to avoid copying data and loosing the alpm_list_t data pointer
pub trait AlpmListItem<T> {
    fn new(data_ptr: *mut c_void) -> Self;
    fn to_ptr(&self) -> *mut c_void;
}

/// Generic almp list
pub struct AlpmList<T> {
    pub(crate) list: *mut alpm_list_t,
    phantom: PhantomData<T>,
}

impl<T: AlpmListItem<T>> AlpmList<T> {
    pub fn new(c_list: *mut alpm_list_t) -> AlpmList<T> {
        AlpmList {
            list: c_list,
            phantom: PhantomData,
        }
    }

    pub fn empty() -> AlpmList<T> {
        AlpmList {
            list: std::ptr::null_mut(),
            phantom: PhantomData,
        }
    }

    pub fn add(&mut self, item: T) {
        unsafe {
            self.list = alpm_list_add(self.list, item.to_ptr());
        }
    }

    pub fn to_ptr(&self) -> *mut alpm_list_t {
        self.list
    }

    pub fn iter(&self) -> AlpmListIterator<T> {
        AlpmListIterator {
            item: self.list,
            phantom: PhantomData,
        }
    }
}

impl<T: AlpmListItem<T>> IntoIterator for AlpmList<T> {
    type Item = T;
    type IntoIter = AlpmListIterator<T>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<T: AlpmListItem<T>> IntoIterator for &AlpmList<T> {
    type Item = T;
    type IntoIter = AlpmListIterator<T>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<T: AlpmListItem<T>> From<*mut alpm_list_t> for AlpmList<T> {
    fn from(c_list: *mut alpm_list_t) -> AlpmList<T> {
        AlpmList::new(c_list)
    }
}

pub struct AlpmListIterator<T> {
    item: *mut alpm_list_t,
    phantom: PhantomData<T>,
}

impl<T: AlpmListItem<T>> std::iter::Iterator for AlpmListIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        unsafe {
            if self.item == ptr::null_mut() {
                return None;
            }
            let data = T::new((*self.item).data);
            self.item = (*self.item).next;
            Some(data)
        }
    }
}

impl<T> Drop for AlpmList<T> {
    fn drop(&mut self) {
        /* unsafe{
            if self.list != std::ptr::null_mut(){
                causing undefined behaviours
                alpm_list_free(self.list);
            }
        }*/
    }
}
/*
 impl<T: AlpmListItem<T>> std::iter::Iterator for AlpmList<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        unsafe{
            if self.current == ptr::null_mut() {
                return None;
            }

            let data =  T::new((*self.current).data);
            self.current = (*self.current).next;
            Some(data)
        }
    }
 }
*/

pub struct StringItem {
    string_ptr: *const c_char,
}

impl StringItem {}

impl AlpmListItem<StringItem> for StringItem {
    fn new(data: *mut c_void) -> Self {
        StringItem {
            string_ptr: data as *const c_char,
        }
    }
    fn to_ptr(&self) -> *mut c_void {
        self.string_ptr as *mut c_void
    }
}

impl<'a, T: AsRef<str> + 'a> From<T> for StringItem {
    fn from(s: T) -> Self {
        StringItem {
            string_ptr: strc_noctx!(s.as_ref()),
        }
    }
}

pub struct StringList {
    alpm_list: AlpmList<StringItem>,
}

impl From<&[&str]> for StringList {
    fn from(strs: &[&str]) -> Self {
        let mut list = StringList::empty();
        for s in strs {
            list.add(s.into());
        }
        list
    }
}

impl From<*mut alpm_list_t> for StringList {
    fn from(lst: *mut alpm_list_t) -> Self {
        StringList::new(lst)
    }
}

impl StringList {
    pub fn empty() -> Self {
        StringList {
            alpm_list: AlpmList::empty(),
        }
    }

    pub fn new(list: *mut alpm_list_t) -> Self {
        StringList {
            alpm_list: AlpmList::new(list),
        }
    }

    pub fn add(&mut self, item: StringItem) {
        self.alpm_list.add(item);
    }

    pub fn to_ptr(&self) -> *mut alpm_list_t {
        self.alpm_list.to_ptr()
    }

    pub fn iter(&self) -> AlpmListIterator<StringItem> {
        self.alpm_list.iter()
    }
}

impl Drop for StringList {
    fn drop(&mut self) {
        unsafe {
            for s in self.alpm_list.iter() {
                let _ = str_fromraw!(s.string_ptr);
            }
        }
    }
}

pub struct AnyListItem {
    raw_ptr: *mut c_void,
}

impl AlpmListItem<AnyListItem> for AnyListItem {
    fn new(ptr: *mut c_void) -> Self {
        AnyListItem { raw_ptr: ptr }
    }

    fn to_ptr(&self) -> *mut c_void {
        self.raw_ptr
    }
}

pub type AnyList = AlpmList<AnyListItem>;

impl AnyList {
    pub fn into_list<T: From<*mut alpm_list_t>>(&self) -> T {
        self.to_ptr().into()
    }
}
