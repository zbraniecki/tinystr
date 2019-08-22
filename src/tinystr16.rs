use std::cmp::Ordering;
use std::convert::Into;
use std::fmt;
use std::num::NonZeroU128;
use std::ops::Deref;
use std::ptr::copy_nonoverlapping;
use std::str::FromStr;

use crate::Error;

/// A tiny string that is from 1 to 16 non-NUL ASCII characters.
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct TinyStr16(NonZeroU128);

impl TinyStr16 {
    #[inline(always)]
    pub const unsafe fn new_unchecked(text: u128) -> Self {
        Self(NonZeroU128::new_unchecked(u128::from_le(text)))
    }

    #[inline(always)]
    pub fn as_str(&self) -> &str {
        self.deref()
    }

    pub fn to_ascii_uppercase(self) -> Self {
        let word = self.0.get();
        let result = word
            & !(((word + 0x1f1f1f1f_1f1f1f1f_1f1f1f1f_1f1f1f1f)
                & !(word + 0x05050505_05050505_05050505_05050505)
                & 0x80808080_80808080_80808080_80808080)
                >> 2);
        unsafe { Self(NonZeroU128::new_unchecked(result)) }
    }

    pub fn to_ascii_lowercase(self) -> Self {
        let word = self.0.get();
        let result = word
            | (((word + 0x3f3f3f3f_3f3f3f3f_3f3f3f3f_3f3f3f3f)
                & !(word + 0x25252525_25252525_25252525_25252525)
                & 0x80808080_80808080_80808080_80808080)
                >> 2);
        unsafe { Self(NonZeroU128::new_unchecked(result)) }
    }

    pub fn is_ascii_alpha(self) -> bool {
        let word = self.0.get();
        let mask =
            (word + 0x7f7f7f7f_7f7f7f7f_7f7f7f7f_7f7f7f7f) & 0x80808080_80808080_80808080_80808080;
        let lower = word | 0x20202020_20202020_20202020_20202020;
        let alpha = !(lower + 0x1f1f1f1f_1f1f1f1f_1f1f1f1f_1f1f1f1f)
            | (lower + 0x05050505_05050505_05050505_05050505);
        (alpha & mask) == 0
    }

    pub fn is_ascii_alphanumeric(self) -> bool {
        let word = self.0.get();
        let mask =
            (word + 0x7f7f7f7f_7f7f7f7f_7f7f7f7f_7f7f7f7f) & 0x80808080_80808080_80808080_80808080;
        let numeric = !(word + 0x50505050_50505050_50505050_50505050)
            | (word + 0x46464646_46464646_46464646_46464646);
        let lower = word | 0x20202020_20202020_20202020_20202020;
        let alpha = !(lower + 0x1f1f1f1f_1f1f1f1f_1f1f1f1f_1f1f1f1f)
            | (lower + 0x05050505_05050505_05050505_05050505);
        (alpha & numeric & mask) == 0
    }

    pub fn to_ascii_titlecase(self) -> Self {
        let word = self.0.get().to_le();
        let mask = ((word + 0x3f3f3f3f_3f3f3f3f_3f3f3f3f_3f3f3f1f)
            & !(word + 0x25252525_25252525_25252525_25252505)
            & 0x80808080_80808080_80808080_80808080)
            >> 2;
        let result = (word | mask) & !(0x20 & mask);
        unsafe { Self(NonZeroU128::new_unchecked(u128::from_le(result))) }
    }
}

impl fmt::Display for TinyStr16 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.deref())
    }
}

impl fmt::Debug for TinyStr16 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.deref())
    }
}

impl Deref for TinyStr16 {
    type Target = str;

    #[inline(always)]
    fn deref(&self) -> &str {
        // Again, could use #cfg to hand-roll a big-endian implementation.
        let word = self.0.get().to_le();
        let len = (16 - word.leading_zeros() / 8) as usize;
        unsafe {
            let slice = core::slice::from_raw_parts(&self.0 as *const _ as *const u8, len);
            std::str::from_utf8_unchecked(slice)
        }
    }
}

impl PartialEq<&str> for TinyStr16 {
    fn eq(&self, other: &&str) -> bool {
        self.deref() == *other
    }
}

impl PartialOrd for TinyStr16 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for TinyStr16 {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.get().to_be().cmp(&other.0.get().to_be())
    }
}

impl FromStr for TinyStr16 {
    type Err = Error;

    #[inline(always)]
    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let len = text.len();
        if len < 1 || len > 16 {
            return Err(Error::InvalidSize);
        }
        unsafe {
            let mut word: u128 = 0;
            copy_nonoverlapping(text.as_ptr(), &mut word as *mut u128 as *mut u8, len);
            let mask = 0x80808080_80808080_80808080_80808080u128 >> (8 * (16 - len));
            // TODO: could do this with #cfg(target_endian), but this is clearer and
            // more confidence-inspiring.
            let mask = u128::from_le(mask);
            if (word & mask) != 0 {
                return Err(Error::NonAscii);
            }
            if ((mask - word) & mask) != 0 {
                return Err(Error::InvalidNull);
            }
            Ok(Self(NonZeroU128::new_unchecked(word)))
        }
    }
}

impl Into<u128> for TinyStr16 {
    fn into(self) -> u128 {
        self.0.get().to_le()
    }
}
