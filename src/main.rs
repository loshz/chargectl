use std::process;

use clap::Parser;

mod cli;
mod daemon;
mod error;
mod sysfs;

fn main() {
    let cli = cli::App::parse();

    if let Err(e) = cli.run() {
        eprintln!("Error: {e}");
        process::exit(1);
    }
}
