//! Cigar and operations on it.

use std::io::{self, Write};
use std::fmt::{self, Display, Formatter};
use std::slice::Iter;

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

/// Cigar operation class:
/// * Match: M, = and X,
/// * Insertion: I and S,
/// * Deletion: D and N,
/// * Hard clipping: H and P,
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Class {
    Match = 0,
    Insertion = 1,
    Deletion = 2,
    Hard = 3,
}

/// Cigar operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operation {
    AlnMatch = 0,
    Insertion = 1,
    Deletion = 2,
    Skip = 3,
    Soft = 4,
    Hard = 5,
    Padding = 6,
    SeqMatch = 7,
    SeqMismatch = 8,
}

impl Operation {
    /// Convert `u8` symbol (for example `b'M'`) into [Operation](enum.Operation.html).
    ///
    /// To convert a number `0-8` into [Operation](enum.Operation.html) use `Operation::from(number)`.
    pub fn from_symbol(symbol: u8) -> Result<Operation, String> {
        use Operation::*;
        match symbol {
            b'M' => Ok(AlnMatch),
            b'I' => Ok(Insertion),
            b'D' => Ok(Deletion),
            b'N' => Ok(Skip),
            b'S' => Ok(Soft),
            b'H' => Ok(Hard),
            b'P' => Ok(Padding),
            b'=' => Ok(SeqMatch),
            b'X' => Ok(SeqMismatch),
            _ => Err(format!("Unexpected cigar operation: {}", symbol as char)),
        }
    }

    /// Convert [Operation](enum.Operation.html) into `u8` symbol (for example `b'M'`).
    ///
    /// To convert [Operation](enum.Operation.html) into a number `0-8` use `operation as u8`.
    pub fn to_byte(self) -> u8 {
        b"MIDNSHP=X"[self as usize]
    }

    /// Checks if the operation consumes query. For example, `M` consumes query, while `D` does not.
    pub fn consumes_query(self) -> bool {
        match self {
            Operation::AlnMatch
            | Operation::Insertion
            | Operation::Soft
            | Operation::SeqMatch
            | Operation::SeqMismatch => true,
            _ => false
        }
    }

    /// Checks if the operation consumes reference.
    /// For example, `M` consumes reference, while `I` does not.
    pub fn consumes_ref(self) -> bool {
        match self {
            Operation::AlnMatch
            | Operation::Deletion
            | Operation::Skip
            | Operation::SeqMatch
            | Operation::SeqMismatch => true,
            _ => false
        }
    }

    /// Returns [operation class](enum.Class.html): operations combined by their behavior
    /// (soft and insertion both consume query but not reference, so they represent the same
    /// class [Insertion](enum.Class.html#variant.Insertion)).
    pub fn class(self) -> Class {
        match self {
            Operation::AlnMatch | Operation::SeqMatch | Operation::SeqMismatch => Class::Match,
            Operation::Insertion | Operation::Soft => Class::Insertion,
            Operation::Deletion | Operation::Skip => Class::Deletion,
            Operation::Hard | Operation::Padding => Class::Hard,
        }
    }

    /// Returns `true` if the operation consumes both query and reference (M, = or X).
    pub fn is_match(self) -> bool {
        match self {
            Operation::AlnMatch | Operation::SeqMatch | Operation::SeqMismatch => true,
            _ => false,
        }
    }

    /// Returns `true` if the operation consumes only reference (I or S).
    pub fn is_insertion(self) -> bool {
        match self {
            Operation::Insertion | Operation::Soft => true,
            _ => false,
        }
    }

    /// Returns `true` if the operation consumes only query (D or N).
    pub fn is_deletion(self) -> bool {
        match self {
            Operation::Deletion | Operation::Skip => true,
            _ => false,
        }
    }

    /// Returns `true` if the operation does not consume query nor reference (H or P).
    pub fn is_hard_clipping(self) -> bool {
        match self {
            Operation::Hard | Operation::Padding => true,
            _ => false,
        }
    }
}

impl Display for Operation {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.to_byte() as char)
    }
}

impl From<u32> for Operation {
    fn from(value: u32) -> Operation {
        use Operation::*;
        match value {
            0 => AlnMatch,
            1 => Insertion,
            2 => Deletion,
            3 => Skip,
            4 => Soft,
            5 => Hard,
            6 => Padding,
            7 => SeqMatch,
            8 => SeqMismatch,
            _ => panic!("Unexpected cigar operation: {}", value),
        }
    }
}

/// A wrapper around raw Cigar.
#[derive(Clone)]
pub struct Cigar(Vec<u32>);

impl Cigar {
    /// Creates a new empty CIGAR.
    pub fn new() -> Self {
        Cigar(Vec::new())
    }

    /// Creates a cigar from raw data.
    pub fn from_raw(raw: &[u32]) -> Self {
        Cigar(raw.to_vec())
    }

    /// Clears the contents but does not touch capacity.
    pub fn clear(&mut self) {
        self.0.clear();
    }

    /// Extends CIGAR from raw data.
    pub fn extend_from_raw<I: IntoIterator<Item = u32>>(&mut self, iter: I) {
        self.0.extend(iter);
    }

    /// Pushes a single operation to the end of the CIGAR. Does nothing if `len` is 0.
    pub fn push(&mut self, len: u32, operation: Operation) {
        if len > 0 {
            self.0.push(len << 4 | operation as u32)
        }
    }

    /// Extends Cigar from text representation.
    /// If an error occured, the cigar may be filled partially.
    pub fn extend_from_text<I: IntoIterator<Item = u8>>(&mut self, text: I) -> Result<(), String> {
        let mut op_len = 0_u32;
        for symb in text {
            if symb >= b'0' && symb <= b'9' {
                op_len = 10 * op_len + (symb - b'0') as u32;
            } else {
                let op = Operation::from_symbol(symb)?;
                if op_len == 0 {
                    return Err("Operation length cannot be zero".to_string());
                }
                self.0.push(op_len << 4 | op as u32);
                op_len = 0;
            }
        }
        Ok(())
    }

    pub(crate) fn fill_from<R: ReadBytesExt>(&mut self, stream: &mut R, len: usize) -> io::Result<()> {
        super::resize(&mut self.0, len);
        stream.read_u32_into::<LittleEndian>(&mut self.0)?;
        Ok(())
    }

    /// Returns a pair `(length, operation)` by its index.
    pub fn at(&self, index: usize) -> (u32, Operation) {
        let v = self.0[index];
        (v >> 4, Operation::from(v & 0xf))
    }

    /// Returns an iterator over typles `(length, operation)`.
    pub fn iter<'a>(&'a self) -> CigarIter<'a> {
        CigarIter {
            parent: self,
            i: 0,
            j: self.0.len(),
        }
    }

    /// Cigar length.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns `true` if Cigar is empty.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns raw Cigar. Each `u32` value represents `length << 4 | operation`, where
    /// operations are encoded from 0 to 8.
    pub fn raw(&self) -> &[u32] {
        &self.0
    }

    /// Calculates reference alignment length. Consider using
    /// [Record::calculate_end](../struct.Record.html#method.calculate_end), as the
    /// record alignment end is stored once calculated.
    pub fn calculate_ref_len(&self) -> u32 {
        self.0.iter().map(|value| match value & 0xf {
            0 | 2 | 3 | 7 | 8 => value >> 4,
            1 | 4 | 5 | 6 => 0,
            _ => panic!("Unexpected Cigar operation: {}", value & 0xf),
        }).sum::<u32>()
    }

    /// Calculates query length.
    pub fn calculate_query_len(&self) -> u32 {
        self.0.iter().map(|value| match value & 0xf {
            0 | 1 | 4 | 7 | 8 => value >> 4,
            2 | 3 | 5 | 6 => 0,
            _ => panic!("Unexpected Cigar operation: {}", value & 0xf),
        }).sum::<u32>()
    }

    /// Shrinks inner vector.
    pub fn shrink_to_fit(&mut self) {
        self.0.shrink_to_fit();
    }

    /// Writes to `f` in a human readable format. Write `*` if empty.
    pub fn write_readable<W: Write>(&self, f: &mut W) -> io::Result<()> {
        if self.is_empty() {
            return f.write_u8(b'*');
        }
        for (len, op) in self.iter() {
            write!(f, "{}", len)?;
            f.write_u8(op.to_byte())?;
        }
        Ok(())
    }

    /// Returns aligned pairs (considering that reference starts at position `r_pos`).
    #[doc(hidden)]
    pub fn aligned_pairs(&self, r_pos: u32) -> AlignedPairs {
        AlignedPairs {
            raw_iter: self.0.iter(),
            q_pos: 0,
            r_pos,
            remaining_len: 0,
            operation: Operation::AlnMatch,
        }
    }

    /// Returns matching pairs (considering that reference starts at position `r_pos`).
    #[doc(hidden)]
    pub fn matching_pairs(&self, r_pos: u32) -> MatchingPairs {
        MatchingPairs {
            raw_iter: self.0.iter(),
            q_pos: 0,
            r_pos,
            remaining_len: 0,
        }
    }

    /// Returns the size of the hard clipping
    /// on the left side if `left_side` and on the right side otherwise.
    pub fn hard_clipping(&self, left_side: bool) -> u32 {
        if left_side {
            self.iter().take_while(|(_len, op)| !op.consumes_ref() && !op.consumes_query())
                .map(|(len, _op)| len).sum()
        } else {
            self.iter().rev().take_while(|(_len, op)| !op.consumes_ref() && !op.consumes_query())
                .map(|(len, _op)| len).sum()
        }
    }

    /// Returns the size of the soft clipping
    /// on the left side if `left_side` and on the right side otherwise.
    /// The function ignores any hard clipping in the process.
    pub fn soft_clipping(&self, left_side: bool) -> u32 {
        let iter: Box<dyn Iterator<Item = _>> = if left_side {
            Box::new(self.iter())
        } else {
            Box::new(self.iter().rev())
        };
        let mut res = 0;
        for (len, op) in iter {
            match op.class() {
                Class::Hard => {},
                Class::Insertion => res += len,
                _ => break,
            }
        }
        res
    }
}

/// Double-ended iterator over CIGAR operations `(usize, Operation)`.
#[derive(Clone)]
pub struct CigarIter<'a> {
    parent: &'a Cigar,
    i: usize,
    j: usize,
}

impl<'a> Iterator for CigarIter<'a> {
    type Item = (u32, Operation);

    fn next(&mut self) -> Option<Self::Item> {
        if self.i < self.j {
            self.i += 1;
            Some(self.parent.at(self.i - 1))
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.j - self.i;
        (len, Some(len))
    }
}

impl<'a> DoubleEndedIterator for CigarIter<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.i < self.j {
            self.j -= 1;
            Some(self.parent.at(self.j))
        } else {
            None
        }
    }
}

impl<'a> ExactSizeIterator for CigarIter<'a> {}

impl<'a> std::iter::FusedIterator for CigarIter<'a> {}

/// Iterator over pairs `(Option<u32>, Option<u32>)`.
/// The first element represents a sequence index, and the second element represents a
/// reference index. If the current operation is an insertion or a deletion, the respective
/// element will be `None.`
#[derive(Clone)]
pub struct AlignedPairs<'a> {
    raw_iter: Iter<'a, u32>,
    q_pos: u32,
    r_pos: u32,
    remaining_len: u32,
    operation: Operation,
}

impl<'a> Iterator for AlignedPairs<'a> {
    type Item = (Option<u32>, Option<u32>);

    fn next(&mut self) -> Option<Self::Item> {
        while self.remaining_len == 0 {
            let v = self.raw_iter.next()?;
            self.operation = Operation::from(v & 0xf);
            if !self.operation.is_hard_clipping() {
                self.remaining_len = v >> 4;
                break;
            }
        }
        self.remaining_len -= 1;
        let q_pos = if self.operation.consumes_query() {
            self.q_pos += 1;
            Some(self.q_pos - 1)
        } else {
            None
        };
        let r_pos = if self.operation.consumes_ref() {
            self.r_pos += 1;
            Some(self.r_pos - 1)
        } else {
            None
        };
        Some((q_pos, r_pos))
    }
}

impl<'a> std::iter::FusedIterator for AlignedPairs<'a> { }

/// Iterator over pairs `(u32, u32)`.
/// The first element represents a sequence index, and the second element represents a
/// reference index. This iterator skips insertions and deletions.
#[derive(Clone)]
pub struct MatchingPairs<'a> {
    raw_iter: Iter<'a, u32>,
    q_pos: u32,
    r_pos: u32,
    remaining_len: u32,
}

impl<'a> Iterator for MatchingPairs<'a> {
    type Item = (u32, u32);

    fn next(&mut self) -> Option<Self::Item> {
        use Operation::*;

        while self.remaining_len == 0 {
            let v = self.raw_iter.next()?;
            let operation = Operation::from(v & 0xf);
            let op_len = v >> 4;
            match operation {
                AlnMatch | SeqMatch | SeqMismatch => self.remaining_len = v >> 4,
                Insertion | Soft => self.q_pos += op_len,
                Deletion | Skip => self.r_pos += op_len,
                _ => {},
            }
        }

        self.remaining_len -= 1;
        self.q_pos += 1;
        self.r_pos += 1;
        Some((self.q_pos - 1, self.r_pos - 1))
    }
}

impl<'a> std::iter::FusedIterator for MatchingPairs<'a> { }

impl Display for Cigar {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        if self.is_empty() {
            return write!(f, "*");
        }
        for (len, op) in self.iter() {
            write!(f, "{}{}", len, op)?;
        }
        Ok(())
    }
}
