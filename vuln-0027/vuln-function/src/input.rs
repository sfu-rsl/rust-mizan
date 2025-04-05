// Claxon -- A FLAC decoding library in Rust
// Copyright 2014 Ruud van Asseldonk
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// A copy of the License has been included in the root of the repository.

//! Exposes traits that help reading data at the bit level with low overhead.
//!
//! The traits in this module deal with reading bytes (with a given endianness)
//! from a buffered reader, in such a way that still allows efficient
//! checksumming of the data read. There is also a bitstream which is used
//! internally to read the bitstream.

use std::io;

/// Provides convenience methods to make input less cumbersome.
pub trait ReadBytes {
    /// Reads a single byte, failing on EOF.
    fn read_u8(&mut self) -> io::Result<u8>;

    /// Reads a single byte, not failing on EOF.
    fn read_u8_or_eof(&mut self) -> io::Result<Option<u8>>;

    /// Reads until the provided buffer is full.
    fn read_into(&mut self, buffer: &mut [u8]) -> io::Result<()>;

    /// Skips over the specified number of bytes.
    ///
    /// For a buffered reader, this can help a lot by just bumping a pointer.
    fn skip(&mut self, amount: u32) -> io::Result<()>;

    /// Reads two bytes and interprets them as a big-endian 16-bit unsigned integer.
    fn read_be_u16(&mut self) -> io::Result<u16> {
        let b0 = try!(self.read_u8()) as u16;
        let b1 = try!(self.read_u8()) as u16;
        Ok(b0 << 8 | b1)
    }

    /// Reads two bytes and interprets them as a big-endian 16-bit unsigned integer.
    fn read_be_u16_or_eof(&mut self) -> io::Result<Option<u16>> {
        if let Some(b0) = try!(self.read_u8_or_eof()) {
            if let Some(b1) = try!(self.read_u8_or_eof()) {
                return Ok(Some((b0 as u16) << 8 | (b1 as u16)));
            }
        }
        Ok(None)
    }

    /// Reads three bytes and interprets them as a big-endian 24-bit unsigned integer.
    fn read_be_u24(&mut self) -> io::Result<u32> {
        let b0 = try!(self.read_u8()) as u32;
        let b1 = try!(self.read_u8()) as u32;
        let b2 = try!(self.read_u8()) as u32;
        Ok(b0 << 16 | b1 << 8 | b2)
    }

    /// Reads four bytes and interprets them as a big-endian 32-bit unsigned integer.
    fn read_be_u32(&mut self) -> io::Result<u32> {
        let b0 = try!(self.read_u8()) as u32;
        let b1 = try!(self.read_u8()) as u32;
        let b2 = try!(self.read_u8()) as u32;
        let b3 = try!(self.read_u8()) as u32;
        Ok(b0 << 24 | b1 << 16 | b2 << 8 | b3)
    }

    /// Reads four bytes and interprets them as a little-endian 32-bit unsigned integer.
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

/// Left shift that does not panic when shifting by the integer width.
#[inline(always)]
fn shift_left(x: u8, shift: u32) -> u8 {
    debug_assert!(shift <= 8);

    // We cannot shift a u8 by 8 or more, because Rust panics when shifting by
    // the integer width. But we can definitely shift a u32.
    ((x as u32) << shift) as u8
}

/// Right shift that does not panic when shifting by the integer width.
#[inline(always)]
fn shift_right(x: u8, shift: u32) -> u8 {
    debug_assert!(shift <= 8);

    // We cannot shift a u8 by 8 or more, because Rust panics when shifting by
    // the integer width. But we can definitely shift a u32.
    ((x as u32) >> shift) as u8
}

/// Wraps a `Reader` to facilitate reading that is not byte-aligned.
pub struct Bitstream<R: ReadBytes> {
    /// The source where bits are read from.
    reader: R,
    /// Data read from the reader, but not yet fully consumed.
    data: u8,
    /// The number of bits of `data` that have not been consumed.
    bits_left: u32,
}

impl<R: ReadBytes> Bitstream<R> {
    /// Wraps the reader with a reader that facilitates reading individual bits.
    pub fn new(reader: R) -> Bitstream<R> {
        Bitstream {
            reader: reader,
            data: 0,
            bits_left: 0,
        }
    }

    /// Generates a bitmask with 1s in the `bits` most significant bits.
    #[inline(always)]
    fn mask_u8(bits: u32) -> u8 {
        debug_assert!(bits <= 8);

        shift_left(0xff, 8 - bits)
    }

    /// Reads a single bit.
    ///
    /// Reading a single bit can be done more efficiently than reading
    /// more than one bit, because a bit never straddles a byte boundary.
    #[inline(always)]
    pub fn read_bit(&mut self) -> io::Result<bool> {

        // If no bits are left, we will need to read the next byte.
        let result = if self.bits_left == 0 {
            let fresh_byte = try!(self.reader.read_u8());

            // What remains later are the 7 least significant bits.
            self.data = fresh_byte << 1;
            self.bits_left = 7;

            // What we report is the most significant bit of the fresh byte.
            fresh_byte & 0b1000_0000
        } else {
            // Consume the most significant bit of the buffer byte.
            let bit = self.data & 0b1000_0000;
            self.data = self.data << 1;
            self.bits_left = self.bits_left - 1;
            bit
        };

        Ok(result != 0)
    }

    /// Reads bits until a 1 is read, and returns the number of zeros read.
    ///
    /// Because the reader buffers a byte internally, reading unary can be done
    /// more efficiently than by just reading bit by bit.
    #[inline(always)]
    pub fn read_unary(&mut self) -> io::Result<u32> {
        // Start initially with the number of zeros that are in the buffer byte
        // already (counting from the most significant bit).
        let mut n = self.data.leading_zeros();

        // If the number of zeros plus the one following it was not more than
        // the bytes left, then there is no need to look further.
        if n < self.bits_left {
            // Note: this shift never shifts by more than 7 places, because
            // bits_left is always at most 7 in between read calls, and the
            // least significant bit of the buffer byte is 0 in that case. So
            // we count either 8 zeros, or less than 7. In the former case we
            // would not have taken this branch, in the latter the shift below
            // is safe.
            self.data = self.data << (n + 1);
            self.bits_left = self.bits_left - (n + 1);
        } else {
            // We inspected more bits than available, so our count is incorrect,
            // and we need to look at the next byte.
            n = self.bits_left;

            // Continue reading bytes until we encounter a one.
            loop {
                let fresh_byte = try!(self.reader.read_u8());
                let zeros = fresh_byte.leading_zeros();
                n = n + zeros;
                if zeros < 8 {
                    // We consumed the zeros, plus the one following it.
                    self.bits_left = 8 - (zeros + 1);
                    self.data = shift_left(fresh_byte, zeros + 1);
                    break;
                }
            }
        }

        Ok(n)
    }

    /// Reads at most eight bits.
    #[inline(always)]
    pub fn read_leq_u8(&mut self, bits: u32) -> io::Result<u8> {
        // Of course we can read no more than 8 bits, but we do not want the
        // performance overhead of the assertion, so only do it in debug mode.
        debug_assert!(bits <= 8);

        // If not enough bits left, we will need to read the next byte.
        let result = if self.bits_left < bits {
            // Most significant bits are shifted to the right position already.
            let msb = self.data;

            // Read a single byte.
            self.data = try!(self.reader.read_u8());

            // From the next byte, we take the additional bits that we need.
            // Those start at the most significant bit, so we need to shift so
            // that it does not overlap with what we have already.
            let lsb = (self.data & Bitstream::<R>::mask_u8(bits - self.bits_left))
                >> self.bits_left;

            // Shift out the bits that we have consumed.
            self.data = shift_left(self.data, bits - self.bits_left);
            self.bits_left = 8 - (bits - self.bits_left);

            msb | lsb
        } else {
            let result = self.data & Bitstream::<R>::mask_u8(bits);

            // Shift out the bits that we have consumed.
            self.data = self.data << bits;
            self.bits_left = self.bits_left - bits;

            result
        };

        // If there are more than 8 bits left, we read too far.
        debug_assert!(self.bits_left < 8);

        // The least significant bits should be zero.
        debug_assert_eq!(self.data & !Bitstream::<R>::mask_u8(self.bits_left), 0u8);

        // The resulting data is padded with zeros in the least significant
        // bits, but we want to pad in the most significant bits, so shift.
        Ok(shift_right(result, 8 - bits))
    }

    /// Read n bits, where 8 < n <= 16.
    #[inline(always)]
    pub fn read_gt_u8_leq_u16(&mut self, bits: u32) -> io::Result<u32> {
        debug_assert!((8 < bits) && (bits <= 16));

        // The most significant bits of the current byte are valid. Shift them
        // by 2 so they become the most significant bits of the 10 bit number.
        let mask_msb = 0xffffffff << (bits - self.bits_left);
        let msb = ((self.data as u32) << (bits - 8)) & mask_msb;

        // Continue reading the next bits, because no matter how many bits were
        // still left, there were less than 10.
        let bits_to_read = bits - self.bits_left;
        let fresh_byte = try!(self.reader.read_u8()) as u32;
        let lsb = if bits_to_read >= 8 {
            fresh_byte << (bits_to_read - 8)
        } else {
            fresh_byte >> (8 - bits_to_read)
        };
        let combined = msb | lsb;

        let result = if bits_to_read <= 8 {
            // We have all bits already, update the internal state. If no
            // bits are left we might shift by 8 which is invalid, but in that
            // case the value is not used, so a masked shift is appropriate.
            self.bits_left = 8 - bits_to_read;
            self.data = fresh_byte.wrapping_shl(8 - self.bits_left) as u8;
            combined
        } else {
            // We need to read one more byte to get the final bits.
            let fresher_byte = try!(self.reader.read_u8()) as u32;
            let lsb = fresher_byte >> (16 - bits_to_read);

            // Update the reader state. The wrapping shift is appropriate for
            // the same reason as above.
            self.bits_left = 16 - bits_to_read;
            self.data = fresher_byte.wrapping_shl(8 - self.bits_left) as u8;

            combined | lsb
        };

        Ok(result)
    }

    /// Reads at most 16 bits.
    #[inline(always)]
    pub fn read_leq_u16(&mut self, bits: u32) -> io::Result<u16> {
        // As with read_leq_u8, this only makes sense if we read <= 16 bits.
        debug_assert!(bits <= 16);

        // Note: the following is not the most efficient implementation
        // possible, but it avoids duplicating the complexity of `read_leq_u8`.

        if bits <= 8 {
            let result = try!(self.read_leq_u8(bits));
            Ok(result as u16)
        } else {
            // First read the 8 most significant bits, then read what is left.
            let msb = try!(self.read_leq_u8(8)) as u16;
            let lsb = try!(self.read_leq_u8(bits - 8)) as u16;
            Ok((msb << (bits - 8)) | lsb)
        }
    }

    /// Reads at most 32 bits.
    #[inline(always)]
    pub fn read_leq_u32(&mut self, bits: u32) -> io::Result<u32> {
        // As with read_leq_u8, this only makes sense if we read <= 32 bits.
        debug_assert!(bits <= 32);

        // Note: the following is not the most efficient implementation
        // possible, but it avoids duplicating the complexity of `read_leq_u8`.

        if bits <= 16 {
            let result = try!(self.read_leq_u16(bits));
            Ok(result as u32)
        } else {
            // First read the 16 most significant bits, then read what is left.
            let msb = try!(self.read_leq_u16(16)) as u32;
            let lsb = try!(self.read_leq_u16(bits - 16)) as u32;
            Ok((msb << (bits - 16)) | lsb)
        }
    }
}
