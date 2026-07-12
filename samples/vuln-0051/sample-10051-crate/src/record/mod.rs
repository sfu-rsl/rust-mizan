//! SAM/BAM record, sequence, qualities and operations on them.

use std::io::{self, Read, Write};
use std::io::ErrorKind::{self, InvalidData, UnexpectedEof};
use std::cell::Cell;
use std::str::from_utf8;
use std::fmt;

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

pub mod tags;
pub mod cigar;
pub mod sequence;

use super::header::Header;
use super::index;

pub use cigar::Cigar;
pub use sequence::Sequence;
pub use sequence::Qualities;

/// `= 0x1`. Record has a mate.
pub const RECORD_PAIRED: u16 = 0x1;
/// `= 0x2`. Record and its mate mapped properly.
pub const ALL_SEGMENTS_ALIGNED: u16 = 0x2;
/// `= 0x4`. Record is unmapped.
pub const RECORD_UNMAPPED: u16 = 0x4;
/// `= 0x8`. Record mate is unmapped.
pub const MATE_UNMAPPED: u16 = 0x8;
/// `= 0x10`. Record is on the reverse strand.
pub const RECORD_REVERSE_STRAND: u16 = 0x10;
/// `= 0x20`. Record mate is on the reverse strand.
pub const MATE_REVERSE_STRAND: u16 = 0x20;
/// `= 0x40`. Record is the first segment in a template.
pub const FIRST_IN_PAIR: u16 = 0x40;
/// `= 0x80`. Record is the last segment in a template.
pub const LAST_IN_PAIR: u16 = 0x80;
/// `= 0x100`. Alignment is secondary (not primary).
pub const SECONDARY: u16 = 0x100;
/// `= 0x200`. Record fails platform/vendor quality checks.
pub const RECORD_FAILS_QC: u16 = 0x200;
/// `= 0x400`. Record is PCR or optical duplicate.
pub const PCR_OR_OPTICAL_DUPLICATE: u16 = 0x400;
/// `= 0x800`. Alignment is supplementary (chimeric/split).
pub const SUPPLEMENTARY: u16 = 0x800;

/// A wrapper around BAM/SAM flag.
///
/// You can check flag as `record.flag().is_paired()` or `record.flag().0 | IS_PAIRED == 0`.
/// You can also modify the flag as `record.flag_mut().set_paired(true)` or
/// `record.flag_mut().0 |= IS_PAIRED`.
#[derive(Clone, Copy, Debug)]
pub struct Flag(pub u16);

impl Flag {
    /// Checks if all bits match the mask.
    pub fn all_bits(&self, mask: u16) -> bool {
        (self.0 & mask) == mask
    }

    /// Checks if any bit matches the mask.
    pub fn any_bit(&self, mask: u16) -> bool {
        (self.0 & mask) != 0
    }

    /// Checks if all bits do not match the mask.
    pub fn no_bits(&self, mask: u16) -> bool {
        (self.0 & mask) == 0
    }

    pub fn is_paired(&self) -> bool {
        self.any_bit(RECORD_PAIRED)
    }

    pub fn all_segments_aligned(&self) -> bool {
        self.any_bit(ALL_SEGMENTS_ALIGNED)
    }

    pub fn is_mapped(&self) -> bool {
        self.no_bits(RECORD_UNMAPPED)
    }

    pub fn mate_is_mapped(&self) -> bool {
        self.no_bits(MATE_UNMAPPED)
    }

    pub fn is_reverse_strand(&self) -> bool {
        self.any_bit(RECORD_REVERSE_STRAND)
    }

    pub fn mate_is_reverse_strand(&self) -> bool {
        self.any_bit(MATE_REVERSE_STRAND)
    }

    pub fn first_in_pair(&self) -> bool {
        self.any_bit(FIRST_IN_PAIR)
    }

    pub fn last_in_pair(&self) -> bool {
        self.any_bit(LAST_IN_PAIR)
    }

    pub fn is_secondary(&self) -> bool {
        self.any_bit(SECONDARY)
    }

    /// Returns `true` if the record fails filters, such as platform/vendor quality controls.
    pub fn fails_quality_controls(&self) -> bool {
        self.any_bit(RECORD_FAILS_QC)
    }

    /// Returns `true` if the record is PCR or optical duplicate.
    pub fn is_duplicate(&self) -> bool {
        self.any_bit(PCR_OR_OPTICAL_DUPLICATE)
    }

    pub fn is_supplementary(&self) -> bool {
        self.any_bit(SUPPLEMENTARY)
    }

    /// Modifies the record flag. This function does not do any checks.
    pub fn set_paired(&mut self, paired: bool) {
        if paired {
            self.0 |= RECORD_PAIRED;
        } else {
            self.0 &= !RECORD_PAIRED;
        }
    }

    /// Modifies the record flag. This function does not do any checks.
    pub fn set_all_segments_aligned(&mut self, all_segments_aligned: bool) {
        if all_segments_aligned {
            self.0 |= ALL_SEGMENTS_ALIGNED;
        } else {
            self.0 &= !ALL_SEGMENTS_ALIGNED;
        }
    }

    /// Modifies the record flag. This function does not do any checks.
    pub fn set_mapped(&mut self, mapped: bool) {
        if mapped {
            self.0 &= !RECORD_UNMAPPED;
        } else {
            self.0 |= RECORD_UNMAPPED;
        }
    }

    /// Modifies the record flag. This function does not do any checks and does not modify the
    /// mate record.
    pub fn set_mate_mapped(&mut self, mate_mapped: bool) {
        if mate_mapped {
            self.0 &= !MATE_UNMAPPED;
        } else {
            self.0 |= MATE_UNMAPPED;
        }
    }

    /// Sets the record strand. Use `true` to set to forward strand, and `false` for the
    /// reverse strand.
    pub fn set_strand(&mut self, forward_strand: bool) {
        if forward_strand {
            self.0 &= !RECORD_REVERSE_STRAND;
        } else {
            self.0 |= RECORD_REVERSE_STRAND;
        }
    }

    /// Sets the strand of the mate. Use `true` to set to forward strand, and `false` for the
    /// reverse strand. This function does not do any checks and does not modify the
    /// mate record.
    pub fn set_mate_strand(&mut self, forward_strand: bool) {
        if forward_strand {
            self.0 &= !MATE_REVERSE_STRAND;
        } else {
            self.0 |= MATE_REVERSE_STRAND;
        }
    }

    /// Modifies the record flag. This function does not do any checks.
    ///
    /// Use `true` to set the flag bit and `false` to unset.
    pub fn set_first_in_pair(&mut self, is_first: bool) {
        if is_first {
            self.0 |= FIRST_IN_PAIR;
        } else {
            self.0 &= !FIRST_IN_PAIR;
        }
    }

    /// Modifies the record flag. This function does not do any checks.
    ///
    /// Use `true` to set the flag bit and `false` to unset.
    pub fn set_last_in_pair(&mut self, is_last: bool) {
        if is_last {
            self.0 |= LAST_IN_PAIR;
        } else {
            self.0 &= !LAST_IN_PAIR;
        }
    }

    /// Modifies the record flag. This function does not do any checks.
    ///
    /// Use `true` to set the flag bit and `false` to unset.
    pub fn set_secondary(&mut self, is_secondary: bool) {
        if is_secondary {
            self.0 |= SECONDARY;
        } else {
            self.0 &= !SECONDARY;
        }
    }

    /// Sets the record flag to fail or pass filters, such as platform/vendor quality controls.
    ///
    /// Use `true` to set the flag bit and `false` to unset.
    pub fn make_fail_quality_controls(&mut self, fails: bool) {
        if fails {
            self.0 |= RECORD_FAILS_QC;
        } else {
            self.0 &= !RECORD_FAILS_QC;
        }
    }

    /// Sets the record as PCR or optical duplicate.
    pub fn set_duplicate(&mut self, is_duplicate: bool) {
        if is_duplicate {
            self.0 |= PCR_OR_OPTICAL_DUPLICATE;
        } else {
            self.0 &= !PCR_OR_OPTICAL_DUPLICATE;
        }
    }

    pub fn set_supplementary(&mut self, is_supplementary: bool) {
        if is_supplementary {
            self.0 |= SUPPLEMENTARY;
        } else {
            self.0 &= !SUPPLEMENTARY;
        }
    }
}

impl fmt::Display for Flag {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "Flag({})", self.0)
    }
}

/// This function resizes the vector and fills with initialized values if the capacity increases.
/// Therefore, the vector is required to have initialized values below its capacity.
pub(crate) fn resize<T: Default + Copy>(v: &mut Vec<T>, new_len: usize) {
    unsafe {
        if v.capacity() < new_len {
            v.reserve(new_len);
            v.set_len(v.capacity());

            let new_val = T::default();
            // NOTE: Can be replaced with `v.fill` after Rust 1.50.
            v.iter_mut().map(|x| *x = new_val).count();
        }
        v.set_len(new_len);
    }
}

pub(crate) fn write_iterator<W, I>(writer: &mut W, mut iterator: I) -> io::Result<()>
where W: Write,
      I: Iterator<Item = u8>,
{
    const SIZE: usize = 1024;
    let mut buffer = [0_u8; SIZE];
    loop {
        for i in 0..SIZE {
            match iterator.next() {
                Some(value) => buffer[i] = value,
                None => {
                    return writer.write_all(&buffer[..i]);
                }
            }
        }
        writer.write_all(&buffer)?;
    }
}

/// A helper trait to get next from a splitted string.
trait NextToErr<'a> {
    fn try_next(&mut self, field: &'static str) -> io::Result<&'a str>;
}

impl<'a> NextToErr<'a> for std::str::Split<'a, char> {
    fn try_next(&mut self, field: &'static str) -> io::Result<&'a str> {
        self.next().ok_or_else(|| io::Error::new(UnexpectedEof,
            format!("Truncated file: Cannot extract field {}", field)))
    }
}

/// BAM Record.
///
/// Allows to get and set name, [sequence](sequence/struct.Sequence.html),
/// [qualities](sequence/struct.Qualities.html),
/// [CIGAR](cigar/struct.Cigar.html), [flag](struct.Flag.html), [tags](tags/struct.TagViewer.html)
/// and all other BAM/SAM record fields.
///
/// You can use [aligned_pairs](#method.aligned_pairs) and [matching_pairs](#method.matching_pairs)
/// to iterate over record/reference aligned positions.
///
/// If the record has an MD tag, you can use [alignment_entries](#method.alignment_entries) to get record/reference
/// positions and corresponding nucleotides.
#[derive(Clone)]
pub struct Record {
    ref_id: i32,
    mate_ref_id: i32,
    start: i32,
    end: Cell<i32>,
    mate_start: i32,
    bin: Cell<u16>,
    mapq: u8,
    flag: Flag,
    template_len: i32,

    name: Vec<u8>,
    cigar: Cigar,
    seq: Sequence,
    qual: Qualities,
    tags: tags::TagViewer,
}

const BIN_UNKNOWN: u16 = std::u16::MAX;

impl Record {
    /// Creates an empty record. Can be filled using
    /// [read_into](../bam_reader/struct.RegionViewer.html#method.read_into).
    pub fn new() -> Record {
        Record {
            ref_id: -1,
            mate_ref_id: -1,
            start: -1,
            end: Cell::new(0),
            mate_start: -1,
            bin: Cell::new(BIN_UNKNOWN),
            mapq: 0,
            flag: Flag(0),
            template_len: 0,

            name: Vec::new(),
            cigar: Cigar::new(),
            seq: Sequence::new(),
            qual: Qualities::new(),
            tags: tags::TagViewer::new(),
        }
    }

    /// Clears the record.
    pub fn clear(&mut self) {
        self.ref_id = -1;
        self.mate_ref_id = -1;
        self.start = -1;
        self.end.set(0);
        self.mate_start = -1;
        self.bin.set(BIN_UNKNOWN);
        self.mapq = 0;
        self.flag.0 = 0;
        self.template_len = 0;

        self.name.clear();
        self.cigar.clear();
        self.seq.clear();
        self.qual.clear();
        self.tags.clear();
    }

    fn corrupt(&mut self, text: &str) -> io::Error {
        if self.name.is_empty() {
            io::Error::new(InvalidData,
                format!("Corrupted record {}: {}",
                    std::str::from_utf8(&self.name).unwrap_or("*NAME NOT UTF-8*"), text))
        } else {
            io::Error::new(InvalidData, format!("Corrupted record: {}", text))
        }
    }

    /// Fills the record from a `stream` of uncompressed BAM contents.
    /// Returns `false`, if the file ended and the record was not read.
    ///
    /// Use [BamReader](../bam_reader/struct.BamReader.html) or [IndexedReader](../bam_reader/struct.BamReader.html)
    /// instead of this function whenever possible.
    pub fn fill_from_bam<R: Read>(&mut self, stream: &mut R) -> io::Result<bool> {
        self.name.clear();
        let block_size = match stream.read_i32::<LittleEndian>() {
            Ok(value) => {
                if value < 0 {
                    return Err(self.corrupt("Negative block size"));
                }
                value as usize
            },
            Err(e) => {
                if e.kind() == ErrorKind::UnexpectedEof {
                    return Ok(false);
                } else {
                    return Err(e);
                }
            },
        };

        let ref_id = stream.read_i32::<LittleEndian>()?;
        if ref_id < -1 {
            return Err(self.corrupt("Reference id < 1"));
        }
        self.set_ref_id(ref_id);

        self.end.set(0);
        let start = stream.read_i32::<LittleEndian>()?;
        if start < -1 {
            return Err(self.corrupt("Start < 1"));
        }
        self.set_start(start);
        let name_len = stream.read_u8()?;
        if name_len == 0 {
            return Err(self.corrupt("Name length == 0"));
        }

        self.set_mapq(stream.read_u8()?);
        self.bin.set(stream.read_u16::<LittleEndian>()?);
        let cigar_len = stream.read_u16::<LittleEndian>()?;
        self.set_flag(stream.read_u16::<LittleEndian>()?);
        let qual_len = stream.read_i32::<LittleEndian>()?;
        if qual_len < 0 {
            return Err(self.corrupt("Negative sequence length"));
        }
        let qual_len = qual_len as usize;

        if self.flag().is_mapped() && cigar_len == 0 {
            return Err(self.corrupt("Mapped read has an empty CIGAR"));
        }

        let mate_ref_id = stream.read_i32::<LittleEndian>()?;
        if mate_ref_id < -1 {
            return Err(self.corrupt("Mate reference id < 1"));
        }
        self.set_mate_ref_id(mate_ref_id);

        let mate_start = stream.read_i32::<LittleEndian>()?;
        if mate_start < -1 {
            return Err(self.corrupt("Mate start < 1"));
        }
        self.set_mate_start(mate_start);
        self.set_template_len(stream.read_i32::<LittleEndian>()?);

        resize(&mut self.name, name_len as usize - 1);
        stream.read_exact(&mut self.name)?;
        let _null_symbol = stream.read_u8()?;

        self.cigar.fill_from(stream, cigar_len as usize)?;
        self.seq.fill_from(stream, qual_len)?;
        self.qual.fill_from(stream, qual_len)?;

        let seq_len = (qual_len + 1) / 2;
        let remaining_size = block_size - 32 - name_len as usize - 4 * cigar_len as usize
            - seq_len - qual_len;
        self.tags.fill_from(stream, remaining_size)?;
        self.replace_cigar_if_needed()?;
        Ok(true)
    }

    /// Fills the record from SAM. If an error is return, the record may be corrupted.
    pub fn fill_from_sam(&mut self, line: &str, header: &Header) -> io::Result<()> {
        let mut split = line.split('\t');
        self.set_name(split.try_next("record name (QNAME)")?.bytes());
        let flag = split.try_next("flag")?;
        let flag = flag.parse()
            .map_err(|_| self.corrupt(&format!("Cannot convert flag '{}' to int", flag)))?;
        self.set_flag(flag);

        let rname = split.try_next("reference name")?;
        if rname == "*" {
            self.set_ref_id(-1);
        } else {
            let r_id = header.reference_id(rname).ok_or_else(||
                self.corrupt(&format!("Reference '{}' is not in the header", rname)))?;
            self.set_ref_id(r_id as i32);
        }

        let start = split.try_next("start (POS)")?;
        let start = start.parse::<i32>()
            .map_err(|_| self.corrupt(&format!("Cannot convert POS '{}' to int", start)))? - 1;
        if start < -1 {
            return Err(self.corrupt("Start < -1"));
        }
        self.set_start(start);

        let mapq = split.try_next("mapq")?;
        let mapq = mapq.parse()
            .map_err(|_| self.corrupt(&format!("Cannot convert MAPQ '{}' to int", mapq)))?;
        self.set_mapq(mapq);

        let cigar = split.try_next("CIGAR")?;
        if cigar == "*" {
            self.set_raw_cigar(std::iter::empty());
        } else {
            self.set_cigar(cigar.bytes()).map_err(|e| self.corrupt(&e))?;
        }
        if self.flag().is_mapped() && self.cigar.len() == 0 {
            return Err(self.corrupt("Mapped read has an empty CIGAR"));
        }

        let rnext = split.try_next("mate reference name (RNEXT)")?;
        if rnext == "*" {
            self.set_mate_ref_id(-1);
        } else if rnext == "=" {
            self.set_mate_ref_id(self.ref_id);
        } else {
            let nr_id = header.reference_id(rnext).ok_or_else(||
                self.corrupt(&format!("Reference '{}' is not in the header", rnext)))?;
            self.set_mate_ref_id(nr_id as i32);
        }

        let mate_start = split.try_next("mate start (PNEXT)")?;
        let mate_start = mate_start.parse::<i32>().map_err(|_|
            self.corrupt(&format!("Cannot convert PNEXT '{}' to int", mate_start)))? - 1;
        if mate_start < -1 {
            return Err(self.corrupt("Mate start < -1"));
        }
        self.set_mate_start(mate_start);

        let template_len = split.try_next("template length (TLEN)")?;
        let template_len = template_len.parse::<i32>().map_err(|_|
            self.corrupt(&format!("Cannot convert TLEN '{}' to int", template_len)))?;
        self.set_template_len(template_len);

        let seq = split.try_next("sequence")?;
        let qual = split.try_next("qualities")?;
        if seq == "*" {
            self.set_seq_qual(std::iter::empty(), std::iter::empty()).map_err(|e| self.corrupt(&e))?;
        } else if qual == "*" {
            self.set_seq_qual(seq.bytes(), std::iter::empty()).map_err(|e| self.corrupt(&e))?;
        } else {
            self.set_seq_qual(seq.bytes(), qual.bytes().map(|q| q - 33)).map_err(|e| self.corrupt(&e))?;
        }

        self.tags.clear();
        for tag in split {
            self.tags.push_sam(tag)?;
        }
        Ok(())
    }

    /// Replace Cigar by CG tag if Cigar has placeholder *kSmN*.
    fn replace_cigar_if_needed(&mut self) -> io::Result<()> {
        if !self.cigar.is_empty() && self.cigar.at(0) ==
                (self.seq.len() as u32, cigar::Operation::Soft) {
            if self.cigar.len() != 2 {
                return Err(self.corrupt("Record contains invalid Cigar"));
            }
            let (len, op) = self.cigar.at(1);
            if op != cigar::Operation::Skip {
                return Err(self.corrupt("Record contains invalid Cigar"));
            }
            self.end.set(self.start + len as i32);

            let cigar_arr = match self.tags.get(b"CG") {
                Some(tags::TagValue::IntArray(array_view)) => {
                    if array_view.int_type() != tags::IntegerType::U32 {
                        return Err(self.corrupt("CG tag has an incorrect type"));
                    }
                    array_view
                },
                _ => return Err(
                    self.corrupt("Record should contain tag CG, but does not")),
            };
            self.cigar.clear();
            self.cigar.extend_from_raw(cigar_arr.iter().map(|el| el as u32));
            std::mem::drop(cigar_arr);
            self.tags.remove(b"CG");
        }
        Ok(())
    }

    /// Shrinks record contents. The more records were read into the same `Record` instance, the
    /// bigger would be inner vectors (to save time on memory allocation).
    /// Use this function if you do not plan to read into this record in the future.
    pub fn shrink_to_fit(&mut self) {
        self.name.shrink_to_fit();
        self.cigar.shrink_to_fit();
        self.seq.shrink_to_fit();
        self.qual.shrink_to_fit();
        self.tags.shrink_to_fit();
    }

    /// Returns record name as bytes.
    pub fn name(&self) -> &[u8] {
        &self.name
    }

    /// Returns record sequence. You can check if sequence is present in the record using
    /// [sequence().available()](sequence/struct.Sequence.html#method.available).
    pub fn sequence(&self) -> &Sequence {
        &self.seq
    }

    /// Returns record qualities. You can check if qualities are present in the record using
    /// [qualities().available()](sequence/struct.Qualities.html#method.available).
    pub fn qualities(&self) -> &Qualities {
        &self.qual
    }

    /// Returns record CIGAR (can be empty).
    pub fn cigar(&self) -> &Cigar {
        &self.cigar
    }

    /// Returns 0-based reference index. Returns -1 for unmapped records.
    pub fn ref_id(&self) -> i32 {
        self.ref_id
    }

    /// Returns 0-based left-most aligned reference position. Same as *POS - 1* in SAM specification.
    /// Returns -1 for unmapped records.
    pub fn start(&self) -> i32 {
        self.start
    }

    /// For a mapped record aligned to reference positions `[start-end)`, the function returns `end`.
    /// The first calculation takes O(n), where *n* is the length of Cigar.
    /// Consecutive calculations take O(1).
    /// If the record was fetched from a specific region, it should have `end` already calculated.
    ///
    /// Returns zero for unmapped records.
    pub fn calculate_end(&self) -> i32 {
        if self.cigar.is_empty() {
            return 0;
        }

        let end = self.end.get();
        if end != 0 {
            return end;
        }

        let end = self.start + self.cigar.calculate_ref_len() as i32;
        self.end.set(end);
        end
    }

    /// Returns query length. The function returns the length of the sequence if it is present.
    /// Otherwise, the function returns the length calculated from the CIGAR.
    /// Unmapped records without sequence would get length 0.
    pub fn query_len(&self) -> u32 {
        if self.seq.available() {
            self.seq.len() as u32
        } else {
            self.cigar.calculate_query_len()
        }
    }

    /// Returns the index of the first aligned base in the record. Returns the length of the query for unmapped records.
    pub fn aligned_query_start(&self) -> u32 {
        if self.flag.is_mapped() {
            self.cigar.soft_clipping(true)
        } else {
            self.query_len()
        }
    }

    /// Returns the index after the last aligned base in the record. Returns zero for unmapped records.
    pub fn aligned_query_end(&self) -> u32 {
        if self.flag.is_mapped() {
            self.query_len() - self.cigar.soft_clipping(false)
        } else {
            0
        }
    }

    /// Returns BAI bin. If the bin is unknown and the end has not been calculated,
    /// the bin will be calculated in `O(n_cigar)`, otherwise `O(1)`.
    ///
    /// Returns 4680 for unmapped reads.
    pub fn calculate_bin(&self) -> u16 {
        let bin = self.bin.get();
        if bin != BIN_UNKNOWN {
            return bin;
        }
        let end = self.calculate_end();
        let bin = index::region_to_bin(self.start, end) as u16;
        self.bin.set(bin);
        bin
    }

    /// Returns record MAPQ.
    pub fn mapq(&self) -> u8 {
        self.mapq
    }

    /// Returns 0-based reference index for the pair record. Returns -1 for unmapped records,
    /// and records without a pair.
    pub fn mate_ref_id(&self) -> i32 {
        self.mate_ref_id
    }

    /// Returns 0-based left-most aligned reference position for the pair record.
    /// Same as *PNEXT - 1* in SAM specification.
    /// Returns -1 for unmapped records and records without a pair.
    pub fn mate_start(&self) -> i32 {
        self.mate_start
    }

    /// Observed template length (TLEN in SAM specification).
    pub fn template_len(&self) -> i32 {
        self.template_len
    }

    /// Returns [TagViewer](tags/struct.TagViewer.html), which provides operations of tags.
    pub fn tags(&self) -> &tags::TagViewer {
        &self.tags
    }

    /// Returns mutable [TagViewer](tags/struct.TagViewer.html), which provides operations of tags.
    pub fn tags_mut(&mut self) -> &mut tags::TagViewer {
        &mut self.tags
    }

    /// Write the record in SAM format to `f`. The function needs
    /// [header](../header/struct.Header.html), as the record itself does not store reference
    /// names.
    pub fn write_sam<W: Write>(&self, f: &mut W, header: &Header) -> io::Result<()> {
        f.write_all(&self.name)?;
        write!(f, "\t{}\t", self.flag.0)?;
        if self.ref_id < 0 {
            f.write_all(b"*\t")?;
        } else {
            write!(f, "{}\t", header.reference_name(self.ref_id as u32)
                .ok_or_else(|| io::Error::new(InvalidData, "Record has a reference id not in the header"))?)?;
        }
        write!(f, "{}\t{}\t", self.start + 1, self.mapq)?;
        self.cigar.write_readable(f)?;

        if self.mate_ref_id < 0 {
            f.write_all(b"\t*\t")?;
        } else if self.mate_ref_id == self.ref_id {
            f.write_all(b"\t=\t")?;
        } else {
            write!(f, "\t{}\t", header.reference_name(self.mate_ref_id as u32)
                .ok_or_else(|| io::Error::new(InvalidData, "Record has a reference id not in the header"))?)?;
        }
        write!(f, "{}\t{}\t", self.mate_start + 1, self.template_len)?;
        self.seq.write_readable(f)?;
        f.write_u8(b'\t')?;
        self.qual.write_readable(f)?;
        self.tags.write_sam(f)?;
        writeln!(f)
    }

    /// Writes a record in BAM format.
    pub fn write_bam<W: Write>(&self, stream: &mut W) -> io::Result<()> {
        let raw_cigar_len = if self.cigar.len() <= 0xffff {
            4 * self.cigar.len()
        } else {
            16 + 4 * self.cigar.len()
        };
        let total_block_len = 32 + self.name.len() + 1 + raw_cigar_len
            + self.seq.raw().len() + self.seq.len() + self.tags.raw().len();
        stream.write_i32::<LittleEndian>(total_block_len as i32)?;

        stream.write_i32::<LittleEndian>(self.ref_id)?;
        stream.write_i32::<LittleEndian>(self.start)?;
        stream.write_u8(self.name.len() as u8 + 1)?;
        stream.write_u8(self.mapq)?;
        stream.write_u16::<LittleEndian>(self.calculate_bin())?;

        if self.cigar.len() <= 0xffff {
            stream.write_u16::<LittleEndian>(self.cigar.len() as u16)?;
        } else {
            stream.write_u16::<LittleEndian>(2)?;
        }

        stream.write_u16::<LittleEndian>(self.flag.0)?;
        stream.write_i32::<LittleEndian>(self.seq.len() as i32)?;
        stream.write_i32::<LittleEndian>(self.mate_ref_id)?;
        stream.write_i32::<LittleEndian>(self.mate_start)?;
        stream.write_i32::<LittleEndian>(self.template_len)?;

        stream.write_all(&self.name)?;
        stream.write_u8(0)?;
        if self.cigar.len() <= 0xffff {
            for &el in self.cigar.raw() {
                stream.write_u32::<LittleEndian>(el)?;
            }
        } else {
            let seq_len = if self.seq.available() {
                self.seq.len() as u32
            } else {
                self.cigar.calculate_query_len()
            };
            stream.write_u32::<LittleEndian>(seq_len << 4 | 4)?;
            let ref_len = (self.calculate_end() - self.start) as u32;
            stream.write_u32::<LittleEndian>(ref_len << 4 | 3)?;
        }

        stream.write_all(&self.seq.raw())?;
        if self.qual.raw().len() == self.seq.len() {
            stream.write_all(&self.qual.raw())?;
        } else {
            write_iterator(stream, std::iter::repeat(0xff).take(self.seq.len()))?;
        }

        stream.write_all(self.tags.raw())?;

        if self.cigar.len() > 0xffff {
            stream.write_all(b"CGBI")?;
            stream.write_i32::<LittleEndian>(self.cigar.len() as i32)?;
            for &el in self.cigar.raw() {
                stream.write_u32::<LittleEndian>(el)?;
            }
        }
        Ok(())
    }

    /// Sets record name (only first 254 letters will be used).
    pub fn set_name<T: IntoIterator<Item = u8>>(&mut self, name: T) {
        self.name.clear();
        self.name.extend(name.into_iter().take(254));
    }

    /// Returns record [flag](struct.Flag.html).
    ///
    /// It supports predicates like `record.flag().is_paired()`. You can compare with flags directly
    /// using `record.flag().0 & RECORD_PAIRED != 0`.
    pub fn flag(&self) -> Flag {
        self.flag
    }

    /// Returns mutable record [flag](struct.Flag.html).
    ///
    /// It can be changed like `record.flag_mut().set_paired(true)`.
    /// You can change it directly `record.flag_mut().0 |= RECORD_PAIRED`. You can also you
    /// [set_flag](#method.set_flag).
    pub fn flag_mut(&mut self) -> &mut Flag {
        &mut self.flag
    }

    pub fn set_flag(&mut self, flag: u16) {
        self.flag.0 = flag;
    }

    /// Sets reference id. Panics if less than -1. This function does not update record flag.
    pub fn set_ref_id(&mut self, ref_id: i32) {
        assert!(ref_id >= -1, "Reference id < -1");
        self.ref_id = ref_id;
    }

    /// Sets record 0-based start. Panics if less than -1.
    ///
    /// If the end position was already calculated, it is updated.
    pub fn set_start(&mut self, start: i32) {
        assert!(start >= -1, "Start < -1");
        let difference = start - self.start;
        self.start = start;
        if self.end.get() != 0 {
            *self.end.get_mut() += difference;
        }
        self.bin.set(BIN_UNKNOWN);
    }

    pub fn set_mapq(&mut self, mapq: u8) {
        self.mapq = mapq;
    }

    /// Sets reference id of the mate record.
    /// Panics if argument is less than -1. This function does not update record flag.
    pub fn set_mate_ref_id(&mut self, mate_ref_id: i32) {
        assert!(mate_ref_id >= -1, "Mate reference id < -1");
        self.mate_ref_id = mate_ref_id;
    }

    /// Sets record 0-based start of the mate record. Panics if less than -1.
    pub fn set_mate_start(&mut self, mate_start: i32) {
        assert!(mate_start >= -1, "Mate start < -1");
        self.mate_start = mate_start;
    }

    pub fn set_template_len(&mut self, template_len: i32) {
        self.template_len = template_len;
    }

    /// Sets a sequence and qualities for a record. If you do not need to set qualities, use
    /// `std::iter::empty` for `qualities`. If qualities are non-empty,
    /// both iterators should have the same length.
    ///
    /// # Arguments
    /// * sequence - in text format (for example `b"ACGT"`),
    /// * qualities - in raw format (without +33 added).
    ///
    /// If the function returns an error, the sequence and qualities are cleared.
    pub fn set_seq_qual<T, U>(&mut self, sequence: T, qualities: U) -> Result<(), String>
    where T: IntoIterator<Item = u8>,
          U: IntoIterator<Item = u8>,
    {
        self.seq.clear();
        if let Err(e) = self.seq.extend_from_text(sequence) {
            self.seq.clear();
            self.qual.clear();
            return Err(e);
        }
        self.qual.clear();
        self.qual.extend_from_raw(qualities);
        if self.qual.available() && self.seq.len() != self.qual.len() {
            let err = Err(format!("Trying to set sequence and qualities of different lengths: \
                {} and {}", self.seq.len(), self.qual.len()));
            self.seq.clear();
            self.qual.clear();
            err
        } else {
            Ok(())
        }
    }

    /// Sets raw sequence and qualities for a record. If you do not need to set qualities, use
    /// `std::iter::empty` for `qualities`.
    ///
    /// # Arguments
    /// * sequence - in raw format: each nucleotide takes 4 bits,
    /// * qualities - in raw format (without +33 added).
    /// * len - number of nucleotides. If qualities are non-empty, they should have the same
    /// number of bytes.
    ///
    /// If the function returns an error, the sequence and qualities are cleared.
    pub fn set_raw_seq_qual<U>(&mut self, raw_seq: &[u8], qualities: U, len: usize)
        -> Result<(), String>
    where U: IntoIterator<Item = u8>,
    {
        self.seq.clear();
        self.qual.clear();
        self.qual.extend_from_raw(qualities);
        if self.qual.available() && self.qual.len() != len {
            self.qual.clear();
            return Err(format!("Expected qualities length: {}, got {}", len, self.qual.len()));
        }

        if (len + 1) / 2 != raw_seq.len() {
            self.seq.clear();
            self.qual.clear();
            return Err(format!("Expected raw sequence length: {}, got {}",
                (len + 1) / 2, raw_seq.len()));
        }

        let mut slice = &raw_seq[..];
        self.seq.fill_from(&mut slice, len).unwrap();
        Ok(())
    }

    /// Sets record cigar from u8 iterator. This resets end position and BAI bin.
    pub fn set_cigar<I: IntoIterator<Item = u8>>(&mut self, cigar: I) -> Result<(), String> {
        self.end.set(0);
        self.bin.set(BIN_UNKNOWN);
        self.cigar.clear();
        self.cigar.extend_from_text(cigar)
    }

    /// Sets raw record cigar. This resets end position and BAI bin.
    pub fn set_raw_cigar<I: IntoIterator<Item = u32>>(&mut self, cigar: I) {
        self.end.set(0);
        self.bin.set(BIN_UNKNOWN);
        self.cigar.clear();
        self.cigar.extend_from_raw(cigar);
    }

    /// Returns an iterator over pairs `(Option<u32>, Option<u32>)`.
    /// The first element contains a sequence index, and the second element contains a
    /// reference index. If the current operation is an insertion or a deletion, the respective
    /// element will be `None.`
    ///
    /// If the record is unmapped, returns an empty iterator.
    pub fn aligned_pairs(&self) -> cigar::AlignedPairs {
        self.cigar.aligned_pairs(self.start as u32)
    }

    /// Returns an iterator over pairs `(u32, u32)`.
    /// The first element represents a sequence index, and the second element represents a
    /// reference index. This iterator skips insertions and deletions.
    ///
    /// If the record is unmapped, returns an empty iterator.
    pub fn matching_pairs(&self) -> cigar::MatchingPairs {
        self.cigar.matching_pairs(self.start as u32)
    }

    /// Returns an iterator over [AlignmentEntry](struct.AlignmentEntry.html), which stores information
    /// about a single position in the record-reference alignment.
    ///
    /// The function returns [EntriesError](enum.EntriesError.html)
    /// if the record does not have a sequence or an MD tag.
    ///
    /// ```rust
    /// for entry in record.alignment_entries().unwrap() {
    ///     if let Some((record_pos, record_nt)) = entry.record_pos_nt() {
    ///         print!("{} {}", record_pos, record_nt as char);
    ///     } else {
    ///         print!("-");
    ///     }
    ///     print!(", ");
    ///     if let Some((ref_pos, ref_nt)) = entry.ref_pos_nt() {
    ///         println!("{} {}", ref_pos, ref_nt as char);
    ///     } else {
    ///         println!("-");
    ///     }
    /// }
    /// ```
    ///
    /// Iterator may panic if the MD tag does not match the alignment.
    pub fn alignment_entries(&self) -> Result<EntriesIter, EntriesError> {
        if !self.sequence().available() {
            return Err(EntriesError::NoSequence);
        }
        let md_tag = match self.tags().get(b"MD") {
            Some(tags::TagValue::String(tag, _)) => tag,
            Some(_) => return Err(EntriesError::IncorrectMD),
            None => return Err(EntriesError::NoMD),
        };

        Ok(EntriesIter {
            parent: self,
            aligned_pairs: self.cigar.aligned_pairs(self.start as u32),
            md_tag,
            md_remaining_len: 0,
            md_index: 0,
        })
    }
}

impl fmt::Debug for Record {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{} (len {}) aligned to [{}]:{}-{}",
            std::str::from_utf8(&self.name).unwrap_or("*NAME NOT UTF-8*"), self.query_len(),
            self.ref_id(), self.start() + 1, self.calculate_end())
    }
}

/// An error that can arise from [alignment_entries](struct.Record.html#method.alignment_entries).
///
/// Variants:
/// * NoSequence - the record has no sequence,
/// * NoMD - the record has no MD tag,
/// * IncorrectMD - the record has an MD tag, but it has an unexpected type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntriesError {
    NoSequence,
    NoMD,
    IncorrectMD,
}

const MISSING: u32 = std::u32::MAX;

/// Contains single position in the alignment between the record and the reference.
#[derive(Clone)]
pub struct AlignmentEntry {
    record_pos: u32,
    record_nt: u8,
    ref_pos: u32,
    ref_nt: u8,
}

impl AlignmentEntry {
    /// Returns record position unless within a deletion.
    pub fn record_pos(&self) -> Option<u32> {
        if self.record_pos != MISSING {
            Some(self.record_pos)
        } else {
            None
        }
    }

    /// Returns record nucleotide unless within a deletion.
    pub fn record_nt(&self) -> Option<u8> {
        if self.record_pos != MISSING {
            Some(self.record_nt)
        } else {
            None
        }
    }

    /// Returns record position and nucleotide unless within a deletion.
    pub fn record_pos_nt(&self) -> Option<(u32, u8)> {
        if self.record_pos != MISSING {
            Some((self.record_pos, self.record_nt))
        } else {
            None
        }
    }

    /// Returns reference position unless within an insertion.
    pub fn ref_pos(&self) -> Option<u32> {
        if self.ref_pos != MISSING {
            Some(self.ref_pos)
        } else {
            None
        }
    }

    /// Returns reference nucleotide unless within an insertion.
    pub fn ref_nt(&self) -> Option<u8> {
        if self.ref_pos != MISSING {
            Some(self.ref_nt)
        } else {
            None
        }
    }

    /// Returns reference position and nucleotide unless within an insertion.
    pub fn ref_pos_nt(&self) -> Option<(u32, u8)> {
        if self.ref_pos != MISSING {
            Some((self.ref_pos, self.ref_nt))
        } else {
            None
        }
    }

    /// Returns `true` if the current position is an insertion.
    pub fn is_insertion(&self) -> bool {
        self.ref_pos == MISSING
    }

    /// Returns `true` if the current position is a deletion.
    pub fn is_deletion(&self) -> bool {
        self.record_pos == MISSING
    }

    /// Returns `true` if the current position is an alignment match:
    /// both nucleotides are present but not necessarily equal.
    pub fn is_aln_match(&self) -> bool {
        self.record_pos != MISSING && self.ref_pos != MISSING
    }

    /// Returns `true` if the current position is a sequence match:
    /// both nucleotides are present and equal.
    pub fn is_seq_match(&self) -> bool {
        self.record_nt == self.ref_nt
    }
}

impl fmt::Debug for AlignmentEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.record_pos != MISSING {
            write!(f, "{}: {}, ", self.record_pos, self.record_nt as char)?;
        } else {
            write!(f, "- , ")?;
        }
        if self.ref_pos != MISSING {
            write!(f, "{}: {}", self.ref_pos, self.ref_nt as char)
        } else {
            write!(f, " -")
        }
    }
}

/// Iterator over [alignment positions](struct.AlignmentEntry.html).
#[derive(Clone)]
pub struct EntriesIter<'a> {
    parent: &'a Record,
    aligned_pairs: cigar::AlignedPairs<'a>,
    md_tag: &'a [u8],
    md_remaining_len: u32,
    md_index: usize,
}

impl<'a> EntriesIter<'a> {
    fn curr_ref_nt(&mut self, record_nt: Option<u8>) -> u8 {
        let start_with_remaining_md = self.md_remaining_len > 0;
        while !start_with_remaining_md && self.md_index < self.md_tag.len() {
            let curr_char = self.md_tag[self.md_index];
            self.md_index += 1;
            if curr_char >= b'0' && curr_char <= b'9' {
                self.md_remaining_len = self.md_remaining_len * 10 + (curr_char - b'0') as u32;
                continue;
            }

            if self.md_remaining_len > 0 {
                self.md_index -= 1;
                break;
            }
            if curr_char == b'^' {
                assert!(record_nt.is_none(),
                    "Record {}: Failed to parse MD tag: {}. MD tag indicates deletion, but CIGAR does not",
                    from_utf8(self.parent.name()).unwrap_or("*NAME NON UTF-8*"),
                    from_utf8(self.md_tag).unwrap_or("*MD TAG NON UTF-8*"));
            } else {
                assert!(curr_char.is_ascii_uppercase(),
                    "Record {}: Failed to parse MD tag: {}. Unexpected MD char: {}",
                    from_utf8(self.parent.name()).unwrap_or("*NAME NON UTF-8*"),
                    from_utf8(self.md_tag).unwrap_or("*MD TAG NON UTF-8*"), curr_char as char);
                return curr_char;
            }
        }

        if self.md_remaining_len > 0 {
            self.md_remaining_len -= 1;
            return record_nt.unwrap_or_else(|| panic!(
                "Record {}: Failed to parse MD tag: {}. Reference should match record within deletion",
                from_utf8(self.parent.name()).unwrap_or("*NAME NON UTF-8*"),
                from_utf8(self.md_tag).unwrap_or("*MD TAG NON UTF-8*")));
        }
        panic!("Record {}: Failed to parse MD tag: {}. Reached the end of MD tag",
            from_utf8(self.parent.name()).unwrap_or("*NAME NON UTF-8*"),
            from_utf8(self.md_tag).unwrap_or("*MD TAG NON UTF-8*"));
    }
}

impl<'a> Iterator for EntriesIter<'a> {
    type Item = AlignmentEntry;

    fn next(&mut self) -> Option<Self::Item> {
        let pair = self.aligned_pairs.next()?;
        match pair {
            (Some(record_pos), Some(ref_pos)) => {
                let record_nt = self.parent.sequence().at(record_pos as usize);
                let ref_nt = self.curr_ref_nt(Some(record_nt));
                return Some(AlignmentEntry { record_pos, record_nt, ref_pos, ref_nt });
            },
            (Some(record_pos), None) => {
                let record_nt = self.parent.sequence().at(record_pos as usize);
                return Some(AlignmentEntry {
                    record_pos, record_nt,
                    ref_pos: MISSING,
                    ref_nt: 0,
                });
            },
            (None, Some(ref_pos)) => {
                let ref_nt = self.curr_ref_nt(None);
                return Some(AlignmentEntry {
                    ref_pos, ref_nt,
                    record_pos: MISSING,
                    record_nt: 0,
                });
            },
            (None, None) => unreachable!(),
        }
    }
}
