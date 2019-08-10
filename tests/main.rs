use std::fmt::Write;
use std::ops::Deref;
use tinystr::{Error, TinyStr4, TinyStr8};

#[test]
fn tiny4_basic() {
    let s: TinyStr4 = "abc".parse().unwrap();
    assert_eq!(s.deref(), "abc");
}

#[test]
fn tiny4_size() {
    assert_eq!("".parse::<TinyStr4>(), Err(Error::InvalidSize));
    assert!("1".parse::<TinyStr4>().is_ok());
    assert!("12".parse::<TinyStr4>().is_ok());
    assert!("123".parse::<TinyStr4>().is_ok());
    assert!("1234".parse::<TinyStr4>().is_ok());
    assert_eq!("12345".parse::<TinyStr4>(), Err(Error::InvalidSize));
    assert_eq!("123456789".parse::<TinyStr4>(), Err(Error::InvalidSize));
}

#[test]
fn tiny4_null() {
    assert_eq!("a\u{0}b".parse::<TinyStr4>(), Err(Error::InvalidNull));
}

#[test]
fn tiny4_new_unchecked() {
    let reference: TinyStr4 = "en".parse().unwrap();
    let uval: u32 = reference.into();
    let s = unsafe { TinyStr4::new_unchecked(uval) };
    assert_eq!(s, reference);
    assert_eq!(s, "en");
}

#[test]
fn tiny4_nonascii() {
    assert_eq!("\u{4000}".parse::<TinyStr4>(), Err(Error::NonAscii));
}

#[test]
fn tiny4_alpha() {
    let s: TinyStr4 = "@aZ[".parse().unwrap();
    assert!(!s.is_ascii_alphanumeric());
    assert_eq!(s.to_ascii_uppercase().as_str(), "@AZ[");
    assert_eq!(s.to_ascii_lowercase().as_str(), "@az[");

    assert!("abYZ".parse::<TinyStr4>().unwrap().is_ascii_alphanumeric());
}

#[test]
fn tiny4_titlecase() {
    assert_eq!(
        "abcd"
            .parse::<TinyStr4>()
            .unwrap()
            .to_ascii_titlecase()
            .as_str(),
        "Abcd"
    );
    assert_eq!(
        "ABCD"
            .parse::<TinyStr4>()
            .unwrap()
            .to_ascii_titlecase()
            .as_str(),
        "Abcd"
    );
    assert_eq!(
        "aBCD"
            .parse::<TinyStr4>()
            .unwrap()
            .to_ascii_titlecase()
            .as_str(),
        "Abcd"
    );
    assert_eq!(
        "A123"
            .parse::<TinyStr4>()
            .unwrap()
            .to_ascii_titlecase()
            .as_str(),
        "A123"
    );
    assert_eq!(
        "123a"
            .parse::<TinyStr4>()
            .unwrap()
            .to_ascii_titlecase()
            .as_str(),
        "123a"
    );
}

#[test]
fn tiny4_ord() {
    let mut v: Vec<TinyStr4> = vec!["zh".parse().unwrap(), "fr".parse().unwrap()];
    v.sort();

    assert_eq!(v.get(0).unwrap().as_str(), "fr");
    assert_eq!(v.get(1).unwrap().as_str(), "zh");
}

#[test]
fn tiny4_eq() {
    let s1: TinyStr4 = "en".parse().unwrap();
    let s2: TinyStr4 = "fr".parse().unwrap();
    let s3: TinyStr4 = "en".parse().unwrap();

    assert_eq!(s1, s3);
    assert_ne!(s1, s2);
}

#[test]
fn tiny4_display() {
    let s: TinyStr4 = "abcd".parse().unwrap();
    let mut result = String::new();
    write!(result, "{}", s).unwrap();
    assert_eq!(result, "abcd");
    assert_eq!(format!("{}", s), "abcd");
}

#[test]
fn tiny4_debug() {
    let s: TinyStr4 = "abcd".parse().unwrap();
    assert_eq!(format!("{:#?}", s), "\"abcd\"");
}

#[test]
fn tiny8_basic() {
    let s: TinyStr8 = "abcde".parse().unwrap();
    assert_eq!(s.deref(), "abcde");
}

#[test]
fn tiny8_size() {
    assert_eq!("".parse::<TinyStr8>(), Err(Error::InvalidSize));
    assert!("1".parse::<TinyStr8>().is_ok());
    assert!("12".parse::<TinyStr8>().is_ok());
    assert!("123".parse::<TinyStr8>().is_ok());
    assert!("1234".parse::<TinyStr8>().is_ok());
    assert!("12345".parse::<TinyStr8>().is_ok());
    assert!("123456".parse::<TinyStr8>().is_ok());
    assert!("1234567".parse::<TinyStr8>().is_ok());
    assert!("12345678".parse::<TinyStr8>().is_ok());
    assert_eq!("123456789".parse::<TinyStr8>(), Err(Error::InvalidSize));
}

#[test]
fn tiny8_null() {
    assert_eq!("a\u{0}b".parse::<TinyStr8>(), Err(Error::InvalidNull));
}

#[test]
fn tiny8_new_unchecked() {
    let reference: TinyStr8 = "Windows".parse().unwrap();
    let uval: u64 = reference.into();
    let s = unsafe { TinyStr8::new_unchecked(uval) };
    assert_eq!(s, reference);
    assert_eq!(s, "Windows");
}

#[test]
fn tiny8_nonascii() {
    assert_eq!("\u{4000}".parse::<TinyStr8>(), Err(Error::NonAscii));
}

#[test]
fn tiny8_alpha() {
    let s: TinyStr8 = "@abcXYZ[".parse().unwrap();
    assert!(!s.is_ascii_alphanumeric());
    assert_eq!(s.to_ascii_uppercase().as_str(), "@ABCXYZ[");
    assert_eq!(s.to_ascii_lowercase().as_str(), "@abcxyz[");

    assert!("abcXYZ"
        .parse::<TinyStr8>()
        .unwrap()
        .is_ascii_alphanumeric());
}

#[test]
fn tiny8_ord() {
    let mut v: Vec<TinyStr8> = vec!["nedis".parse().unwrap(), "macos".parse().unwrap()];
    v.sort();

    assert_eq!(v.get(0).unwrap().as_str(), "macos");
    assert_eq!(v.get(1).unwrap().as_str(), "nedis");
}

#[test]
fn tiny8_eq() {
    let s1: TinyStr8 = "windows".parse().unwrap();
    let s2: TinyStr8 = "mac".parse().unwrap();
    let s3: TinyStr8 = "windows".parse().unwrap();

    assert_eq!(s1, s3);
    assert_ne!(s1, s2);
}

#[test]
fn tiny8_display() {
    let s: TinyStr8 = "abcdef".parse().unwrap();
    let mut result = String::new();
    write!(result, "{}", s).unwrap();
    assert_eq!(result, "abcdef");
    assert_eq!(format!("{}", s), "abcdef");
}

#[test]
fn tiny8_debug() {
    let s: TinyStr8 = "abcdef".parse().unwrap();
    assert_eq!(format!("{:#?}", s), "\"abcdef\"");
}
