#![doc= " Iteration processes for `BitVec`."]

use super::*;
use crate::{
    order::BitOrder,
    store::BitStore,
};
use core::iter::{
    FromIterator,
    FusedIterator,
};

#[doc= " State keeper for draining iteration.\n\n# Type Parameters\n\n- `O: BitOrder`: The ordering type of the underlying vector.\n- `T: 'a + BitStore`: The storage type of the underlying vector.\n\n# Lifetimes\n\n- `'a`: The lifetime of the underlying vector.\n*"]
pub(crate) struct Drain<'a, O, T>
where
    O: BitOrder,
    T: 'a + BitStore {
    #[doc= " Current remaining range to remove."]
    pub(super) iter: crate::slice::iter::Iter<'a, O, T>,
}

#[doc= " A splicing iterator for `BitVec`.\n\nThis removes a segment from the vector and inserts another bitstream into its\nspot. Any bits from the original `BitVec` after the removed segment are kept,\nafter the inserted bitstream.\n\nOnly the removed segment is available for iteration.\n\n# Type Parameters\n\n- `I: Iterator<Item=bool>`: Any bitstream. This will be used to fill the\n  removed span.\n*"]
pub(crate) struct Splice<'a, O, T, I>
where
    O: BitOrder,
    T: 'a + BitStore,
    I: Iterator<Item = bool> {
    pub(super) drain: Drain<'a, O, T>,
    pub(super) splice: I,
}
