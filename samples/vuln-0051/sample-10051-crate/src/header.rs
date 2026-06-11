//! SAM/BAM header.

use std::io::{Write, Read, Result, Error};
use std::io::ErrorKind::InvalidData;
use std::string::String;
use std::collections::{hash_map, HashMap};

use byteorder::{ReadBytesExt, WriteBytesExt, LittleEndian};

pub type TagName = [u8; 2];

/// A single tag in a line.
#[derive(Clone, Debug)]
pub struct Tag {
    name: TagName,
    value: String,
}

impl Tag {
    /// Creates a new tag.
    pub fn new(name: &TagName, value: String) -> Tag {
        Tag {
            name: name.clone(),
            value
        }
    }

    pub fn name(&self) -> &TagName {
        &self.name
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn value_mut(&mut self) -> &mut String {
        &mut self.value
    }

    /// Sets a new value and returns an old one.
    pub fn set_value(&mut self, new_value: String) -> String {
        std::mem::replace(&mut self.value, new_value)
    }

    /// Consumes tag and returns value.
    pub fn take_value(self) -> String {
        self.value
    }

    pub fn write<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_all(&self.name)?;
        writer.write_u8(b':')?;
        writer.write_all(self.value.as_bytes())
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum EntryType {
    HeaderLine,
    RefSequence,
    ReadGroup,
    Program,
}

impl EntryType {
    /// Returns record two-letter name of the type (such as HD or SQ).
    pub fn name(self) -> &'static TagName {
        use EntryType::*;
        match self {
            HeaderLine => b"HD",
            RefSequence => b"SQ",
            ReadGroup => b"RG",
            Program => b"PG",
        }
    }

    /// Returns entry type from two-letter name.
    pub fn from_name(name: &TagName) -> Option<Self> {
        use EntryType::*;
        match name {
            b"HD" => Some(HeaderLine),
            b"SQ" => Some(RefSequence),
            b"RG" => Some(ReadGroup),
            b"PG" => Some(Program),
            _ => None,
        }
    }
}

/// A single header line.
///
/// You can create a new entry using [header_line](#method.header_line),
/// [ref_sequence](#method.ref_sequence) and so on. After that you can modify line tags
/// using [push](#method.push), [remove](#method.remove) and others.
///
/// However, be careful not to delete the required tag, as well as check the required tag format.
#[derive(Clone, Debug)]
pub struct HeaderEntry {
    tags: Vec<Tag>,
    entry_type: EntryType,
}

impl HeaderEntry {
    fn new(entry_type: EntryType) -> HeaderEntry {
        HeaderEntry {
            tags: Vec::new(),
            entry_type,
        }
    }

    /// Creates a new @HD header entry.
    pub fn header_line(version: String) -> HeaderEntry {
        let mut res = HeaderEntry::new(EntryType::HeaderLine);
        res.push(b"VN", version);
        res
    }

    /// Creates a new @SQ header entry.
    pub fn ref_sequence(seq_name: String, seq_len: u32) -> HeaderEntry {
        let mut res = HeaderEntry::new(EntryType::RefSequence);
        res.push(b"SN", seq_name);
        res.push(b"LN", seq_len.to_string());
        res
    }

    /// Creates a new @RG header entry.
    pub fn read_group(ident: String) -> HeaderEntry {
        let mut res = HeaderEntry::new(EntryType::ReadGroup);
        res.push(b"ID", ident);
        res
    }

    /// Creates a new @PG header entry.
    pub fn program(ident: String) -> HeaderEntry {
        let mut res = HeaderEntry::new(EntryType::Program);
        res.push(b"ID", ident);
        res
    }

    pub fn entry_type(&self) -> EntryType {
        self.entry_type
    }

    /// Returns record two-letter name of the line (such as HD or SQ).
    pub fn entry_name(&self) -> &'static TagName {
        self.entry_type.name()
    }

    /// Returns an iterator over all tags.
    pub fn iter(&self) -> std::slice::Iter<Tag> {
        self.tags.iter()
    }

    /// Returns a tag with `name` if present. Takes `O(n_tags)`.
    pub fn get(&self, name: &TagName) -> Option<&str> {
        for tag in self.tags.iter() {
            if &tag.name == name {
                return Some(&tag.value);
            }
        }
        None
    }

    /// Returns a mutable tag with `name` if present. Takes `O(n_tags)`.
    pub fn get_mut(&mut self, name: &TagName) -> Option<&mut String> {
        for tag in self.tags.iter_mut() {
            if &tag.name == name {
                return Some(&mut tag.value);
            }
        }
        None
    }

    /// Pushes a tag to the end. Takes `O(1)`.
    pub fn push(&mut self, name: &TagName, value: String) -> &mut Self {
        self.tags.push(Tag::new(name, value));
        self
    }

    /// Replaces tag value if present and returns previous value. If there is no tag
    /// with the same name, pushes the new tag to the end. Takes `O(n_tags)`.
    pub fn insert(&mut self, name: &TagName, value: String) -> Option<String> {
        for tag in self.tags.iter_mut() {
            if &tag.name == name {
                return Some(tag.set_value(value));
            }
        }
        self.tags.push(Tag::new(name, value));
        None
    }

    /// Removes the tag, if present, and returns its value. Takes `O(n_tags)`.
    pub fn remove(&mut self, name: &TagName) -> Option<String> {
        for i in 0..self.tags.len() {
            if &self.tags[i].name == name {
                return Some(self.tags.remove(i).take_value());
            }
        }
        None
    }

    /// Returns the number of tags in the line.
    pub fn len(&self) -> usize {
        self.tags.len()
    }

    /// Write the whole entry in a line.
    pub fn write<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_u8(b'@')?;
        writer.write_all(self.entry_name())?;
        for tag in self.tags.iter() {
            writer.write_u8(b'\t')?;
            tag.write(writer)?;
        }
        Ok(())
    }

    pub fn parse_line(line: &str) -> Result<Self> {
        let split_line: Vec<_> = line.split('\t').collect();
        let entry_type = split_line[0].as_bytes();
        if entry_type.len() != 3 || entry_type[0] != b'@' {
            return Err(Error::new(InvalidData,
                format!("Invalid header tag: {}", split_line[0])));
        }
        let entry_type = EntryType::from_name(&[entry_type[1], entry_type[2]])
            .ok_or_else(|| Error::new(InvalidData,
                format!("Invalid header tag: {}", split_line[0])))?;
        let mut res = HeaderEntry::new(entry_type);

        for i in 1..split_line.len() {
            let tag = split_line[i].as_bytes();
            if tag.len() < 3 || tag[2] != b':' {
                return Err(Error::new(InvalidData,
                    format!("Invalid header tag: {} in line '{}'", split_line[i], line)));
            }
            // .expect(...) here as input is already &str.
            let tag_value = String::from_utf8(tag[3..].to_vec())
                .expect("Tag value not in UTF-8");
            res.push(&[tag[0], tag[1]], tag_value);
        }
        Ok(res)
    }
}

#[derive(Clone)]
pub enum HeaderLine {
    Entry(HeaderEntry),
    Comment(String),
}

/// BAM/SAM Header.
///
/// You can modify it by pushing new lines using [push_entry](#method.push_entry),
/// [push_comment](#method.push_line) and [push_line](#method.push_line).
///
/// You cannot remove lines, but you can create a new header and push there only a subset of lines.
#[derive(Clone)]
pub struct Header {
    lines: Vec<HeaderLine>,
    ref_names: Vec<String>,
    ref_lengths: Vec<u32>,
    ref_ids: HashMap<String, u32>,
}

impl Header {
    /// Creates an empty header.
    pub fn new() -> Header {
        Header {
            lines: Vec::new(),
            ref_names: Vec::new(),
            ref_lengths: Vec::new(),
            ref_ids: HashMap::new(),
        }
    }

    /// Iterator over lines.
    pub fn lines(&self) -> std::slice::Iter<HeaderLine> {
        self.lines.iter()
    }

    /// Pushes a new header entry.
    ///
    /// Returns an error if the same reference appears twice or @SQ line has an incorrect format.
    pub fn push_entry(&mut self, entry: HeaderEntry) -> std::result::Result<(), String> {
        if entry.entry_type() == EntryType::RefSequence {
            let name = entry.get(b"SN")
                .ok_or_else(|| "Header: @SQ entry does not have a SN tag".to_string())?.to_string();
            let len: u32 = entry.get(b"LN")
                .ok_or_else(|| "Header: @SQ entry does not have a LN tag".to_string())?
                .parse().map_err(|_| "Header: @SQ entry has a non-integer LN tag".to_string())?;
            if len <= 0 {
                return Err("Reference length must be positive".to_string());
            }
            match self.ref_ids.entry(name.clone()) {
                hash_map::Entry::Occupied(_) =>
                    return Err(format!("Reference {} appears twice in the reference", name)),
                hash_map::Entry::Vacant(v) => v.insert(self.ref_names.len() as u32),
            };
            self.ref_names.push(name.clone());
            self.ref_lengths.push(len);
        }
        self.lines.push(HeaderLine::Entry(entry));
        Ok(())
    }

    /// Pushes a new comment.
    pub fn push_comment(&mut self, comment: String) {
        self.lines.push(HeaderLine::Comment(comment));
    }

    /// Pushes a lines to the header.
    pub fn push_line(&mut self, line: &str) -> Result<()> {
        if line.starts_with("@CO") {
            let comment = line.splitn(2, '\t').skip(1).next()
                .ok_or_else(|| Error::new(InvalidData,
                    format!("Failed to parse comment line '{}'", line)))?;
            self.push_comment(comment.to_string());
        } else {
            self.push_entry(HeaderEntry::parse_line(line)?)
                .map_err(|e| Error::new(InvalidData, e))?;
        }
        Ok(())
    }

    /// Write header in SAM format.
    pub fn write_text<W: Write>(&self, writer: &mut W) -> Result<()> {
        for line in self.lines.iter() {
            match line {
                HeaderLine::Entry(entry) => entry.write(writer)?,
                HeaderLine::Comment(comment) => {
                    writer.write_all(b"@CO\t")?;
                    writer.write_all(comment.as_bytes())?;
                },
            }
            writeln!(writer)?;
        }
        Ok(())
    }

    /// Writes header in an uncompressed BAM format.
    pub fn write_bam<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_all(&[b'B', b'A', b'M', 1])?;
        let mut header_text = Vec::new();
        self.write_text(&mut header_text).expect("Failed to write BAM text to a vector");
        writer.write_i32::<LittleEndian>(header_text.len() as i32)?;
        writer.write_all(&header_text)?;
        writer.write_i32::<LittleEndian>(self.ref_names.len() as i32)?;
        for (name, len) in self.ref_names.iter().zip(self.ref_lengths.iter()) {
            writer.write_i32::<LittleEndian>(name.len() as i32 + 1)?;
            writer.write_all(name.as_bytes())?;
            writer.write_u8(0)?;
            writer.write_i32::<LittleEndian>(*len as i32)?;
        }
        Ok(())
    }

    /// Parse uncompressed BAM header, starting with magic `b"BAM\1"`.
    pub fn from_bam<R: Read>(stream: &mut R) -> Result<Self> {
        let mut magic = [0_u8; 4];
        stream.read_exact(&mut magic)?;
        if magic != [b'B', b'A', b'M', 1] {
            return Err(Error::new(InvalidData, "Input is not in BAM format"));
        }

        let l_text = stream.read_i32::<LittleEndian>()?;
        if l_text < 0 {
            return Err(Error::new(InvalidData, "BAM file corrupted: negative header length"));
        }
        let mut text = vec![0_u8; l_text as usize];
        stream.read_exact(&mut text)?;
        let text = String::from_utf8(text)
            .map_err(|_| Error::new(InvalidData, "BAM header is not in UTF-8"))?;
        let mut header = Header::new();

        for line in text.split('\n') {
            let line = line.trim_end();
            if line.is_empty() {
                continue;
            }
            header.push_line(line)?;
        }

        let n_refs = stream.read_i32::<LittleEndian>()?;
        if n_refs < 0 {
            return Err(Error::new(InvalidData,
                "BAM file corrupted: negative number of references"));
        }
        let n_refs = n_refs as usize;
        let no_header_refs = header.ref_names.len() == 0;
        if !no_header_refs && n_refs != header.ref_names.len() {
            return Err(Error::new(InvalidData,
                format!("BAM file corrupted: number of references does not match header text ({} != {})",
                n_refs, header.ref_names.len())));
        }

        for i in 0..n_refs {
            let l_name = stream.read_i32::<LittleEndian>()?;
            if l_name <= 0 {
                return Err(Error::new(InvalidData,
                    "BAM file corrupted: negative reference name length"));
            }
            let mut name = vec![0_u8; l_name as usize - 1];
            stream.read_exact(&mut name)?;
            let _null = stream.read_u8()?;
            let name = std::string::String::from_utf8(name)
                .map_err(|_| Error::new(InvalidData,
                    "BAM file corrupted: reference name not in UTF-8"))?;

            let l_ref = stream.read_i32::<LittleEndian>()?;
            if l_ref < 0 {
                return Err(Error::new(InvalidData,
                    "BAM file corrupted: negative reference length"));
            }

            if no_header_refs {
                header.push_entry(HeaderEntry::ref_sequence(name, l_ref as u32))
                    .map_err(|e| Error::new(InvalidData, e))?;
            } else if name != header.ref_names[i] || l_ref as u32 != header.ref_lengths[i] {
                return Err(Error::new(InvalidData,
                    format!("BAM file corrupted: plain text header does not match the list of BAM references: \
                    reference #{}: ({}, {}) != ({}, {})", i, header.ref_names[i], name, l_ref, header.ref_lengths[i])));
            }
        }
        Ok(header)
    }

    /// Returns the number of reference sequences in the BAM file.
    pub fn n_references(&self) -> usize {
        self.ref_names.len()
    }

    /// Returns the name of the reference with `ref_id` (0-based).
    /// Returns None if there is no such reference
    pub fn reference_name(&self, ref_id: u32) -> Option<&str> {
        if ref_id as usize > self.ref_names.len() {
            None
        } else {
            Some(&self.ref_names[ref_id as usize])
        }
    }

    /// Returns the length of the reference with `ref_id` (0-based).
    /// Returns None if there is no such reference
    pub fn reference_len(&self, ref_id: u32) -> Option<u32> {
        if ref_id as usize > self.ref_lengths.len() {
            None
        } else {
            Some(self.ref_lengths[ref_id as usize])
        }
    }

    /// Returns reference id from its name, if possible.
    pub fn reference_id(&self, ref_name: &str) -> Option<u32> {
        self.ref_ids.get(ref_name).cloned()
    }

    /// Returns reference names.
    pub fn reference_names(&self) -> &[String] {
        &self.ref_names
    }

    /// Returns reference names.
    pub fn reference_lengths(&self) -> &[u32] {
        &self.ref_lengths
    }
}