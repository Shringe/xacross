use clap::Parser;
use std::path::PathBuf;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// File to solve
    #[arg(short, long)]
    pub file: PathBuf,

    /// Extra debug information
    #[arg(short, long, default_value_t = false)]
    pub debug: bool,
}
