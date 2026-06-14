use std::convert::TryFrom;
use std::fs;
use std::str::FromStr;

use crate::{Result};
use crate::args::{DecodeArgs, EncodeArgs, PrintArgs, RemoveArgs};
use crate::png::{Chunk, ChunkType, Png};

/// Encodes a message into a PNG file and saves the result
pub fn encode(args: EncodeArgs) -> Result<()> {
    let file_in = fs::read(args.in_file)?;
    let mut png: Png = Png::try_from(file_in.as_slice())?;
    let chunk_type = ChunkType::from_str(args.chunk_type.as_str())?;
    let chunk = Chunk::new(chunk_type, args.message.into_bytes());
    png.append_chunk(chunk);
    match args.out_file {
        Some(path) => {
            fs::write(path, png.as_bytes())?;
        },
        None => {
            println!("{}", png);
        }
    }
    
    Ok(())
}

/// Searches for a message hidden in a PNG file and prints the message if one is found
pub fn decode(args: DecodeArgs) -> Result<()> {
    let file_in = fs::read(&args.in_file)?;
    let png: Png = Png::try_from(file_in.as_slice())?;
    match png.chunk_by_type(args.chunk_type.as_str()) {
        Some(ct) => {
            println!("{}", ct);
        },
        None => {
            println!("No chunks found")
        },
    }
    Ok(())
}

/// Removes a chunk from a PNG file and saves the result
pub fn remove(args: RemoveArgs) -> Result<()> {
    let file_in = fs::read(&args.in_file)?;
    let mut png: Png = Png::try_from(file_in.as_slice())?;
    let chunk = png.remove_first_chunk(args.chunk_type.as_str())?;
    println!("Removed chunk: {}", chunk);
    fs::write(args.in_file, png.as_bytes())?;
    Ok(())
}

/// Prints all of the chunks in a PNG file
pub fn print_chunks(args: PrintArgs) -> Result<()> {
    let file_in = fs::read(args.in_file)?;
    let png: Png = Png::try_from(file_in.as_slice())?;
    println!("{}", png);
    Ok(())
}