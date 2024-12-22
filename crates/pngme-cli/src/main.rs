#![warn(missing_docs)]
//! CLI using clap for hiding secret messages in PNG files
//!
//! Encode, decode, and remove secret messages from PNG files.
//!
//! Based on the [`pngme book`].
//!
//! [`pngme book`]: https://jrdngr.github.io/pngme_book/
use anyhow::Result;
use std::path::PathBuf;

use clap::{command, Parser, Subcommand};

use pngme_lib::{decode, encode, print_chunks, remove};

#[derive(Parser, Debug)]
#[command(name = "pngme")]
#[command(about = "Hide secret message in png files", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(arg_required_else_help = true)]
    Encode {
        path: PathBuf,
        chunk_type: String,
        message: String,
    },
    #[command(arg_required_else_help = true)]
    Decode { path: PathBuf, chunk_type: String },
    #[command(arg_required_else_help = true)]
    Remove { path: PathBuf, chunk_type: String },
    #[command(arg_required_else_help = true)]
    Print { path: PathBuf },
}

fn main() -> Result<()> {
    let args = Cli::parse();
    match args.command {
        Commands::Encode {
            path,
            chunk_type,
            message,
        } => encode(path, chunk_type, message)?,
        Commands::Decode { path, chunk_type } => {
            let msg = decode(path, chunk_type)?;
            println!("{}", msg);
        }
        Commands::Remove { path, chunk_type } => remove(path, chunk_type)?,
        Commands::Print { path } => print_chunks(path)?,
    }
    Ok(())
}
