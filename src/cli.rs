use clap::Parser;
use clap::Args;
use clap::Command;
use clap::Subcommand;
use std::path::PathBuf;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    /// File to solve
    #[arg(short, long)]
    pub file: PathBuf,

    /// Render solved grid
    #[command(subcommand)]
    pub commands: Commands,

    /// Extra debug information
    #[arg(short, long, default_value_t = false)]
    pub debug: bool,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    RenderOptions(RenderOptions),
}

#[derive(Args, Debug)]
pub struct RenderOptions {
    /// Renders raw
    pub raw: Option<bool>,

    /// Renders the grid
    pub grid: Option<bool>,
    
    /// Renders the word bank
    pub bank: Option<bool>,

    /// renders all
    pub complete: Option<bool>,
}
