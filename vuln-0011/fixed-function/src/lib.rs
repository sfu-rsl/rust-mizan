//! Small crate for initializing an uninitialized slice
#![no_std]

use core::mem::MaybeUninit;

#[repr(C)]
struct Wrapper<T, const Y: usize, const Z: usize>([T; Y], [T; Z]);

const fn check_impl<T, const X: usize, const Y: usize, const Z: usize>() {
    // Check that the array sizes match
    assert!(X == Y + Z, "Array cannot be split: sizes don't match");
    assert!(
        usize::checked_add(Y, Z).is_some(),
        "Array cannot be split: length would overflow"
    );
    // Make doubly sure that nothing funky is going on with the memory representations
    assert!(core::mem::size_of::<Wrapper<T, Y, Z>>() == core::mem::size_of::<[T; X]>());
    assert!(core::mem::align_of::<Wrapper<T, Y, Z>>() == core::mem::align_of::<[T; X]>());
}

trait SizeCheck<T, const X: usize, const Y: usize, const Z: usize> {
    const CHECK: Self;
}

impl<T, const X: usize, const Y: usize, const Z: usize> SizeCheck<T, X, Y, Z> for () {
    const CHECK: () = check_impl::<T, X, Y, Z>();
}

#[allow(clippy::let_unit_value)]
fn check<T, const X: usize, const Y: usize, const Z: usize>() {
    let _: () = SizeCheck::<T, X, Y, Z>::CHECK;
    // Do the same checks at run-time, so even if rustc changes to allow ignore
    // our compile-time errors, we will at least not create UB
    check_impl::<T, X, Y, Z>();
}

/// A fixed-size cursor for initializing [`MaybeUninit`] arrays
///
/// The cursor will guarantee that all values have been
/// initialized when the value is dropped, which means
/// that it is safe to call [`MaybeUninit::assume_init()`].
///
/// **NOTE:** This guarantee only holds as long as [`Drop::drop()`] is called.
///           If the value goes out of scope without drop being called (e.g. because
///           of [`core::mem::forget()`]), then this guarantee no longer applies.
pub struct Cursor<'a, T, const N: usize> {
    slice: &'a mut [MaybeUninit<T>; N],
}

impl<'a, T, const N: usize> Cursor<'a, T, N> {
    fn write_impl(&mut self, value: [T; N]) {
        *self.slice = value.map(|v| MaybeUninit::new(v));
    }
}