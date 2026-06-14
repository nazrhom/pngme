mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

use crate::args::{Cli, PngMeArgs};
use crate::commands::*;
use clap::Parser;


pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    match cli.command {
        PngMeArgs::Encode(encode_args) => {
            encode(encode_args)?;
        }
        PngMeArgs::Decode(decode_args) => {
            decode(decode_args)?;
        },
        PngMeArgs::Remove(remove_args) => {
            remove(remove_args)?;
        },
        PngMeArgs::Print(print_args) => {
            print_chunks(print_args)?;
        },
    }
    Ok(())
}