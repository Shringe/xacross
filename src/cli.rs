use clap::Parser;
use clap::Args;
use clap::Command;
use clap::Subcommand;
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct Cli {
    /// File to solve
    #[arg(short, long)]
    pub file: PathBuf,

    /// Extra debug information
    #[arg(short, long, default_value_t = false)]
    pub debug: bool,

    /// Skip rendering unsolved grid
    #[arg(long, default_value_t = false)]
    pub no_raw_grid: bool,

    /// Skip rendering unsolved bank
    #[arg(long, default_value_t = false)]
    pub no_raw_bank: bool,

    /// Skip rendering solved grid
    #[arg(long, default_value_t = false)]
    pub no_grid: bool,

    /// Skip rendering solved bank
    #[arg(long, default_value_t = false)]
    pub no_bank: bool,
}
