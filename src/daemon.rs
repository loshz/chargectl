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

    loop {
        // Sleep until the next cycle.
        // TODO: make this interval customizable.
        if recv.recv_timeout(Duration::from_secs(30)).is_ok() {
            // Sleep was interrupted
            break;
        }

        // Attempt to write start/stop thresholds.
        // TODO: This currently exists on error, do we want to add retries?
        sysfs::write_threshold(sysfs_bat.join(sysfs::THRESHOLD_START), start)?;
        sysfs::write_threshold(sysfs_bat.join(sysfs::THRESHOLD_STOP), stop)?;
    }

    Ok(())
}
