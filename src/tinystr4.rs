use std::cmp::Ordering;
use std::fmt;
use std::num::NonZeroU32;
use std::ops::Deref;
use std::str::FromStr;

use crate::Error;

/// A tiny string that is from 1 to 4 non-NUL ASCII characters.
///
/// # Examples
///
/// ```
/// use tinystr::TinyStr4;
///
/// let s1: TinyStr4 = "Test".parse()
///     .expect("Failed to parse.");
///
/// assert_eq!(s1, "Test");
/// assert!(s1.is_ascii_alphabetic());
/// ```
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct TinyStr4(NonZeroU32);

impl TinyStr4 {
    /// Creates a TinyStr4 from a byte slice.
    ///
    /// # Examples
    ///
    /// ```
    /// use tinystr::TinyStr4;
    ///
    /// let s1 = TinyStr4::from_bytes("Test".as_bytes())
    ///     .expect("Failed to parse.");
    ///
    /// assert_eq!(s1, "Test");
    /// ```
    #[inline(always)]
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, Error> {
        tinystr_raw::try_u32_from_bytes(bytes).map(Self)
    }

    /// An unsafe constructor intended for cases where the consumer
    /// guarantees that the input is a little endian integer which
    /// is a correct representation of a `TinyStr4` string.
    ///
    /// # Examples
    ///
    /// ```
    /// use tinystr::TinyStr4;
    ///
    /// let s1: TinyStr4 = "Test".parse()
    ///     .expect("Failed to parse.");
    ///
    /// let num: u32 = s1.into();
    ///
    /// let s2 = unsafe { TinyStr4::new_unchecked(num) };
    ///
    /// assert_eq!(s1, s2);
    /// assert_eq!(s2.as_str(), "Test");
    /// ```
    ///
    /// # Safety
    ///
    /// The method does not validate the `u32` to be properly encoded
    /// value for `TinyStr4`.
    /// The value can be retrieved via `Into<u32> for TinyStr4`.
    #[inline(always)]
    pub const unsafe fn new_unchecked(text: u32) -> Self {
        Self(NonZeroU32::new_unchecked(u32::from_le(text)))
    }

    /// Extracts a string slice containing the entire `TinyStr4`.
    ///
    /// # Examples
    ///
    /// ```
    /// use tinystr::TinyStr4;
    ///
    /// let s1: TinyStr4 = "Test".parse()
    ///     .expect("Failed to parse.");
    ///
    /// assert_eq!(s1.as_str(), "Test");
    /// ```
    #[inline(always)]
    pub fn as_str(&self) -> &str {
        self.deref()
    }

    /// Gets a representation of this TinyStr4 as a primitive, valid for the
    /// current machine. This value is not necessarily compatible with
    /// [`TinyStr4::new_unchecked()`], use [`TinyStr4::from_native_unchecked()`]
    /// instead.
    ///
    /// # Examples
    ///
    /// ```
    /// use tinystr::{tinystr4, TinyStr4};
    ///
    /// const fn const_equals(a: TinyStr4, b: TinyStr4) -> bool {
    ///     a.as_unsigned() == b.as_unsigned()
    /// }
    ///
    /// const S1: TinyStr4 = tinystr4!("foo");
    /// const S2: TinyStr4 = tinystr4!("foo");
    /// const S3: TinyStr4 = tinystr4!("bar");
    ///
    /// assert!(const_equals(S1, S2));
    /// assert!(!const_equals(S1, S3));
    /// ```
    pub const fn as_unsigned(&self) -> u32 {
        self.0.get()
    }

    /// An unsafe constructor intended for cases where the consumer
    /// guarantees that the input is a native endian integer which
    /// is a correct representation of a `TinyStr4` string
    ///
    /// # Examples
    ///
    /// ```
    /// use tinystr::TinyStr4;
    ///
    /// let s1: TinyStr4 = "Test".parse()
    ///     .expect("Failed to parse.");
    ///
    /// let num: u32 = s1.as_unsigned();
    ///
    /// let s2 = unsafe { TinyStr4::new_unchecked(num) };
    ///
    /// assert_eq!(s1, s2);
    /// assert_eq!(s2.as_str(), "Test");
    /// ```
    ///
    /// # Safety
    ///
    /// The method does not validate the `u32` to be properly encoded
    /// value for `TinyStr4`.
    /// The value can be retrieved via [`TinyStr4::as_unsigned()`].
    #[inline(always)]
    pub const unsafe fn from_native_unchecked(text: u32) -> Self {
        Self(NonZeroU32::new_unchecked(text))
    }

    /// Checks if the value is composed of ASCII alphabetic characters:
    ///
    ///  * U+0041 'A' ..= U+005A 'Z', or
    ///  * U+0061 'a' ..= U+007A 'z'.
    ///
    /// # Examples
    ///
    /// ```
    /// use tinystr::TinyStr4;
    ///
    /// let s1: TinyStr4 = "Test".parse()
    ///     .expect("Failed to parse.");
    /// let s2: TinyStr4 = "Te3t".parse()
    ///     .expect("Failed to parse.");
    ///
    /// assert!(s1.is_ascii_alphabetic());
    /// assert!(!s2.is_ascii_alphabetic());
    /// ```
    pub const fn is_ascii_alphabetic(self) -> bool {
        let word = self.0.get();
        let mask = (word + 0x7f7f_7f7f) & 0x8080_8080;
        let lower = word | 0x2020_2020;
        let alpha = !(lower + 0x1f1f_1f1f) | (lower + 0x0505_0505);
        (alpha & mask) == 0
    }

    /// Checks if the value is composed of ASCII alphanumeric characters:
    ///
    ///  * U+0041 'A' ..= U+005A 'Z', or
    ///  * U+0061 'a' ..= U+007A 'z', or
    ///  * U+0030 '0' ..= U+0039 '9'.
    ///
    /// # Examples
    ///
    /// ```
    /// use tinystr::TinyStr4;
    ///
    /// let s1: TinyStr4 = "A15b".parse()
    ///     .expect("Failed to parse.");
    /// let s2: TinyStr4 = "[3@w".parse()
    ///     .expect("Failed to parse.");
    ///
    /// assert!(s1.is_ascii_alphanumeric());
    /// assert!(!s2.is_ascii_alphanumeric());
    /// ```
    pub const fn is_ascii_alphanumeric(self) -> bool {
        let word = self.0.get();
        let mask = (word + 0x7f7f_7f7f) & 0x8080_8080;
        let numeric = !(word + 0x5050_5050) | (word + 0x4646_4646);
        let lower = word | 0x2020_2020;
        let alpha = !(lower + 0x1f1f_1f1f) | (lower + 0x0505_0505);
        (alpha & numeric & mask) == 0
    }

    /// Checks if the value is composed of ASCII decimal digits:
    ///
    ///  * U+0030 '0' ..= U+0039 '9'.
    ///
    /// # Examples
    ///
    /// ```
    /// use tinystr::TinyStr4;
    ///
    /// let s1: TinyStr4 = "312".parse()
    ///     .expect("Failed to parse.");
    /// let s2: TinyStr4 = "3d".parse()
    ///     .expect("Failed to parse.");
    ///
    /// assert!(s1.is_ascii_numeric());
    /// assert!(!s2.is_ascii_numeric());
    /// ```
    pub const fn is_ascii_numeric(self) -> bool {
        let word = self.0.get();
        let mask = (word + 0x7f7f_7f7f) & 0x8080_8080;
        let numeric = !(word + 0x5050_5050) | (word + 0x4646_4646);
        (numeric & mask) == 0
    }

    /// Converts this type to its ASCII lower case equivalent in-place.
    ///
    /// ASCII letters 'A' to 'Z' are mapped to 'a' to 'z', other characters are unchanged.
    ///
    /// # Examples
    ///
    /// ```
    /// use tinystr::TinyStr4;
    ///
    /// let s1: TinyStr4 = "TeS3".parse()
    ///     .expect("Failed to parse.");
    ///
    /// assert_eq!(s1.to_ascii_lowercase(), "tes3");
    /// ```
    pub const fn to_ascii_lowercase(self) -> Self {
        let word = self.0.get();
        let result = word | (((word + 0x3f3f_3f3f) & !(word + 0x2525_2525) & 0x8080_8080) >> 2);
        unsafe { Self(NonZeroU32::new_unchecked(result)) }
    }

    /// Converts this type to its ASCII title case equivalent in-place.
    ///
    /// First character, if is an ASCII letter 'a' to 'z' is mapped to 'A' to 'Z',
    /// other characters are unchanged.
    ///
    /// # Examples
    ///
    /// ```
    /// use tinystr::TinyStr4;
    ///
    /// let s1: TinyStr4 = "test".parse()
    ///     .expect("Failed to parse.");
    ///
    /// assert_eq!(s1.to_ascii_titlecase(), "Test");
    /// ```
    pub const fn to_ascii_titlecase(self) -> Self {
        let word = self.0.get().to_le();
        let mask = ((word + 0x3f3f_3f1f) & !(word + 0x2525_2505) & 0x8080_8080) >> 2;
        let result = (word | mask) & !(0x20 & mask);
        unsafe { Self(NonZeroU32::new_unchecked(u32::from_le(result))) }
    }

    /// Converts this type to its ASCII upper case equivalent in-place.
    ///
    /// ASCII letters 'a' to 'z' are mapped to 'A' to 'Z', other characters are unchanged.
    ///
    /// # Examples
    ///
    /// ```
    /// use tinystr::TinyStr4;
    ///
    /// let s1: TinyStr4 = "Tes3".parse()
    ///     .expect("Failed to parse.");
    ///
    /// assert_eq!(s1.to_ascii_uppercase(), "TES3");
    /// ```
    pub const fn to_ascii_uppercase(self) -> Self {
        let word = self.0.get();
        let result = word & !(((word + 0x1f1f_1f1f) & !(word + 0x0505_0505) & 0x8080_8080) >> 2);
        unsafe { Self(NonZeroU32::new_unchecked(result)) }
    }
}

impl fmt::Display for TinyStr4 {
    #[inline(always)]
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
    #[inline(always)]
    fn eq(&self, other: &&str) -> bool {
        self.deref() == *other
    }
}

impl PartialOrd for TinyStr4 {
    #[inline(always)]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for TinyStr4 {
    #[inline(always)]
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.get().to_ne_bytes().cmp(&other.0.get().to_ne_bytes())
    }
}

impl FromStr for TinyStr4 {
    type Err = Error;

    #[inline(always)]
    fn from_str(text: &str) -> Result<Self, Self::Err> {
        Self::from_bytes(text.as_bytes())
    }
}

impl From<TinyStr4> for u32 {
    fn from(input: TinyStr4) -> Self {
        input.0.get().to_le()
    }
}

serde_impl!(TinyStr4, u32);
