//! A wrappers around raw sequence and raw qualities.

use std::io::{self, Read, Write};
use std::ops::{Range, RangeBounds};

use byteorder::WriteBytesExt;

use super::{resize, write_iterator};

/// Converts nucleotide to BAM u4 (for example `b'T'` -> `8`).
fn nt_to_raw(nt: u8) -> Result<u8, String> {
    match nt {
        b'=' => Ok(0),
        b'A' | b'a' => Ok(1),
        b'C' | b'c' => Ok(2),
        b'M' | b'm' => Ok(3),
        b'G' | b'g' => Ok(4),
        b'R' | b'r' => Ok(5),
        b'S' | b's' => Ok(6),
        b'V' | b'v' => Ok(7),
        b'T' | b't' => Ok(8),
        b'W' | b'w' => Ok(9),
        b'Y' | b'y' => Ok(10),
        b'H' | b'h' => Ok(11),
        b'K' | b'k' => Ok(12),
        b'D' | b'd' => Ok(13),
        b'B' | b'b' => Ok(14),
        b'N' | b'n' => Ok(15),
        _ => Err(format!("Nucleotide not expected: {}", nt as char)),
    }
}

/// Wrapper around raw sequence, stored as an `[u8; (len + 1) / 2]`. Each four bits encode a
/// nucleotide in the following order: `=ACMGRSVTWYHKDBN`.
#[derive(Clone)]
pub struct Sequence {
    raw: Vec<u8>,
    len: usize,
}

impl Sequence {
    /// Creates an empty sequence.
    pub fn new() -> Self {
        Sequence {
            raw: Vec::new(),
            len: 0,
        }
    }

    /// Creates a new sequence from text representation.
    pub fn from_text<I: IntoIterator<Item = u8>>(sequence: I) -> Result<Self, String> {
        let mut seq = Sequence::new();
        seq.extend_from_text(sequence)?;
        Ok(seq)
    }

    /// Clears sequence but does not touch capacity.
    pub fn clear(&mut self) {
        self.raw.clear();
        self.len = 0;
    }

    /// Shrinks inner vector.
    pub fn shrink_to_fit(&mut self) {
        self.raw.shrink_to_fit();
    }

    /// Pushes a single nucleotide to the end.
    pub fn push(&mut self, nt: u8) -> Result<(), String> {
        if self.len % 2 == 0 {
            self.raw.push(nt_to_raw(nt)? << 4);
        } else {
            self.raw[self.len / 2] |= nt_to_raw(nt)?;
        }
        self.len += 1;
        Ok(())
    }

    /// Extends sequence from the text representation.
    pub fn extend_from_text<I: IntoIterator<Item = u8>>(&mut self, nucleotides: I)
            -> Result<(), String> {
        for nt in nucleotides.into_iter() {
            self.push(nt)?;
        };
        Ok(())
    }

    /// Clears sequence and fills from a raw stream. `new_len` represents the number of nucleotides,
    /// not the number of bytes.
    pub fn fill_from<R: Read>(&mut self, stream: &mut R, new_len: usize)
            -> io::Result<()> {
        let short_len = (new_len + 1) / 2;
        unsafe {
            resize(&mut self.raw, short_len);
        }
        stream.read_exact(&mut self.raw)?;
        self.len = new_len;
        Ok(())
    }

    /// Returns raw data.
    pub fn raw(&self) -> &[u8] {
        &self.raw
    }

    /// Returns full length of the sequence, O(1).
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns `true`, if the sequence is present.
    pub fn available(&self) -> bool {
        self.len > 0
    }

    /// Returns transformed data, each byte represents a single nucleotde, O(n).
    pub fn to_vec(&self) -> Vec<u8> {
        (0..self.len).map(|i| self.at(i)).collect()
    }

    /// Returns transformed data using only nucleotides A, C, G, T and N,
    /// all other values are transformed into N, each byte represents a single nucleotde, O(n).
    pub fn to_vec_acgtn_only(&self) -> Vec<u8> {
        (0..self.len).map(|i| self.at_acgtn_only(i)).collect()
    }

    /// Returns a nucleotide at the position `index`, represented by a single byte, O(1).
    pub fn at(&self, index: usize) -> u8 {
        assert!(index < self.len, "Index out of range ({} >= {})", index, self.len);
        let nt = if index % 2 == 0 {
            self.raw[index / 2] >> 4
        } else {
            self.raw[index / 2] & 0x0f
        };
        b"=ACMGRSVTWYHKDBN"[nt as usize]
    }

    /// Returns a nucleotide at the position `index`, represented by a single byte, O(1).
    /// If the nucleotide is not A, C, G or T, the function returns N.
    pub fn at_acgtn_only(&self, index: usize) -> u8 {
        assert!(index < self.len, "Index out of range ({} >= {})", index, self.len);
        let nt = if index % 2 == 0 {
            self.raw[index / 2] >> 4
        } else {
            self.raw[index / 2] & 0x0f
        };
        b"NACNGNNNTNNNNNNN"[nt as usize]
    }

    /// Returns a nucleotide, complement to the nucleotide at the position `index`, O(1).
    pub fn compl_at(&self, index: usize) -> u8 {
        assert!(index < self.len, "Index out of range ({} >= {})", index, self.len);
        let nt = if index % 2 == 0 {
            self.raw[index / 2] >> 4
        } else {
            self.raw[index / 2] & 0x0f
        };
        b"=TGKCYSBAWRDMHVN"[nt as usize]
    }

    /// Returns a nucleotide, complement to the nucleotide at the position `index`, O(1).
    /// If the nucleotide is not A, C, G or T, the function returns N.
    pub fn compl_at_acgtn_only(&self, index: usize) -> u8 {
        assert!(index < self.len, "Index out of range ({} >= {})", index, self.len);
        let nt = if index % 2 == 0 {
            self.raw[index / 2] >> 4
        } else {
            self.raw[index / 2] & 0x0f
        };
        b"NTGNCNNNANNNNNNN"[nt as usize]
    }

    /// Returns an iterator over a subsequence.
    pub fn subseq<'a, R: RangeBounds<usize>>(&'a self, range: R) -> SubseqIter<'a> {
        use std::ops::Bound::*;
        let start = match range.start_bound() {
            Included(&n) => n,
            Excluded(&n) => n + 1,
            Unbounded => 0,
        };
        let end = match range.end_bound() {
            Included(&n) => n + 1,
            Excluded(&n) => n,
            Unbounded => self.len,
        };
        assert!(start <= end);
        assert!(end <= self.len);
        SubseqIter {
            parent: self,
            indices: start..end,
        }
    }

    /// Returns an iterator over a subsequence using only nucleotides A, C, G, T and N.
    pub fn subseq_acgtn_only<R: RangeBounds<usize>>(&self, range: R) -> SubseqIterAcgtn {
        use std::ops::Bound::*;
        let start = match range.start_bound() {
            Included(&n) => n,
            Excluded(&n) => n + 1,
            Unbounded => 0,
        };
        let end = match range.end_bound() {
            Included(&n) => n + 1,
            Excluded(&n) => n,
            Unbounded => self.len,
        };
        assert!(start <= end);
        assert!(end <= self.len);
        SubseqIterAcgtn {
            parent: self,
            indices: start..end,
        }
    }

    /// Returns an iterator over a reverse complement of a subsequence.
    pub fn rev_compl<R: RangeBounds<usize>>(&self, range: R) -> RevComplIter {
        use std::ops::Bound::*;
        let start = match range.start_bound() {
            Included(&n) => n,
            Excluded(&n) => n + 1,
            Unbounded => 0,
        };
        let end = match range.end_bound() {
            Included(&n) => n + 1,
            Excluded(&n) => n,
            Unbounded => self.len,
        };
        assert!(start <= end);
        assert!(end <= self.len);
        RevComplIter {
            parent: self,
            indices: (start..end).rev(),
        }
    }

    /// Returns an iterator over a reverse complement of a subsequence using only
    /// nucleotides A, C, G, T and N.
    pub fn rev_compl_acgtn_only<R: RangeBounds<usize>>(&self, range: R) -> RevComplIterAcgtn {
        use std::ops::Bound::*;
        let start = match range.start_bound() {
            Included(&n) => n,
            Excluded(&n) => n + 1,
            Unbounded => 0,
        };
        let end = match range.end_bound() {
            Included(&n) => n + 1,
            Excluded(&n) => n,
            Unbounded => self.len,
        };
        assert!(start <= end);
        assert!(end <= self.len);
        RevComplIterAcgtn {
            parent: self,
            indices: (start..end).rev(),
        }
    }

    /// Writes in human readable format. Writes `*` if empty.
    pub fn write_readable<W: Write>(&self, f: &mut W) -> io::Result<()> {
        if self.len == 0 {
            return f.write_u8(b'*');
        }
        write_iterator(f, (0..self.len).map(|i| self.at(i)))
    }
}

macro_rules! subseq_iter {
    ($name:ident, $ind_ty:ty, $fun:ident) => {
        /// Double-ended iterator over subsequence.
        #[derive(Clone)]
        pub struct $name<'a> {
            parent: &'a Sequence,
            indices: $ind_ty,
        }

        impl<'a> std::iter::Iterator for $name<'a> {
            type Item = u8;

            fn next(&mut self) -> Option<u8> {
                self.indices.next().map(|i| self.parent.$fun(i))
            }

            fn size_hint(&self) -> (usize, Option<usize>) {
                self.indices.size_hint()
            }
        }

        impl<'a> std::iter::DoubleEndedIterator for $name<'a> {
            fn next_back(&mut self) -> Option<Self::Item> {
                self.indices.next_back().map(|i| self.parent.$fun(i))
            }
        }

        impl<'a> std::iter::ExactSizeIterator for $name<'a> {}

        impl<'a> std::iter::FusedIterator for $name<'a> {}
    }
}

subseq_iter!(SubseqIter, Range<usize>, at);
subseq_iter!(SubseqIterAcgtn, Range<usize>, at_acgtn_only);
subseq_iter!(RevComplIter, std::iter::Rev<Range<usize>>, compl_at);
subseq_iter!(RevComplIterAcgtn, std::iter::Rev<Range<usize>>, compl_at_acgtn_only);

/// Wrapper around qualities.
#[derive(Clone)]
pub struct Qualities {
    raw: Vec<u8>
}

impl Qualities {
    /// Creates a new instance of `Qualities`.
    pub fn new() -> Self {
        Qualities {
            raw: Vec::new()
        }
    }

    pub fn from_raw<I: IntoIterator<Item = u8>>(raw: I) -> Self {
        let mut qualities = Qualities::new();
        qualities.extend_from_raw(raw);
        qualities
    }

    /// Clears qualities and fills from a raw stream.
    pub fn fill_from<R: Read>(&mut self, stream: &mut R, len: usize) -> io::Result<()> {
        unsafe {
            resize(&mut self.raw, len);
        }
        stream.read_exact(&mut self.raw)?;
        Ok(())
    }

    /// Returns raw qualities, they contain values 0-93, without +33 added.
    ///
    /// If qualities are empty, they have the same length as `Sequence`, but are filled with `0xff`.
    pub fn raw(&self) -> &[u8] {
        &self.raw
    }

    /// Returns zero if qualities are not available.
    pub fn len(&self) -> usize {
        if self.available() {
            self.raw.len()
        } else {
            0
        }
    }

    /// Returns `false` if raw qualities have length 0 or are filled with `0xff`.
    /// Only the first element is checked, O(1).
    pub fn available(&self) -> bool {
        !self.raw.is_empty() && self.raw[0] != 0xff
    }

    /// Returns vector with +33 added, O(n).
    pub fn to_readable(&self) -> Vec<u8> {
        self.raw.iter().map(|qual| qual + 33).collect()
    }

    /// Writes to `f` in human readable format (qual + 33). Writes `*` if empty.
    pub fn write_readable<W: Write>(&self, f: &mut W) -> io::Result<()> {
        if !self.available() {
            return f.write_u8(b'*');
        }
        write_iterator(f, self.raw.iter().map(|qual| qual + 33))
    }

    /// Clears the contents but does not touch capacity.
    pub fn clear(&mut self) {
        self.raw.clear();
    }

    /// Extends the qualities from raw qualities (without + 33).
    pub fn extend_from_raw<I: IntoIterator<Item = u8>>(&mut self, qualities: I) {
        self.raw.extend(qualities);
    }

    /// Shrinks inner vector.
    pub fn shrink_to_fit(&mut self) {
        self.raw.shrink_to_fit();
    }
}