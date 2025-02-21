use std::mem;
use std::slice;

/// Growable byte buffer implemented as a ring buffer.
///
/// Optimized for repeated appending of bytes to the end and removing bytes from the front of the buffer.
#[derive(Clone, Debug)]
pub struct Buffer {
    array: Box<[u8]>,
    head: usize,
    len: usize,
}

/// Macro for making memory copies more readable.
macro_rules! copy {
    ($src:expr, $src_start:expr, $dest:expr, $dest_start:expr, $len:expr) => {
        (&mut $dest[$dest_start..$dest_start + $len])
            .copy_from_slice(&$src[$src_start..$src_start + $len])
    };
}

impl Buffer {
    /// Create a new buffer with a given minimum capacity pre-allocated.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            array: Buffer::allocate(capacity.next_power_of_two()),
            head: 0,
            len: 0,
        }
    }

    #[inline]
    pub fn capacity(&self) -> usize {
        self.array.len()
    }

    /// Copy bytes from the front of the buffer into the given slice.
    ///
    /// Returns the number of bytes copied. If there are less bytes in the buffer than the length of `dest`, then only
    /// part of `dest` will be written to.
    pub fn copy_to(&self, dest: &mut [u8]) -> usize {
        // Determine the number of bytes to copy.
        let count = dest.len().min(self.len);

        // Nothing to do.
        if count == 0 {
            return 0;
        }

        // Current buffer is wrapped; copy head segment and tail segment separately.
        let tail = self.offset(count);
        if tail <= self.head {
            let head_len = self.capacity() - self.head;
            copy!(self.array, self.head, dest, 0, head_len);
            copy!(self.array, 0, dest, head_len, tail);
        }
        // Buffer is contiguous; copy in one step.
        else {
            copy!(self.array, self.head, dest, 0, count);
        }

        count
    }

    /// Calculate the internal offset of the given byte position.
    fn offset(&self, index: usize) -> usize {
        let mut offset = self.head + index;

        if offset >= self.capacity() {
            offset -= self.capacity();
        }

        offset
    }

    /// Allocate an array of memory on the heap.
    ///
    /// Note that the contents of the array are not initialized and the values are undefined. This is safe only because
    /// we are asking for an array of bytes anyway.
    fn allocate(size: usize) -> Box<[u8]> {
        unsafe {
            let mut vec = Vec::<u8>::with_capacity(size);
            let slice = slice::from_raw_parts_mut(vec.as_mut_ptr(), vec.capacity());
            mem::forget(vec);
            Box::from_raw(slice)
        }
    }
}

impl From<Buffer> for Vec<u8> {
    fn from(buffer: Buffer) -> Vec<u8> {
        let mut slice = Buffer::allocate(buffer.len);
        let len = buffer.copy_to(&mut slice);

        unsafe {
            let vec = Vec::from_raw_parts(slice.as_mut_ptr(), len, slice.len());
            mem::forget(slice);
            vec
        }
    }
}
