use std::fmt;
use std::ops::Deref;
use std::str::FromStr;

use crate::Error;
use crate::TinyStr16;
use crate::helpers::String;

/// An ASCII string that is tiny when <= 16 chars and a String otherwise.
///
/// # Examples
///
/// ```
/// use tinystr::TinyStrAuto;
///
/// let s1: TinyStrAuto = "Testing".parse()
///     .expect("Failed to parse.");
///
/// assert_eq!(s1, "Testing");
/// ```
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub enum TinyStrAuto {
    Tiny(TinyStr16),
    Long(String),
}

impl fmt::Display for TinyStrAuto {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.deref())
    }
}

impl Deref for TinyStrAuto {
    type Target = str;

    fn deref(&self) -> &str {
        use TinyStrAuto::*;
        match self {
            Tiny(value) => value.deref(),
            Long(value) => value.deref(),
        }
    }
}

impl PartialEq<&str> for TinyStrAuto {
    fn eq(&self, other: &&str) -> bool {
        self.deref() == *other
    }
}

impl FromStr for TinyStrAuto {
    type Err = Error;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        if text.len() <= 16 {
            match TinyStr16::from_str(text) {
                Ok(result) => Ok(TinyStrAuto::Tiny(result)),
                Err(err) => Err(err),
            }
        } else {
            if !text.is_ascii() {
                return Err(Error::NonAscii)
            }
            match String::from_str(text) {
                Ok(result) => Ok(TinyStrAuto::Long(result)),
                Err(_) => unreachable!(),
            }
        }
    }
}
