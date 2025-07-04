# Vulnerability: CVE-2020-35866

| **Information**       | **Details**                                                                                     |
| --------------------- | ----------------------------------------------------------------------------------------------- |
| **CVE**               | [CVE-2020-35866](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2020-35866)                 |
| **Vulnerable Commit** | [156fa9f](https://github.com/rusqlite/rusqlite/commit/156fa9fcf23cadd5db3bdeeca11e5a25cd813e5b) |
| **Fixed Commit**      | [54043c8](https://github.com/rusqlite/rusqlite/commit/54043c803c83517aa54b463b098a61dfa28c54f4) |
| **Variants**          | - [fixed-crate](fixed-crate)                                                                    |
|                       | - [vuln-crate](vuln-crate)                                                                      |

### Vulnerable Lines

`src/vtab/mod.rs`

````rust
// VULNERABILITY: These traits should be marked unsafe
// SQLite requires that virtual table structs start with `sqlite3_vtab` and use a C-compatible layout (`#[repr(C)]`)
// Implementing them on the wrong type is unsound. See https://github.com/rusqlite/rusqlite/pull/704


/// `feature = "vtab"` Virtual table instance trait.
///
/// Implementations must be like:
/// ```rust,ignore
/// #[repr(C)]
/// struct MyTab {
///    /// Base class. Must be first
///    base: ffi::sqlite3_vtab,
///    /* Virtual table implementations will typically add additional fields */
/// }
/// ```
///
/// (See [SQLite doc](https://sqlite.org/c3ref/vtab.html))
// VULNERABILITY: should be marked unsafe
pub trait VTab: Sized {
    type Aux;
    type Cursor: VTabCursor;

    /// Establish a new connection to an existing virtual table.
    ///
    /// (See [SQLite doc](https://sqlite.org/vtab.html#the_xconnect_method))
    fn connect(
        db: &mut VTabConnection,
        aux: Option<&Self::Aux>,
        args: &[&[u8]],
    ) -> Result<(String, Self)>;

    /// Determine the best way to access the virtual table.
    /// (See [SQLite doc](https://sqlite.org/vtab.html#the_xbestindex_method))
    fn best_index(&self, info: &mut IndexInfo) -> Result<()>;

    /// Create a new cursor used for accessing a virtual table.
    /// (See [SQLite doc](https://sqlite.org/vtab.html#the_xopen_method))
    fn open(&self) -> Result<Self::Cursor>;
}

// ..

/// `feature = "vtab"` Virtual table cursor trait.
///
/// Implementations must be like:
/// ```rust,ignore
/// #[repr(C)]
/// struct MyTabCursor {
///    /// Base class. Must be first
///    base: ffi::sqlite3_vtab_cursor,
///    /* Virtual table implementations will typically add additional fields */
/// }
/// ```
///
/// (See [SQLite doc](https://sqlite.org/c3ref/vtab_cursor.html))
// VULNERABILITY: should be marked unsafe
pub trait VTabCursor: Sized {
    /// Begin a search of a virtual table.
    /// (See [SQLite doc](https://sqlite.org/vtab.html#the_xfilter_method))
    fn filter(&mut self, idx_num: c_int, idx_str: Option<&str>, args: &Values<'_>) -> Result<()>;
    /// Advance cursor to the next row of a result set initiated by `filter`.
    /// (See [SQLite doc](https://sqlite.org/vtab.html#the_xnext_method))
    fn next(&mut self) -> Result<()>;
    /// Must return `false` if the cursor currently points to a valid row of
    /// data, or `true` otherwise.
    /// (See [SQLite doc](https://sqlite.org/vtab.html#the_xeof_method))
    fn eof(&self) -> bool;
    /// Find the value for the `i`-th column of the current row.
    /// `i` is zero-based so the first column is numbered 0.
    /// May return its result back to SQLite using one of the specified `ctx`.
    /// (See [SQLite doc](https://sqlite.org/vtab.html#the_xcolumn_method))
    fn column(&self, ctx: &mut Context, i: c_int) -> Result<()>;
    /// Return the rowid of row that the cursor is currently pointing at.
    /// (See [SQLite doc](https://sqlite.org/vtab.html#the_xrowid_method))
    fn rowid(&self) -> Result<i64>;
}
````
