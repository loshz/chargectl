use std::ffi::OsString;
use std::fmt;
use std::io::ErrorKind;

use crate::sysfs;

// Wrapped operation errors.
#[derive(Debug)]
pub enum ChargeError {
    AC,
    Battery(OsString),
    IO(std::io::Error),
    Unsupported,
    Threshold,
}

impl fmt::Display for ChargeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let description: String = match self {
            ChargeError::AC => "AC power is not connected".to_string(),
            ChargeError::Battery(bat) => format!("battery not found: {:?}", bat),
            ChargeError::IO(err) => {
                match err.kind() {
                    // Usually fixed by running sudo.
                    ErrorKind::PermissionDenied => {
                        "permission denied, try running the same command with sudo privileges"
                            .to_string()
                    }
                    // If we already know that the power supply class in sysfs exists, then this file
                    // _should_ exist.
                    ErrorKind::NotFound => {
                        format!(
                            "battery thresholds not found {:?}",
                            sysfs::CLASS_POWER_SUPPLY
                        )
                    }
                    // Generic catch-all error.
                    _ => format!("failed to write charge threshold: {err}"),
                }
            }
            ChargeError::Unsupported => "unsupported platform".to_string(),
            ChargeError::Threshold => {
                "thresholds must be numerical [1-100], and start < stop".to_string()
            }
        };
        f.write_str(description.as_str())
    }
}

impl std::error::Error for ChargeError {}
