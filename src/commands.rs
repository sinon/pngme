use std::fs;
use std::str::FromStr;
use std::{convert::TryFrom, path::PathBuf};

use anyhow::{Error, Result};

/// Encodes a message into a PNG file and saves the result
pub fn encode(path: PathBuf, chunk_type: String, message: String) -> Result<()> {
    todo!()
}

/// Searches for a message hidden in a PNG file and prints the message if one is found
pub fn decode(path: PathBuf, chunk_type: String) -> Result<()> {
    todo!()
}

/// Removes a chunk from a PNG file and saves the result
pub fn remove(path: PathBuf, chunk_type: String) -> Result<()> {
    todo!()
}

/// Prints all of the chunks in a PNG file
pub fn print_chunks(path: PathBuf) -> Result<()> {
    todo!()
}
