use std::cmp::Ordering;
use std::convert::Into;
use std::fmt;
use std::num::NonZeroU32;
use std::ops::Deref;
use std::str::FromStr;

use crate::helpers::make_4byte_str;
use crate::Error;

/// A tiny string that is from 1 to 4 non-NUL ASCII characters.
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct TinyStr4(NonZeroU32);

impl TinyStr4 {
    #[inline(always)]
    pub const unsafe fn new_unchecked(text: u32) -> Self {
        Self(NonZeroU32::new_unchecked(u32::from_le(text)))
    }

    #[inline(always)]
    pub fn as_str(&self) -> &str {
        self.deref()
    }

    pub fn to_ascii_uppercase(self) -> Self {
        let word = self.0.get();
        let result = word & !(((word + 0x1f1f_1f1f) & !(word + 0x0505_0505) & 0x8080_8080) >> 2);
        unsafe { Self(NonZeroU32::new_unchecked(result)) }
    }

    pub fn to_ascii_lowercase(self) -> Self {
        let word = self.0.get();
        let result = word | (((word + 0x3f3f_3f3f) & !(word + 0x2525_2525) & 0x8080_8080) >> 2);
        unsafe { Self(NonZeroU32::new_unchecked(result)) }
    }

    pub fn is_ascii_alphanumeric(self) -> bool {
        let word = self.0.get();
        let mask = (word + 0x7f7f_7f7f) & 0x8080_8080;
        let lower = word | 0x2020_2020;
        ((!(lower + 0x1f1f_1f1f) | (lower + 0x0505_0505)) & mask) == 0
    }

    /// Makes the string all lowercase except for the first character,
    /// which is made uppercase.
    pub fn to_ascii_titlecase(self) -> TinyStr4 {
        let word = self.0.get().to_le();
        let mask = ((word + 0x3f3f_3f1f) & !(word + 0x2525_2505) & 0x8080_8080) >> 2;
        let result = (word | mask) & !(0x20 & mask);
        unsafe { TinyStr4(NonZeroU32::new_unchecked(u32::from_le(result))) }
    }
}

impl fmt::Display for TinyStr4 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.deref())
    }
}

impl fmt::Debug for TinyStr4 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.deref())
    }
}

impl Deref for TinyStr4 {
    type Target = str;

    #[inline(always)]
    fn deref(&self) -> &str {
        // Again, could use #cfg to hand-roll a big-endian implementation.
        let word = self.0.get().to_le();
        let len = (4 - word.leading_zeros() / 8) as usize;
        unsafe {
            let slice = core::slice::from_raw_parts(&self.0 as *const _ as *const u8, len);
            std::str::from_utf8_unchecked(slice)
        }
    }
}

impl PartialEq<&str> for TinyStr4 {
    fn eq(&self, other: &&str) -> bool {
        self.deref() == *other
    }
}

impl PartialOrd for TinyStr4 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for TinyStr4 {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.get().to_be().cmp(&other.0.get().to_be())
    }
}

impl FromStr for TinyStr4 {
    type Err = Error;

    #[inline(always)]
    fn from_str(text: &str) -> Result<Self, Self::Err> {
        unsafe {
            match text.len() {
                1 => make_4byte_str(text, 1, 0x80).map(Self),
                2 => make_4byte_str(text, 2, 0x8080).map(Self),
                3 => make_4byte_str(text, 3, 0x0080_8080).map(Self),
                4 => make_4byte_str(text, 4, 0x8080_8080).map(Self),
                _ => Err(Error::InvalidSize),
            }
        }
    }
}

impl Into<u32> for TinyStr4 {
    fn into(self) -> u32 {
        self.0.get().to_le()
    }
}
