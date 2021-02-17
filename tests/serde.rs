use tinystr::*;
use serde_json;

macro_rules! test_roundtrip {
    ($f:ident, $ty:ident, $val:expr, $bincode:expr) => {
        #[test]
        fn $f() {
            let tiny: $ty = $val.parse().unwrap();
            let json_string = serde_json::to_string(&tiny).unwrap();
            let expected_json = concat!("\"", $val, "\"");
            assert_eq!(json_string, expected_json);
            let recover: $ty = serde_json::from_str(&json_string).unwrap();
            assert_eq!(&*tiny, &*recover);

            let bin = bincode::serialize(&tiny).unwrap();
            assert_eq!(bin, $bincode);
            let debin: $ty = bincode::deserialize(&bin).unwrap();
            assert_eq!(&*tiny, &*debin);
        }
    };
}

test_roundtrip!(test_roundtrip4_1, TinyStr4, "en", [101, 110, 0, 0]);
test_roundtrip!(test_roundtrip4_2, TinyStr4, "Latn", [76, 97, 116, 110]);
test_roundtrip!(test_roundtrip8, TinyStr8, "calendar", [99, 97, 108, 101, 110, 100, 97, 114]);
test_roundtrip!(test_roundtrip16, TinyStr16, "verylongstring", [118, 101, 114, 121, 108, 111, 110, 103, 115, 116, 114, 105, 110, 103, 0, 0]);
test_roundtrip!(test_roundtripauto_1, TinyStrAuto, "shortstring", [11, 0, 0, 0, 0, 0, 0, 0, 115, 104, 111, 114, 116, 115, 116, 114, 105, 110, 103]);
test_roundtrip!(test_roundtripauto_2, TinyStrAuto, "veryveryverylongstring", [22, 0, 0, 0, 0, 0, 0, 0, 118, 101, 114, 121, 118, 101, 114, 121, 118, 101, 114, 121, 108, 111, 110, 103, 115, 116, 114, 105, 110, 103]);
