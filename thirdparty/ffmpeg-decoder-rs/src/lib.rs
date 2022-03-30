//! Decodes audio files using ffmpeg bindings
//!
//! Create a [`Decoder`](struct.Decoder.html) by supplying a `Path` to an audio file. [`Decoder`](struct.Decoder.html)
//! implies `Iterator` where each iteration returns a single `i16` signed 16bit sample.
//! Also implements [rodio's](https://github.com/RustAudio/rodio) [`Source`](https://docs.rs/rodio/latest/rodio/source/trait.Source.html) trait, where
//! the [`Decoder`](struct.Decoder.html) can be supplied as a sink source for playback.
//!
//! ### Features Flags
//!
//! - `rodio_source` to enable rodio's [`Source`](https://docs.rs/rodio/latest/rodio/source/trait.Source.html) trait
//!
//!
//! ## Example as Rodio Source
//!
//! ```rust
//! use rodio::Sink;
//! use std::path::PathBuf;
//!
//! fn play_file(input: PathBuf) -> Result<(), Error> {
//!     let decoder = ffmpeg_decoder::Decoder::open(&input)?;
//!
//!     let device = rodio::default_output_device().unwrap();
//!     let sink = Sink::new(&device);
//!
//!     sink.append(decoder);
//!     sink.play();
//!     sink.sleep_until_end();
//!
//!     Ok(())
//! }
//! ```
mod decoder;
pub use decoder::Decoder;

mod error;
pub use error::Error;

#[cfg(feature = "rodio_source")]
mod rodio;
