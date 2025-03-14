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
    /// The maximum number of elements this `Chunk` can contain.
    pub const CAPACITY: usize = N::USIZE;

    /// Construct a new chunk with one item.
    pub fn unit(value: A) -> Self {
        assert!(Self::CAPACITY >= 1);
        let mut chunk = Self {
            left: 0,
            right: 1,
            data: MaybeUninit::uninit(),
        };
        unsafe {
            Chunk::force_write(0, value, &mut chunk);
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
