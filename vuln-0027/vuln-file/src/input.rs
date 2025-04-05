#![doc= " Exposes traits that help reading data at the bit level with low overhead."]
#![doc= ""]
#![doc= " The traits in this module deal with reading bytes (with a given endianness)"]
#![doc= " from a buffered reader, in such a way that still allows efficient"]
#![doc= " checksumming of the data read. There is also a bitstream which is used"]
#![doc= " internally to read the bitstream."]

use std::io;

#[doc= " Provides convenience methods to make input less cumbersome."]
pub(crate) trait ReadBytes {
    #[doc= " Reads a single byte, failing on EOF."]
    fn read_u8(&mut self) -> io::Result<u8>;
    #[doc= " Reads a single byte, not failing on EOF."]
    fn read_u8_or_eof(&mut self) -> io::Result<Option<u8>>;
    #[doc= " Reads until the provided buffer is full."]
    fn read_into(&mut self, buffer: &mut [u8]) -> io::Result<()>;
    #[doc= " Skips over the specified number of bytes."]
    #[doc= ""]
    #[doc= " For a buffered reader, this can help a lot by just bumping a pointer."]
    fn skip(&mut self, amount: u32) -> io::Result<()>;

    #[doc= " Reads two bytes and interprets them as a big-endian 16-bit unsigned integer."]
    fn read_be_u16(&mut self) -> io::Result<u16> {
        let b0 = try!(self.read_u8()) as u16;
        let b1 = try!(self.read_u8()) as u16;
        Ok(b0 << 8 | b1)
    }

    #[doc= " Reads two bytes and interprets them as a big-endian 16-bit unsigned integer."]
    fn read_be_u16_or_eof(&mut self) -> io::Result<Option<u16>> {
        if let Some(b0) = try!(self.read_u8_or_eof()) {
            if let Some(b1) = try!(self.read_u8_or_eof()) {
                return Ok(Some((b0 as u16) << 8 | (b1 as u16)));
            }
        }
        Ok(None)
    }

    #[doc= " Reads three bytes and interprets them as a big-endian 24-bit unsigned integer."]
    fn read_be_u24(&mut self) -> io::Result<u32> {
        let b0 = try!(self.read_u8()) as u32;
        let b1 = try!(self.read_u8()) as u32;
        let b2 = try!(self.read_u8()) as u32;
        Ok(b0 << 16 | b1 << 8 | b2)
    }

    #[doc= " Reads four bytes and interprets them as a big-endian 32-bit unsigned integer."]
    fn read_be_u32(&mut self) -> io::Result<u32> {
        let b0 = try!(self.read_u8()) as u32;
        let b1 = try!(self.read_u8()) as u32;
        let b2 = try!(self.read_u8()) as u32;
        let b3 = try!(self.read_u8()) as u32;
        Ok(b0 << 24 | b1 << 16 | b2 << 8 | b3)
    }

    #[doc= " Reads four bytes and interprets them as a little-endian 32-bit unsigned integer."]
    fn read_le_u32(&mut self) -> io::Result<u32> {
        let b0 = try!(self.read_u8()) as u32;
        let b1 = try!(self.read_u8()) as u32;
        let b2 = try!(self.read_u8()) as u32;
        let b3 = try!(self.read_u8()) as u32;
        Ok(b3 << 24 | b2 << 16 | b1 << 8 | b0)
    }
}

impl<'r, R: ReadBytes> ReadBytes for &'r mut R {
    #[inline(always)]
    fn read_u8(&mut self) -> io::Result<u8> {
        (*self).read_u8()
    }

    fn read_u8_or_eof(&mut self) -> io::Result<Option<u8>> {
        (*self).read_u8_or_eof()
    }

    fn read_into(&mut self, buffer: &mut [u8]) -> io::Result<()> {
        (*self).read_into(buffer)
    }

    fn skip(&mut self, amount: u32) -> io::Result<()> {
        (*self).skip(amount)
    }
}

#[doc= " Left shift that does not panic when shifting by the integer width."]
#[inline(always)]
fn shift_left(x: u8, shift: u32) -> u8 {
    debug_assert!(shift <= 8);
    ((x as u32) << shift) as u8
}

#[doc= " Right shift that does not panic when shifting by the integer width."]
#[inline(always)]
fn shift_right(x: u8, shift: u32) -> u8 {
    debug_assert!(shift <= 8);
    ((x as u32) >> shift) as u8
}

#[doc= " Wraps a `Reader` to facilitate reading that is not byte-aligned."]
pub(crate) struct Bitstream<R: ReadBytes> {
    #[doc= " The source where bits are read from."]
    reader: R,
    #[doc= " Data read from the reader, but not yet fully consumed."]
    data: u8,
    #[doc= " The number of bits of `data` that have not been consumed."]
    bits_left: u32,
}

impl<R: ReadBytes> Bitstream<R> {
    #[doc= " Wraps the reader with a reader that facilitates reading individual bits."]
    pub(crate) fn new(reader: R) -> Bitstream<R> {
        Bitstream {
            reader: reader,
            data: 0,
            bits_left: 0,
        }
    }

    #[doc= " Generates a bitmask with 1s in the `bits` most significant bits."]
    #[inline(always)]
    fn mask_u8(bits: u32) -> u8 {
        debug_assert!(bits <= 8);
        shift_left(0xff, 8 - bits)
    }

    #[doc= " Reads a single bit."]
    #[doc= ""]
    #[doc= " Reading a single bit can be done more efficiently than reading"]
    #[doc= " more than one bit, because a bit never straddles a byte boundary."]
    #[inline(always)]
    pub(crate) fn read_bit(&mut self) -> io::Result<bool> {
        let result = if self.bits_left == 0 {
            let fresh_byte = try!(self.reader.read_u8());
            self.data = fresh_byte << 1;
            self.bits_left = 7;
            fresh_byte & 0b1000_0000
        } else {
            let bit = self.data & 0b1000_0000;
            self.data = self.data << 1;
            self.bits_left = self.bits_left - 1;
            bit
        };
        Ok(result != 0)
    }

    #[doc= " Reads bits until a 1 is read, and returns the number of zeros read."]
    #[doc= ""]
    #[doc= " Because the reader buffers a byte internally, reading unary can be done"]
    #[doc= " more efficiently than by just reading bit by bit."]
    #[inline(always)]
    pub(crate) fn read_unary(&mut self) -> io::Result<u32> {
        let mut n = self.data.leading_zeros();
        if n < self.bits_left {
            self.data = self.data << (n + 1);
            self.bits_left = self.bits_left - (n + 1);
        } else {
            n = self.bits_left;
            loop {
                let fresh_byte = try!(self.reader.read_u8());
                let zeros = fresh_byte.leading_zeros();
                n = n + zeros;
                if zeros < 8 {
                    self.bits_left = 8 - (zeros + 1);
                    self.data = shift_left(fresh_byte, zeros + 1);
                    break;
                }
            }
        }
        Ok(n)
    }

    #[doc= " Reads at most eight bits."]
    #[inline(always)]
    pub(crate) fn read_leq_u8(&mut self, bits: u32) -> io::Result<u8> {
        debug_assert!(bits <= 8);
        let result = if self.bits_left < bits {
            let msb = self.data;
            self.data = try!(self.reader.read_u8());
            let lsb = (self.data & Bitstream::<R>::mask_u8(bits - self.bits_left)) >> self.bits_left;
            self.data = shift_left(self.data, bits - self.bits_left);
            self.bits_left = 8 - (bits - self.bits_left);
            msb | lsb
        } else {
            let result = self.data & Bitstream::<R>::mask_u8(bits);
            self.data = self.data << bits;
            self.bits_left = self.bits_left - bits;
            result
        };
        debug_assert!(self.bits_left < 8);
        debug_assert_eq!(self.data & !Bitstream::<R>::mask_u8(self.bits_left), 0u8);
        Ok(shift_right(result, 8 - bits))
    }

    #[doc= " Read n bits, where 8 < n <= 16."]
    #[inline(always)]
    pub(crate) fn read_gt_u8_leq_u16(&mut self, bits: u32) -> io::Result<u32> {
        debug_assert!((8 < bits) && (bits <= 16));
        let mask_msb = 0xffffffff << (bits - self.bits_left);
        let msb = ((self.data as u32) << (bits - 8)) & mask_msb;
        let bits_to_read = bits - self.bits_left;
        let fresh_byte = try!(self.reader.read_u8()) as u32;
        let lsb = if bits_to_read >= 8 {
            fresh_byte << (bits_to_read - 8)
        } else {
            fresh_byte >> (8 - bits_to_read)
        };
        let combined = msb | lsb;
        let result = if bits_to_read <= 8 {
            self.bits_left = 8 - bits_to_read;
            self.data = fresh_byte.wrapping_shl(8 - self.bits_left) as u8;
            combined
        } else {
            let fresher_byte = try!(self.reader.read_u8()) as u32;
            let lsb = fresher_byte >> (16 - bits_to_read);
            self.bits_left = 16 - bits_to_read;
            self.data = fresher_byte.wrapping_shl(8 - self.bits_left) as u8;
            combined | lsb
        };
        Ok(result)
    }

    #[doc= " Reads at most 16 bits."]
    #[inline(always)]
    pub(crate) fn read_leq_u16(&mut self, bits: u32) -> io::Result<u16> {
        debug_assert!(bits <= 16);
        if bits <= 8 {
            let result = try!(self.read_leq_u8(bits));
            Ok(result as u16)
        } else {
            let msb = try!(self.read_leq_u8(8)) as u16;
            let lsb = try!(self.read_leq_u8(bits - 8)) as u16;
            Ok((msb << (bits - 8)) | lsb)
        }
    }

    #[doc= " Reads at most 32 bits."]
    #[inline(always)]
    pub(crate) fn read_leq_u32(&mut self, bits: u32) -> io::Result<u32> {
        debug_assert!(bits <= 32);
        if bits <= 16 {
            let result = try!(self.read_leq_u16(bits));
            Ok(result as u32)
        } else {
            let msb = try!(self.read_leq_u16(16)) as u32;
            let lsb = try!(self.read_leq_u16(bits - 16)) as u32;
            Ok((msb << (bits - 16)) | lsb)
        }
    }
}
