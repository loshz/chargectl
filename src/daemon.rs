use std::ffi::OsString;
use std::sync::mpsc;
use std::time::Duration;

use crate::error::Error;
use crate::sysfs;

pub fn start(start: u8, stop: u8, battery: Option<OsString>) -> Result<(), Error> {
    // Generic check for platform support and valid thresholds.
    sysfs::is_platform_supported()?;
    sysfs::validate_thresholds(start, stop)?;

    // Register stop handler.
    let (send, recv) = mpsc::channel();
    ctrlc::set_handler(move || {
        println!("stop signal received");
        send.send(()).unwrap();
    })
    .unwrap();

    // Get sysfs path from given battery.
    let sysfs_bat = sysfs::get_battery_path(battery)?;
    println!("Starting charge threshold daemon");
    // println!("Will attempt to periodically set charge thresholds for {}");

    loop {
        // Sleep until the next cycle, or break if we received a stop signal.
        // TODO: make this interval customizable.
        if recv.recv_timeout(Duration::from_secs(30)).is_ok() {
            break;
        }

        // Continually check if AC power is connected, and continue on error
        // as to not fail the service.
        if sysfs::is_ac_power_online().is_err() {
            continue;
        }

        // Attempt to write start/stop thresholds.
        // TODO: This currently exists on error, do we want to add retries?
        sysfs::write_threshold(sysfs_bat.join(sysfs::THRESHOLD_START), start)?;
        sysfs::write_threshold(sysfs_bat.join(sysfs::THRESHOLD_STOP), stop)?;
    }

    Ok(())
}
