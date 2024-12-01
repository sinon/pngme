use std::path::PathBuf;

use clap::{command, Parser, Subcommand};

/*
pngme encode ./dice.png ruSt "This is a secret message!

pngme decode ./dice.png ruSt

pngme remove ./dice.png ruSt

pngme print ./dice.png
 */

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
