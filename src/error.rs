use std::fmt;
use std::io::ErrorKind;

// Wrapped operation errors.
#[derive(Debug)]
pub enum Error {
    IO(std::io::Error),
    Unsupported,
    Threshold,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::IO(err) => {
                match err.kind() {
                    // Usually fixed by running sudo.
                    ErrorKind::PermissionDenied => {
                        write!(
                            f,
                            "permission denied, try running the same command with sudo privileges"
                        )
                    }
                    // If we already know that the power supply class in sysfs exists, then this file
                    // _should_ exist.
                    ErrorKind::NotFound => write!(f, "unsupported platform"),
                    // Generic catch-all error.
                    _ => write!(f, "failed to write charge threshold: {err}"),
                }
            }
            Error::Unsupported => write!(f, "unsupported platform"),
        }
    }
}

impl std::error::Error for Error {}
