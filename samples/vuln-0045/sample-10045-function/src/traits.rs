pub(crate) use internal::Beef;
pub(crate) use internal::Capacity;

pub(crate) mod internal {
    use alloc::borrow::ToOwned;
    // use alloc::string::String;
    // use alloc::vec::Vec;
    // use core::mem::ManuallyDrop;
    use core::ptr::NonNull;

    pub trait Capacity {
        type Field: Copy;
        type NonZero: Copy;

        fn len(fat: usize) -> usize;

        fn empty(len: usize) -> (usize, Self::Field);

        fn store(len: usize, capacity: usize) -> (usize, Self::Field);

        fn unpack(fat: usize, capacity: Self::NonZero) -> (usize, usize);

        fn maybe(fat: usize, capacity: Self::Field) -> Option<Self::NonZero>;
    }

    /// Helper trait required by `Cow<T>` to extract capacity of owned
    /// variant of `T`, and manage conversions.
    ///
    /// This can be only implemented on types that match requirements:
    ///
    /// + `T::Owned` has a `capacity`, which is an extra word that is absent in `T`.
    /// + `T::Owned` with `capacity` of `0` does not allocate memory.
    /// + `T::Owned` can be reconstructed from `*mut T` borrowed out of it, plus capacity.
    pub unsafe trait Beef: ToOwned {
        type PointerT;

        fn ref_into_parts<U>(&self) -> (NonNull<Self::PointerT>, usize, U::Field)
        where
            U: Capacity;

        unsafe fn ref_from_parts<U>(ptr: NonNull<Self::PointerT>, len: usize) -> *const Self
        where
            U: Capacity;

        /// Convert `T::Owned` to `NonNull<T>` and capacity.
        /// Return `None` for `0` capacity.
        fn owned_into_parts<U>(owned: Self::Owned) -> (NonNull<Self::PointerT>, usize, U::Field)
        where
            U: Capacity;

        /// Rebuild `T::Owned` from `NonNull<T>` and `capacity`. This can be done by the likes
        /// of [`Vec::from_raw_parts`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.from_raw_parts).
        unsafe fn owned_from_parts<U>(
            ptr: NonNull<Self::PointerT>,
            fat: usize,
            capacity: U::NonZero,
        ) -> Self::Owned
        where
            U: Capacity;
    }
}
