use std::process;

use clap::Parser;

mod cli;
mod system;

use crate::cli::Cli;

fn main() {
    let cli = Cli::parse();
    process::exit(match cli.run() {
        Ok(_) => 0,
        Err(e) => {
            eprintln!("Error: {e}");
            1
        }
    });
}
