use std::os::raw::{c_char, c_void};

use crate::db::alpm_db_t;
use crate::dependency::{alpm_conflict_t, alpm_depend_t, Conflict, Depend};
use crate::enums;
use crate::list::alpm_list_t;
use crate::package::{alpm_pkg_t, Package, PackageList};

#[allow(non_camel_case_types)]
type alpm_time_t = i64;

#[repr(C)]
#[derive(Copy, Clone)]
struct alpm_pgpkey_t {
    data: *mut c_void,
    fingerprint: *const c_char,
    uid: *const c_char,
    name: *const c_char,
    email: *const c_char,
    created: alpm_time_t,
    expires: alpm_time_t,
    length: u32,
    revoked: u32,
    pubkey_algo: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct alpm_question_any_t {
    question_type: i32,
    answer: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct alpm_question_install_ignorepkg_t {
    question_type: i32,
    answer: i32,
    pkg: *mut alpm_pkg_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct alpm_question_replace_t {
    question_type: i32,
    answer: i32,
    oldpkg: *mut alpm_pkg_t,
    newpkg: *mut alpm_pkg_t,
    newdb: *mut alpm_db_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct alpm_question_conflict_t {
    question_type: i32,
    answer: i32,
    conflict: *mut alpm_conflict_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct alpm_question_corrupted_t {
    question_type: i32,
    answer: i32,
    filepath: *const c_char,
    reason: enums::ErrorNo,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct alpm_question_remove_pkgs_t {
    question_type: i32,
    answer: i32,
    packages: *mut alpm_list_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct alpm_question_select_provider_t {
    question_type: i32,
    answer: i32,
    providers: *mut alpm_list_t, //List of alpm_pkg_t
    dependant: *mut alpm_depend_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct alpm_question_import_key_t {
    question_type: i32,
    answer: i32,
    key: *mut alpm_pgpkey_t,
}

#[repr(C)]
pub union alpm_question_t {
    question_type: i32,
    any: alpm_question_any_t,
    install_ignorepkg: alpm_question_install_ignorepkg_t,
    replace: alpm_question_replace_t,
    conflict: alpm_question_conflict_t,
    corrupted: alpm_question_corrupted_t,
    remove_pkgs: alpm_question_remove_pkgs_t,
    select_provider: alpm_question_select_provider_t,
    import_key: alpm_question_import_key_t,
}

pub struct PgpKey {
    pub data: *mut c_void,
    pub fingerprint: String,
    pub uid: String,
    pub name: String,
    pub email: String,
    pub created: i64,
    pub expires: i64,
    pub length: u32,
    pub revoked: u32,
    pub pubkey_algo: u8,
}

impl From<*mut alpm_pgpkey_t> for PgpKey {
    fn from(pgpk: *mut alpm_pgpkey_t) -> Self {
        let k = unsafe { *pgpk };
        PgpKey {
            data: k.data,
            fingerprint: cstring!(k.fingerprint),
            uid: cstring!(k.uid),
            name: cstring!(k.name),
            email: cstring!(k.email),
            created: k.created,
            expires: k.expires,
            length: k.length,
            revoked: k.revoked,
            pubkey_algo: k.pubkey_algo,
        }
    }
}

pub struct QuestionAny {
    pub question_type: i32,
    pub answer: i32,
}

impl From<*mut alpm_question_t> for QuestionAny {
    fn from(q_raw: *mut alpm_question_t) -> Self {
        let q = unsafe { *(q_raw as *mut alpm_question_any_t) };
        QuestionAny {
            question_type: q.question_type,
            answer: q.answer,
        }
    }
}

pub struct QuestionInstallIgnorePkg {
    pub question_type: i32,
    pub answer: i32,
    pub pkg: Package,
}

impl From<*mut alpm_question_t> for QuestionInstallIgnorePkg {
    fn from(q_raw: *mut alpm_question_t) -> Self {
        let q = unsafe { *(q_raw as *mut alpm_question_install_ignorepkg_t) };
        QuestionInstallIgnorePkg {
            question_type: q.question_type,
            answer: q.answer,
            pkg: q.pkg.into(),
        }
    }
}

pub struct QuestionReplacePkg {
    pub question_type: i32,
    pub answer: i32,
    pub conflict: Conflict,
}

impl From<*mut alpm_question_t> for QuestionReplacePkg {
    fn from(q_raw: *mut alpm_question_t) -> Self {
        let q = unsafe { *(q_raw as *mut alpm_question_conflict_t) };
        QuestionReplacePkg {
            question_type: q.question_type,
            answer: q.answer,
            conflict: q.conflict.into(),
        }
    }
}

pub struct QuestionConflict {
    pub question_type: i32,
    pub answer: i32,
    pub conflict: Conflict,
}

impl From<*mut alpm_question_t> for QuestionConflict {
    fn from(q_raw: *mut alpm_question_t) -> Self {
        let q = unsafe { *(q_raw as *mut alpm_question_conflict_t) };
        QuestionConflict {
            question_type: q.question_type,
            answer: q.answer,
            conflict: q.conflict.into(),
        }
    }
}

pub struct QuestionCorrupted {
    pub question_type: i32,
    pub answer: i32,
    pub filepath: String,
    pub reason: enums::ErrorNo,
}

impl From<*mut alpm_question_t> for QuestionCorrupted {
    fn from(q_raw: *mut alpm_question_t) -> Self {
        let q = unsafe { *(q_raw as *mut alpm_question_corrupted_t) };
        QuestionCorrupted {
            question_type: q.question_type,
            answer: q.answer,
            filepath: cstring!(q.filepath),
            reason: q.reason,
        }
    }
}

pub struct QuestionRemovePackages {
    pub question_type: i32,
    pub answer: i32,
    pub packages: PackageList,
}

impl From<*mut alpm_question_t> for QuestionRemovePackages {
    fn from(q_raw: *mut alpm_question_t) -> Self {
        let q = unsafe { *(q_raw as *mut alpm_question_remove_pkgs_t) };
        QuestionRemovePackages {
            question_type: q.question_type,
            answer: q.answer,
            packages: q.packages.into(),
        }
    }
}

pub struct QuestionSelectProvider {
    pub question_type: i32,
    pub answer: i32,
    pub providers: PackageList,
    pub dependant: Depend,
}

impl From<*mut alpm_question_t> for QuestionSelectProvider {
    fn from(q_raw: *mut alpm_question_t) -> Self {
        let q = unsafe { *(q_raw as *mut alpm_question_select_provider_t) };
        QuestionSelectProvider {
            question_type: q.question_type,
            answer: q.answer,
            providers: q.providers.into(),
            dependant: q.dependant.into(),
        }
    }
}

pub struct QuestionImportKey {
    pub question_type: i32,
    pub answer: i32,
    pub key: PgpKey,
}

impl From<*mut alpm_question_t> for QuestionImportKey {
    fn from(q_raw: *mut alpm_question_t) -> Self {
        let q = unsafe { *(q_raw as *mut alpm_question_import_key_t) };
        QuestionImportKey {
            question_type: q.question_type,
            answer: q.answer,
            key: q.key.into(),
        }
    }
}

pub enum Question {
    Any(QuestionAny),
    InstallIgnorePkg(QuestionInstallIgnorePkg),
    Replace(QuestionReplacePkg),
    Conflict(QuestionConflict),
    Corrupted(QuestionCorrupted),
    RemovePkgs(QuestionRemovePackages),
    SelectProvider(QuestionSelectProvider),
    ImportKey(QuestionImportKey),
}

pub struct QuestionArgs {
    question_raw: *mut alpm_question_any_t,
    pub question: Question,
}

impl QuestionArgs {
    pub fn set_answer(&self, ans: i32) {
        unsafe {
            (*self.question_raw).answer = ans;
        }
    }

    pub fn answer(&self) -> i32 {
        unsafe { (*self.question_raw).answer }
    }

    pub fn to_any(&self) -> QuestionAny {
        (self.question_raw as *mut alpm_question_t).into()
    }
}

impl From<*mut alpm_question_t> for QuestionArgs {
    fn from(q: *mut alpm_question_t) -> Self {
        QuestionArgs {
            question_raw: q as *mut alpm_question_any_t,
            question: unsafe {
                match (*q).question_type {
                    enums::ALPM_QUESTION_INSTALL_IGNOREPKG => Question::InstallIgnorePkg(q.into()),
                    enums::ALPM_QUESTION_REPLACE_PKG => Question::Replace(q.into()),
                    enums::ALPM_QUESTION_CONFLICT_PKG => Question::Conflict(q.into()),
                    enums::ALPM_QUESTION_CORRUPTED_PKG => Question::Corrupted(q.into()),
                    enums::ALPM_QUESTION_REMOVE_PKGS => Question::RemovePkgs(q.into()),
                    enums::ALPM_QUESTION_SELECT_PROVIDER => Question::SelectProvider(q.into()),
                    enums::ALPM_QUESTION_IMPORT_KEY => Question::ImportKey(q.into()),
                    _ => Question::Any(q.into()),
                }
            },
        }
    }
}
