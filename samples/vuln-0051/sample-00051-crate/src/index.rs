//! BAI index, virtual offset and bgzip chunks.

use std::io::{Result, Error, Read};
use std::io::ErrorKind::InvalidData;
use std::path::Path;
use std::fs::File;
use std::fmt::{self, Debug, Display, Formatter};
use std::result;
use std::collections::HashMap;
use std::cmp::{min, max};

use byteorder::{LittleEndian, ReadBytesExt};

/// Virtual offset. Represents `block_offset << 16 | contents_offset`, where
/// `block_offset` is `u48` and represents the offset in the bgzip file to the beginning of the
/// block (also known as `coffset` or `compressed_offset`).
///
/// `contents_offset` is `u16` and represents offset in the uncompressed data in a single block
/// (also known as `uoffset` or `uncompressed_offset`).
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct VirtualOffset(u64);

impl VirtualOffset {
    /// Construct virtual offset from raw value.
    pub fn from_raw(raw: u64) -> VirtualOffset {
        VirtualOffset(raw)
    }

    fn from_stream<R: Read>(stream: &mut R) -> Result<Self> {
        Ok(VirtualOffset(stream.read_u64::<LittleEndian>()?))
    }

    /// Constructs virtual offset from `block_offset` and `contents_offset`.
    pub fn new(block_offset: u64, contents_offset: u16) -> Self {
        VirtualOffset(block_offset << 16 | contents_offset as u64)
    }

    /// Returns the raw value.
    pub fn raw(&self) -> u64 {
        self.0
    }

    /// Returns the block offset. Represents the offset in the Bgzip file to the beginning of the block.
    pub fn block_offset(&self) -> u64 {
        self.0 >> 16
    }

    /// Represents the contents offset. Represents the offset into the uncompressed contents of the block.
    pub fn contents_offset(&self) -> u16 {
        self.0 as u16
    }

    /// Checks if the `self` is the same as `block_offset << 16 | contents_offset`.
    pub fn equal(&self, block_offset: u64, contents_offset: u16) -> bool {
        self.0 == (block_offset << 16 | contents_offset as u64)
    }

    /// Minimal possible offset, same as `VirtualOffset::from_raw(0)`.
    pub const MIN: VirtualOffset = VirtualOffset(0);
    /// Maximal possible offset, same as `VirtualOffset::from_raw(std::u64::MAX)`.
    pub const MAX: VirtualOffset = VirtualOffset(std::u64::MAX);
}

impl Display for VirtualOffset {
    fn fmt(&self, f: &mut Formatter) -> result::Result<(), fmt::Error> {
        write!(f, "c={},u={}", self.block_offset(), self.contents_offset())
    }
}

/// Chunk `[start-end)`, where `start` and `end` are [virtual offsets](struct.VirtualOffset.html).
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Chunk {
    start: VirtualOffset,
    end: VirtualOffset,
}

impl Chunk {
    /// Constructs a `Chunk` from two virtual offsets.
    pub fn new(start: VirtualOffset, end: VirtualOffset) -> Self {
        Chunk { start, end }
    }

    fn from_stream<R: Read>(stream: &mut R, check: bool) -> Result<Self> {
        let start = VirtualOffset::from_stream(stream)?;
        let end = VirtualOffset::from_stream(stream)?;
        if check && end <= start {
            Err(Error::new(InvalidData, format!("BAI chunk end < start ({}  <  {})", end, start)))
        } else {
            Ok(Chunk { start, end })
        }
    }

    /// Checks if two chunks intersect.
    pub fn intersect(&self, other: &Chunk) -> bool {
        self.start < other.end && other.start < self.end
    }

    /// Checks if two chunks intersect or one of the chunks goes right after another.
    pub fn can_combine(&self, other: &Chunk) -> bool {
        self.start <= other.end && other.start <= self.end
    }

    /// Combines two intersecting chunks. Panics if chunks do not intersect.
    pub fn combine(&self, other: &Chunk) -> Chunk {
        Chunk {
            start: min(self.start, other.start),
            end: max(self.end, other.end),
        }
    }

    /// Returns the start of the chunk.
    pub fn start(&self) -> VirtualOffset {
        self.start
    }

    /// Returns the end of the chunk.
    pub fn end(&self) -> VirtualOffset {
        self.end
    }
}

impl Debug for Chunk {
    fn fmt(&self, f: &mut Formatter) -> result::Result<(), fmt::Error> {
        write!(f, "{{{}__{}}}", self.start, self.end)
    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut Formatter) -> result::Result<(), fmt::Error> {
        write!(f, "{{{}__{}}}", self.start, self.end)
    }
}

/// Single bin that stores chunks within the BAM file.
#[derive(Clone)]
pub struct Bin {
    bin_id: u32,
    chunks: Vec<Chunk>,
}

impl Bin {
    fn from_stream<R: Read>(stream: &mut R) -> Result<Self> {
        let bin_id = stream.read_u32::<LittleEndian>()?;
        let n_chunks = stream.read_i32::<LittleEndian>()? as usize;
        let check_chunks = bin_id != SUMMARY_BIN;
        let mut chunks = Vec::with_capacity(n_chunks);
        for i in 0..n_chunks {
            chunks.push(Chunk::from_stream(stream, check_chunks)?);
            if check_chunks && i > 0 && chunks[i].start() < chunks[i - 1].end() {
                return Err(Error::new(InvalidData, format!("Invalid index: chunks are not sorted for bin {}", bin_id)));
            }
        }
        Ok(Bin { bin_id, chunks })
    }

    /// Returns the bin ID.
    pub fn bin_id(&self) -> u32 {
        self.bin_id
    }

    /// Returns all the chunks in the bin.
    pub fn chunks(&self) -> &[Chunk] {
        &self.chunks
    }
}

impl Display for Bin {
    fn fmt(&self, f: &mut Formatter) -> result::Result<(), fmt::Error> {
        write!(f, "Bin {}:  ", self.bin_id)?;
        for (i, chunk) in self.chunks.iter().enumerate() {
            write!(f, "{}{}", if i > 0 { ",  " } else { "" }, chunk)?;
        }
        Ok(())
    }
}

const WINDOW_SIZE: u32 = 16384;

/// Stores linear index: for each tiling 16384bp window it stores the smallest file offset of an alignment
/// that overlaps it.
#[derive(Clone)]
pub struct LinearIndex {
    /// each element stores the index and offset of the first interval with such offset.
    intervals: Vec<(u32, VirtualOffset)>,
}

impl LinearIndex {
    fn from_stream<R: Read>(stream: &mut R) -> Result<Self> {
        let n_intervals = stream.read_i32::<LittleEndian>()? as u32;
        let mut intervals = Vec::new();
        for i in 0..n_intervals {
            let offset = VirtualOffset::from_stream(stream)?;
            match intervals.last() {
                Some((_, prev_offset)) if *prev_offset == offset => {},
                _ => intervals.push((i, offset)),
            }
        }
        intervals.shrink_to_fit();
        Ok(LinearIndex { intervals })
    }

    /// Returns true if the linear index is empty.
    pub fn is_empty(&self) -> bool {
        self.intervals.is_empty()
    }

    /// Returns the first offset. Panics, if the index is empty.
    pub fn smallest_offset(&self) -> VirtualOffset {
        self.intervals[0].1
    }

    /// Returns a slice where each element represents
    /// a first window with a specific offset in form of `(index, offset)`.
    pub fn intervals(&self) -> &[(u32, VirtualOffset)] {
        &self.intervals
    }

    /// Retuns an offset *x*, such that for a region with genomic coordinates [start-end)
    /// we only need to visit chunks with end offset > *x*.
    pub fn min_end_offset(&self, start: i32) -> VirtualOffset {
        let start_index = if start < 0 {
            0
        } else {
            start as u32 / WINDOW_SIZE
        };
        match self.intervals.binary_search_by(|(index, _)| index.cmp(&start_index)) {
            Ok(i) => self.intervals[i].1,
            Err(0) => VirtualOffset::MIN,
            Err(i) => self.intervals[i - 1].1
        }
    }
}

impl Display for LinearIndex {
    fn fmt(&self, f: &mut Formatter) -> result::Result<(), fmt::Error> {
        for (i, (index, offset)) in self.intervals.iter().enumerate() {
            if i > 0 {
                write!(f, ";  ")?;
            }
            write!(f, "window {}: {}", index, offset)?;
        }
        Ok(())
    }
}

/// Index for a single reference sequence. Contains [bins](struct.Bin.html) and a linear index.
#[derive(Clone)]
pub struct Reference {
    bins: HashMap<u32, Bin>,
    linear_index: LinearIndex,
}

/// Per BAM specification, bin with `bin_id == SUMMARY_BIN` contains summary over the reference.
const SUMMARY_BIN: u32 = 37450;

impl Reference {
    fn from_stream<R: Read>(stream: &mut R) -> Result<Self> {
        let n_bins = stream.read_i32::<LittleEndian>()? as usize;
        let mut bins = HashMap::with_capacity(n_bins);
        for _ in 0..n_bins {
            let bin = Bin::from_stream(stream)?;
            bins.insert(bin.bin_id, bin);
        }

        let linear_index = LinearIndex::from_stream(stream)?;
        Ok(Reference { bins, linear_index })
    }

    /// Returns all bins for the reference.
    pub fn bins(&self) -> &HashMap<u32, Bin> {
        &self.bins
    }

    /// Returns linear index.
    pub fn linear_index(&self) -> &LinearIndex {
        &self.linear_index
    }

    /// Returns the maximal end offset. Panics, if the reference is empty.
    pub fn max_end_offset(&self) -> VirtualOffset {
        let mut max_offset = match self.bins.get(&0) {
            Some(bin) => bin.chunks[bin.chunks.len() - 1].end(),
            None => VirtualOffset::MIN,
        };

        let mut t = 0;
        for i in 0..5 {
            t += 1 << (i * 3);
            let next_t = t + (1 << (3 + i * 3));
            for bin_id in (t..next_t).rev() {
                if let Some(bin) = self.bins.get(&bin_id) {
                    max_offset = max(max_offset, bin.chunks[bin.chunks.len() - 1].end());
                    break;
                }
            }
        }
        max_offset
    }
}

impl Display for Reference {
    fn fmt(&self, f: &mut Formatter) -> result::Result<(), fmt::Error> {
        if !self.bins.is_empty() {
            writeln!(f, "    Bins:")?;
            for bin in self.bins.values() {
                writeln!(f, "        {}", bin)?;
            }
        }
        if !self.linear_index.is_empty() {
            writeln!(f, "    Linear index:")?;
            writeln!(f, "        {}", self.linear_index)?;
        }
        Ok(())
    }
}

/// BAI Index. Allows to get chunks in a bgzip file, that contain records from a specific genomic region.
#[derive(Clone)]
pub struct Index {
    references: Vec<Reference>,
    n_unmapped: Option<u64>,
}

impl Index {
    /// Loads index from stream.
    pub fn from_stream<R: Read>(mut stream: R) -> Result<Index> {
        let mut magic = [0_u8; 4];
        stream.read_exact(&mut magic)?;
        if magic != [b'B', b'A', b'I', 1] {
            return Err(Error::new(InvalidData, "Input is not in BAI format"));
        }

        let n_ref = stream.read_i32::<LittleEndian>()? as usize;
        let mut references = Vec::with_capacity(n_ref);
        for _ in 0..n_ref {
            references.push(Reference::from_stream(&mut stream)?);
        }
        let n_unmapped = stream.read_u64::<LittleEndian>().ok();
        Ok(Index { references, n_unmapped })
    }

    /// Loads index from path.
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Index> {
        let f = File::open(&path)?;
        Index::from_stream(f)
    }

    /// Fetches [chunks](struct.Chunk.html) of the BAM file that contain all records for a given region.
    pub fn fetch_chunks(&self, ref_id: u32, start: i32, end: i32) -> Vec<Chunk> {
        let mut chunks = Vec::new();
        let ref_id = ref_id as usize;

        let min_end_offset = self.references[ref_id].linear_index.min_end_offset(start);
        for bin_id in region_to_bins(start, end) {
            if let Some(bin) = self.references[ref_id].bins.get(&bin_id) {
                chunks.extend(bin.chunks.iter().filter(|chunk| chunk.end() > min_end_offset));
            }
        }
        let mut res = Vec::new();
        if chunks.is_empty() {
            return res;
        }

        chunks.sort();
        let mut curr = chunks[0].clone();
        for i in 1..chunks.len() {
            if !curr.can_combine(&chunks[i]) {
                res.push(curr);
                curr = chunks[i].clone();
            } else {
                curr = curr.combine(&chunks[i]);
            }
        }
        res.push(curr);
        res
    }

    /// Returns the offset to the start of the data, if the index is not empty.
    pub fn start_offset(&self) -> Option<VirtualOffset> {
        for (i, reference) in self.references.iter().enumerate() {
            if reference.linear_index.is_empty() {
                assert!(reference.bins.is_empty(), "BAI Index contiains bins for reference {}, but no linear index", i);
                continue;
            }
            return Some(reference.linear_index.smallest_offset());
        }
        None
    }

    /// Returns the offset of the end of all mapped records, if the index is not empty.
    /// Takes at most 37448 bins lookups.
    pub fn end_offset(&self) -> Option<VirtualOffset> {
        for reference in self.references.iter().rev() {
            if !reference.bins.is_empty() {
                return Some(reference.max_end_offset());
            }
        }
        None
    }

    /// Returns all [references](struct.Reference.html) present in the BAI index.
    pub fn references(&self) -> &[Reference] {
        &self.references
    }

    /// Returns the number of unmapped records, if present in the index.
    pub fn n_unmapped(&self) -> Option<u64> {
        self.n_unmapped
    }
}

impl Display for Index {
    fn fmt(&self, f: &mut Formatter) -> result::Result<(), fmt::Error> {
        for (i, reference) in self.references.iter().enumerate() {
            writeln!(f, "Reference {}:", i)?;
            reference.fmt(f)?;
        }
        write!(f, "Unmapped records: ")?;
        match self.n_unmapped {
            Some(count) => write!(f, "{}", count),
            None => write!(f, "Unknown")
        }
    }
}

/// Returns a BAI bin for the record with alignment `[beg-end)`.
pub fn region_to_bin(beg: i32, end: i32) -> u32 {
    let end = end - 1;
    let mut res = 0_i32;
    for i in (14..27).step_by(3) {
        if beg >> i == end >> i {
            res = ((1 << 29 - i) - 1) / 7 + (beg >> i);
            break;
        }
    }
    res as u32
}

/// Returns all possible BAI bins for the region `[beg-end)`.
pub fn region_to_bins(start: i32, end: i32) -> BinsIter {
    BinsIter {
        i: -1,
        t: 0,
        start,
        end,
        curr_bin: 0,
        bins_end: 0,
    }
}

/// Iterator over bins.
pub struct BinsIter {
    i: i32,
    t: i32,
    start: i32,
    end: i32,
    curr_bin: u32,
    bins_end: u32,
}

impl Iterator for BinsIter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr_bin == self.bins_end {
            if self.i >= 4 {
                return None;
            }
            self.i += 1;
            self.t += 1 << (self.i * 3);
            self.curr_bin = (self.t + (self.start >> 26 - 3 * self.i)) as u32 - 1;
            self.bins_end = (self.t + (self.end >> 26 - 3 * self.i)) as u32;

            if self.i == 0 {
                return Some(0);
            }
        }
        self.curr_bin += 1;
        Some(self.curr_bin)
    }
}

impl std::iter::FusedIterator for BinsIter {}

/// Maximal possible bin value.
pub const MAX_BIN: u16 = 37448;

/// Returns a maximal region for a given bin.
pub fn bin_to_region(bin: u16) -> (i32, i32) {
    if bin == 0 {
        return (std::i32::MIN, std::i32::MAX);
    }
    let mut left = 1;
    for i in 1..6 {
        let right = left + (1 << 3 * i);
        if bin >= left && bin < right {
            let beg = (bin - left) as i32;
            return (beg << 29 - 3 * i, beg + 1 << 29 - 3 * i);
        }
        left = right;
    }
    panic!("Bin id should be not bigger than MAX_BIN ({} > {})", bin, MAX_BIN);
}
