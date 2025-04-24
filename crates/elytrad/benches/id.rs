use criterion::{black_box, criterion_group, criterion_main, Criterion};
use elytrad::id::snowflake;
use rayon::prelude::*;

// Benchmark single-threaded snowflake generation
fn bench_single_thread(c: &mut Criterion) {
    c.bench_function("snowflake_single_thread", |b| {
        b.iter(|| {
            for _ in 0..1_000 {
                black_box(snowflake(0));
            }
        });
    });
}

// Benchmark multithreaded snowflake generation with Rayon
fn bench_multi_thread(c: &mut Criterion) {
    c.bench_function("snowflake_multi_thread", |b| {
        b.iter(|| {
            (0..100_000).into_par_iter().for_each(|_| {
                black_box(snowflake(0));
            });
        });
    });
}

criterion_group!(benches, bench_single_thread, bench_multi_thread);
criterion_main!(benches);
