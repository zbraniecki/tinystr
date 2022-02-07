use std::cmp::Ordering;
use std::fmt;
use std::num::NonZeroU64;
use std::ops::Deref;
use std::str::FromStr;

use crate::Error;

/// A tiny string that is from 1 to 8 non-NUL ASCII characters.
///
/// # Examples
///
/// ```
/// use tinystr::TinyStr8;
///
/// let s1: TinyStr8 = "Testing".parse()
///     .expect("Failed to parse.");
///
/// assert_eq!(s1, "Testing");
/// assert!(s1.is_ascii_alphabetic());
/// ```
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct TinyStr8(NonZeroU64);

impl TinyStr8 {
    /// Creates a TinyStr8 from a byte slice.
    ///
    /// # Examples
    ///
    /// ```
    /// use tinystr::TinyStr8;
    ///
    /// let s1 = TinyStr8::from_bytes("Testing".as_bytes())
    ///     .expect("Failed to parse.");
    ///
    /// assert_eq!(s1, "Testing");
    /// ```
    #[inline(always)]
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, Error> {
        tinystr_raw::try_u64_from_bytes(bytes).map(Self)
    }

    /// An unsafe constructor intended for cases where the consumer
    /// guarantees that the input is a little endian integer which
    /// is a correct representation of a `TinyStr8` string.
    ///
    /// # Examples
    ///
    /// ```
    /// use tinystr::TinyStr8;
    ///
    /// let s1: TinyStr8 = "Testing".parse()
    ///     .expect("Failed to parse.");
    ///
    /// let num: u64 = s1.into();
    ///
    /// let s2 = unsafe { TinyStr8::new_unchecked(num) };
    ///
    /// assert_eq!(s1, s2);
    /// assert_eq!(s2.as_str(), "Testing");
    /// ```
    ///
    /// # Safety
    ///
    /// The method does not validate the `u64` to be properly encoded
    /// value for `TinyStr8`.
    /// The value can be retrieved via `Into<u64> for TinyStr8`.
    #[inline(always)]
    pub const unsafe fn new_unchecked(text: u64) -> Self {
        Self(NonZeroU64::new_unchecked(u64::from_le(text)))
    }

    /// Extracts a string slice containing the entire `TinyStr8`.
    ///
    /// # Examples
    ///
    /// ```
    /// use tinystr::TinyStr8;
    ///
    /// let s1: TinyStr8 = "Testing".parse()
    ///     .expect("Failed to parse.");
    ///
    /// assert_eq!(s1.as_str(), "Testing");
    /// ```
    #[inline(always)]
    pub fn as_str(&self) -> &str {
        self.deref()
    }

    /// Gets a representation of this TinyStr8 as a primitive, valid for the
    /// current machine. This value is not necessarily compatible with
    /// [`TinyStr8::new_unchecked()`], use [`TinyStr8::from_native_unchecked()`]
    /// instead.
    ///
    /// # Examples
    ///
    /// ```
    /// use tinystr::{tinystr8, TinyStr8};
    ///
    /// const fn const_equals(a: TinyStr8, b: TinyStr8) -> bool {
    ///     a.as_unsigned() == b.as_unsigned()
    /// }
    ///
    /// const S1: TinyStr8 = tinystr8!("foo");
    /// const S2: TinyStr8 = tinystr8!("foo");
    /// const S3: TinyStr8 = tinystr8!("bar");
    ///
    /// assert!(const_equals(S1, S2));
    /// assert!(!const_equals(S1, S3));
    /// ```
    pub const fn as_unsigned(&self) -> u64 {
        self.0.get()
    }

    /// An unsafe constructor intended for cases where the consumer
    /// guarantees that the input is a native endian integer which
    /// is a correct representation of a `TinyStr8` string
    ///
    /// # Examples
    ///
    /// ```
    /// use tinystr::TinyStr8;
    ///
    /// let s1: TinyStr8 = "Test".parse()
    ///     .expect("Failed to parse.");
    ///
    /// let num: u64 = s1.as_unsigned();
    ///
    /// let s2 = unsafe { TinyStr8::new_unchecked(num) };
    ///
    /// assert_eq!(s1, s2);
    /// assert_eq!(s2.as_str(), "Test");
    /// ```
    ///
    /// # Safety
    ///
    /// The method does not validate the `u32` to be properly encoded
    /// value for `TinyStr8`.
    /// The value can be retrieved via [`TinyStr8::as_unsigned()`].
    #[inline(always)]
    pub const unsafe fn from_native_unchecked(text: u64) -> Self {
        Self(NonZeroU64::new_unchecked(text))
    }

    /// Checks if the value is composed of ASCII alphabetic characters:
    ///
    ///  * U+0041 'A' ..= U+005A 'Z', or
    ///  * U+0061 'a' ..= U+007A 'z'.
    ///
    /// # Examples
    ///
    /// ```
    /// use tinystr::TinyStr8;
    ///
    /// let s1: TinyStr8 = "Testing".parse()
    ///     .expect("Failed to parse.");
    /// let s2: TinyStr8 = "Te3ting".parse()
    ///     .expect("Failed to parse.");
    ///
    /// assert!(s1.is_ascii_alphabetic());
    /// assert!(!s2.is_ascii_alphabetic());
    /// ```
    pub const fn is_ascii_alphabetic(self) -> bool {
        let word = self.0.get();
        let mask = (word + 0x7f7f_7f7f_7f7f_7f7f) & 0x8080_8080_8080_8080;
        let lower = word | 0x2020_2020_2020_2020;
        let alpha = !(lower + 0x1f1f_1f1f_1f1f_1f1f) | (lower + 0x0505_0505_0505_0505);
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
    /// use tinystr::TinyStr8;
    ///
    /// let s1: TinyStr8 = "A15bing".parse()
    ///     .expect("Failed to parse.");
    /// let s2: TinyStr8 = "[3@wing".parse()
    ///     .expect("Failed to parse.");
    ///
    /// assert!(s1.is_ascii_alphanumeric());
    /// assert!(!s2.is_ascii_alphanumeric());
    /// ```
    pub const fn is_ascii_alphanumeric(self) -> bool {
        let word = self.0.get();
        let mask = (word + 0x7f7f_7f7f_7f7f_7f7f) & 0x8080_8080_8080_8080;
        let numeric = !(word + 0x5050_5050_5050_5050) | (word + 0x4646_4646_4646_4646);
        let lower = word | 0x2020_2020_2020_2020;
        let alpha = !(lower + 0x1f1f_1f1f_1f1f_1f1f) | (lower + 0x0505_0505_0505_0505);
        (alpha & numeric & mask) == 0
    }

    /// Checks if the value is composed of ASCII decimal digits:
    ///
    ///  * U+0030 '0' ..= U+0039 '9'.
    ///
    /// # Examples
    ///
    /// ```
    /// use tinystr::TinyStr8;
    ///
    /// let s1: TinyStr8 = "3121029".parse()
    ///     .expect("Failed to parse.");
    /// let s2: TinyStr8 = "3d212d".parse()
    ///     .expect("Failed to parse.");
    ///
    /// assert!(s1.is_ascii_numeric());
    /// assert!(!s2.is_ascii_numeric());
    /// ```
    pub const fn is_ascii_numeric(self) -> bool {
        let word = self.0.get();
        let mask = (word + 0x7f7f_7f7f_7f7f_7f7f) & 0x8080_8080_8080_8080;
        let numeric = !(word + 0x5050_5050_5050_5050) | (word + 0x4646_4646_4646_4646);
        (numeric & mask) == 0
    }

    /// Converts this type to its ASCII lower case equivalent in-place.
    ///
    /// ASCII letters 'A' to 'Z' are mapped to 'a' to 'z', other characters are unchanged.
    ///
    /// # Examples
    ///
    /// ```
    /// use tinystr::TinyStr8;
    ///
    /// let s1: TinyStr8 = "TeS3ing".parse()
    ///     .expect("Failed to parse.");
    ///
    /// assert_eq!(s1.to_ascii_lowercase(), "tes3ing");
    /// ```
    pub const fn to_ascii_lowercase(self) -> Self {
        let word = self.0.get();
        let result = word
            | (((word + 0x3f3f_3f3f_3f3f_3f3f)
                & !(word + 0x2525_2525_2525_2525)
                & 0x8080_8080_8080_8080)
                >> 2);
        unsafe { Self(NonZeroU64::new_unchecked(result)) }
    }

    /// Converts this type to its ASCII title case equivalent in-place.
    ///
    /// First character, if is an ASCII letter 'a' to 'z' is mapped to 'A' to 'Z',
    /// other characters are unchanged.
    ///
    /// # Examples
    ///
    /// ```
    /// use tinystr::TinyStr8;
    ///
    /// let s1: TinyStr8 = "testing".parse()
    ///     .expect("Failed to parse.");
    ///
    /// assert_eq!(s1.to_ascii_titlecase(), "Testing");
    /// ```
    pub const fn to_ascii_titlecase(self) -> Self {
        let word = self.0.get().to_le();
        let mask = ((word + 0x3f3f_3f3f_3f3f_3f1f)
            & !(word + 0x2525_2525_2525_2505)
            & 0x8080_8080_8080_8080)
            >> 2;
        let result = (word | mask) & !(0x20 & mask);
        unsafe { Self(NonZeroU64::new_unchecked(u64::from_le(result))) }
    }

    /// Converts this type to its ASCII upper case equivalent in-place.
    ///
    /// ASCII letters 'a' to 'z' are mapped to 'A' to 'Z', other characters are unchanged.
    ///
    /// # Examples
    ///
    /// ```
    /// use tinystr::TinyStr8;
    ///
    /// let s1: TinyStr8 = "Tes3ing".parse()
    ///     .expect("Failed to parse.");
    ///
    /// assert_eq!(s1.to_ascii_uppercase(), "TES3ING");
    /// ```
    pub const fn to_ascii_uppercase(self) -> Self {
        let word = self.0.get();
        let result = word
            & !(((word + 0x1f1f_1f1f_1f1f_1f1f)
                & !(word + 0x0505_0505_0505_0505)
                & 0x8080_8080_8080_8080)
                >> 2);
        unsafe { Self(NonZeroU64::new_unchecked(result)) }
    }
}

impl fmt::Display for TinyStr8 {
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.deref().fmt(f)
    }
}

impl fmt::Debug for TinyStr8 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.deref())
    }
}

impl Deref for TinyStr8 {
    type Target = str;

    #[inline(always)]
    fn deref(&self) -> &str {
        let word = self.0.get();
        #[cfg(target_endian = "little")]
        let len = (8 - word.leading_zeros() / 8) as usize;
        #[cfg(target_endian = "big")]
        let len = (8 - word.trailing_zeros() / 8) as usize;
        unsafe {
            let slice = core::slice::from_raw_parts(&self.0 as *const _ as *const u8, len);
            std::str::from_utf8_unchecked(slice)
        }
    }
}

impl PartialEq<&str> for TinyStr8 {
    #[inline(always)]
    fn eq(&self, other: &&str) -> bool {
        self.deref() == *other
    }
}

impl PartialOrd for TinyStr8 {
    #[inline(always)]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for TinyStr8 {
    #[inline(always)]
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.get().to_ne_bytes().cmp(&other.0.get().to_ne_bytes())
    }
}

impl FromStr for TinyStr8 {
    type Err = Error;

    #[inline(always)]
    fn from_str(text: &str) -> Result<Self, Self::Err> {
        TinyStr8::from_bytes(text.as_bytes())
    }
}

impl From<TinyStr8> for u64 {
    fn from(input: TinyStr8) -> Self {
        input.0.get().to_le()
    }
}

serde_impl!(TinyStr8, u64);
