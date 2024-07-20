use std::process::ExitCode;

use clap::Parser;

mod cli;
mod error;
mod sysfs;

fn main() -> ExitCode {
    if let Err(e) = cli::Chargectl::parse().run() {
        eprintln!("Error: {e}");
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}
