use std::process;

use clap::Parser;

mod cli;
mod error;
mod sysfs;

fn main() {
    let cli = cli::Chargectl::parse();

    if let Err(e) = cli.run() {
        eprintln!("Error: {e}");
        process::exit(1);
    }
}
