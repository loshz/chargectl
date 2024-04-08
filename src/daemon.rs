use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::{thread, time};

use crate::error::Error;
use crate::sysfs;

pub fn start(start: u8, stop: u8, battery: Option<String>) -> Result<(), Error> {
    // Generic check for platform support and valid thresholds.
    sysfs::is_platform_supported()?;
    sysfs::validate_thresholds(start, stop)?;

    // Get sysfs path from given battery.
    let sysfs_bat = sysfs::get_battery_path(battery)?;
    println!("{:?}", sysfs_bat);

    // Register stop handler.
    let stop = Arc::new(AtomicBool::new(false));
    let s = stop.clone();
    ctrlc::set_handler(move || {
        s.store(true, Ordering::Relaxed);
    })
    .unwrap();

    loop {
        if stop.load(Ordering::Relaxed) {
            break;
        }

        // Only exit if finished writing threshold?
        thread::sleep(time::Duration::from_secs(1));
        println!("polling");
    }

    Ok(())
}
