//! A module that allows to read and write bgzip files directly, as well as modify bgzip blocks.

use std::io::{self, Read, Write, ErrorKind};
use std::cmp::min;
use std::fmt::{self, Display, Debug, Formatter};
use std::time::Duration;

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use flate2::write::{DeflateDecoder, DeflateEncoder};

pub mod read;
pub mod write;

/// Error produced while reading or decompressing a bgzip block.
///
/// # Variants
///
/// * `EndOfStream` - failed to read a block because the stream has ended.
/// * `Corrupted(s)` - the block has incorrect header or contents.
/// `s` contains additional information about the problem.
/// * `IoError(e)` - the stream raised `io::Error`.
pub enum BlockError {
    EndOfStream,
    Corrupted(String),
    IoError(io::Error),
}

impl From<io::Error> for BlockError {
    fn from(e: io::Error) -> BlockError {
        BlockError::IoError(e)
    }
}

impl Into<io::Error> for BlockError {
    fn into(self) -> io::Error {
        use BlockError::*;
        match self {
            EndOfStream => io::Error::new(ErrorKind::UnexpectedEof,
                "EOF: Failed to read bgzip block"),
            Corrupted(s) => io::Error::new(ErrorKind::InvalidData,
                format!("Corrupted bgzip block: {}", s)),
            IoError(e) => e,
        }
    }
}

impl Display for BlockError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        use BlockError::*;
        match self {
            EndOfStream => write!(f, "EOF: Failed to read bgzip block"),
            Corrupted(s) => write!(f, "Corrupted bgzip block: {}", s),
            IoError(e) => write!(f, "{}", e),
        }
    }
}

impl Debug for BlockError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        Display::fmt(self, f)
    }
}

fn as_u16(buffer: &[u8], start: usize) -> u16 {
    buffer[start] as u16 + ((buffer[start + 1] as u16) << 8)
}

/// Analyzes first 12 bytes of a block.
/// Returns the total length of extra subfields (XLEN).
fn analyze_header(header: &[u8]) -> Result<u16, BlockError> {
    if header[0] != 31 || header[1] != 139 || header[2] != 8 || header[3] != 4 {
        return Err(BlockError::Corrupted("bgzip block has an invalid header".to_string()));
    }
    Ok(as_u16(header, 10))
}

/// Analyzes extra fields following the header.
/// Returns total block size - 1 (BSIZE).
fn analyze_extra_fields(extra_fields: &[u8]) -> Result<u16, BlockError> {
    let mut i = 0;
    while i + 3 < extra_fields.len() {
        let subfield_id1 = extra_fields[i];
        let subfield_id2 = extra_fields[i + 1];
        let subfield_len = as_u16(extra_fields, i + 2);
        if subfield_id1 == 66 && subfield_id2 == 67 && subfield_len == 2 {
            if subfield_len != 2 || i + 5 >= extra_fields.len() {
                return Err(BlockError::Corrupted("bgzip block has an invalid header"
                    .to_string()));
            }
            return Ok(as_u16(extra_fields, i + 4));
        }
        i += 4 + subfield_len as usize;
    }
    Err(BlockError::Corrupted("bgzip block has an invalid header".to_string()))
}

/// Enum that describes the block state.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BlockState {
    /// Block is empty and contains no information.
    Empty,
    /// Block contains only uncompressed data.
    Uncompressed,
    /// Block contains only compressed data.
    Compressed,
    /// Block contains both uncompressed data and its compressed representation.
    Full,
}

impl BlockState {
    /// Returns `true` if the block contains uncompressed data.
    pub fn contains_uncompressed(self) -> bool {
        match self {
            BlockState::Full | BlockState::Uncompressed => true,
            _ => false,
        }
    }

    /// Returns `true` if the block contains compressed data.
    pub fn contains_compressed(self) -> bool {
        match self {
            BlockState::Full | BlockState::Compressed => true,
            _ => false,
        }
    }
}

/// Biggest possible size of the compressed and uncompressed block (`= 65536`).
pub const MAX_BLOCK_SIZE: usize = 65536;

const HEADER_SIZE: usize = 12;
const MIN_EXTRA_SIZE: usize = 6;
const FOOTER_SIZE: usize = 8;
// WRAPPER_SIZE = 26
const WRAPPER_SIZE: usize = HEADER_SIZE + MIN_EXTRA_SIZE + FOOTER_SIZE;

/// Biggest possible length of the compressed data (excluding header + footer).
/// Equal to [MAX_BLOCK_SIZE](constant.MAX_BLOCK_SIZE.html) `- 26 = 65510`.
pub const MAX_COMPRESSED_SIZE: usize = MAX_BLOCK_SIZE - WRAPPER_SIZE;

/// A bgzip block, that can contain compressed, uncompressed data, or both.
///
/// You can extend uncompressed data using [extend_contents](#method.extend_contents), and
/// and then compress the block using [compress](#method.compress).
#[derive(Clone)]
pub struct Block {
    // Uncompressed contents, max size = [MAX_BLOCK_SIZE](constant.MAX_BLOCK_SIZE.html).
    uncompressed: Vec<u8>,
    // Compressed contents + footer (empty if uncompressed),
    // max size = `MAX_COMPRESSED_SIZE + FOOTER_SIZE`.
    compressed: Vec<u8>,

    // Buffer used to read the header.
    buffer: Vec<u8>,
    offset: Option<u64>,
}

impl Block {
    /// Creates an empty block.
    pub fn new() -> Self {
        Self {
            uncompressed: Vec::with_capacity(MAX_BLOCK_SIZE),
            compressed: Vec::with_capacity(MAX_COMPRESSED_SIZE + FOOTER_SIZE),
            buffer: Vec::new(),
            offset: None,
        }
    }

    /// Resets a block (clears both compressed and uncompressed data).
    pub fn reset(&mut self) {
        self.uncompressed.clear();
        self.compressed.clear();
        self.offset = None;
    }

    /// Resets compressed data, if present. This function is needed if you want to
    /// [update uncompressed contents](#method.extend_contents).
    pub fn reset_compression(&mut self) {
        self.compressed.clear();
    }

    /// Makes a block a compressed block with empty contents. It is different from
    /// [reset](#method.reset) as the block will have valid compressed data and can be written.
    pub fn make_empty(&mut self) {
        self.uncompressed.clear();
        self.compressed.clear();
        self.compressed.extend(&[3, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    }

    /// Extends uncompressed contents and returns the number of consumed bytes. The only case when
    /// the number of consumed bytes is less then the size of the `buf` is when the content size
    /// reaches maximum size [MAX_BLOCK_SIZE](constant.MAX_BLOCK_SIZE.html).
    /// However, it is not recommended to fill the
    /// contents completely to [MAX_BLOCK_SIZE](constant.MAX_BLOCK_SIZE.html)
    ///  as the compressed size may become too big
    /// (bigger than [MAX_COMPRESSED_SIZE](constant.MAX_COMPRESSED_SIZE.html)).
    ///
    /// This function panics if the block contains compressed data
    /// (see [reset_compression](#method.reset_compression)).
    pub fn extend_contents(&mut self, buf: &[u8]) -> usize {
        assert!(self.compressed.is_empty(), "Cannot update contents, as the block was compressed. \
            Consider using reset_compression()");
        let consume_bytes = min(buf.len(), MAX_BLOCK_SIZE - self.uncompressed.len());
        self.uncompressed.extend(&buf[..consume_bytes]);
        consume_bytes
    }

    /// Returns the size of the uncompressed data
    /// (this works even if the block was not decompressed).
    pub fn uncompressed_size(&self) -> u32 {
        if !self.uncompressed.is_empty() {
            self.uncompressed.len() as u32
        } else if !self.compressed.is_empty() {
            (&self.compressed[self.compressed.len() - 4..]).read_u32::<LittleEndian>().unwrap()
        } else {
            0
        }
    }

    /// Returns the size of the compressed data. If the block was not compressed, the function
    /// returns zero. Note, that the compressed size does not include
    /// header and footer of the bgzip block.
    pub fn compressed_size(&self) -> u32 {
        self.compressed.len().saturating_sub(FOOTER_SIZE) as u32
    }

    /// Returns the size of the block (sum size of compressed data, header and footer).
    /// Returns None if the block was not compressed yet.
    pub fn block_size(&self) -> Option<u32> {
        if self.compressed.is_empty() {
            None
        } else {
            Some((self.compressed.len() + HEADER_SIZE + MIN_EXTRA_SIZE) as u32)
        }
    }

    /// Returns a block offset, if present.
    pub fn offset(&self) -> Option<u64> {
        self.offset
    }

    /// Returns the state of the block.
    pub fn state(&self) -> BlockState {
        match (self.uncompressed.is_empty(), self.compressed.is_empty()) {
            (true, true) => BlockState::Empty,
            (true, false) => BlockState::Uncompressed,
            (false, true) => BlockState::Compressed,
            (false, false) => BlockState::Full,
        }
    }

    /// Compresses block contents. Note that if the contents are empty, the function will compress
    /// them into a valid block (see [make_empty](#method.make_empty)).
    ///
    /// If the compressed size is bigger than
    /// [MAX_COMPRESSED_SIZE](constant.MAX_COMPRESSED_SIZE.html), the function returns
    /// `WriteZero`. Function panics if the block is already compressed.
    pub fn compress(&mut self, compression: flate2::Compression) -> io::Result<()> {
        assert!(self.compressed.is_empty(), "Cannot compress an already compressed block");
        if self.uncompressed.is_empty() {
            self.make_empty();
            return Ok(())
        }

        unsafe {
            self.compressed.set_len(MAX_COMPRESSED_SIZE);
        }
        let mut encoder = DeflateEncoder::new(&mut self.compressed[..], compression);
        encoder.write_all(&self.uncompressed)?;
        let remaining_size = encoder.finish()?.len();
        self.compressed.truncate(MAX_COMPRESSED_SIZE - remaining_size);

        let mut crc_hasher = crc32fast::Hasher::new();
        crc_hasher.update(&self.uncompressed);
        self.compressed.write_u32::<LittleEndian>(crc_hasher.finalize()).unwrap();
        self.compressed.write_u32::<LittleEndian>(self.uncompressed.len() as u32).unwrap();
        Ok(())
    }

    /// Writes the compressed block to `stream`. The function panics if the block was not compressed.
    pub fn dump<W: Write>(&self, stream: &mut W) -> io::Result<()> {
        assert!(!self.compressed.is_empty(), "Cannot write an uncompressed block");
        let block_size = self.block_size().expect("Block size should be defined already") - 1;
        let block_header: &[u8; 18] = &[
            31, 139,   8,   4,  // ID1, ID2, Compression method, Flags
             0,   0,   0,   0,  // Modification time
             0, 255,   6,   0,  // Extra flags, OS (255 = unknown), extra length (2 bytes)
            66,  67,   2,   0, // SI1, SI2, subfield len (2 bytes)
            block_size as u8, (block_size >> 8) as u8];
        stream.write_all(block_header)?;
        stream.write_all(&self.compressed)
    }

    /// Reads the compressed contents from `stream`. Panics if the block is non-empty
    /// (consider using [reset](#method.reset)).
    pub fn load<R: Read>(&mut self, offset: Option<u64>, stream: &mut R) -> Result<(), BlockError> {
        assert!(self.compressed.is_empty() && self.uncompressed.is_empty(),
            "Cannot load into a non-empty block");
        self.offset = offset;

        let extra_len = {
            self.buffer.resize(HEADER_SIZE + MIN_EXTRA_SIZE, 0);
            match stream.read_exact(&mut self.buffer) {
                Ok(()) => {},
                Err(e) => {
                    if e.kind() == ErrorKind::UnexpectedEof {
                        return Err(BlockError::EndOfStream);
                    } else {
                        return Err(BlockError::from(e));
                    }
                }
            }
            analyze_header(&self.buffer)? as usize
        };

        if extra_len > MIN_EXTRA_SIZE {
            self.buffer.resize(HEADER_SIZE + extra_len, 0);
            stream.read_exact(&mut self.buffer[HEADER_SIZE..])?;
        }
        let block_size = analyze_extra_fields(&self.buffer[HEADER_SIZE..])? as usize + 1;
        if block_size > MAX_BLOCK_SIZE {
            return Err(BlockError::Corrupted(
                format!("Block size {} > {}", block_size, MAX_BLOCK_SIZE)));
        }

        unsafe {
            // Include footer in self.compressed to read footer in one go.
            self.compressed.set_len(block_size - HEADER_SIZE - MIN_EXTRA_SIZE);
        }
        stream.read_exact(&mut self.compressed)?;
        Ok(())
    }

    /// Decompresses block contents. This function panics if the block was already decompressed or
    /// if the block is empty.
    pub fn decompress(&mut self) -> Result<(), BlockError> {
        assert!(!self.compressed.is_empty(), "Cannot decompress an empty block");
        assert!(self.uncompressed.is_empty(), "Cannot decompress an already decompressed block");

        let compressed_size = self.compressed.len();
        let exp_uncompressed_size = (&self.compressed[compressed_size - 4..])
            .read_u32::<LittleEndian>().unwrap() as usize;
        unsafe {
            self.uncompressed.set_len(exp_uncompressed_size);
        }
        {
            let mut decoder = DeflateDecoder::new(&mut self.uncompressed[..]);
            match decoder.write_all(&self.compressed[..compressed_size - FOOTER_SIZE]) {
                Ok(()) => {},
                Err(ref e) if e.kind() == ErrorKind::WriteZero => return Err(BlockError::Corrupted(
                    format!("Could not decompress block contents: \
                    uncompressed size is bigger than expected {}", exp_uncompressed_size))),
                Err(e) => return Err(BlockError::Corrupted(
                    format!("Could not decompress block contents: {:?}", e))),
            }
            let remaining_contents = match decoder.finish() {
                Ok(remaining_buf) => remaining_buf.len(),
                Err(ref e) if e.kind() == ErrorKind::WriteZero => return Err(BlockError::Corrupted(
                    format!("Could not decompress block contents: \
                    uncompressed size is bigger than expected {}", exp_uncompressed_size))),
                Err(e) => return Err(BlockError::Corrupted(
                    format!("Could not decompress block contents: {:?}", e))),
            };
            if remaining_contents != 0 {
                return Err(BlockError::Corrupted(
                    format!("Uncompressed sizes do not match: expected {}, observed {}",
                    exp_uncompressed_size, exp_uncompressed_size - remaining_contents)));
            }
        }

        let exp_crc32 = self.crc32();
        let mut hasher = crc32fast::Hasher::new();
        hasher.update(&self.uncompressed);
        let obs_crc32 = hasher.finalize();
        if obs_crc32 != exp_crc32 {
            return Err(BlockError::Corrupted(
                format!("CRC do not match: expected {}, observed {}", exp_crc32, obs_crc32)));
        }
        Ok(())
    }

    /// Access uncompressed data.
    pub fn uncompressed_data(&self) -> &[u8] {
        &self.uncompressed
    }

    /// Access compressed data (without header and footer). Panics if the block is not compressed.
    pub fn compressed_data(&self) -> &[u8] {
        &self.compressed[..self.compressed.len() - FOOTER_SIZE]
    }

    /// Returns CRC32 hash of the uncompressed data. Panics if the block is not compressed.
    pub fn crc32(&self) -> u32 {
        (&self.compressed[self.compressed.len() - 8..self.compressed.len() - 4])
            .read_u32::<LittleEndian>().unwrap()
    }

    /// Truncates uncompressed contents on `first_size`,
    /// writes the remaining data into `second_part`, and
    /// returns the number of bytes written into `second_part`.
    ///
    /// Panics, if the `second_part` is not big enough.
    pub fn split_contents(&mut self, first_size: usize, second_part: &mut [u8]) -> usize {
        assert!(self.uncompressed.len() >= first_size,
            "Cannot split a block with: size {} < {}", self.uncompressed.len(), first_size);
        assert!(self.compressed.is_empty(), "Cannot split an already compressed block");

        let second_size = self.uncompressed.len() - first_size;
        second_part[..second_size].copy_from_slice(&self.uncompressed[first_size..]);
        self.uncompressed.truncate(first_size);
        second_size
    }

    /// The function trims the contents of `self`, and returns the second half in a new `Block`.
    /// The function panics if the initial block was compressed
    /// or its uncompressed size is less than 2 bytes.
    pub fn split_into_two(&mut self) -> Block {
        assert!(self.uncompressed.len() > 1, "Cannot split a block with size < 2 bytes");
        assert!(self.compressed.is_empty(), "Cannot split an already compressed block");

        let first_half_size = self.uncompressed.len() / 2;
        let mut second_half = Block::new();
        assert!(second_half.extend_contents(&self.uncompressed[first_half_size..])
            == self.uncompressed.len() - first_half_size);
        self.uncompressed.truncate(first_half_size);
        second_half
    }
}

struct ObjectPool<T> {
    objects: Vec<T>,
    constructor: Box<dyn Fn() -> T>,
    taken: u64,
    brought: u64,
}

impl<T> ObjectPool<T> {
    pub fn new<F: 'static + Fn() -> T>(constructor: F) -> Self {
        Self {
            objects: vec![],
            constructor: Box::new(constructor),
            taken: 0,
            brought: 0,
        }
    }

    pub fn take(&mut self) -> T {
        self.taken += 1;
        match self.objects.pop() {
            Some(object) => object,
            None => (self.constructor)(),
        }
    }

    pub fn bring(&mut self, object: T) {
        self.brought += 1;
        self.objects.push(object);
    }
}

const SLEEP_TIME: Duration = Duration::from_nanos(50);
const TIMEOUT: Duration = Duration::from_secs(10);

pub use read::{SeekReader, ConsecutiveReader, ReadBgzip};
pub use write::{Writer, WriterBuilder};
