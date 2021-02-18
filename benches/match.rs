use criterion::black_box;
use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;

use tinystr::{tinystr16, tinystr4, tinystr8};
use tinystr::{TinyStr16, TinyStr4, TinyStr8};

macro_rules! count {
    () => (0usize);
    ( $x:tt $($xs:tt)* ) => (1usize + count!($($xs)*));
}

macro_rules! gen_match {
    ($TinyType:ident, [$( ($item:ident, $tiny:expr) ),*]) => {
        const STRINGS: [&str; count!($($item)*)] = [
            $(stringify!($item),)*
        ];
        const TINYSTRS: [$TinyType; count!($($item)*)] = [
            $($tiny,)*
        ];
        fn match_str(s: &str) -> &'static str {
            match s {
                $(stringify!($item) => stringify!($item),)*
                _ => "",
            }
        }
        #[allow(non_upper_case_globals)]
        fn match_tiny(t: $TinyType) -> &'static str {
            $(const $item: $TinyType = $tiny;)*
            match t {
                $($item => stringify!($item),)*
                _ => "",
            }
        }
    }
}

fn tinystr4_match(c: &mut Criterion) {
    gen_match!(
        TinyStr4,
        [
            (US, tinystr4!("US")),
            (GB, tinystr4!("GB")),
            (AR, tinystr4!("AR")),
            (Hans, tinystr4!("Hans")),
            (CN, tinystr4!("CN")),
            (AT, tinystr4!("AT")),
            (PL, tinystr4!("PL")),
            (Cyrl, tinystr4!("Cyrl")),
            (SR, tinystr4!("SR")),
            (NO, tinystr4!("NO")),
            (FR, tinystr4!("FR")),
            (MK, tinystr4!("MK")),
            (UK, tinystr4!("UK")),
            (ZH, tinystr4!("ZH")),
            (Mymr, tinystr4!("Mymr"))
        ]
    );

    c.bench_function("match str 4", |b| {
        b.iter(|| {
            for s in STRINGS.iter() {
                black_box(match_str(s));
            }
        })
    });

    c.bench_function("match tiny 4", |b| {
        b.iter(|| {
            for s in TINYSTRS.iter() {
                black_box(match_tiny(*s));
            }
        })
    });
}

fn tinystr8_match(c: &mut Criterion) {
    gen_match!(
        TinyStr8,
        [
            (Latn, tinystr8!("Latn")),
            (windows, tinystr8!("windows")),
            (AR, tinystr8!("AR")),
            (Hans, tinystr8!("Hans")),
            (macos, tinystr8!("macos")),
            (AT, tinystr8!("AT")),
            (pl, tinystr8!("pl")),
            (FR, tinystr8!("FR")),
            (en, tinystr8!("en")),
            (Cyrl, tinystr8!("Cyrl")),
            (SR, tinystr8!("SR")),
            (NO, tinystr8!("NO")),
            (A419, tinystr8!("A419")),
            (und, tinystr8!("und")),
            (UK, tinystr8!("UK"))
        ]
    );

    c.bench_function("match str 8", |b| {
        b.iter(|| {
            for s in STRINGS.iter() {
                black_box(match_str(s));
            }
        })
    });

    c.bench_function("match tiny 8", |b| {
        b.iter(|| {
            for s in TINYSTRS.iter() {
                black_box(match_tiny(*s));
            }
        })
    });
}

fn tinystr16_match(c: &mut Criterion) {
    gen_match!(
        TinyStr16,
        [
            (Latn, tinystr16!("Latn")),
            (windows, tinystr16!("windows")),
            (AR, tinystr16!("AR")),
            (Hans, tinystr16!("Hans")),
            (macos, tinystr16!("macos")),
            (AT, tinystr16!("AT")),
            (infiniband, tinystr16!("infiniband")),
            (FR, tinystr16!("FR")),
            (en, tinystr16!("en")),
            (Cyrl, tinystr16!("Cyrl")),
            (FromIntegral, tinystr16!("FromIntegral")),
            (NO, tinystr16!("NO")),
            (A419, tinystr16!("A419")),
            (MacintoshOSX2019, tinystr16!("MacintoshOSX2019")),
            (UK, tinystr16!("UK"))
        ]
    );

    c.bench_function("match str 16", |b| {
        b.iter(|| {
            for s in STRINGS.iter() {
                black_box(match_str(s));
            }
        })
    });

    c.bench_function("match tiny 16", |b| {
        b.iter(|| {
            for s in TINYSTRS.iter() {
                black_box(match_tiny(*s));
            }
        })
    });
}

criterion_group!(benches, tinystr4_match, tinystr8_match, tinystr16_match,);
criterion_main!(benches);
