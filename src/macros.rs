/// Macro to create a const TinyStr4, validated with zero runtime cost.
///
/// The argument must be a string literal:
/// https://doc.rust-lang.org/reference/tokens.html#string-literals
///
/// # Example
///
/// ```
/// use tinystr::{tinystr4, TinyStr4};
///
/// const S1: TinyStr4 = tinystr4!("abc");
/// let s2: TinyStr4 = "abc".parse().unwrap();
/// assert_eq!(S1, s2);
/// ```
#[macro_export]
macro_rules! tinystr4 {
    ($s:literal) => {
        unsafe { $crate::TinyStr4::new_unchecked($crate::raw_macros::u32_from_bytes!($s)) }
    };
}

#[test]
fn test_tinystr4() {
    use crate::TinyStr4;
    const X1: TinyStr4 = tinystr4!("foo");
    let x2: TinyStr4 = "foo".parse().unwrap();
    assert_eq!(X1, x2);
}

/// Macro to create a const TinyStr8, validated with zero runtime cost.
///
/// The argument must be a string literal:
/// https://doc.rust-lang.org/reference/tokens.html#string-literals
///
/// # Example
///
/// ```
/// use tinystr::{tinystr8, TinyStr8};
///
/// const S1: TinyStr8 = tinystr8!("abcdefg");
/// let s2: TinyStr8 = "abcdefg".parse().unwrap();
/// assert_eq!(S1, s2);
/// ```
#[macro_export]
macro_rules! tinystr8 {
    ($s:literal) => {
        unsafe { $crate::TinyStr8::new_unchecked($crate::raw_macros::u64_from_bytes!($s)) }
    };
}

#[test]
fn test_tinystr8() {
    use crate::TinyStr8;
    const X1: TinyStr8 = tinystr8!("barbaz");
    let x2: TinyStr8 = "barbaz".parse().unwrap();
    assert_eq!(X1, x2);
}

/// Macro to create a const TinyStr8, validated with zero runtime cost.
///
/// The argument must be a string literal:
/// https://doc.rust-lang.org/reference/tokens.html#string-literals
///
/// # Example
///
/// ```
/// use tinystr::{tinystr16, TinyStr16};
///
/// const S1: TinyStr16 = tinystr16!("longer-string");
/// let s2: TinyStr16 = "longer-string".parse().unwrap();
/// assert_eq!(S1, s2);
/// ```
#[macro_export]
macro_rules! tinystr16 {
    ($s:literal) => {
        unsafe { $crate::TinyStr16::new_unchecked($crate::raw_macros::u128_from_bytes!($s)) }
    };
}

#[test]
fn test_tinystr16() {
    use crate::TinyStr16;
    const X1: TinyStr16 = tinystr16!("metamorphosis");
    let x2: TinyStr16 = "metamorphosis".parse().unwrap();
    assert_eq!(X1, x2);
}
