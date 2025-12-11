//! A module that allows to read and write bgzip files directly, as well as modify bgzip blocks.

use std::cmp::min;
use std::fmt::{self, Debug, Display, Formatter};
use std::io::{self, ErrorKind, Read, Write};
use std::time::Duration;

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use flate2::write::{DeflateDecoder, DeflateEncoder};

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
            EndOfStream => {
                io::Error::new(ErrorKind::UnexpectedEof, "EOF: Failed to read bgzip block")
            }
            Corrupted(s) => io::Error::new(
                ErrorKind::InvalidData,
                format!("Corrupted bgzip block: {}", s),
            ),
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
        return Err(BlockError::Corrupted(
            "bgzip block has an invalid header".to_string(),
        ));
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
                return Err(BlockError::Corrupted(
                    "bgzip block has an invalid header".to_string(),
                ));
            }
            return Ok(as_u16(extra_fields, i + 4));
        }
        i += 4 + subfield_len as usize;
    }
    Err(BlockError::Corrupted(
        "bgzip block has an invalid header".to_string(),
    ))
}

/// Biggest possible size of the compressed and uncompressed block (`= 65536`).
pub const MAX_BLOCK_SIZE: usize = 65536;

const HEADER_SIZE: usize = 12;
const MIN_EXTRA_SIZE: usize = 6;
const FOOTER_SIZE: usize = 8;

/// Biggest possible length of the compressed data (excluding header + footer).
/// Equal to [MAX_BLOCK_SIZE](constant.MAX_BLOCK_SIZE.html) `- 26 = 65510`.
pub const MAX_COMPRESSED_SIZE: usize = MAX_BLOCK_SIZE - HEADER_SIZE - MIN_EXTRA_SIZE - FOOTER_SIZE;

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
        // Initialize vectors so that we do not have problems with uninitialized memory.
        let mut uncompressed = vec![0; MAX_BLOCK_SIZE];
        uncompressed.clear();
        let mut compressed = vec![0; MAX_COMPRESSED_SIZE + FOOTER_SIZE];
        compressed.clear();

        Self {
            uncompressed,
            compressed,
            buffer: Vec::new(),
            offset: None,
        }
    }
    /// Returns the size of the compressed data. If the block was not compressed, the function
    /// returns zero. Note, that the compressed size does not include
    /// header and footer of the bgzip block.
    pub fn compressed_size(&self) -> u32 {
        self.compressed.len().saturating_sub(FOOTER_SIZE) as u32
    }

    /// Reads the compressed contents from `stream`. Panics if the block is non-empty
    /// (consider using [reset](#method.reset)).
    pub fn load<R: Read>(&mut self, offset: Option<u64>, stream: &mut R) -> Result<(), BlockError> {
        assert!(
            self.compressed.is_empty() && self.uncompressed.is_empty(),
            "Cannot load into a non-empty block"
        );
        self.offset = offset;

        let extra_len = {
            self.buffer.resize(HEADER_SIZE + MIN_EXTRA_SIZE, 0);
            match stream.read_exact(&mut self.buffer) {
                Ok(()) => {}
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
        if block_size > MAX_BLOCK_SIZE || block_size < HEADER_SIZE + MIN_EXTRA_SIZE {
            return Err(BlockError::Corrupted(format!(
                "Block size {} > {} or < {}",
                block_size,
                MAX_BLOCK_SIZE,
                HEADER_SIZE + MIN_EXTRA_SIZE
            )));
        }

        unsafe {
            // Include footer in self.compressed to read footer in one go.
            self.compressed
                .set_len(block_size - HEADER_SIZE - MIN_EXTRA_SIZE);
        }
        stream.read_exact(&mut self.compressed)?;
        Ok(())
    }
}
