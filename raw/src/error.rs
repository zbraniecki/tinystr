use core::fmt;

#[cfg(feature = "std")]
use std::error;

/// Enum to store the various types of errors that can cause parsing a TinyStr to fail.
#[derive(PartialEq, Eq, Debug)]
pub enum Error {
    /// String is too large or too small to store as TinyStr.
    InvalidSize,
    /// String is empty.
    InvalidNull,
    /// String contains non-ASCII character(s).
    NonAscii,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InvalidSize => write!(f, "invalid size"),
            Error::InvalidNull => write!(f, "string is empty"),
            Error::NonAscii => write!(f, "contains non-ASCII"),
        }
    }
}

#[cfg(feature = "std")]
impl error::Error for Error {}
