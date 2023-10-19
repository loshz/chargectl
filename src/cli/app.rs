use anyhow::Error;
use clap::{Args, Parser, Subcommand};

use crate::system;

#[derive(Parser)]
#[command(version, about, long_about = None, propagate_version = true, disable_help_subcommand = true)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Set start and stop charge thresholds
    Setcharge(Thresholds),
    /// Set threshold to enable immediate charging until full
    Fullcharge(Battery),
}

#[derive(Args)]
#[command(disable_version_flag = true)]
struct Thresholds {
    /// Battery charge percentage below which charging will begin
    start: u8,

    /// Battery charge percentage above which charging will stop
    stop: u8,

    /// Battery to set charge thresholds on
    battery: Option<String>,
}

#[derive(Args)]
#[command(disable_version_flag = true)]
struct Battery {
    /// Battery to fully charge
    battery: Option<String>,
}

impl Cli {
    pub fn run(self) -> Result<(), Error> {
        match self.command {
            Commands::Setcharge(args) => system::set_threshold(args.start, args.stop, args.battery),
            Commands::Fullcharge(args) => system::set_threshold(0, 100, args.battery),
        }
    }
}
