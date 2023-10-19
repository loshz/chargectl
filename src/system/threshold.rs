use std::fs::OpenOptions;
use std::io::{ErrorKind, Write};
use std::path::{Path, PathBuf};
use std::str;

use anyhow::{anyhow, Context, Error};

// Power supply class used to represent battery in sysfs.
// Ref: https://www.kernel.org/doc/Documentation/ABI/testing/sysfs-class-power
const SYSFS_CLASS_POWER: &str = "/sys/class/power_supply/";

// Default inbuilt batter indicator.
const DEFAULT_BATTERY: &str = "BAT0";

// Sets the start and stop battery charge thresholds in sysfs.
// TODO: allow passing different battery specifiers?
pub fn set_threshold(start: u8, stop: u8, battery: Option<String>) -> Result<(), Error> {
    // Simple sanity check for valid threshold values.
    // The kernel will also enforce these values, but it's a simple check for us to do.
    if start > 100 || stop > 100 {
        return Err(anyhow!("thresholds must be valid numbers between 0-100"));
    }

    // Check that the start threshold is lower than the stop threshold.
    if start >= stop {
        return Err(anyhow!("start threshold must be lower than stop threshold"));
    }

    // Generic check for platform support.
    // TODO: could this be better?
    let sysfs = Path::new(SYSFS_CLASS_POWER);
    if !sysfs.exists() {
        return Err(anyhow!("unsupported platform"));
    }

    // Set battery default if not specified.
    let bat: String = match battery {
        Some(b) => b,
        None => DEFAULT_BATTERY.to_string(),
    };

    // Check if the battery exists.
    let sysfs_bat = sysfs.join(bat);
    if !sysfs_bat.exists() {
        return Err(anyhow!("battery not present"));
    }

    // Set start thresholds.
    write_threshold(sysfs_bat.join("charge_control_start_threshold"), start)?;

    // Set stop thresholds.
    write_threshold(sysfs_bat.join("charge_control_end_threshold"), stop)?;

    if start == 0 {
        println!("Battery will start charging immediately and stop charing at {stop}%");
    } else {
        println!("Battery will start charging below {start}% and stop charing at {stop}%");
    };

    Ok(())
}

// Attempts to write a charge threshold value to a given path.
fn write_threshold(path: PathBuf, threshold: u8) -> Result<(), Error> {
    // Attempt to open the file in write mode while truncating any existing data.
    // This will fail if the file does not already exist.
    let mut f = match OpenOptions::new().write(true).truncate(true).open(path) {
        Ok(file) => file,
        Err(e) => match e.kind() {
            // Usually fixed by running sudo.
            ErrorKind::PermissionDenied => {
                return Err(anyhow!(
                    "permission denied, try running the same command with sudo privileges"
                ))
            }
            // If we already know that the power supply class in sysfs exists, then this file
            // _should_ exist.
            ErrorKind::NotFound => return Err(anyhow!("unsupported platform")),
            // Generic catch-all error.
            _ => return Err(anyhow!(format!("failed to write charge threshold: {e}"))),
        },
    };

    // Attempt to write the charge threshold.
    write!(f, "{}", threshold).context("failed to write charge threshold")?;
    Ok(())
}
