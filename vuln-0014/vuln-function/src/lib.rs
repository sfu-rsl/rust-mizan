use core::marker::PhantomData;

#[derive(Copy, Clone, PartialOrd, PartialEq, Eq)]
struct InvariantLifetime<'a>(PhantomData<fn(&'a ()) -> &'a ()>);

/// A "Small" arena based on a resizable slice (i.e. a `Vec`) that can be
/// indexed with 32-bit `Idx32`s. This can help reduce memory overhead when
/// using many pointer-heavy objects on 64-bit systems.
///
/// You can obtain an instance of this type by calling `mk_arena`.
pub struct SmallArena<'tag, T> {
    tag: InvariantLifetime<'tag>,
    // TODO: Use a custom structure, forbid resizing over 2G items
    data: Vec<T>,
}

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
    pub unsafe fn new(_: &'tag mut (), capacity: usize) -> Self {
        SmallArena {
            tag: InvariantLifetime(PhantomData),
            data: Vec::with_capacity(capacity),
        }
    }
}
