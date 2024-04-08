// use std::{thread, time};

use crate::error::Error;
use crate::sysfs;

pub fn start(start: u8, stop: u8, battery: Option<String>) -> Result<(), Error> {
    // Generic check for platform support and validate thresholds.
    sysfs::is_platform_supported()?;
    sysfs::validate_thresholds(start, stop)?;

    // Set battery default if not specified.
    let bat: String = match battery {
        Some(b) => b.to_uppercase(),
        None => sysfs::DEFAULT_BATTERY.to_string(),
    };

    // Get sysfs path from given battery.
    let sysfs_bat = sysfs::battery_path(bat)?;
    println!("{:?}", sysfs_bat);

    // loop {
    //     // TODO: capture CTRL+C
    //     // Only exit if fiished writing threshold?
    //     thread::sleep(time::Duration::from_secs(1));
    // }

    Ok(())
}
