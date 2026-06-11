use {super::endian::LittleEndian, super::write::BiteWriteExpandedExt, std, std::io::Write};

pub trait BiteWriteExt: Write {
    #[inline]
    fn write_u8(&mut self, value: u8) -> Result<(), std::io::Error> {
        BiteWriteExpandedExt::write_u8(self, value)
    }
    #[inline]
    fn write_i8(&mut self, value: i8) -> Result<(), std::io::Error> {
        BiteWriteExpandedExt::write_i8(self, value)
    }
    #[inline]
    fn write_u16(&mut self, value: u16) -> Result<(), std::io::Error> {
        BiteWriteExpandedExt::write_u16::<LittleEndian>(self, value)
    }
    #[inline]
    fn write_i16(&mut self, value: i16) -> Result<(), std::io::Error> {
        BiteWriteExpandedExt::write_i16::<LittleEndian>(self, value)
    }
    #[inline]
    fn write_u32(&mut self, value: u32) -> Result<(), std::io::Error> {
        BiteWriteExpandedExt::write_u32::<LittleEndian>(self, value)
    }
    #[inline]
    fn write_i32(&mut self, value: i32) -> Result<(), std::io::Error> {
        BiteWriteExpandedExt::write_i32::<LittleEndian>(self, value)
    }
    #[inline]
    fn write_u24(&mut self, value: u32) -> Result<(), std::io::Error> {
        BiteWriteExpandedExt::write_u24::<LittleEndian>(self, value)
    }
    #[inline]
    fn write_i24(&mut self, value: i32) -> Result<(), std::io::Error> {
        BiteWriteExpandedExt::write_i24::<LittleEndian>(self, value)
    }
    #[inline]
    fn write_u64(&mut self, value: u64) -> Result<(), std::io::Error> {
        BiteWriteExpandedExt::write_u64::<LittleEndian>(self, value)
    }
    #[inline]
    fn write_i64(&mut self, value: i64) -> Result<(), std::io::Error> {
        BiteWriteExpandedExt::write_i64::<LittleEndian>(self, value)
    }
    #[inline]
    fn write_u128(&mut self, value: u128) -> Result<(), std::io::Error> {
        BiteWriteExpandedExt::write_u128::<LittleEndian>(self, value)
    }
    #[inline]
    fn write_i128(&mut self, value: i128) -> Result<(), std::io::Error> {
        BiteWriteExpandedExt::write_i128::<LittleEndian>(self, value)
    }
    #[inline]
    fn write_uint(&mut self, value: u64, bytes: usize) -> Result<(), std::io::Error> {
        BiteWriteExpandedExt::write_uint::<LittleEndian>(self, value, bytes)
    }
    #[inline]
    fn write_int(&mut self, value: i64, bytes: usize) -> Result<(), std::io::Error> {
        BiteWriteExpandedExt::write_int::<LittleEndian>(self, value, bytes)
    }
    #[inline]
    fn write_uint128(&mut self, value: u128, bytes: usize) -> Result<(), std::io::Error> {
        BiteWriteExpandedExt::write_uint128::<LittleEndian>(self, value, bytes)
    }
    #[inline]
    fn write_int128(&mut self, value: i128, bytes: usize) -> Result<(), std::io::Error> {
        BiteWriteExpandedExt::write_int128::<LittleEndian>(self, value, bytes)
    }
    #[inline]
    fn write_f32(&mut self, value: f32) -> Result<(), std::io::Error> {
        BiteWriteExpandedExt::write_f32::<LittleEndian>(self, value)
    }
    #[inline]
    fn write_f64(&mut self, value: f64) -> Result<(), std::io::Error> {
        BiteWriteExpandedExt::write_f64::<LittleEndian>(self, value)
    }

    // #[inline] fn write_slice_u16 (&mut self, values: &[u16])         -> Result<(), std::io::Error> { BiteWriteExpandedExt::write_slice_u16 ::<LittleEndian>(self, values)       }
    // #[inline] fn write_slice_i16 (&mut self, values: &[i16])         -> Result<(), std::io::Error> { BiteWriteExpandedExt::write_slice_i16 ::<LittleEndian>(self, values)       }
    // #[inline] fn write_slice_u32 (&mut self, values: &[u32])         -> Result<(), std::io::Error> { BiteWriteExpandedExt::write_slice_u32 ::<LittleEndian>(self, values)       }
    // #[inline] fn write_slice_i32 (&mut self, values: &[i32])         -> Result<(), std::io::Error> { BiteWriteExpandedExt::write_slice_i32 ::<LittleEndian>(self, values)       }
    // #[inline] fn write_slice_u64 (&mut self, values: &[u64])         -> Result<(), std::io::Error> { BiteWriteExpandedExt::write_slice_u64 ::<LittleEndian>(self, values)       }
    // #[inline] fn write_slice_i64 (&mut self, values: &[i64])         -> Result<(), std::io::Error> { BiteWriteExpandedExt::write_slice_i64 ::<LittleEndian>(self, values)       }
    // #[inline] fn write_slice_u128(&mut self, values: &[u128])        -> Result<(), std::io::Error> { BiteWriteExpandedExt::write_slice_u128::<LittleEndian>(self, values)       }
    // #[inline] fn write_slice_i128(&mut self, values: &[i128])        -> Result<(), std::io::Error> { BiteWriteExpandedExt::write_slice_i128::<LittleEndian>(self, values)       }
    // #[inline] fn write_slice_f32 (&mut self, values: &[f32])         -> Result<(), std::io::Error> { BiteWriteExpandedExt::write_slice_f32 ::<LittleEndian>(self, values)       }
    // #[inline] fn write_slice_f64 (&mut self, values: &[f64])         -> Result<(), std::io::Error> { BiteWriteExpandedExt::write_slice_f64 ::<LittleEndian>(self, values)       }

    #[inline]
    fn write_framed(&mut self, value: &[u8]) -> Result<(), std::io::Error> {
        BiteWriteExpandedExt::write_framed::<LittleEndian>(self, value)
    }
}

impl<T> BiteWriteExt for T where T: Write + ?Sized {}
