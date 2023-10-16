use std::fs::OpenOptions;
use std::io::{ErrorKind, Write};
use std::str;

use anyhow::{anyhow, Context, Error};

pub fn set_threshold(start: u8, stop: u8) -> Result<(), Error> {
    if start > 100 || stop > 100 {
        return Err(anyhow!("thresholds must be valid numbers between 0-100"));
    }

    if start >= stop {
        return Err(anyhow!("start threshold must be lower than stop threshold"));
    }

    // Set start thresholds.
    write_threshold("/sys/class/power_supply/BAT0/charge_start_threshold", start)?;
    write_threshold(
        "/sys/class/power_supply/BAT0/charge_control_start_threshold",
        start,
    )?;

    // Set stop thresholds.
    write_threshold("/sys/class/power_supply/BAT0/charge_stop_threshold", stop)?;
    write_threshold(
        "/sys/class/power_supply/BAT0/charge_control_end_threshold",
        stop,
    )?;

    println!(
        "Battery will start charging below {}% and stop charing at {}%",
        start, stop
    );

    Ok(())
}

fn write_threshold(path: &str, threshold: u8) -> Result<(), Error> {
    let mut f = match OpenOptions::new().write(true).truncate(true).open(path) {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::PermissionDenied => {
                return Err(anyhow!(
                    "permission denied, try running the same command with sudo privileges"
                ))
            }
            _ => return Err(anyhow!(format!("failed to open {}", path))),
        },
    };

    write!(f, "{}", threshold).context("failed to write threshold")?;
    Ok(())
}
