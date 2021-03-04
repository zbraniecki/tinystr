use criterion::black_box;
use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Bencher;
use criterion::Criterion;

use tinystr::{TinyStr16, TinyStr4, TinyStr8, TinyStrAuto};

static STRINGS_4: &[&str] = &[
    "US", "GB", "AR", "Hans", "CN", "AT", "PL", "FR", "AT", "Cyrl", "SR", "NO", "FR", "MK", "UK",
];

static STRINGS_8: &[&str] = &[
    "Latn", "windows", "AR", "Hans", "macos", "AT", "pl", "FR", "en", "Cyrl", "SR", "NO", "419",
    "und", "UK",
];

static STRINGS_16: &[&str] = &[
    "Latn",
    "windows",
    "AR",
    "Hans",
    "macos",
    "AT",
    "infiniband",
    "FR",
    "en",
    "Cyrl",
    "FromIntegral",
    "NO",
    "419",
    "MacintoshOSX2019",
    "UK",
];

macro_rules! bench_block {
    ($c:expr, $name:expr, $action:ident) => {
        let mut group4 = $c.benchmark_group(&format!("{}/4", $name));
        group4.bench_function("String", $action!(String, STRINGS_4));
        group4.bench_function("TinyStr4", $action!(TinyStr4, STRINGS_4));
        group4.bench_function("TinyStr8", $action!(TinyStr8, STRINGS_4));
        group4.bench_function("TinyStr16", $action!(TinyStr16, STRINGS_4));
        group4.bench_function("TinyStrAuto", $action!(TinyStrAuto, STRINGS_4));
        group4.finish();

        let mut group8 = $c.benchmark_group(&format!("{}/8", $name));
        group8.bench_function("String", $action!(String, STRINGS_8));
        group8.bench_function("TinyStr8", $action!(TinyStr8, STRINGS_8));
        group8.bench_function("TinyStr16", $action!(TinyStr16, STRINGS_8));
        group8.bench_function("TinyStrAuto", $action!(TinyStrAuto, STRINGS_8));
        group8.finish();

        let mut group16 = $c.benchmark_group(&format!("{}/16", $name));
        group16.bench_function("String", $action!(String, STRINGS_16));
        group16.bench_function("TinyStr16", $action!(TinyStr16, STRINGS_16));
        group16.bench_function("TinyStrAuto", $action!(TinyStrAuto, STRINGS_16));
        group16.finish();
    };
}

fn construct_from_str(c: &mut Criterion) {
    macro_rules! cfs {
        ($r:ty, $inputs:expr) => {
            |b: &mut Bencher| {
                b.iter(|| {
                    for s in $inputs {
                        let _: $r = black_box(s.parse().unwrap());
                    }
                })
            }
        };
    }

    bench_block!(c, "construct_from_str", cfs);
}

fn construct_from_bytes(c: &mut Criterion) {
    macro_rules! cfu {
        ($r:ty, $inputs:expr) => {
            |b| {
                let raw: Vec<&[u8]> = $inputs.iter().map(|s| s.as_bytes()).collect();
                b.iter(move || {
                    for u in &raw {
                        let _ = black_box(<$r>::from_bytes(*u).unwrap());
                    }
                })
            }
        };
    }

    let mut group4 = c.benchmark_group("construct_from_bytes/4");
    group4.bench_function("TinyStr4", cfu!(TinyStr4, STRINGS_4));
    group4.bench_function("TinyStr8", cfu!(TinyStr8, STRINGS_4));
    group4.bench_function("TinyStr16", cfu!(TinyStr16, STRINGS_4));
    group4.finish();

    let mut group8 = c.benchmark_group("construct_from_bytes/8");
    group8.bench_function("TinyStr8", cfu!(TinyStr8, STRINGS_8));
    group8.bench_function("TinyStr16", cfu!(TinyStr16, STRINGS_8));
    group8.finish();

    let mut group16 = c.benchmark_group("construct_from_bytes/16");
    group16.bench_function("TinyStr16", cfu!(TinyStr16, STRINGS_16));
    group16.finish();
}

fn construct_unchecked(c: &mut Criterion) {
    macro_rules! cu {
        ($tty:ty, $rty:ty, $inputs:expr) => {
            |b| {
                let raw: Vec<$rty> = $inputs
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
    }

    let mut group4 = c.benchmark_group("construct_unchecked/4");
    group4.bench_function("TinyStr4", cu!(TinyStr4, u32, STRINGS_4));
    group4.finish();

    let mut group8 = c.benchmark_group("construct_unchecked/8");
    group8.bench_function("TinyStr8", cu!(TinyStr8, u64, STRINGS_8));
    group8.finish();

    let mut group16 = c.benchmark_group("construct_unchecked/16");
    group16.bench_function("TinyStr16", cu!(TinyStr16, u128, STRINGS_16));
    group16.finish();
}

criterion_group!(
    benches,
    construct_from_str,
    construct_from_bytes,
    construct_unchecked,
);
criterion_main!(benches);
