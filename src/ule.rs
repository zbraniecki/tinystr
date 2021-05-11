// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This module contains adapters to allow `tinystr` to work with [`zerovec`](https://docs.rs/zerovec)
//! and is enabled by enabling the `"zerovec"` feature of the `tinystr` crate.

use crate::{Error, TinyStr16, TinyStr4, TinyStr8};
use std::mem;
use zerovec::ule::{AsULE, PlainOldULE, ULE};

/// This is an unaligned little-endian version of TinyStr. It MUST contain a nonempty
/// ASCII-only byte sequence.
///
/// TinyStr is already endian-agnostic (like str), so the only difference is alignment.
///
/// This type is made available by enabling the `"zerovec"` feature of the `tinystr` crate.
#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct AsciiULE<const N: usize>(PlainOldULE<N>);

macro_rules! impl_str_ule_size {
    ($size:literal, $tiny:ty, $integer:ty) => {
        impl From<$tiny> for AsciiULE<$size> {
            fn from(s: $tiny) -> Self {
                // This converts between endiannesses twice: TinyStr::into converts
                // from little-endian into native, and PlainOldULE::from
                // converts from native to little again
                let int: $integer = s.into();
                AsciiULE(int.into())
            }
        }

        /// This impl is made available by enabling the `"zerovec"` feature of the `tinystr` crate.
        impl AsULE for $tiny {
            type ULE = AsciiULE<$size>;
            #[inline]
            fn as_unaligned(&self) -> Self::ULE {
                (*self).into()
            }
            #[inline]
            fn from_unaligned(unaligned: &Self::ULE) -> Self {
                unsafe {
                    // This is safe since AsciiULE guarantees that it comes from
                    // a valid TinyStr

                    // This converts between endiannesses twice: TinyStr::new_unchecked()
                    // takes in a native endian integer, which we produce via from_unaligned()
                    Self::new_unchecked(<$integer>::from_unaligned(&unaligned.0))
                }
            }
        }

        impl AsciiULE<$size> {
            #[inline]
            pub fn as_bytes(&self) -> &[u8] {
                self.0.as_bytes()
            }
        }

        /// This impl is made available by enabling the `"zerovec"` feature of the `tinystr` crate.
        impl ULE for AsciiULE<$size> {
            type Error = Error;
            #[inline]
            fn parse_byte_slice(bytes: &[u8]) -> Result<&[Self], Self::Error> {
                debug_assert!(mem::size_of::<$tiny>() == mem::size_of::<[u8; $size]>());

                let data = bytes.as_ptr();
                let len = bytes.len() / $size;

                let bytes_slice: &[[u8; $size]] =
                    unsafe { std::slice::from_raw_parts(data as *const [u8; $size], len) };
                for bytes in bytes_slice {
                    let bytes = bytes.split(|t| *t == 0).next().ok_or(Error::InvalidNull)?;
                    let _ = <$tiny>::from_bytes(&*bytes)?;
                }
                Ok(unsafe { std::slice::from_raw_parts(data as *const Self, len) })
            }
            #[inline]
            fn as_byte_slice(slice: &[Self]) -> &[u8] {
                let data = slice.as_ptr();
                let len = slice.len() * $size;
                // Safe because Self is transparent over [u8; $size]
                unsafe { std::slice::from_raw_parts(data as *const u8, len) }
            }
        }
    };
}

impl_str_ule_size!(4, TinyStr4, u32);
impl_str_ule_size!(8, TinyStr8, u64);
impl_str_ule_size!(16, TinyStr16, u128);

#[cfg(test)]
mod tests {
    use super::*;
    use std::vec;
    use std::vec::Vec;

    #[test]
    fn test_roundtrip() {
        let strings = vec!["en", "us", "zh-CN"];
        let tinies: Vec<TinyStr8> = strings.iter().map(|s| s.parse().unwrap()).collect();
        let individually_converted: Vec<AsciiULE<8>> =
            tinies.iter().map(|s| s.as_unaligned()).collect();
        let slice = AsciiULE::as_byte_slice(&individually_converted);
        let parsed_ules = AsciiULE::<8>::parse_byte_slice(slice).expect("Slice must parse");
        assert_eq!(individually_converted, parsed_ules);
        let recouped_tinies: Vec<TinyStr8> = parsed_ules
            .iter()
            .map(|u| TinyStr8::from_unaligned(&u))
            .collect();
        assert_eq!(tinies, recouped_tinies);
    }
}
