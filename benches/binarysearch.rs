use criterion::black_box;
use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;

use tinystr::{TinyStr16, TinyStr4, TinyStr8};

static STRINGS_4: &[&str] = &[
    "en", "es", "it", "zh", "de", "arab", "pl", "fr", "sr", "nb", "mk", "uk", "hans", "und", "ug",
    "mn", "lif", "gan", "yue", "unr", "tuq", "klx", "kk", "cyrl",
];

macro_rules! bench_block {
    ($r:ty, $group:expr, $name:expr) => {
        let keys: Vec<$r> = STRINGS_4.iter().map(|s| s.parse::<$r>().unwrap()).collect();

        // Create about 36000 entries, with 2, 3 and 4 characters.
        // Some keys will not be present in this data.
        let mut strings = Vec::new();
        for i in 'a'..='z' {
            for j in 'a'..='z' {
                let raw = [i as u8, j as u8];
                strings.push(<$r>::from_bytes(&raw).unwrap());
                for k in 'a'..='z' {
                    let raw = [i as u8, j as u8, k as u8];
                    strings.push(<$r>::from_bytes(&raw).unwrap());
                    let raw = [i as u8, j as u8, i as u8, k as u8];
                    strings.push(<$r>::from_bytes(&raw).unwrap());
                }
            }
        }
        strings.sort_unstable();

        $group.bench_function($name, |b| {
            b.iter(|| {
                for key in keys.iter() {
                    let _ = black_box(strings.binary_search_by_key(&key, |l| l));
                }
            })
        });
    };
}

fn binarysearch_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("binarysearch");
    bench_block!(TinyStr4, group, "tinystr4");
    bench_block!(TinyStr8, group, "tinystr8");
    bench_block!(TinyStr16, group, "tinystr16");
    group.finish();
}

criterion_group!(benches, binarysearch_bench);
criterion_main!(benches);
