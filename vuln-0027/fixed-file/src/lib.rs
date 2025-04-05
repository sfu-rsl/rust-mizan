// Claxon -- A FLAC decoding library in Rust
// Copyright 2014 Ruud van Asseldonk
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// A copy of the License has been included in the root of the repository.

//! Claxon, a FLAC decoding library.
//!
//! Examples
//! ========
//!
//! The following example computes the root mean square (RMS) of a FLAC file.
//!
//! ```
//! # use claxon;
//! let mut reader = claxon::FlacReader::open("testsamples/pop.flac").unwrap();
//! let mut sqr_sum = 0.0;
//! let mut count = 0;
//! for sample in reader.samples() {
//!     let s = sample.unwrap() as f64;
//!     sqr_sum += s * s;
//!     count += 1;
//! }
//! println!("RMS is {}", (sqr_sum / count as f64).sqrt());
//! ```
//!
//! A simple way to decode a file to wav with Claxon and
//! [Hound](https://github.com/ruuda/hound):
//!
//! ```
//! # extern crate hound;
//! # extern crate claxon;
//! # use std::path::Path;
//! # fn decode_file(fname: &Path) {
//! let mut reader = claxon::FlacReader::open(fname).expect("failed to open FLAC stream");
//!
//! let spec = hound::WavSpec {
//!     channels: reader.streaminfo().channels as u16,
//!     sample_rate: reader.streaminfo().sample_rate,
//!     bits_per_sample: reader.streaminfo().bits_per_sample as u16,
//!     sample_format: hound::SampleFormat::Int,
//! };
//!
//! let fname_wav = fname.with_extension("wav");
//! let opt_wav_writer = hound::WavWriter::create(fname_wav, spec);
//! let mut wav_writer = opt_wav_writer.expect("failed to create wav file");
//!
//! for opt_sample in reader.samples() {
//!     let sample = opt_sample.expect("failed to decode FLAC stream");
//!     wav_writer.write_sample(sample).expect("failed to write wav file");
//! }
//! # }
//! ```
//!
//! Retrieving the artist metadata:
//!
//! ```
//! # use claxon;
//! let reader = claxon::FlacReader::open("testsamples/pop.flac").unwrap();
//! for artist in reader.get_tag("ARTIST") {
//!     println!("{}", artist);
//! }
//! ```
//!
//! For more examples, see the [examples](https://github.com/ruuda/claxon/tree/master/examples)
//! directory in the crate.

#![warn(missing_docs)]

mod crc;
mod error;
#[cfg(not(KEEP_CARGO_MINIMIZE))]
pub mod frame;
pub mod input;
#[cfg(not(KEEP_CARGO_MINIMIZE))]
pub mod subframe;
