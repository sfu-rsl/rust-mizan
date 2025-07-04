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
    /// Construct a new chunk with two items.
    pub fn pair(left: A, right: A) -> Self {
        let mut chunk = Self {
            left: 0,
            right: 2,
            data: MaybeUninit::uninit(),
        };
        unsafe {
            Chunk::force_write(0, left, &mut chunk);
            Chunk::force_write(1, right, &mut chunk);
        }
        chunk
    }

    /// Write a value at an index without trying to drop what's already there
    #[inline]
    unsafe fn force_write(index: usize, value: A, chunk: &mut Self) {
        chunk.mut_ptr(index).write(value)
    }

    #[inline]
    unsafe fn mut_ptr(&mut self, index: usize) -> *mut A {
        (&mut self.data as *mut _ as *mut A).add(index)
    }
}
