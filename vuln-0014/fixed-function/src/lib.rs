use core::marker::PhantomData;

/// This is one part of the secret sauce that ensures that indices from
/// different arenas cannot be mixed. You should never need to use this type in
/// your code.
#[derive(Copy, Clone, PartialOrd, PartialEq, Eq)]
pub struct InvariantLifetime<'a>(PhantomData<fn(&'a ()) -> &'a ()>);

/// Create an invariant lifetime. This is one part of the secret sauce that
/// ensures that indices from different arenas cannot be mixed. You should
/// never need to use this type in your code.
pub fn invariant_lifetime<'tag>() -> InvariantLifetime<'tag> {
    InvariantLifetime(PhantomData)
}

/// A "Small" arena based on a resizable slice (i.e. a `Vec`) that can be
/// indexed with 32-bit `Idx32`s. This can help reduce memory overhead when
/// using many pointer-heavy objects on 64-bit systems.
///
/// You can obtain an instance of this type by calling `mk_arena`.
#[cfg(feature = "alloc")]
pub struct SmallArena<'tag, T> {
    tag: InvariantLifetime<'tag>,
    // TODO: Use a custom structure, forbid resizing over 2G items
    data: Vec<T>,
}

#[cfg(feature = "alloc")]
#[macro_export]
macro_rules! mk_arena {
    ($name:ident) => { $crate::mk_arena!($name, 128*1024) };
    ($name:ident, $cap:expr) => {
        let tag = $crate::invariant_lifetime();
        let _guard;
        let mut $name = unsafe {
            // this is not per-se unsafe but we need it to be public and
            // calling it with a non-unique `tag` would allow arena mixups,
            // which may introduce UB in `Index`/`IndexMut`
            $crate::SmallArena::new(tag, $cap)
        };
        // this doesn't make it to MIR, but ensures that borrowck will not
        // unify the lifetimes of two macro calls by binding the lifetime to
        // drop scope
        if false {
            struct Guard<'tag>(&'tag $crate::InvariantLifetime<'tag>);
            impl<'tag> ::core::ops::Drop for Guard<'tag> {
                fn drop(&mut self) { }
            }
            _guard = Guard(&tag);
        }
    };
}

#[cfg(feature = "alloc")]
impl<'tag, T> SmallArena<'tag, T> {
    /// create a new SmallArena. Don't do this manually. Use the
    /// [`in_arena`] macro instead.
    ///
    /// # Safety
    ///
    /// The whole tagged indexing trick relies on the `'tag` you give to this
    /// constructor. You must never use this value in another arena, lest you
    /// might be able to mix up the indices of the two, which could lead to
    /// out of bounds access and thus **Undefined Behavior**!
    pub unsafe fn new(tag: InvariantLifetime<'tag>, capacity: usize) -> Self {
        SmallArena {
            tag,
            data: Vec::with_capacity(capacity),
        }
    }
}