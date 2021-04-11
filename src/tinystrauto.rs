use std::fmt;
use std::ops::Deref;
use std::str::FromStr;

use crate::Error;
use crate::TinyStr16;

#[cfg(any(feature = "std", test))]
pub use std::string::String;

#[cfg(all(not(feature = "std"), not(test)))]
extern crate alloc;

#[cfg(all(not(feature = "std"), not(test)))]
pub use alloc::string::String;

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
    /// Up to 16 characters stored on the stack.
    Tiny(TinyStr16),
    /// 17 or more characters stored on the heap.
    Heap(String),
}

impl fmt::Display for TinyStrAuto {
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.deref().fmt(f)
    }
}

impl Deref for TinyStrAuto {
    type Target = str;

    #[inline(always)]
    fn deref(&self) -> &str {
        use TinyStrAuto::*;
        match self {
            Tiny(value) => value.deref(),
            Heap(value) => value.deref(),
        }
    }
}

impl PartialEq<&str> for TinyStrAuto {
    #[inline(always)]
    fn eq(&self, other: &&str) -> bool {
        self.deref() == *other
    }
}

impl FromStr for TinyStrAuto {
    type Err = Error;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        if text.len() <= 16 {
            TinyStr16::from_str(text).map(TinyStrAuto::Tiny)
        } else if text.is_ascii() {
            Ok(TinyStrAuto::Heap(text.into()))
        } else {
            Err(Error::NonAscii)
        }
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for TinyStrAuto {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self)
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for TinyStrAuto {
    fn deserialize<D>(deserializer: D) -> Result<TinyStrAuto, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error as SerdeError;
        use std::borrow::Cow;
        use std::string::ToString;

        let x: Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        x.parse()
            .map_err(|e: Error| SerdeError::custom(e.to_string()))
    }
}
