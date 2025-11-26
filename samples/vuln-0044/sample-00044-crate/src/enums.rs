#![allow(dead_code)]
#[allow(non_camel_case_types)]

pub const ALPM_LOG_ERROR: i32  = 1;
pub const ALPM_LOG_WARNING : i32 =  (1 << 1);
pub const ALPM_LOG_DEBUG :  i32 = (1 << 2);
pub const ALPM_LOG_FUNCTION : i32 = (1 << 3);

pub const ALPM_PKG_VALIDATION_UNKNOWN: i32  = 0;
pub const ALPM_PKG_VALIDATION_NONE : i32 =  (1 << 0);
pub const ALPM_PKG_VALIDATION_MD5SUM :  i32 = (1 << 1);
pub const ALPM_PKG_VALIDATION_SHA256SUM : i32 = (1 << 2);
pub const ALPM_PKG_VALIDATION_SIGNATURE : i32 = (1 << 3);

#[allow(non_camel_case_types)]
#[repr(C)]
pub enum PkgFrom {
	_invalid,
	ALPM_PKG_FROM_FILE,
	ALPM_PKG_FROM_LOCALDB,
	ALPM_PKG_FROM_SYNCDB
}


#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
#[repr(C)]
pub enum alpm_depmod {
	_invalid,
	ALPM_DEP_MOD_ANY,
	ALPM_DEP_MOD_EQ,
	ALPM_DEP_MOD_GE,
	ALPM_DEP_MOD_LE,
	ALPM_DEP_MOD_GT,
	ALPM_DEP_MOD_LT
}

pub const ALPM_QUESTION_INSTALL_IGNOREPKG : i32 = (1 << 0);
pub const ALPM_QUESTION_REPLACE_PKG : i32 = (1 << 1);
pub const ALPM_QUESTION_CONFLICT_PKG : i32 = (1 << 2);
pub const ALPM_QUESTION_CORRUPTED_PKG : i32 = (1 << 3);
pub const ALPM_QUESTION_REMOVE_PKGS : i32 = (1 << 4);
pub const ALPM_QUESTION_SELECT_PROVIDER : i32 = (1 << 5);
pub const ALPM_QUESTION_IMPORT_KEY : i32 = (1 << 6);

pub const ALPM_TRANS_FLAG_NODEPS : i32 = 1;
/** Ignore file conflicts and overwrite files. */
pub const ALPM_TRANS_FLAG_FORCE : i32 = (1 << 1);
/** Delete files even if they are tagged as backup. */
pub const ALPM_TRANS_FLAG_NOSAVE : i32 = (1 << 2);
/** Ignore version numbers when checking dependencies. */
pub const ALPM_TRANS_FLAG_NODEPVERSION : i32 = (1 << 3);
/** Remove also any packages depending on a package being removed. */
pub const ALPM_TRANS_FLAG_CASCADE : i32 = (1 << 4);
/** Remove packages and their unneeded deps (not explicitly installed). */
pub const ALPM_TRANS_FLAG_RECURSE : i32 = (1 << 5);
/** Modify database but do not commit changes to the filesystem. */
pub const ALPM_TRANS_FLAG_DBONLY : i32 = (1 << 6);
/* (1 << 7) flag can go here */
/** Use pub const ALPM_PKG_REASON_DEPEND when installing packages. */
pub const ALPM_TRANS_FLAG_ALLDEPS : i32 = (1 << 8);
/** Only download packages and do not actually install. */
pub const ALPM_TRANS_FLAG_DOWNLOADONLY : i32 = (1 << 9);
/** Do not execute install scriptlets after installing. */
pub const ALPM_TRANS_FLAG_NOSCRIPTLET : i32 = (1 << 10);
/** Ignore dependency conflicts. */
pub const ALPM_TRANS_FLAG_NOCONFLICTS : i32 = (1 << 11);
/* (1 << 12) flag can go here */
/** Do not install a package if it is already installed and up to date. */
pub const ALPM_TRANS_FLAG_NEEDED : i32 = (1 << 13);
/** Use pub const ALPM_PKG_REASON_EXPLICIT when installing packages. */
pub const ALPM_TRANS_FLAG_ALLEXPLICIT : i32 = (1 << 14);
/** Do not remove a package if it is needed by another one. */
pub const ALPM_TRANS_FLAG_UNNEEDED : i32 = (1 << 15);
/** Remove also explicitly installed unneeded deps (use with pub const ALPM_TRANS_FLAG_RECURSE). */
pub const ALPM_TRANS_FLAG_RECURSEALL : i32 = (1 << 16);
/** Do not lock the database during the operation. */
pub const ALPM_TRANS_FLAG_NOLOCK : i32 = (1 << 17);


#[derive(Debug,Copy,Clone)]
#[allow(non_camel_case_types)]
#[repr(C)]
pub enum ErrorNo {
	ALPM_ERR_OK,
	ALPM_ERR_MEMORY,
	ALPM_ERR_SYSTEM,
	ALPM_ERR_BADPERMS,
	ALPM_ERR_NOT_A_FILE,
	ALPM_ERR_NOT_A_DIR,
	ALPM_ERR_WRONG_ARGS,
	ALPM_ERR_DISK_SPACE,
	/* Interface */
	ALPM_ERR_HANDLE_NULL,
	ALPM_ERR_HANDLE_NOT_NULL,
	ALPM_ERR_HANDLE_LOCK,
	/* Databases */
	ALPM_ERR_DB_OPEN,
	ALPM_ERR_DB_CREATE,
	ALPM_ERR_DB_NULL,
	ALPM_ERR_DB_NOT_NULL,
	ALPM_ERR_DB_NOT_FOUND,
	ALPM_ERR_DB_INVALID,
	ALPM_ERR_DB_INVALID_SIG,
	ALPM_ERR_DB_VERSION,
	ALPM_ERR_DB_WRITE,
	ALPM_ERR_DB_REMOVE,
	/* Servers */
	ALPM_ERR_SERVER_BAD_URL,
	ALPM_ERR_SERVER_NONE,
	/* Transactions */
	ALPM_ERR_TRANS_NOT_NULL,
	ALPM_ERR_TRANS_NULL,
	ALPM_ERR_TRANS_DUP_TARGET,
	ALPM_ERR_TRANS_NOT_INITIALIZED,
	ALPM_ERR_TRANS_NOT_PREPARED,
	ALPM_ERR_TRANS_ABORT,
	ALPM_ERR_TRANS_TYPE,
	ALPM_ERR_TRANS_NOT_LOCKED,
	ALPM_ERR_TRANS_HOOK_FAILED,
	/* Packages */
	ALPM_ERR_PKG_NOT_FOUND,
	ALPM_ERR_PKG_IGNORED,
	ALPM_ERR_PKG_INVALID,
	ALPM_ERR_PKG_INVALID_CHECKSUM,
	ALPM_ERR_PKG_INVALID_SIG,
	ALPM_ERR_PKG_MISSING_SIG,
	ALPM_ERR_PKG_OPEN,
	ALPM_ERR_PKG_CANT_REMOVE,
	ALPM_ERR_PKG_INVALID_NAME,
	ALPM_ERR_PKG_INVALID_ARCH,
	ALPM_ERR_PKG_REPO_NOT_FOUND,
	/* Signatures */
	ALPM_ERR_SIG_MISSING,
	ALPM_ERR_SIG_INVALID,
	/* Deltas */
	ALPM_ERR_DLT_INVALID,
	ALPM_ERR_DLT_PATCHFAILED,
	/* Dependencies */
	ALPM_ERR_UNSATISFIED_DEPS,
	ALPM_ERR_CONFLICTING_DEPS,
	ALPM_ERR_FILE_CONFLICTS,
	/* Misc */
	ALPM_ERR_RETRIEVE,
	ALPM_ERR_INVALID_REGEX,
	/* External library errors */
	ALPM_ERR_LIBARCHIVE,
	ALPM_ERR_LIBCURL,
	ALPM_ERR_EXTERNAL_DOWNLOAD,
	ALPM_ERR_GPGME,
}
