//! Contains an implementation of accumulating reader.
//!
//! See `AccReader` documentation for more information and examples.

use std::cmp;
use std::io::{self, BufRead, Read, Seek, SeekFrom, Write};
use std::ptr;

/// Default capacity for the internal buffer of `AccReader`.
pub const DEFAULT_BUF_CAPACITY: usize = 4096;

/// Default increment for the internal buffer of `AccReader`.
pub const DEFAULT_BUF_INCREMENT: usize = 1024;

/// An accumulating reader which provides `Seek` for any `Read`.
///
/// An accumulating reader wraps an instance of `std::io::Read` trait and provides
/// implementations of `std::io::Read`, `std::io::BufRead` and `std::io::Seek` which use
/// the wrapped `Read` as a source.
///
/// This struct keeps an internal buffer which contains everything read so far from the
/// wrapped stream and allows "revisiting" the previously read data through the `Seek` interface.
/// When the user needs to seek beyond what was read from the stream, the accumulating reader
/// will automatically read the necessary number of bytes from the wrapped stream to fulfill
/// the request, if possible.
///
/// Seeking to beyond the end of the underlying stream is not possible and will result in
/// an error. Seeking using `SeekFrom::End`, naturally, involves buffering the whole underlying
/// stream, therefore it will either hang with blocking infinite streams like sockets or will fill
/// up all of the available memory with truly infinite streams.
///
/// This struct will buffer all of the underlying stream in order to provide seeking, therefore
/// you should discard it as soon as you don't need it if you are working with large streams
/// of data.
///
/// `AccReader` is parameterized by two values, initial capacity and increment. Initial capacity
/// defines the initial size of the internal buffer. This buffer automatically grows with each
/// successful read operation, if necessary, by the number of bytes read. If `BufRead` interface
/// is used, however, increment value is used to expand the internal buffer capacity when it is
/// filled.
///
/// # Examples
///
/// ```no_run
/// use std::io::{self, Read, Seek, SeekFrom};
///
/// use acc_reader::AccReader;
///
/// let mut ar = AccReader::new(io::stdin());
///
/// // read everything starting from the 12th byte
/// // will panic if the input contains less than 12 bytes
/// ar.seek(SeekFrom::Start(12)).unwrap();
/// let mut input = Vec::new();
/// ar.read_to_end(&mut input).unwrap();
/// ```
///
/// It is also possible to seek starting from the end of stream, but this requires reading
/// the whole stream to the end:
///
/// ```no_run
/// use std::io::{self, Read, Seek, SeekFrom};
///
/// use acc_reader::AccReader;
///
/// let mut ar = AccReader::new(io::stdin());
///
/// // read last 12 bytes
/// // will panic if the input contains less than 12 bytes
/// ar.seek(SeekFrom::End(-12)).unwrap();
/// let mut input = Vec::new();
/// ar.read_to_end(&mut input).unwrap();
/// ```
pub struct AccReader<R: Read> {
    source: R,
    buf: Vec<u8>,
    // invariant: pos <= buf.len()
    pos: usize,
    inc: usize,
}

impl<R: Read> AccReader<R> {
    /// Creates a new accumulating reader from the provided `Read` instance.
    ///
    /// Default values for the initial buffer capacity and increment are used.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::io;
    ///
    /// use acc_reader::AccReader;
    ///
    /// let input = io::stdin();
    /// let mut ar = AccReader::new(input);
    /// ```
    #[inline]
    pub fn new(source: R) -> AccReader<R> {
        AccReader::with_initial_capacity_and_increment(
            DEFAULT_BUF_CAPACITY,
            DEFAULT_BUF_INCREMENT,
            source,
        )
    }

    /// Creates a new accumulating reader from the provided `Read` instance with the specified
    /// initial capacity for the internal buffer.
    ///
    /// The default value for the buffer increment is used.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::io;
    ///
    /// use acc_reader::AccReader;
    ///
    /// let input = io::stdin();
    /// let mut ar = AccReader::with_initial_capacity(512, input);
    /// ```
    #[inline]
    pub fn with_initial_capacity(cap: usize, source: R) -> AccReader<R> {
        AccReader::with_initial_capacity_and_increment(cap, DEFAULT_BUF_INCREMENT, source)
    }

    /// Creates a new accumulating reader from the provided `Read` instance with the specified
    /// increment for the internal buffer.
    ///
    /// The default value for the initial capacity is used.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::io;
    ///
    /// use acc_reader::AccReader;
    ///
    /// let input = io::stdin();
    /// let mut ar = AccReader::with_increment(128, input);
    /// ```
    #[inline]
    pub fn with_increment(inc: usize, source: R) -> AccReader<R> {
        AccReader::with_initial_capacity_and_increment(DEFAULT_BUF_CAPACITY, inc, source)
    }

    /// Creates a new accumulating reader from the provided `Read` instance with the specified
    /// increment and initial capacity for the internal buffer.
    ///
    /// Initial capacity determines the initial size of the internal buffer. The increment
    /// is only needed if `BufRead` interface is used, and it defined the buffer expansion
    /// size when `fill_buf()` is called and no more space in the buffer is available.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::io;
    ///
    /// use acc_reader::AccReader;
    ///
    /// let input = io::stdin();
    /// let mut ar = AccReader::with_initial_capacity_and_increment(512, 128, input);
    /// ```
    #[inline]
    pub fn with_initial_capacity_and_increment(cap: usize, inc: usize, source: R) -> AccReader<R> {
        AccReader {
            source: source,
            buf: Vec::with_capacity(cap),
            pos: 0,
            inc: inc,
        }
    }

    /// Unwraps this accumulating reader, returning the underlying `BufRead` instance.
    ///
    /// Note that any accumulated data will be lost.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::io;
    ///
    /// use acc_reader::AccReader;
    ///
    /// let input = io::stdin();
    /// let mut ar = AccReader::new(input);
    ///
    /// let input2 = ar.into_inner();
    /// ```
    #[inline]
    pub fn into_inner(self) -> R {
        self.source
    }

    // Read from the stream into the internal buffer as much as possible,
    // but no more than the provided number of bytes.
    // Updates the buffer length to the actual number of bytes read, even
    // in case of errors.
    fn read_up_to(&mut self, n: u64) -> io::Result<()> {
        let old_len = self.buf.len();
        self.buf.reserve(n as usize);
        unsafe {
            self.buf.set_len(old_len + n as usize);
        }

        let mut error = None;
        let mut read = 0;
        {
            let mut target = &mut self.buf[old_len..];
            while !target.is_empty() {
                match self.source.read(target) {
                    Ok(0) => break,
                    Ok(n) => {
                        read += n;
                        let tmp = target;
                        target = &mut tmp[n..];
                    }
                    Err(ref e) if e.kind() == io::ErrorKind::Interrupted => {}
                    Err(e) => {
                        error = Some(e);
                        break;
                    }
                }
            }
        }
        unsafe {
            self.buf.set_len(old_len + read as usize);
        }

        if let Some(e) = error {
            Err(e)
        } else {
            Ok(())
        }
    }
}

impl<R: Read> Read for AccReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let need_to_read = cmp::min(self.buf.len() - self.pos, buf.len());
        if need_to_read > 0 {
            unsafe {
                ptr::copy_nonoverlapping(
                    self.buf.as_ptr().offset(self.pos as isize),
                    buf.as_mut_ptr(),
                    need_to_read,
                );
            }
            self.pos += need_to_read;
            Ok(need_to_read)
        } else {
            // need_to_read == 0
            let read = self.source.read(buf)?;
            let _ = self.buf.write_all(&buf[..read]);
            self.pos += read;
            Ok(read)
        }
    }
}

impl<R: Read> BufRead for AccReader<R> {
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        let available = self.buf.len() - self.pos; // self.buf.len() >= pos
        if available == 0 {
            let old_len = self.buf.len();
            self.buf.reserve(self.inc);
            unsafe {
                self.buf.set_len(old_len + self.inc);
            }

            let (read, error) = match self.source.read(&mut self.buf[self.pos..]) {
                Ok(n) => (n, None),
                Err(e) => (0, Some(e)),
            };
            unsafe {
                self.buf.set_len(old_len + read);
            }

            if let Some(e) = error {
                Err(e)
            } else {
                Ok(&self.buf[self.pos..])
            }
        } else {
            Ok(&self.buf[self.pos..])
        }
    }

    fn consume(&mut self, amt: usize) {
        self.pos = cmp::min(self.pos + amt, self.buf.len());
    }
}

impl<R: Read> Seek for AccReader<R> {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        match pos {
            SeekFrom::End(n) => {
                if n > 0 {
                    Err(io::Error::new(
                        io::ErrorKind::UnexpectedEof,
                        "seeking beyond end of stream",
                    ))
                } else {
                    // just read everything that's left and seek from that
                    self.source.read_to_end(&mut self.buf)?;

                    let d = (-n) as u64;
                    if d > self.buf.len() as u64 {
                        Err(io::Error::new(
                            io::ErrorKind::InvalidInput,
                            "seeking before the begining of stream",
                        ))
                    } else {
                        self.pos = (self.buf.len() as u64 - d) as usize;
                        Ok(self.pos as u64)
                    }
                }
            }
            SeekFrom::Start(n) if n <= self.buf.len() as u64 => {
                self.pos = n as usize;
                Ok(self.pos as u64)
            }
            SeekFrom::Start(n) => {
                // n > self.buf.len()
                let need_to_read = n - self.buf.len() as u64;
                self.read_up_to(need_to_read)?;
                if n > self.buf.len() as u64 {
                    // still not enough
                    Err(io::Error::new(
                        io::ErrorKind::UnexpectedEof,
                        "seeking beyond end of stream",
                    ))
                } else {
                    self.pos = n as usize;
                    Ok(n)
                }
            }
            SeekFrom::Current(0) => Ok(self.pos as u64),
            SeekFrom::Current(n) if n < 0 => {
                let d = (-n) as u64;
                if d > self.pos as u64 {
                    Err(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        "seeking before the beginning of stream",
                    ))
                } else {
                    self.pos = (self.pos as u64 - d) as usize;
                    Ok(self.pos as u64)
                }
            }
            SeekFrom::Current(n) => {
                // n > 0
                let new_pos = self.pos as u64 + n as u64;
                if new_pos > self.buf.len() as u64 {
                    let need_to_read = new_pos - self.buf.len() as u64;
                    self.read_up_to(need_to_read)?;
                    if new_pos > self.buf.len() as u64 {
                        // still not enough
                        Err(io::Error::new(
                            io::ErrorKind::UnexpectedEof,
                            "seeking beyond end of stream",
                        ))
                    } else {
                        self.pos = new_pos as usize;
                        Ok(new_pos)
                    }
                } else {
                    self.pos = new_pos as usize;
                    Ok(self.pos as u64)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::io::{self, BufRead, Read, Seek, SeekFrom};

    use super::*;

    #[test]
    fn test_acc_reader_read() {
        let inner: &[u8] = &[5, 6, 7, 0, 1, 2, 3];
        let mut reader = AccReader::new(inner);

        let mut buf = [0, 0];
        assert_eq!(reader.read(&mut buf).unwrap(), 2);
        assert_eq!(buf, [5, 6]);
        assert_eq!(reader.read(&mut buf).unwrap(), 2);
        assert_eq!(buf, [7, 0]);
        assert_eq!(reader.read(&mut buf).unwrap(), 2);
        assert_eq!(buf, [1, 2]);
        assert_eq!(reader.read(&mut buf).unwrap(), 1);
        assert_eq!(buf[0], 3);
    }

    #[test]
    fn test_acc_reader_buf_read() {
        let inner: &[u8] = &[5, 6, 7, 0, 1, 2, 3, 4];
        let mut reader = AccReader::with_initial_capacity_and_increment(3, 3, inner);

        assert_eq!(reader.fill_buf().ok(), Some(&[5, 6, 7][..]));
        reader.consume(3);
        assert_eq!(reader.fill_buf().ok(), Some(&[0, 1, 2][..]));
        reader.consume(3);
        assert_eq!(reader.fill_buf().ok(), Some(&[3, 4][..]));
        reader.consume(2);
        assert_eq!(reader.fill_buf().ok(), Some(&[][..]));
    }

    #[test]
    fn test_acc_reader_seek() {
        let inner: &[u8] = &[5, 6, 7, 0, 1, 2, 3, 4];
        let mut reader = AccReader::new(inner);

        let mut buf = [0, 0];

        assert_eq!(reader.seek(SeekFrom::Start(2)).unwrap(), 2);
        reader.read_exact(&mut buf).unwrap();
        assert_eq!(buf, [7, 0]);

        assert_eq!(reader.seek(SeekFrom::Current(-1)).unwrap(), 3);
        reader.read_exact(&mut buf).unwrap();
        assert_eq!(buf, [0, 1]);

        assert_eq!(reader.seek(SeekFrom::End(-3)).unwrap(), 5);
        reader.read_exact(&mut buf).unwrap();
        assert_eq!(buf, [2, 3]);

        // seek to beyond the end and to before the start
        assert_eq!(
            reader.seek(SeekFrom::End(3)).err().unwrap().kind(),
            io::ErrorKind::UnexpectedEof
        );
        assert_eq!(
            reader.seek(SeekFrom::Current(-128)).err().unwrap().kind(),
            io::ErrorKind::InvalidInput
        );

        // seek to the end from the start
        let mut reader = AccReader::new(inner);
        assert_eq!(
            reader.seek(SeekFrom::Start(inner.len() as u64)).unwrap(),
            inner.len() as u64
        );
        assert_eq!(reader.read(&mut buf).unwrap(), 0);

        // seek to beyond the end from the start
        let mut reader = AccReader::new(inner);
        assert_eq!(
            reader.seek(SeekFrom::Start(128)).err().unwrap().kind(),
            io::ErrorKind::UnexpectedEof
        );
    }
}
