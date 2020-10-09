mod error;
mod helpers;

pub use error::Error;

use std::num::{NonZeroU32, NonZeroU64, NonZeroU128};

#[inline(always)]
pub fn u32_from_bytes(bytes: &[u8]) -> Result<NonZeroU32, Error> {
    unsafe {
        match bytes.len() {
            1 => helpers::make_u32_bytes(bytes, 1, 0x80),
            2 => helpers::make_u32_bytes(bytes, 2, 0x8080),
            3 => helpers::make_u32_bytes(bytes, 3, 0x0080_8080),
            4 => helpers::make_u32_bytes(bytes, 4, 0x8080_8080),
            _ => Err(Error::InvalidSize),
        }
    }
}

#[test]
fn test_u32_from_bytes() {
    assert_eq!(
        NonZeroU32::new(0x62626161).unwrap(),
        u32_from_bytes(b"aabb").unwrap()
    );
}

#[inline(always)]
pub fn u64_from_bytes(bytes: &[u8]) -> Result<NonZeroU64, Error> {
    let len = bytes.len();
    if len < 1 || len > 8 {
        return Err(Error::InvalidSize);
    }
    let mask = 0x80808080_80808080u64 >> (8 * (8 - len));
    unsafe {
        helpers::make_u64_bytes(bytes, len, mask)
    }
}

#[test]
fn test_u64_from_bytes() {
    assert_eq!(
        NonZeroU64::new(0x6262626261616161).unwrap(),
        u64_from_bytes(b"aaaabbbb").unwrap()
    );
}

#[inline(always)]
pub fn u128_from_bytes(bytes: &[u8]) -> Result<NonZeroU128, Error> {
    let len = bytes.len();
    if len < 1 || len > 16 {
        return Err(Error::InvalidSize);
    }
    let mask = 0x80808080_80808080_80808080_80808080u128 >> (8 * (16 - len));
    unsafe {
        helpers::make_u128_bytes(bytes, len, mask)
    }
}

#[test]
fn test_u128_from_bytes() {
    assert_eq!(
        NonZeroU128::new(0x62626262626262626161616161616161).unwrap(),
        u128_from_bytes(b"aaaaaaaabbbbbbbb").unwrap()
    );
}
