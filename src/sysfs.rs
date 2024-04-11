use std::ffi::OsString;
use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use crate::error::Error;

// Class used to represent power supply in sysfs.
// REF: https://www.kernel.org/doc/Documentation/ABI/testing/sysfs-class-power
pub const CLASS_POWER_SUPPLY: &str = "/sys/class/power_supply/";

// Start and stop threshold sysfs files.
pub const THRESHOLD_START: &str = "charge_control_start_threshold";
pub const THRESHOLD_STOP: &str = "charge_control_end_threshold";

// Default inbuilt batter indicator.
const DEFAULT_BATTERY: &str = "BAT0";

// General check to determine if the current OS is supported.
// TODO: could this be better?
pub fn is_platform_supported() -> Result<(), Error> {
    if !Path::new(CLASS_POWER_SUPPLY).exists() {
        return Err(Error::Unsupported);
    }

    Ok(())
}

// Construct a sysfs path from a given battery.
pub fn get_battery_path(battery: Option<OsString>) -> Result<PathBuf, Error> {
    // Set battery default if not specified.
    let bat: OsString = match battery {
        Some(b) => b.to_ascii_uppercase(),
        None => DEFAULT_BATTERY.into(),
    };

    let sysfs_bat = Path::new(CLASS_POWER_SUPPLY).join(bat.clone());
    if !sysfs_bat.exists() {
        return Err(Error::Battery(bat));
    }

    Ok(sysfs_bat)
}

// Sets the start and stop battery charge thresholds in sysfs.
pub fn set_thresholds(start: u8, stop: u8, battery: Option<OsString>) -> Result<(), Error> {
    // Generic check for platform support and valid thresholds.
    is_platform_supported()?;
    validate_thresholds(start, stop)?;

    // Get sysfs path from given battery.
    let sysfs_bat = get_battery_path(battery)?;

    // Set start and stop thresholds.
    write_threshold(sysfs_bat.join(THRESHOLD_START), start)?;
    write_threshold(sysfs_bat.join(THRESHOLD_STOP), stop)?;

    if start == 0 {
        println!("Battery will start charging immediately and stop charing at {stop}%");
    } else {
        println!("Battery will start charging below {start}% and stop charing at {stop}%");
    };

    Ok(())
}

// Gets the start and stop battery charge thresholds from sysfs.
pub fn get_thresholds(battery: Option<OsString>) -> Result<(), Error> {
    // Generic check for platform support and valid thresholds.
    is_platform_supported()?;

    // Get sysfs path from given battery.
    let sysfs_bat = get_battery_path(battery)?;

    // Get start and stop thresholds.
    let start = read_threshold(sysfs_bat.join(THRESHOLD_START))?;
    let stop = read_threshold(sysfs_bat.join(THRESHOLD_STOP))?;

    println!("Current charge thresholds: start: {start}%, stop: {stop}%");
    Ok(())
}

// General validation to check start and stop charge thresholds.
pub fn validate_thresholds(start: u8, stop: u8) -> Result<(), Error> {
    // Simple sanity check for valid threshold values.
    // The kernel will also enforce these values, but it's a simple check for us to do.
    if start > 100 || stop > 100 || start >= stop {
        return Err(Error::Threshold);
    }

    Ok(())
}

// Attempts to write a charge threshold value.
pub fn write_threshold(path: PathBuf, threshold: u8) -> Result<(), Error> {
    // Attempt to open the file in write mode while truncating any existing data.
    // This will fail if the file does not already exist.
    let mut f = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(path)
        .map_err(Error::IO)?;

    // Attempt to write the charge threshold.
    write!(f, "{}", threshold).map_err(Error::IO)?;
    Ok(())
}

// Attempts to read a charge threshold value.
pub fn read_threshold(path: PathBuf) -> Result<String, Error> {
    let mut f = OpenOptions::new()
        .write(false)
        .read(true)
        .open(path)
        .map_err(Error::IO)?;

    let mut buf = String::new();
    f.read_to_string(&mut buf).map_err(Error::IO)?;
    Ok(buf)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_thresholds() {
        // Start > 100
        assert!(validate_thresholds(101, 100).is_err(), "start > 100");
        // Stop > 100
        assert!(validate_thresholds(75, 101).is_err(), "stop > 100");
        // Start == stop
        assert!(validate_thresholds(75, 75).is_err(), "start == stop");
        // Start > stop
        assert!(validate_thresholds(80, 75).is_err(), "start > stop");
    }
}
