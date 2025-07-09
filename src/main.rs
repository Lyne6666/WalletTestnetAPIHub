// src/main.rs
/*
 * Main executable for WalletTestnetAPIHub
 */

use clap::Parser;
use wallettestnetapihub::{Result, run};

#[derive(Parser)]
#[command(version, about = "WalletTestnetAPIHub - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
