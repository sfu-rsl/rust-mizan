mod inline_array;
use crate::inline_array::InlineArray;
use core::ptr;

mod types;

use core::mem::MaybeUninit;

use typenum::U64;

use crate::types::ChunkLength;

pub struct Chunk<A, N = U64>
where
    N: ChunkLength<A>,
{
    left: usize,
    right: usize,
    data: MaybeUninit<N::SizedType>,
}

impl<A, N> Chunk<A, N>
where
    N: ChunkLength<A>,
{
    /// Construct a new empty chunk.
    pub fn new() -> Self {
        Self {
            left: 0,
            right: 0,
            data: MaybeUninit::uninit(),
        }
    }

    #[inline]
    unsafe fn mut_ptr(&mut self, index: usize) -> *mut A {
        (&mut self.data as *mut _ as *mut A).add(index)
    }
}

impl<'a, A, N, T> From<&'a mut InlineArray<A, T>> for Chunk<A, N>
where
    N: ChunkLength<A>,
{
    fn from(array: &mut InlineArray<A, T>) -> Self {
        let mut out = Self::new();
        out.left = 0;
        out.right = array.len();
        unsafe {
            ptr::copy_nonoverlapping(array.data(), out.mut_ptr(0), out.right);
            *array.len_mut() = 0;
        }
        out
    }
}
