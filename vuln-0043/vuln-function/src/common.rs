use core::marker::PhantomData;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AsciiStr<'a> {
    ptr: *const u8,
    end: *const u8,
    _marker: PhantomData<&'a [u8]>,
}

impl<'a> AsciiStr<'a> {
    #[inline]
    pub fn first(&self) -> u8 {
        unsafe { *self.ptr }
    }
}