use tinystr::*;
use serde_json;

macro_rules! test_roundtrip {
    ($f:ident, $ty:ident, $val:expr) => {
        #[test]
        fn $f() {
            let tiny: $ty = $val.parse().unwrap();
            let json_string = serde_json::to_string(&tiny).unwrap();
            let expected_json = concat!("\"", $val, "\"");
            assert_eq!(json_string, expected_json);
            let recover: $ty = serde_json::from_str(&json_string).unwrap();
            assert_eq!(&*tiny, &*recover);
        }
    };
}

test_roundtrip!(test_roundtrip4_1, TinyStr4, "en");
test_roundtrip!(test_roundtrip4_2, TinyStr4, "Latn");
test_roundtrip!(test_roundtrip8, TinyStr8, "calendar");
test_roundtrip!(test_roundtrip16, TinyStr16, "verylongstring");
test_roundtrip!(test_roundtripauto_1, TinyStrAuto, "shortstring");
test_roundtrip!(test_roundtripauto_2, TinyStrAuto, "veryveryverylongstring");
