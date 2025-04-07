extern crate byteorder;

use std::{fmt, error, str};

use byteorder::{BigEndian, ByteOrder};

mod tables;

/// Available encoding character sets
#[derive(Clone, Copy, Debug)]
pub enum CharacterSet {
    /// The standard character set (uses `+` and `/`)
    Standard,
    /// The URL safe character set (uses `-` and `_`)
    UrlSafe
}

#[derive(Clone, Copy, Debug)]
pub enum LineEnding {
    LF,
    CRLF,
}

#[derive(Clone, Copy, Debug)]
pub enum LineWrap {
    NoWrap,
    Wrap(usize, LineEnding)
}

/// Contains configuration parameters for base64 encoding
#[derive(Clone, Copy, Debug)]
pub struct Config {
    /// Character set to use
    char_set: CharacterSet,
    /// True to pad output with `=` characters
    pad: bool,
    /// Remove whitespace before decoding, at the cost of an allocation
    strip_whitespace: bool,
    /// ADT signifying whether to linewrap output, and if so by how many characters and with what ending
    line_wrap: LineWrap,
}

impl Config {
    pub fn new(char_set: CharacterSet,
               pad: bool,
               strip_whitespace: bool,
               input_line_wrap: LineWrap) -> Config {
        let line_wrap = match input_line_wrap  {
            LineWrap::Wrap(0, _) => LineWrap::NoWrap,
            _ => input_line_wrap,
        };

        Config {
            char_set: char_set,
            pad: pad,
            strip_whitespace: strip_whitespace,
            line_wrap: line_wrap,
        }
    }
}

pub static STANDARD: Config = Config {
    char_set: CharacterSet::Standard,
    pad: true,
    strip_whitespace: false,
    line_wrap: LineWrap::NoWrap,
};

pub static MIME: Config = Config {
    char_set: CharacterSet::Standard,
    pad: true,
    strip_whitespace: true,
    line_wrap: LineWrap::Wrap(76, LineEnding::CRLF),
};

pub static URL_SAFE: Config = Config {
    char_set: CharacterSet::UrlSafe,
    pad: true,
    strip_whitespace: false,
    line_wrap: LineWrap::NoWrap,
};

pub static URL_SAFE_NO_PAD: Config = Config {
    char_set: CharacterSet::UrlSafe,
    pad: false,
    strip_whitespace: false,
    line_wrap: LineWrap::NoWrap,
};

///Encode arbitrary octets as base64.
///Returns a String.
///Convenience for `encode_config(input, base64::STANDARD);`.
///
///# Example
///
///```rust
///extern crate base64;
///
///fn main() {
///    let b64 = base64::encode(b"hello world");
///    println!("{}", b64);
///}
///```
pub fn encode<T: ?Sized + AsRef<[u8]>>(input: &T) -> String {
    encode_config(input, STANDARD)
}

///Encode arbitrary octets as base64.
///Returns a String.
///
///# Example
///
///```rust
///extern crate base64;
///
///fn main() {
///    let b64 = base64::encode_config(b"hello world~", base64::STANDARD);
///    println!("{}", b64);
///
///    let b64_url = base64::encode_config(b"hello internet~", base64::URL_SAFE);
///    println!("{}", b64_url);
///}
///```
pub fn encode_config<T: ?Sized + AsRef<[u8]>>(input: &T, config: Config) -> String {
    let mut buf = String::with_capacity(encoded_size(input.as_ref().len(), config));

    encode_config_buf(input, config, &mut buf);

    buf
}

/// calculate the base64 encoded string size, including padding
fn encoded_size(bytes_len: usize, config: Config) -> usize {
    let rem = bytes_len % 3;

    let complete_input_chunks = bytes_len / 3;
    let complete_output_chars = complete_input_chunks * 4;
    let printing_output_chars = if rem == 0 {
        complete_output_chars
    } else {
        complete_output_chars + 4
    };
    let line_ending_output_chars = match config.line_wrap {
        LineWrap::NoWrap => 0,
        LineWrap::Wrap(n, LineEnding::CRLF) => printing_output_chars / n * 2,
        LineWrap::Wrap(n, LineEnding::LF) => printing_output_chars / n,
    };

    return printing_output_chars + line_ending_output_chars;
}

///Encode arbitrary octets as base64.
///Writes into the supplied buffer to avoid allocations.
///
///# Example
///
///```rust
///extern crate base64;
///
///fn main() {
///    let mut buf = String::new();
///    base64::encode_config_buf(b"hello world~", base64::STANDARD, &mut buf);
///    println!("{}", buf);
///
///    buf.clear();
///    base64::encode_config_buf(b"hello internet~", base64::URL_SAFE, &mut buf);
///    println!("{}", buf);
///}
///```
pub fn encode_config_buf<T: ?Sized + AsRef<[u8]>>(input: &T, config: Config, buf: &mut String) {
    let input_bytes = input.as_ref();
    let ref charset = match config.char_set {
        CharacterSet::Standard => tables::STANDARD_ENCODE,
        CharacterSet::UrlSafe => tables::URL_SAFE_ENCODE,
    };

    // reserve to make sure the memory we'll be writing to with unsafe is allocated
    buf.reserve(encoded_size(input_bytes.len(), config));

    let orig_buf_len = buf.len();
    let mut fast_loop_output_buf_len = orig_buf_len;

    let input_chunk_len = 6;

    let last_fast_index = input_bytes.len().saturating_sub(8);

    // we're only going to insert valid utf8
    let mut raw = unsafe { buf.as_mut_vec() };
    // start at the first free part of the output buf
    let mut output_ptr = unsafe { raw.as_mut_ptr().offset(orig_buf_len as isize) };
    let mut input_index: usize = 0;
    if input_bytes.len() >= 8 {
        while input_index <= last_fast_index {
            let input_chunk = BigEndian::read_u64(&input_bytes[input_index..(input_index + 8)]);

            // strip off 6 bits at a time for the first 6 bytes
            unsafe {
                std::ptr::write(output_ptr, charset[((input_chunk >> 58) & 0x3F) as usize]);
                std::ptr::write(output_ptr.offset(1), charset[((input_chunk >> 52) & 0x3F) as usize]);
                std::ptr::write(output_ptr.offset(2), charset[((input_chunk >> 46) & 0x3F) as usize]);
                std::ptr::write(output_ptr.offset(3), charset[((input_chunk >> 40) & 0x3F) as usize]);
                std::ptr::write(output_ptr.offset(4), charset[((input_chunk >> 34) & 0x3F) as usize]);
                std::ptr::write(output_ptr.offset(5), charset[((input_chunk >> 28) & 0x3F) as usize]);
                std::ptr::write(output_ptr.offset(6), charset[((input_chunk >> 22) & 0x3F) as usize]);
                std::ptr::write(output_ptr.offset(7), charset[((input_chunk >> 16) & 0x3F) as usize]);
                output_ptr = output_ptr.offset(8);
            }

            input_index += input_chunk_len;
            fast_loop_output_buf_len += 8;
        }
    }

    unsafe {
        // expand len to include the bytes we just wrote
        raw.set_len(fast_loop_output_buf_len);
    }

    // encode the 0 to 7 bytes left after the fast loop

    let rem = input_bytes.len() % 3;
    let start_of_rem = input_bytes.len() - rem;

    // start at the first index not handled by fast loop, which may be 0.
    let mut leftover_index = input_index;

    while leftover_index < start_of_rem {
        raw.push(charset[(input_bytes[leftover_index] >> 2) as usize]);
        raw.push(charset[((input_bytes[leftover_index] << 4 | input_bytes[leftover_index + 1] >> 4) & 0x3f) as usize]);
        raw.push(charset[((input_bytes[leftover_index + 1] << 2 | input_bytes[leftover_index + 2] >> 6) & 0x3f) as usize]);
        raw.push(charset[(input_bytes[leftover_index + 2] & 0x3f) as usize]);

        leftover_index += 3;
    }

    if rem == 2 {
        raw.push(charset[(input_bytes[start_of_rem] >> 2) as usize]);
        raw.push(charset[((input_bytes[start_of_rem] << 4 | input_bytes[start_of_rem + 1] >> 4) & 0x3f) as usize]);
        raw.push(charset[(input_bytes[start_of_rem + 1] << 2 & 0x3f) as usize]);
    } else if rem == 1 {
        raw.push(charset[(input_bytes[start_of_rem] >> 2) as usize]);
        raw.push(charset[(input_bytes[start_of_rem] << 4 & 0x3f) as usize]);
    }

    if config.pad {
        for _ in 0..((3 - rem) % 3) {
            raw.push(0x3d);
        }
    }

    //TODO FIXME this does the wrong thing for nonempty buffers
    if orig_buf_len == 0 {
        if let LineWrap::Wrap(line_size, line_end) = config.line_wrap {
            let len = raw.len();
            let mut i = 0;
            let mut j = 0;

            while i < len {
                if i > 0 && i % line_size == 0 {
                    match line_end {
                        LineEnding::LF => { raw.insert(j, b'\n'); j += 1; }
                        LineEnding::CRLF => { raw.insert(j, b'\r'); raw.insert(j + 1, b'\n'); j += 2; }
                    }
                }

                i += 1;
                j += 1;
            }
        }
    }
}
