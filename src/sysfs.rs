use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};

use crate::error::Error;

// Class used to represent power supply in sysfs.
// REF: https://www.kernel.org/doc/Documentation/ABI/testing/sysfs-class-power
pub const SYSFS_CLASS_POWER: &str = "/sys/class/power_supply/";

// Default inbuilt batter indicator.
const DEFAULT_BATTERY: &str = "BAT0";

// Start and stop threshold sysfs files.
pub const THRESHOLD_START: &str = "charge_control_start_threshold";
pub const THRESHOLD_STOP: &str = "charge_control_end_threshold";

// General check to determine if the current OS is supported.
// TODO: could this be better?
pub fn is_platform_supported() -> Result<(), Error> {
    if !Path::new(SYSFS_CLASS_POWER).exists() {
        return Err(Error::Unsupported);
    }

    Ok(())
}

// Construct a sysfs path from a given battery.
pub fn get_battery_path(battery: Option<String>) -> Result<PathBuf, Error> {
    // Set battery default if not specified.
    let bat: String = match battery {
        Some(b) => b.to_uppercase(),
        None => DEFAULT_BATTERY.to_string(),
    };

    let sysfs_bat = Path::new(SYSFS_CLASS_POWER).join(bat.clone());
    if !sysfs_bat.exists() {
        return Err(Error::Battery(bat));
    }

    Ok(sysfs_bat)
}

// Sets the start and stop battery charge thresholds in sysfs.
pub fn set_threshold(start: u8, stop: u8, battery: Option<String>) -> Result<(), Error> {
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

// General validation to check start and stop charge thresholds.
pub fn validate_thresholds(start: u8, stop: u8) -> Result<(), Error> {
    // Simple sanity check for valid threshold values.
    // The kernel will also enforce these values, but it's a simple check for us to do.
    if start > 100 || stop > 100 || start >= stop {
        return Err(Error::Threshold);
    }

    Ok(())
}

// Attempts to write a charge threshold value to a given path.
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

// pub fn get_threshold(path: PathBuf) -> Result<(), Error> {
//     let f = match OpenOptions::new().write(false).read(true).open(path) {
//         Ok(file) => file,
//         Err(err) => return Err(io_error_context(err)),
//     };

//     // TODO: parse file content.

//     Ok(())
// }

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
