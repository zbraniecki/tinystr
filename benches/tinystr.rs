use criterion::black_box;
use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Bencher;
use criterion::Criterion;
use criterion::Fun;

use tinystr::{TinyStr4, TinyStr8};

static STRINGS_4: &[&str] = &[
    "US", "GB", "AR", "Hans", "CN", "AT", "PL", "FR", "AT", "Cyrl", "SR", "NO", "FR", "MK", "UK",
];

static STRINGS_8: &[&str] = &[
    "Latn", "windows", "AR", "Hans", "macos", "AT", "pl", "FR", "en", "Cyrl", "SR", "NO", "419",
    "und", "UK",
];

fn construct_from_str(c: &mut Criterion) {
    fn from_str_test<R: std::str::FromStr>(b: &mut Bencher, strings: &Vec<&str>)
    where
        <R as std::str::FromStr>::Err: std::fmt::Debug,
    {
        b.iter(|| {
            for s in strings {
                let _: R = black_box(s).parse().unwrap();
            }
        })
    };

    let funcs = vec![
        Fun::new("String", from_str_test::<String>),
        Fun::new("TinyStr4", from_str_test::<TinyStr4>),
        Fun::new("TinyStr8", from_str_test::<TinyStr8>),
    ];

    c.bench_functions("construct_from_str/4", funcs, STRINGS_4.to_vec());

    let funcs = vec![
        Fun::new("String", from_str_test::<String>),
        Fun::new("TinyStr8", from_str_test::<TinyStr8>),
    ];

    c.bench_functions("construct_from_str/8", funcs, STRINGS_8.to_vec());
}

fn construct_unchecked(c: &mut Criterion) {
    let funcs = vec![
        Fun::new("TinyStr4", |b, inputs: &Vec<&str>| {
            let raw: Vec<u32> = inputs
                .iter()
                .map(|s| s.parse::<TinyStr4>().unwrap().into())
                .collect();
            b.iter(move || {
                for num in &raw {
                    let _ = unsafe { TinyStr4::new_unchecked(black_box(*num)) };
                }
            })
        }),
        Fun::new("TinyStr8", |b, inputs: &Vec<&str>| {
            let raw: Vec<u64> = inputs
                .iter()
                .map(|s| s.parse::<TinyStr8>().unwrap().into())
                .collect();
            b.iter(move || {
                for num in &raw {
                    let _ = unsafe { TinyStr8::new_unchecked(black_box(*num)) };
                }
            })
        }),
    ];

    c.bench_functions("construct_unchecked/4", funcs, STRINGS_4.to_vec());

    let funcs = vec![Fun::new("TinyStr8", |b, inputs: &Vec<&str>| {
        let raw: Vec<u64> = inputs
            .iter()
            .map(|s| s.parse::<TinyStr8>().unwrap().into())
            .collect();
        b.iter(move || {
            for num in &raw {
                let _ = unsafe { TinyStr8::new_unchecked(black_box(*num)) };
            }
        })
    })];

    c.bench_functions("construct_unchecked/8", funcs, STRINGS_8.to_vec());
}

fn convert_to_ascii_lowercase(c: &mut Criterion) {
    let string_fn = |b: &mut Bencher, inputs: &Vec<&str>| {
        let raw: Vec<String> = inputs
            .iter()
            .map(|s| s.parse::<String>().unwrap())
            .collect();
        b.iter(move || {
            for s in &raw {
                let _ = s.to_ascii_lowercase();
            }
        })
    };
    let tinystr4_fn = |b: &mut Bencher, inputs: &Vec<&str>| {
        let raw: Vec<TinyStr4> = inputs
            .iter()
            .map(|s| s.parse::<TinyStr4>().unwrap())
            .collect();
        b.iter(move || {
            for s in &raw {
                let _ = s.to_ascii_lowercase();
            }
        })
    };
    let tinystr8_fn = |b: &mut Bencher, inputs: &Vec<&str>| {
        let raw: Vec<TinyStr8> = inputs
            .iter()
            .map(|s| s.parse::<TinyStr8>().unwrap())
            .collect();
        b.iter(move || {
            for s in &raw {
                let _ = s.to_ascii_lowercase();
            }
        })
    };

    c.bench_functions(
        "convert_to_ascii_lowercase/4",
        vec![
            Fun::new("String", string_fn.clone()),
            Fun::new("TinyStr4", tinystr4_fn.clone()),
            Fun::new("TinyStr8", tinystr8_fn.clone()),
        ],
        STRINGS_4.to_vec(),
    );
    c.bench_functions(
        "convert_to_ascii_lowercase/8",
        vec![
            Fun::new("String", string_fn.clone()),
            Fun::new("TinyStr8", tinystr8_fn.clone()),
        ],
        STRINGS_8.to_vec(),
    );
}

fn convert_to_ascii_uppercase(c: &mut Criterion) {
    let string_fn = |b: &mut Bencher, inputs: &Vec<&str>| {
        let raw: Vec<String> = inputs
            .iter()
            .map(|s| s.parse::<String>().unwrap())
            .collect();
        b.iter(move || {
            for s in &raw {
                let _ = s.to_ascii_uppercase();
            }
        })
    };
    let tinystr4_fn = |b: &mut Bencher, inputs: &Vec<&str>| {
        let raw: Vec<TinyStr4> = inputs
            .iter()
            .map(|s| s.parse::<TinyStr4>().unwrap())
            .collect();
        b.iter(move || {
            for s in &raw {
                let _ = s.to_ascii_uppercase();
            }
        })
    };
    let tinystr8_fn = |b: &mut Bencher, inputs: &Vec<&str>| {
        let raw: Vec<TinyStr8> = inputs
            .iter()
            .map(|s| s.parse::<TinyStr8>().unwrap())
            .collect();
        b.iter(move || {
            for s in &raw {
                let _ = s.to_ascii_uppercase();
            }
        })
    };

    c.bench_functions(
        "convert_to_ascii_uppercase/4",
        vec![
            Fun::new("String", string_fn.clone()),
            Fun::new("TinyStr4", tinystr4_fn.clone()),
            Fun::new("TinyStr8", tinystr8_fn.clone()),
        ],
        STRINGS_4.to_vec(),
    );
    c.bench_functions(
        "convert_to_ascii_uppercase/8",
        vec![
            Fun::new("String", string_fn.clone()),
            Fun::new("TinyStr8", tinystr8_fn.clone()),
        ],
        STRINGS_8.to_vec(),
    );
}

fn convert_to_ascii_titlecase(c: &mut Criterion) {
    let funcs = vec![
        Fun::new("String", |b, inputs: &Vec<&str>| {
            let raw: Vec<String> = inputs
                .iter()
                .map(|s| s.parse::<String>().unwrap())
                .collect();
            b.iter(move || {
                for s in &raw {
                    let mut result = s.to_ascii_lowercase();
                    result[0..1].make_ascii_uppercase();
                }
            })
        }),
        Fun::new("TinyStr4", |b, inputs: &Vec<&str>| {
            let raw: Vec<TinyStr4> = inputs
                .iter()
                .map(|s| s.parse::<TinyStr4>().unwrap())
                .collect();
            b.iter(move || {
                for s in &raw {
                    let _ = s.to_ascii_titlecase();
                }
            })
        }),
    ];

    c.bench_functions("convert_to_ascii_titlecase/4", funcs, STRINGS_4.to_vec());
}

fn test_is_ascii_alphanumeric(c: &mut Criterion) {
    let string_fn = |b: &mut Bencher, inputs: &Vec<&str>| {
        let raw: Vec<String> = inputs
            .iter()
            .map(|s| s.parse::<String>().unwrap())
            .collect();
        b.iter(move || {
            for s in &raw {
                let _ = s.chars().all(|c| c.is_ascii_alphanumeric());
            }
        })
    };
    let tinystr4_fn = |b: &mut Bencher, inputs: &Vec<&str>| {
        let raw: Vec<TinyStr4> = inputs
            .iter()
            .map(|s| s.parse::<TinyStr4>().unwrap())
            .collect();
        b.iter(move || {
            for s in &raw {
                let _ = s.is_ascii_alphanumeric();
            }
        })
    };
    let tinystr8_fn = |b: &mut Bencher, inputs: &Vec<&str>| {
        let raw: Vec<TinyStr8> = inputs
            .iter()
            .map(|s| s.parse::<TinyStr8>().unwrap())
            .collect();
        b.iter(move || {
            for s in &raw {
                let _ = s.is_ascii_alphanumeric();
            }
        })
    };

    c.bench_functions(
        "test_is_ascii_alphanumeric/4",
        vec![
            Fun::new("String", string_fn.clone()),
            Fun::new("TinyStr4", tinystr4_fn.clone()),
            Fun::new("TinyStr8", tinystr8_fn.clone()),
        ],
        STRINGS_4.to_vec(),
    );
    c.bench_functions(
        "test_is_ascii_alphanumeric/8",
        vec![
            Fun::new("String", string_fn.clone()),
            Fun::new("TinyStr8", tinystr8_fn.clone()),
        ],
        STRINGS_8.to_vec(),
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
);
criterion_main!(benches);
