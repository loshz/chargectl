use clap::{Args, Parser, Subcommand};

use crate::daemon;
use crate::error::Error;
use crate::sysfs;

#[derive(Parser)]
#[command(version, about, long_about = None, propagate_version = true, disable_help_subcommand = true)]
pub struct Chargectl {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Get the current start and stop thresholds for a given battery
    Get(Battery),

    /// Set start and stop charge thresholds for a given battery
    Set(Thresholds),

    /// Set threshold to enable immediate charging until full
    Full(Battery),

    /// Run as a dameon, periodically resetting charge thresholds
    Start(Thresholds),
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

impl Chargectl {
    pub fn run(self) -> Result<(), Error> {
        match self.command {
            Commands::Get(args) => sysfs::get_thresholds(args.battery),
            Commands::Set(args) => sysfs::set_thresholds(args.start, args.stop, args.battery),
            Commands::Full(args) => sysfs::set_thresholds(96, 100, args.battery),
            Commands::Start(args) => daemon::start(args.start, args.stop, args.battery),
        }
    }
}
