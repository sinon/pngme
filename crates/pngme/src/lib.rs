#![warn(missing_docs)]
//! Hide secret messages in PNG files
//!
//! Encode, decode, and remove secret messages from PNG files.
//!
//! Based on the [`pngme book`].
//!
//! [`pngme book`]: https://jrdngr.github.io/pngme_book/

mod chunk;
mod chunk_type;
mod png;

use std::fs::{self, OpenOptions};
use std::io::{Read, Write};
use std::path::PathBuf;
use std::str::FromStr;

use snafu::prelude::*;

/// Errors that can be returned by the library
#[derive(Debug, Snafu)]
pub enum Error {
    /// Supplied file path does not exist
    #[snafu(display("File not found {path:?}"))]
    FileNotFound {
        /// The source error
        source: std::io::Error,
        /// The path that was not found
        path: PathBuf,
    },
    /// Failed to read file
    #[snafu(display("Failed to read file"))]
    Read {
        /// The source error
        source: std::io::Error,
    },
    /// Failed to parse PNG file
    #[snafu(display("Error when parsing PNG"))]
    PNGParse,
    /// Failed to convert data to String
    #[snafu(display("Error when converting PNG to String"))]
    StrConversion,
    /// Invalid chunk type suppied
    #[snafu(display("Supplied chunk type value: `{chunk_type}` is not valid"))]
    InvalidChunkType {
        /// The chunk type that was invalid
        chunk_type: String,
        /// The source error
        source: chunk_type::ChunkTypeError,
    },
    /// Failed to write changes to PNG file
    #[snafu(display("Error when writing PNG"))]
    PNGWrite {
        /// The source error
        source: std::io::Error,
    },
    /// No chunk was found for given `chunk_type`
    ChunkNotFound {
        /// The chunk type that was not found
        chunk_type: String,
    },
}

fn open_png(path: &PathBuf) -> Result<png::Png, Error> {
    let mut f = fs::File::open(path).context(FileNotFoundSnafu { path })?;
    let mut data = vec![];
    f.read_to_end(&mut data).context(ReadSnafu)?;
    let (remaining, png_file) = png::parse_png(&data).unwrap();
    ensure!(remaining.is_empty(), PNGParseSnafu);
    Ok(png_file)
}

/// Encodes a message into a PNG file and saves the result
///
/// # Examples
///
/// ```no_run
/// use std::path::PathBuf;
/// use pngme_lib::encode;
/// let path = PathBuf::from("dice.png");
/// let message = "This is a secret message".to_string();
/// let chunk_type = "RuSt".to_string();
/// encode(path, chunk_type, message).unwrap();
/// ```
pub fn encode(path: PathBuf, chunk_type: String, message: String) -> Result<(), Error> {
    let ct = chunk_type.clone();
    let mut png_file = open_png(&path)?;
    let chunk_type =
        chunk_type::ChunkType::from_str(&chunk_type).map_err(|s| Error::InvalidChunkType {
            chunk_type: ct.clone(),
            source: s,
        })?;

    let secret_chunk = chunk::Chunk::new(chunk_type, message.into());
    png_file.append_chunk(secret_chunk);
    write_png(path, png_file)?;
    Ok(())
}

fn write_png(path: PathBuf, png_file: png::Png) -> Result<(), Error> {
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&path)
        .context(FileNotFoundSnafu { path })?;
    file.write_all(&png_file.as_bytes())
        .context(PNGWriteSnafu)?;
    Ok(())
}

/// Searches for a message hidden in a PNG file and prints the message if one is found
///
/// # Examples
///
/// ```no_run
/// use std::path::PathBuf;
/// use pngme_lib::decode;
/// let path = PathBuf::from("dice.png");
/// let chunk_type = "RuSt".to_string();
/// let msg = decode(path, chunk_type).unwrap();
/// assert_eq!(msg, "This is a secret message");
/// ```
pub fn decode(path: PathBuf, chunk_type: String) -> Result<String, Error> {
    let png_file = open_png(&path)?;

    let chunk = png_file.chunk_by_type(&chunk_type);
    if let Some(x) = chunk {
        Ok((x.data_as_string().map_err(|_| Error::StrConversion)?).to_string())
    } else {
        Ok("No secret message found".to_string())
    }
}

/// Removes a chunk from a PNG file and saves the result
///
/// # Examples
///
/// ```no_run
/// use std::path::PathBuf;
/// use pngme_lib::remove;
/// let path = PathBuf::from("dice.png");
/// let chunk_type = "RuSt".to_string();
/// remove(path, chunk_type).unwrap();
/// ```
pub fn remove(path: PathBuf, chunk_type: String) -> Result<(), Error> {
    let mut png_file = open_png(&path)?;

    png_file
        .remove_first_chunk(&chunk_type)
        .map_err(|_| Error::ChunkNotFound { chunk_type })?;
    write_png(path, png_file)?;
    Ok(())
}

/// Prints all of the chunks in a PNG file
///
/// # Examples
///
/// ```no_run
/// use std::path::PathBuf;
/// use pngme_lib::print_chunks;
/// let path = PathBuf::from("dice.png");
/// print_chunks(path).unwrap();
/// ```
pub fn print_chunks(path: PathBuf) -> Result<(), Error> {
    let png_file = open_png(&path)?;

    for chunk in png_file.chunks() {
        println!("{}", chunk);
    }
    Ok(())
}
