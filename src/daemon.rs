// use std::{thread, time};

use anyhow::{anyhow, Error};

use crate::sysfs;

pub fn start(start: u8, stop: u8, battery: Option<String>) -> Result<(), Error> {
    sysfs::validate_thresholds(start, stop)?;

    // Generic check for platform support.
    if !sysfs::platform_supported() {
        return Err(anyhow!("unsupported platform"));
    }

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
