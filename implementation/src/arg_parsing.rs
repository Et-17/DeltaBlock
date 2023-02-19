use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Encodes a set of images, whose names are listed in file, into a DeltaBlock file
    Encode(EncodeArgs),

    /// Decode a DeltaBlock file into a set of images
    Decode(DecodeArgs),
}

#[derive(Args, Debug)]
pub struct EncodeArgs {
    /// The path to the file that contains the names of the images
    names: PathBuf,

    /// The name of the output file
    #[arg(short, long)]
    output: PathBuf,
}

#[derive(Args, Debug)]
pub struct DecodeArgs {
    /// The path to the DeltaBlock file
    file: PathBuf,

    /// The path to the output directory
    #[arg(short, long)]
    output: PathBuf,
}
