//! BAM writer.

use std::io::{Write, Result};
use std::fs::File;
use std::path::Path;

use super::bgzip;
use super::{Header, Record, RecordWriter};

/// Builder of the [BamWriter](struct.BamWriter.html).
pub struct BamWriterBuilder {
    write_header: bool,
    level: u8,
    additional_threads: u16,
}

impl BamWriterBuilder {
    pub fn new() -> Self {
        Self {
            write_header: true,
            level: 6,
            additional_threads: 0,
        }
    }

    /// The option to write or skip header when creating the BAM writer (writing by default).
    pub fn write_header(&mut self, write: bool) -> &mut Self {
        self.write_header = write;
        self
    }

    /// Specify compression level from 0 to 9 (6 by default).
    pub fn compression_level(&mut self, level: u8) -> &mut Self {
        assert!(level <= 9, "Maximal compression level is 9");
        self.level = level;
        self
    }

    /// Specify the number of additional threads.
    /// Additional threads are used to compress blocks, while the
    /// main thread reads the writes to a file/stream.
    /// If `additional_threads` is 0 (default), the main thread
    /// will compress blocks itself.
    pub fn additional_threads(&mut self, additional_threads: u16) -> &mut Self {
        self.additional_threads = additional_threads;
        self
    }

    /// Creates a BAM writer from a file and a header.
    pub fn from_path<P: AsRef<Path>>(&mut self, path: P, header: Header)
            -> Result<BamWriter<File>> {
        let stream = File::create(path)?;
        self.from_stream(stream, header)
    }

    /// Creates a BAM writer from a stream and a header.
    pub fn from_stream<W: Write>(&mut self, stream: W, header: Header) -> Result<BamWriter<W>> {
        let mut writer = bgzip::Writer::build()
            .additional_threads(self.additional_threads)
            .compression_level(self.level)
            .from_stream(stream);
        if self.write_header {
            header.write_bam(&mut writer)?;
        }
        writer.flush_contents()?;
        Ok(BamWriter { writer, header })
    }
}

/// Bam writer. Can be created using [from_path](#method.from_path) or using
/// [BamWriterBuilder](struct.BamWriterBuilder.html).
///
/// Use [RecordWriter](../trait.RecordWriter.html) trait to write records.
pub struct BamWriter<W: Write> {
    writer: bgzip::Writer<W>,
    header: Header,
}

impl BamWriter<File> {
    /// Creates a [BamWriterBuilder](struct.BamWriterBuilder.html).
    pub fn build() -> BamWriterBuilder {
        BamWriterBuilder::new()
    }

    /// Creates a new `BamWriter` from a path and header.
    pub fn from_path<P: AsRef<Path>>(path: P, header: Header) -> Result<Self> {
        BamWriterBuilder::new().from_path(path, header)
    }
}

impl<W: Write> BamWriter<W> {
    /// Creates a new `BamWriter` from a stream and header.
    pub fn from_stream(stream: W, header: Header) -> Result<Self> {
        BamWriterBuilder::new().from_stream(stream, header)
    }

    /// Returns BAM header.
    pub fn header(&self) -> &Header {
        &self.header
    }

    /// Consumes the writer and returns inner stream.
    pub fn take_stream(self) -> W {
        self.writer.take_stream()
    }


    /// Pauses multi-thread writer until the next write operation. Does nothing to a single-thread writer.
    ///
    /// Use with caution: pausing and unpausing takes some time. Additionally, blocks that are compressed
    /// at the moment will finish compressing, but will not be written.
    /// All other blocks in the queue will not be compressed nor written.
    ///
    /// To compress and write all remaining blocks you can call [flush](#method.flush) before calling `pause`.
    pub fn pause(&mut self) {
        self.writer.pause();
    }
}

impl<W: Write> RecordWriter for BamWriter<W> {
    fn write(&mut self, record: &Record) -> Result<()> {
        record.write_bam(&mut self.writer)?;
        self.writer.end_context();
        Ok(())
    }

    fn finish(&mut self) -> Result<()> {
        self.writer.finish()
    }

    fn flush(&mut self) -> Result<()> {
        self.writer.flush()
    }
}
