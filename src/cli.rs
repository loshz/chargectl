use std::ffi::OsString;

use clap::{Args, Parser, Subcommand};

use crate::error::ChargeError;
use crate::sysfs;

#[derive(Parser)]
#[command(version, about, long_about = None, propagate_version = true, disable_help_subcommand = true)]
pub struct Chargectl {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Set thresholds to enable immediate charging until full
    Full(Battery),

    /// Get the current start and stop thresholds for a given battery
    Get(Battery),

    /// Set start and stop charge thresholds for a given battery
    Set(Thresholds),
}

#[derive(Args)]
#[command(disable_version_flag = true)]
struct Thresholds {
    /// Battery charge percentage below which charging will begin
    start: u8,

    /// Battery charge percentage above which charging will stop
    stop: u8,

    /// Battery to set charge thresholds on
    battery: Option<OsString>,
}

#[derive(Args)]
#[command(disable_version_flag = true)]
struct Battery {
    /// Battery to charge
    battery: Option<OsString>,
}

impl Chargectl {
    pub fn run(self) -> Result<(), ChargeError> {
        match self.command {
            Commands::Full(args) => {
                sysfs::is_ac_power_online()?;
                sysfs::set_thresholds(96, 100, args.battery)
            }
            Commands::Get(args) => sysfs::get_thresholds(args.battery),
            Commands::Set(args) => sysfs::set_thresholds(args.start, args.stop, args.battery),
        }
    }
}
