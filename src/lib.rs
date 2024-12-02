mod chunk;
mod chunk_type;
mod png;

use std::fs::{self, OpenOptions};
use std::io::{Read, Write};
use std::path::PathBuf;
use std::str::FromStr;

use anyhow::{Error, Ok, Result, bail};

fn open_png(path: &PathBuf) -> Result<png::Png> {
    let mut f = fs::File::open(path)?;
    let mut data = vec![];
    f.read_to_end(&mut data)?;
    let (remaining, png_file) = png::parse_png(&data).unwrap();
    assert!(remaining.is_empty());
    Ok(png_file)
}

/// Encodes a message into a PNG file and saves the result
pub fn encode(path: PathBuf, chunk_type: String, message: String) -> Result<()> {
    let mut png_file = open_png(&path)?;
    let chunk_type = chunk_type::ChunkType::from_str(&chunk_type)?;
    if !chunk_type.is_valid() {
        bail!("Supplied chunk type value: {} is not valid", chunk_type);
    }
    let secret_chunk = chunk::Chunk::new(chunk_type, message.into());
    png_file.append_chunk(secret_chunk);
    write_png(path, png_file)?;
    Ok(())
}

fn write_png(path: PathBuf, png_file: png::Png) -> Result<(), Error> {
    let mut file = OpenOptions::new().write(true).truncate(true).open(&path)?;
    file.write_all(&png_file.as_bytes())?;
    Ok(())
}

/// Searches for a message hidden in a PNG file and prints the message if one is found
pub fn decode(path: PathBuf, chunk_type: String) -> Result<()> {
    let png_file = open_png(&path)?;

    let chunk = png_file.chunk_by_type(&chunk_type);
    if let Some(x) = chunk {
        println!("{}", x.data_as_string()?);
    } else {
        println!("No secret message found");
    }
    Ok(())
}

/// Removes a chunk from a PNG file and saves the result
pub fn remove(path: PathBuf, chunk_type: String) -> Result<()> {
    let mut png_file = open_png(&path)?;

    png_file.remove_first_chunk(&chunk_type)?;
    write_png(path, png_file)?;
    Ok(())
}

/// Prints all of the chunks in a PNG file
pub fn print_chunks(path: PathBuf) -> Result<()> {
    let png_file = open_png(&path)?;

    for chunk in png_file.chunks() {
        println!("{}", chunk);
    }
    Ok(())
}
