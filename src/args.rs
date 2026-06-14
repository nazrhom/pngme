use std::path::PathBuf;
use clap::{Parser, Subcommand, Args};

#[derive(Parser)]
#[command(version)]
pub struct Cli {
  #[command(subcommand)]
  pub command: PngMeArgs
}

#[derive(Subcommand)]
pub enum PngMeArgs {
    Encode(EncodeArgs),
    Decode(DecodeArgs),
    Remove(RemoveArgs),
    Print(PrintArgs),
}


#[derive(Args)]
pub struct EncodeArgs {
    pub in_file: PathBuf,
    pub chunk_type: String,
    pub message: String,
    pub out_file: Option<PathBuf>
}

#[derive(Args)]
pub struct DecodeArgs {
    pub in_file: PathBuf,
    pub chunk_type: String,
}

#[derive(Args)]
pub struct RemoveArgs {
    pub in_file: PathBuf,
    pub chunk_type: String,
}

#[derive(Args)]
pub struct PrintArgs {
    pub in_file: PathBuf,
}