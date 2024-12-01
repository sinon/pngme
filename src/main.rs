mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

use anyhow::Result;
use args::{Cli, Commands};
use clap::Parser;
use commands::{decode, encode, print_chunks, remove};

fn main() -> Result<()> {
    let args = Cli::parse();
    match args.command {
        Commands::Encode {
            path,
            chunk_type,
            message,
        } => encode(path, chunk_type, message)?,
        Commands::Decode { path, chunk_type } => decode(path, chunk_type)?,
        Commands::Remove { path, chunk_type } => remove(path, chunk_type)?,
        Commands::Print { path } => print_chunks(path)?,
    }
    Ok(())
}
