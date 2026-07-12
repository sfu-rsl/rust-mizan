use crate::bite::endian::Endianness;
use {std, std::io::Read};
pub trait BiteReadExpandedExt: Read {
    #[inline]
    fn read_u32<T: Endianness>(&mut self) -> Result<u32, std::io::Error> {
        let mut data = [0; 4];
        self.read_exact(&mut data)?;

        let v = T::read_u32(&data);
        Ok(v)
    }

    #[inline]
    fn read_framed_max<T: Endianness>(
        &mut self,
        maximum: usize,
    ) -> Result<Vec<u8>, std::io::Error> {
        let length = match self.read_u32::<T>()? as usize {
            x if x <= maximum => x,
            _ => return Err(std::io::ErrorKind::InvalidData.into()),
        };

        unsafe {
            let mut data = Vec::with_capacity(length);
            let slice = std::slice::from_raw_parts_mut(data.as_mut_ptr(), length);

            self.read_exact(slice)?;
            data.set_len(length);

            Ok(data)
        }
    }
}

impl<T> BiteReadExpandedExt for T where T: std::io::Read + ?Sized {}
