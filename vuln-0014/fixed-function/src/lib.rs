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