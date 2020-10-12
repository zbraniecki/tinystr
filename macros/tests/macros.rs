use tinystr_macros::*;

#[test]
fn test_u32() {
    const VALUE: u32 = u32_from_bytes!("aabb");
    assert_eq!(0x62626161, VALUE);
}

#[test]
fn test_u64() {
    const VALUE: u64 = u64_from_bytes!("aaaabbbb");
    assert_eq!(0x6262626261616161, VALUE);
}

#[test]
fn test_u128() {
    const VALUE: u128 = u128_from_bytes!("aaaaaaaabbbbbbbb");
    assert_eq!(0x62626262626262626161616161616161, VALUE);
}
