/// Marker trait to determine if a type is auto-zeroable. This allows the initialization to simply
/// zero out the buffer on initialization.
pub trait Zeroable {}

impl Zeroable for u8 {}
impl Zeroable for i8 {}
impl Zeroable for u16 {}
impl Zeroable for i16 {}
impl Zeroable for u32 {}
impl Zeroable for i32 {}
impl Zeroable for u64 {}
impl Zeroable for i64 {}
impl Zeroable for usize {}
impl Zeroable for isize {}
impl Zeroable for f32 {}
impl Zeroable for f64 {}

impl<T, const N: usize> Zeroable for [T; N]
  where T: Zeroable {}
