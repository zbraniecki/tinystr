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
