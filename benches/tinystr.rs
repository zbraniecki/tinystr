use criterion::black_box;
use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Bencher;
use criterion::Criterion;
use criterion::Fun;

use tinystr::{TinyStr4, TinyStr8, TinyStr16};

static STRINGS_4: &[&str] = &[
    "US", "GB", "AR", "Hans",
    "CN", "AT", "PL", "FR",
    "AT", "Cyrl", "SR", "NO",
    "FR", "MK", "UK",
];

static STRINGS_8: &[&str] = &[
    "Latn", "windows", "AR", "Hans",
    "macos", "AT", "pl", "FR",
    "en", "Cyrl", "SR", "NO",
    "419", "und", "UK",
];

static STRINGS_16: &[&str] = &[
    "Latn", "windows", "AR", "Hans",
    "macos", "AT", "infiniband", "FR",
    "en", "Cyrl", "FromIntegral", "NO",
    "419", "MacintoshOSX2019", "UK",
];

fn construct_from_str(c: &mut Criterion) {
    macro_rules! cfs {
        ($r:ty) => {
            |b: &mut Bencher, strings: &Vec<&str>| { 
                b.iter(|| {
                    for s in strings {
                        let _: $r = black_box(s.parse().unwrap());
                    }
                })
            }
        };
    };

    let funcs = vec![
        Fun::new("String", cfs!(String)),
        Fun::new("TinyStr4", cfs!(TinyStr4)),
        Fun::new("TinyStr8", cfs!(TinyStr8)),
        Fun::new("TinyStr16", cfs!(TinyStr16)),
    ];

    c.bench_functions("construct_from_str/4", funcs, STRINGS_4.to_vec());

    let funcs = vec![
        Fun::new("String", cfs!(String)),
        Fun::new("TinyStr8", cfs!(TinyStr8)),
        Fun::new("TinyStr16", cfs!(TinyStr16)),
    ];

    c.bench_functions("construct_from_str/8", funcs, STRINGS_8.to_vec());

    let funcs = vec![
        Fun::new("String", cfs!(String)),
        Fun::new("TinyStr16", cfs!(TinyStr16)),
    ];

    c.bench_functions("construct_from_str/16", funcs, STRINGS_16.to_vec());
}

fn construct_unchecked(c: &mut Criterion) {
    macro_rules! cu {
        ($tty:ty, $rty:ty) => {
            |b, inputs: &Vec<&str>| {
                let raw: Vec<$rty> = inputs
                    .iter()
                    .map(|s| s.parse::<$tty>().unwrap().into())
                    .collect();
                b.iter(move || {
                    for num in &raw {
                        let _ = unsafe { <$tty>::new_unchecked(black_box(*num)) };
                    }
                })
            }
        };
    };

    let funcs = vec![
        Fun::new("TinyStr4", cu!(TinyStr4, u32)),
    ];

    c.bench_functions("construct_unchecked/4", funcs, STRINGS_4.to_vec());

    let funcs = vec![
        Fun::new("TinyStr8", cu!(TinyStr8, u64)),
    ];

    c.bench_functions("construct_unchecked/8", funcs, STRINGS_8.to_vec());

    let funcs = vec![
        Fun::new("TinyStr16", cu!(TinyStr16, u128)),
    ];

    c.bench_functions("construct_unchecked/16", funcs, STRINGS_16.to_vec());
}

macro_rules! convert_to_ascii {
    ($ty:ty, $action:ident) => {
        |b: &mut Bencher, inputs: &Vec<&str>| {
            let raw: Vec<$ty> = inputs
                .iter()
                .map(|s| s.parse::<$ty>().unwrap())
                .collect();
            b.iter(move || {
                for s in &raw {
                    let _ = black_box(s.$action());
                }
            })
        }
    };
}

fn convert_to_ascii_lowercase(c: &mut Criterion) {
    macro_rules! ctal {
        ($ty:ty) => {convert_to_ascii!($ty, to_ascii_lowercase)};
    }

    c.bench_functions(
        "convert_to_ascii_lowercase/4",
        vec![
            Fun::new("String", ctal!(String)),
            Fun::new("TinyStr4", ctal!(TinyStr4)),
            Fun::new("TinyStr8", ctal!(TinyStr8)),
            Fun::new("TinyStr16", ctal!(TinyStr16)),
        ],
        STRINGS_4.to_vec(),
    );
    c.bench_functions(
        "convert_to_ascii_lowercase/8",
        vec![
            Fun::new("String", ctal!(String)),
            Fun::new("TinyStr8", ctal!(TinyStr8)),
            Fun::new("TinyStr16", ctal!(TinyStr16)),
        ],
        STRINGS_8.to_vec(),
    );
    c.bench_functions(
        "convert_to_ascii_lowercase/16",
        vec![
            Fun::new("String", ctal!(String)),
            Fun::new("TinyStr16", ctal!(TinyStr16)),
        ],
        STRINGS_16.to_vec(),
    );
}

fn convert_to_ascii_uppercase(c: &mut Criterion) {
    macro_rules! ctau {
        ($ty:ty) => {convert_to_ascii!($ty, to_ascii_uppercase)};
    }

    c.bench_functions(
        "convert_to_ascii_uppercase/4",
        vec![
            Fun::new("String", ctau!(String)),
            Fun::new("TinyStr4", ctau!(TinyStr4)),
            Fun::new("TinyStr8", ctau!(TinyStr8)),
            Fun::new("TinyStr16", ctau!(TinyStr16)),
        ],
        STRINGS_4.to_vec(),
    );
    c.bench_functions(
        "convert_to_ascii_uppercase/8",
        vec![
            Fun::new("String", ctau!(String)),
            Fun::new("TinyStr8", ctau!(TinyStr8)),
            Fun::new("TinyStr16", ctau!(TinyStr16)),
        ],
        STRINGS_8.to_vec(),
    );
    c.bench_functions(
        "convert_to_ascii_uppercase/16",
        vec![
            Fun::new("String", ctau!(String)),
            Fun::new("TinyStr16", ctau!(TinyStr16)),
        ],
        STRINGS_16.to_vec(),
    );
}

trait ExtToAsciiTitlecase {
    #[inline(always)]
    fn to_ascii_titlecase(&self) -> String;
}

impl ExtToAsciiTitlecase for str {
    fn to_ascii_titlecase(&self) -> String {
        let mut result = self.to_ascii_lowercase();
        result[0..1].make_ascii_uppercase();
        result
    }
}

fn convert_to_ascii_titlecase(c: &mut Criterion) {
    macro_rules! ctat {
        ($ty:ty) => {convert_to_ascii!($ty, to_ascii_titlecase)};
    }

    c.bench_functions(
        "convert_to_ascii_titlecase/4",
        vec![
            Fun::new("String", ctat!(String)),
            Fun::new("TinyStr4", ctat!(TinyStr4)),
            Fun::new("TinyStr8", ctat!(TinyStr8)),
            Fun::new("TinyStr16", ctat!(TinyStr16)),
        ],
        STRINGS_4.to_vec(),
    );
    c.bench_functions(
        "convert_to_ascii_titlecase/8",
        vec![
            Fun::new("String", ctat!(String)),
            Fun::new("TinyStr8", ctat!(TinyStr8)),
            Fun::new("TinyStr16", ctat!(TinyStr16)),
        ],
        STRINGS_8.to_vec(),
    );
    c.bench_functions(
        "convert_to_ascii_titlecase/16",
        vec![
            Fun::new("String", ctat!(String)),
            Fun::new("TinyStr16", ctat!(TinyStr16)),
        ],
        STRINGS_16.to_vec(),
    );
}

fn test_is_ascii_alphanumeric(c: &mut Criterion) {
    macro_rules! tiaa {
        ($ty:ty) => {
            |b: &mut Bencher, inputs: &Vec<&str>| {
                let raw: Vec<$ty> = inputs
                    .iter()
                    .map(|s| s.parse::<$ty>().unwrap())
                    .collect();
                b.iter(move || {
                    for s in &raw {
                        let _ = black_box(s.chars().all(|c| c.is_ascii_alphanumeric()));
                    }
                })
            }
        };
    }

    c.bench_functions(
        "test_is_ascii_alphanumeric/4",
        vec![
            Fun::new("String", tiaa!(String)),
            Fun::new("TinyStr4", tiaa!(TinyStr4)),
            Fun::new("TinyStr8", tiaa!(TinyStr8)),
            Fun::new("TinyStr16", tiaa!(TinyStr16)),
        ],
        STRINGS_4.to_vec(),
    );
    c.bench_functions(
        "test_is_ascii_alphanumeric/8",
        vec![
            Fun::new("String", tiaa!(String)),
            Fun::new("TinyStr8", tiaa!(TinyStr8)),
            Fun::new("TinyStr16", tiaa!(TinyStr16)),
        ],
        STRINGS_8.to_vec(),
    );
    c.bench_functions(
        "test_is_ascii_alphanumeric/16",
        vec![
            Fun::new("String", tiaa!(String)),
            Fun::new("TinyStr16", tiaa!(TinyStr16)),
        ],
        STRINGS_16.to_vec(),
    );
}

fn test_eq(c: &mut Criterion) {
    macro_rules! te {
        ($ty:ty) => {
            |b: &mut Bencher, inputs: &Vec<&str>| {
                let raw: Vec<$ty> = inputs
                    .iter()
                    .map(|s| s.parse::<$ty>().unwrap())
                    .collect();
                b.iter(move || {
                    for s in &raw {
                        for l in &raw {
                            let _ = black_box(s == l);
                        }
                    }
                })
            }
        };
    }

    c.bench_functions(
        "test_eq/4",
        vec![
            Fun::new("String", te!(String)),
            Fun::new("TinyStr4", te!(TinyStr4)),
            Fun::new("TinyStr8", te!(TinyStr8)),
            Fun::new("TinyStr16", te!(TinyStr16)),
        ],
        STRINGS_4.to_vec(),
    );
    c.bench_functions(
        "test_eq/8",
        vec![
            Fun::new("String", te!(String)),
            Fun::new("TinyStr8", te!(TinyStr8)),
            Fun::new("TinyStr16", te!(TinyStr16)),
        ],
        STRINGS_8.to_vec(),
    );
    c.bench_functions(
        "test_eq/16",
        vec![
            Fun::new("String", te!(String)),
            Fun::new("TinyStr16", te!(TinyStr16)),
        ],
        STRINGS_16.to_vec(),
    );
}

criterion_group!(
    benches,
    construct_from_str,
    construct_unchecked,
    convert_to_ascii_lowercase,
    convert_to_ascii_uppercase,
    convert_to_ascii_titlecase,
    test_is_ascii_alphanumeric,
    test_eq,
);
criterion_main!(benches);
