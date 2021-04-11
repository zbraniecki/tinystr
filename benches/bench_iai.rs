use criterion::black_box;
use tinystr::*;

const HAYSTACK: &[TinyStr4] = &[
    tinystr4!("ar"),
    tinystr4!("be"),
    tinystr4!("de"),
    tinystr4!("en"),
    tinystr4!("fr"),
    tinystr4!("it"),
    tinystr4!("pl"),
    tinystr4!("ru"),
    tinystr4!("sk"),
    tinystr4!("zh"),
];

const NEEDLE: TinyStr4 = tinystr4!("en");

fn iai_tinystr() {
    if let Ok(idx) = black_box(HAYSTACK).binary_search(&NEEDLE) {
        let _ = black_box(idx);
    }
}

const HAYSTACK_U32: &[u32] = &[
    tinystr4!("ar").as_unsigned(),
    tinystr4!("be").as_unsigned(),
    tinystr4!("de").as_unsigned(),
    tinystr4!("en").as_unsigned(),
    tinystr4!("fr").as_unsigned(),
    tinystr4!("it").as_unsigned(),
    tinystr4!("pl").as_unsigned(),
    tinystr4!("ru").as_unsigned(),
    tinystr4!("sk").as_unsigned(),
    tinystr4!("zh").as_unsigned(),
];

const NEEDLE_U32: u32 = tinystr4!("en").as_unsigned();

fn iai_tinystr_as_unsigned() {
    if let Ok(idx) = black_box(HAYSTACK_U32).binary_search(&NEEDLE_U32) {
        let _ = black_box(idx);
    }
}

iai::main!(iai_tinystr, iai_tinystr_as_unsigned);
