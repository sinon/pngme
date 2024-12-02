use anyhow::Result;
use std::path::PathBuf;

use clap::{command, Parser, Subcommand};

use pngme_lib::{decode, encode, print_chunks, remove};

#[derive(Parser, Debug)]
#[command(name = "pngme")]
#[command(about = "Hide secret message in png files", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
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
        Commands::Decode { path, chunk_type } => decode(path, chunk_type)?,
        Commands::Remove { path, chunk_type } => remove(path, chunk_type)?,
        Commands::Print { path } => print_chunks(path)?,
    }
    Ok(())
}
