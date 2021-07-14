use core::num::{NonZeroU128, NonZeroU32, NonZeroU64};
use core::ptr::copy_nonoverlapping;

use super::Error;

#[inline(always)]
pub(crate) unsafe fn make_u32_bytes(
    bytes: &[u8],
    len: usize,
    mask: u32,
) -> Result<NonZeroU32, Error> {
    // Mask is always supplied as little-endian.
    let mask = u32::from_le(mask);
    let mut word: u32 = 0;
    copy_nonoverlapping(bytes.as_ptr(), &mut word as *mut u32 as *mut u8, len);
    if (word & mask) != 0 {
        return Err(Error::NonAscii);
    }
    if ((mask - word) & mask) != 0 {
        return Err(Error::InvalidNull);
    }
    Ok(NonZeroU32::new_unchecked(word))
}

#[inline(always)]
pub(crate) unsafe fn make_u64_bytes(
    bytes: &[u8],
    len: usize,
    mask: u64,
) -> Result<NonZeroU64, Error> {
    // TODO: could do this with #cfg(target_endian), but this is clearer and
    // more confidence-inspiring.
    let mask = u64::from_le(mask);
    let mut word: u64 = 0;
    copy_nonoverlapping(bytes.as_ptr(), &mut word as *mut u64 as *mut u8, len);
    if (word & mask) != 0 {
        return Err(Error::NonAscii);
    }
    if ((mask - word) & mask) != 0 {
        return Err(Error::InvalidNull);
    }
    Ok(NonZeroU64::new_unchecked(word))
}

#[inline(always)]
pub(crate) unsafe fn make_u128_bytes(
    bytes: &[u8],
    len: usize,
    mask: u128,
) -> Result<NonZeroU128, Error> {
    // TODO: could do this with #cfg(target_endian), but this is clearer and
    // more confidence-inspiring.
    let mask = u128::from_le(mask);
    let mut word: u128 = 0;
    copy_nonoverlapping(bytes.as_ptr(), &mut word as *mut u128 as *mut u8, len);
    if (word & mask) != 0 {
        return Err(Error::NonAscii);
    }
    if ((mask - word) & mask) != 0 {
        return Err(Error::InvalidNull);
    }
    Ok(NonZeroU128::new_unchecked(word))
}
