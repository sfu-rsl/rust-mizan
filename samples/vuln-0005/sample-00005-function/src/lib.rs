macro_rules! define_colors {
    {$(
        $ident:ident,
        $channels: expr,
        $alphas: expr,
        $interpretation: expr,
        $color_type_u8: expr,
        $color_type_u16: expr,
        #[$doc:meta];
    )*} => {

$( // START Structure definitions

#[$doc]
#[derive(PartialEq, Eq, Clone, Debug, Copy, Hash)]
#[repr(C)]
#[allow(missing_docs)]
pub struct $ident<T: Primitive> (pub [T; $channels]);

impl<T: Primitive + 'static> Pixel for $ident<T> {

    fn from_slice_mut(slice: &mut [T]) -> &mut $ident<T> {
        assert_eq!(slice.len(), $channels);
        unsafe { &mut *(slice.as_ptr() as *mut $ident<T>) }
    }

}
)* // END Structure definitions
    }
}