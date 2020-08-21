use tinystr::*;
use tinystr_macros::*;

const TS: TinyStr8 = tinystr8!("test");

#[test]
fn test_macros() {
    let x: TinyStr8 = "test".parse().unwrap();
    assert_eq!(TS, x);

    let x: TinyStr4 = "foo".parse().unwrap();
    assert_eq!(tinystr4!("foo"), x);

    let x: TinyStr8 = "barbaz".parse().unwrap();
    assert_eq!(tinystr8!("barbaz"), x);

    let x: TinyStr16 = "metamorphosis".parse().unwrap();
    assert_eq!(tinystr16!("metamorphosis"), x);
}
