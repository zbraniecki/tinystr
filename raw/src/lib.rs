//! `tinystr-raw` exports functions to convert byte strings to raw TinyStr data.
//!
//! Not intended for public consumption; use `tinystr` instead.

#![cfg_attr(not(feature = "std"), no_std)]

mod error;
mod helpers;

pub use error::Error;

use core::num::{NonZeroU128, NonZeroU32, NonZeroU64};

#[inline(always)]
pub fn try_u32_from_bytes(bytes: &[u8]) -> Result<NonZeroU32, Error> {
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
        NonZeroU32::new(if cfg!(target_endian = "little") {
            0x6262_6161
        } else {
            0x6161_6262
        })
        .unwrap(),
        try_u32_from_bytes(b"aabb").unwrap()
    );
}

#[inline(always)]
pub fn try_u64_from_bytes(bytes: &[u8]) -> Result<NonZeroU64, Error> {
    let len = bytes.len();
    if !(1..=8).contains(&len) {
        return Err(Error::InvalidSize);
    }
    let mask = 0x8080_8080_8080_8080_u64 >> (8 * (8 - len));
    unsafe { helpers::make_u64_bytes(bytes, len, mask) }
}

#[test]
fn test_u64_from_bytes() {
    assert_eq!(
        NonZeroU64::new(if cfg!(target_endian = "little") {
            0x6262_6262_6161_6161
        } else {
            0x6161_6161_6262_6262
        })
        .unwrap(),
        try_u64_from_bytes(b"aaaabbbb").unwrap()
    );
}

#[inline(always)]
pub fn try_u128_from_bytes(bytes: &[u8]) -> Result<NonZeroU128, Error> {
    let len = bytes.len();
    if !(1..=16).contains(&len) {
        return Err(Error::InvalidSize);
    }
    let mask = 0x8080_8080_8080_8080_8080_8080_8080_8080_u128 >> (8 * (16 - len));
    unsafe { helpers::make_u128_bytes(bytes, len, mask) }
}

#[test]
fn test_u128_from_bytes() {
    assert_eq!(
        NonZeroU128::new(if cfg!(target_endian = "little") {
            0x6262_6262_6262_6262_6161_6161_6161_6161
        } else {
            0x6161_6161_6161_6161_6262_6262_6262_6262
        })
        .unwrap(),
        try_u128_from_bytes(b"aaaaaaaabbbbbbbb").unwrap()
    );
}
