use std::fs::OpenOptions;
use std::io::Write;
use std::str;

use anyhow::{anyhow, Context, Error};

use crate::error;

pub fn set(start: u8, stop: u8) -> Result<(), Error> {
    if start > 100 || stop > 100 {
        return Err(anyhow!(error::ERR_INVALID_THRESHOLD));
    }

    if start >= stop {
        return Err(anyhow!(
            "Error: start threshold must be less than stop threshold"
        ));
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
    let mut f = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(path)
        .with_context(|| format!("Failed to open {}", path))?;

    write!(f, "{}", threshold).context("Failed to write threshold")?;
    Ok(())
}
