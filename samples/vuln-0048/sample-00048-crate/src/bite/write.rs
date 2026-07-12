use {super::endian::Endianness, std, std::io::Write};

pub trait BiteWriteExpandedExt: Write {
    #[inline]
    fn write_u8(&mut self, value: u8) -> Result<(), std::io::Error> {
        self.write_all(&[value])
    }

    #[inline]
    fn write_i8(&mut self, value: i8) -> Result<(), std::io::Error> {
        self.write_all(&[value as u8])
    }

    #[inline]
    fn write_u16<T: Endianness>(&mut self, value: u16) -> Result<(), std::io::Error> {
        let mut data = [0; 2];
        T::write_u16(&mut data, value);

        self.write_all(&data)
    }

    #[inline]
    fn write_i16<T: Endianness>(&mut self, value: i16) -> Result<(), std::io::Error> {
        let mut data = [0; 2];
        T::write_i16(&mut data, value);

        self.write_all(&data)
    }

    #[inline]
    fn write_u24<T: Endianness>(&mut self, value: u32) -> Result<(), std::io::Error> {
        let mut data = [0; 4];
        T::write_u24(&mut data, value);

        self.write_all(&data)
    }

    #[inline]
    fn write_i24<T: Endianness>(&mut self, value: i32) -> Result<(), std::io::Error> {
        let mut data = [0; 4];
        T::write_i24(&mut data, value);

        self.write_all(&data)
    }

    #[inline]
    fn write_u32<T: Endianness>(&mut self, value: u32) -> Result<(), std::io::Error> {
        let mut data = [0; 4];
        T::write_u32(&mut data, value);

        self.write_all(&data)
    }

    #[inline]
    fn write_i32<T: Endianness>(&mut self, value: i32) -> Result<(), std::io::Error> {
        let mut data = [0; 4];
        T::write_i32(&mut data, value);

        self.write_all(&data)
    }

    #[inline]
    fn write_u64<T: Endianness>(&mut self, value: u64) -> Result<(), std::io::Error> {
        let mut data = [0; 8];
        T::write_u64(&mut data, value);

        self.write_all(&data)
    }

    #[inline]
    fn write_i64<T: Endianness>(&mut self, value: i64) -> Result<(), std::io::Error> {
        let mut data = [0; 8];
        T::write_i64(&mut data, value);

        self.write_all(&data)
    }

    #[inline]
    fn write_u128<T: Endianness>(&mut self, value: u128) -> Result<(), std::io::Error> {
        let mut data = [0; 16];
        T::write_u128(&mut data, value);

        self.write_all(&data)
    }

    #[inline]
    fn write_i128<T: Endianness>(&mut self, value: i128) -> Result<(), std::io::Error> {
        let mut data = [0; 16];
        T::write_i128(&mut data, value);

        self.write_all(&data)
    }

    #[inline]
    fn write_uint<T: Endianness>(
        &mut self,
        value: u64,
        bytes: usize,
    ) -> Result<(), std::io::Error> {
        let mut data = [0; 8];
        T::write_uint(&mut data, value, bytes);

        self.write_all(&data[0..bytes])
    }

    #[inline]
    fn write_int<T: Endianness>(&mut self, value: i64, bytes: usize) -> Result<(), std::io::Error> {
        let mut data = [0; 8];
        T::write_int(&mut data, value, bytes);

        self.write_all(&data[0..bytes])
    }

    #[inline]
    fn write_uint128<T: Endianness>(
        &mut self,
        value: u128,
        bytes: usize,
    ) -> Result<(), std::io::Error> {
        let mut data = [0; 16];
        T::write_uint128(&mut data, value, bytes);

        self.write_all(&data[0..bytes])
    }

    #[inline]
    fn write_int128<T: Endianness>(
        &mut self,
        value: i128,
        bytes: usize,
    ) -> Result<(), std::io::Error> {
        let mut data = [0; 16];
        T::write_int128(&mut data, value, bytes);

        self.write_all(&data[0..bytes])
    }

    #[inline]
    fn write_f32<T: Endianness>(&mut self, value: f32) -> Result<(), std::io::Error> {
        let mut data = [0; 4];
        T::write_f32(&mut data, value);

        self.write_all(&data)
    }

    #[inline]
    fn write_f64<T: Endianness>(&mut self, value: f64) -> Result<(), std::io::Error> {
        let mut data = [0; 8];
        T::write_f64(&mut data, value);

        self.write_all(&data)
    }

    #[inline]
    fn write_framed<T: Endianness>(&mut self, value: &[u8]) -> Result<(), std::io::Error> {
        let length = value.len() as u32;

        self.write_u32::<T>(length)?;
        self.write_all(value)?;
        Ok(())
    }
}

impl<T> BiteWriteExpandedExt for T where T: Write + ?Sized {}
