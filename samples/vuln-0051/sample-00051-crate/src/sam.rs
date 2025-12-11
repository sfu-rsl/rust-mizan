//! SAM reader and writer.
//!
//! Contains a [SAM reader](struct.SamReader.html) and [SAM writer](struct.SamWriter.html).
//! You can construct them as
//!
//! ```rust
//! let reader = SamReader::from_path("in.sam").unwrap();
//! let writer = SamWriter::from_path("out.sam", reader.header().clone()).unwrap();
//! ```
//!
//! The reader implements [RecordReader](../trait.RecordReader.html) trait,
//! and the writer implements [RecordWriter](../trait.RecordWriter.html) trait. See them for
//! more information.

use std::io::{Write, BufWriter, Result, BufReader, BufRead};
use std::fs::File;
use std::path::Path;

use super::{RecordReader, RecordWriter, Header, Record};

/// Builder of the [SamWriter](struct.SamWriter.html).
pub struct SamWriterBuilder {
    write_header: bool,
}

impl SamWriterBuilder {
    pub fn new() -> Self {
        Self {
            write_header: true,
        }
    }

    /// The option to write or skip header when creating the SAM writer (writing by default).
    pub fn write_header(&mut self, write: bool) -> &mut Self {
        self.write_header = write;
        self
    }

    /// Creates a SAM writer from a file and a header.
    pub fn from_path<P: AsRef<Path>>(&mut self, path: P, header: Header)
            -> Result<SamWriter<BufWriter<File>>> {
        let stream = BufWriter::new(File::create(path)?);
        self.from_stream(stream, header)
    }

    /// Creates a SAM writer from a stream and a header. Preferably the stream should be wrapped
    /// in a buffer writer, such as `BufWriter`.
    pub fn from_stream<W: Write>(&mut self, mut stream: W, header: Header) -> Result<SamWriter<W>> {
        if self.write_header {
            header.write_text(&mut stream)?;
        }
        Ok(SamWriter { stream, header })
    }
}

/// Writes records in SAM format.
///
/// Can be created as
/// ```rust
/// let writer = SamWriter::from_path("out.sam", header).unwrap();
/// ```
/// or using a [builder](struct.SamWriterBuilder.html)
/// ```rust
/// let writer = SamWriter::build()
///     .write_header(false)
///     .from_path("out.sam", header).unwrap();
/// ```
///
/// You can clone a [header](../header/struct.Header.html) from SAM/BAM reader or
/// create one yourself.
///
/// You need to import [RecordWriter](../trait.RecordWriter.html)
/// to write [records](../record/struct.Record.html):
/// ```rust
/// use bam::RecordWriter;
/// let mut writer = bam::SamWriter::from_path("out.sam", header).unwrap();
/// let mut record = bam::Record::new();
/// // Filling the record.
/// writer.write(&record).unwrap();
/// ```
pub struct SamWriter<W: Write> {
    stream: W,
    header: Header,
}

impl SamWriter<BufWriter<File>> {
    /// Create a [builder](struct.SamWriterBuilder.html).
    pub fn build() -> SamWriterBuilder {
        SamWriterBuilder::new()
    }

    /// Creates a SAM writer from a path and a header.
    pub fn from_path<P: AsRef<Path>>(path: P, header: Header) -> Result<Self> {
        SamWriterBuilder::new().from_path(path, header)
    }
}

impl<W: Write> SamWriter<W> {
    /// Creates a SAM writer from a stream and a header. Preferably the stream should be wrapped
    /// in a buffer writer, such as `BufWriter`.
    pub fn from_stream(stream: W, header: Header) -> Result<Self> {
        SamWriterBuilder::new().from_stream(stream, header)
    }

    /// Returns [header](../header/struct.Header.html).
    pub fn header(&self) -> &Header {
        &self.header
    }

    /// Flushes contents to output.
    pub fn flush(&mut self) -> Result<()> {
        self.stream.flush()
    }

    /// Consumes the writer and returns inner stream.
    pub fn take_stream(mut self) -> W {
        let _ignore = self.stream.flush();
        self.stream
    }
}

impl<W: Write> RecordWriter for SamWriter<W> {
    /// Writes a single record in SAM format.
    fn write(&mut self, record: &Record) -> Result<()> {
        record.write_sam(&mut self.stream, &self.header)
    }

    fn finish(&mut self) -> Result<()> {
        self.flush()
    }

    fn flush(&mut self) -> Result<()> {
        self.flush()
    }
}

/// Reads records from SAM format.
///
/// Can be opened as
/// ```rust
/// let reader = SamReader::from_path("in.sam").unwrap();
/// ```
///
/// You can iterate over records:
/// ```rust
/// let mut reader = bam::SamReader::from_path("in.sam").unwrap();
/// for record in reader {
///     let record = record.unwrap();
///     // Do something.
/// }
/// ```
/// You can use [RecordReader](../trait.RecordReader.html) trait to read records without excess
/// allocation.
pub struct SamReader<R: BufRead> {
    stream: R,
    header: Header,
    buffer: String,
}

impl SamReader<BufReader<File>> {
    /// Opens SAM reader from `path`.
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self> {
        let stream = BufReader::new(File::open(path)?);
        SamReader::from_stream(stream)
    }
}

impl<R: BufRead> SamReader<R> {
    /// Opens SAM reader from a buffered stream.
    pub fn from_stream(mut stream: R) -> Result<Self> {
        let mut header = Header::new();
        let mut buffer = String::new();
        loop {
            buffer.clear();
            if stream.read_line(&mut buffer)? == 0 {
                break;
            };
            if buffer.starts_with('@') {
                header.push_line(buffer.trim_end())?;
            } else {
                break;
            }
        }
        Ok(SamReader { stream, header, buffer })
    }

    /// Returns [header](../header/struct.Header.html).
    pub fn header(&self) -> &Header {
        &self.header
    }

    /// Consumes the reader and returns inner stream.
    pub fn take_stream(self) -> R {
        self.stream
    }
}

impl<R: BufRead> RecordReader for SamReader<R> {
    fn read_into(&mut self, record: &mut Record) -> Result<bool> {
        if self.buffer.is_empty() {
            return Ok(false);
        }
        let res = match record.fill_from_sam(self.buffer.trim(), &self.header) {
            Ok(()) => Ok(true),
            Err(e) => {
                record.clear();
                Err(e)
            },
        };
        self.buffer.clear();
        match self.stream.read_line(&mut self.buffer) {
            Ok(_) => res,
            Err(e) => res.or(Err(e)),
        }
    }

    /// Does nothing, as SAM readers are single-thread.
    fn pause(&mut self) {}
}

/// Iterator over records.
impl<R: BufRead> Iterator for SamReader<R> {
    type Item = Result<Record>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut record = Record::new();
        match self.read_into(&mut record) {
            Ok(true) => Some(Ok(record)),
            Ok(false) => None,
            Err(e) => Some(Err(e)),
        }
    }
}
