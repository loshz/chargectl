use std::ffi::OsString;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::{thread, time};

use crate::error::Error;
use crate::sysfs;

pub fn start(start: u8, stop: u8, battery: Option<OsString>) -> Result<(), Error> {
    // Generic check for platform support and valid thresholds.
    sysfs::is_platform_supported()?;
    sysfs::validate_thresholds(start, stop)?;

    // Get sysfs path from given battery.
    let sysfs_bat = sysfs::get_battery_path(battery)?;

    // Register stop handler.
    let stopping = Arc::new(AtomicBool::new(false));
    let s = stopping.clone();
    ctrlc::set_handler(move || {
        s.store(true, Ordering::Relaxed);
    })
    .unwrap();

    loop {
        if stopping.load(Ordering::Relaxed) {
            break;
        }

        // Attempt to write start/stop thresholds.
        // TODO: This currently exists on error, do we want to add retries?
        sysfs::write_threshold(sysfs_bat.join(sysfs::THRESHOLD_START), start)?;
        sysfs::write_threshold(sysfs_bat.join(sysfs::THRESHOLD_STOP), stop)?;

        // Sleep until the next cycle.
        // TODO: make this interval customizable.
        thread::sleep(time::Duration::from_secs(30));
    }

    Ok(())
}
