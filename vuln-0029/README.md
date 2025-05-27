# Vulnerability: RUSTSEC-2025-0016

| **Information**       | **Details**                                                                                   |
|-----------------------|-----------------------------------------------------------------------------------------------|
| **CVE**               | [RUSTSEC-2025-0016](https://rustsec.org/advisories/RUSTSEC-2025-0016.html)               |
| **Vulnerable Commit** | [65bbc62](https://github.com/radekvit/pared/commit/65bbc627e31a2fb43fc3b94c7ca1177536c7c626) |
| **Fixed Commit**      | [fdc4de8](https://github.com/radekvit/pared/commit/fdc4de8be7cffb2b7d71c910b6a12964b61f4f0d) |
| **Variants**          | - [vuln-crate](vuln-crate)                                                                    |
|                       | - [vuln-file](vuln-file)                                                              |
|                       | - [vuln-function](vuln-function)                                                              |
|                       | - [fixed-crate](fixed-crate)                                                                  |
|                       | - [vuln-file](vuln-file)                                                              |
|                       | - [fixed-function](fixed-function)                                                            |

### Vulnerable lines

`src/prc.rs`

```rust
#[inline]
pub fn from_rc<U, F>(rc: &Rc<U>, project: F) -> Self
where
    /// VULNERABILITY: without the 'static lifetime bound on U, the projected pointer can outlive
    /// the original data's lifetime causing a potential use after free
    U: ?Sized,
    T: 'static,
    F: FnOnce(&U) -> &T,
{
    let projected = project(rc);
    // SAFETY: fn shouldn't be able to capture any local references
    // which should mean that the projection done by f is safe
    let projected = unsafe { NonNull::new_unchecked(projected as *const T as *mut T) };
    Self {
        rc: TypeErasedRc::new(rc.clone()),
        projected,
    }
}

#[inline]
pub fn try_from_rc<U, E, F>(rc: &Rc<U>, project: F) -> Result<Self, E>
where
    /// VULNERABILITY: without the 'static lifetime bound on U, the projected pointer can outlive
    /// the original data's lifetime causing a potential use after free
    U: ?Sized,
    T: 'static,
    F: FnOnce(&U) -> Result<&T, E>,
{
    let projected = project(rc)?;
    // SAFETY: fn shouldn't be able to capture any local references
    // which should mean that the projection done by f is safe
    let projected = unsafe { NonNull::new_unchecked(projected as *const T as *mut T) };
    Ok(Self {
        rc: TypeErasedRc::new(rc.clone()),
        projected,
    })
}

pub fn project<U, F>(&self, project: F) -> Prc<U>
where
    U: ?Sized + 'static,
    /// VULNERABILITY: without the 'static lifetime bound on T, the projected pointer can outlive
    /// the original data's lifetime causing a potential use after free
    F: FnOnce(&T) -> &U,
{
    let projected = project(self);
    // SAFETY: fn shouldn't be able to capture any local references
    // which should mean that the projection done by f is safe
    let projected = unsafe { NonNull::new_unchecked(projected as *const U as *mut U) };
    Prc::<U> {
        rc: self.rc.clone(),
        projected,
    }
}
```

`src/sync.rs`

```rust

#[inline]
pub fn from_arc<U, F>(arc: &Arc<U>, project: F) -> Self
where
    T: 'static,
    /// VULNERABILITY: without the 'static lifetime bound on U, the projected pointer can outlive
    /// the original data's lifetime causing a potential use after free
    U: ?Sized + Send + Sync,
    F: FnOnce(&U) -> &T,
{
    let projected = project(arc);
    // SAFETY: the returned reference always converts to a non-null pointer.
    // It's safe to convert the returned reference to a pointer (and then convert it in `Deref`)
    // because the lifetime of the reference returned by `F` must be either the lifetime
    // of the local reference passed to it, or 'static
    let projected = unsafe { NonNull::new_unchecked(projected as *const T as *mut T) };
    Self {
        arc: TypeErasedArc::new(arc.clone()),
        projected,
    }
}

#[inline]
pub fn try_from_arc<U, E, F>(arc: &Arc<U>, project: F) -> Result<Self, E>
where
    /// VULNERABILITY: without the 'static lifetime bound on U, the projected pointer can outlive
    /// the original data's lifetime causing a potential use after free
    U: ?Sized + Sync + Send,
    T: 'static,
    F: FnOnce(&U) -> Result<&T, E>,
{
    let projected = project(arc)?;
    // SAFETY: fn shouldn't be able to capture any local references
    // which should mean that the projection done by f is safe
    let projected = unsafe { NonNull::new_unchecked(projected as *const T as *mut T) };
    Ok(Self {
        arc: TypeErasedArc::new(arc.clone()),
        projected,
    })
}

#[inline]
pub fn project<U, F>(&self, project: F) -> Parc<U>
where
    /// VULNERABILITY: without the 'static lifetime bound on T, the projected pointer can outlive
    /// the original data's lifetime causing a potential use after free
    T: Send + Sync,
    U: ?Sized + 'static,
    F: FnOnce(&T) -> &U,
{
    let projected = project(self);
    // SAFETY: the returned reference always converts to a non-null pointer.
    // It's safe to convert the returned reference to a pointer (and then convert it in `Deref`)
    // because the lifetime of the reference returned by `F` must be either the lifetime
    // of the local reference passed to it, or 'static
    let projected = unsafe { NonNull::new_unchecked(projected as *const U as *mut U) };
    Parc::<U> {
        arc: self.arc.clone(),
        projected,
    }
}
```