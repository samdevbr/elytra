use criterion::{Criterion, black_box, criterion_group, criterion_main};
use rayon::prelude::*;
use snowflake::snowflake;

// Benchmark single-threaded snowflake generation
fn bench_single_thread(c: &mut Criterion) {
    snowflake::set_node_id(1);

    c.bench_function("snowflake_single_thread", |b| {
        b.iter(|| {
            for _ in 0..1_000 {
                black_box(snowflake());
            }
        });
    });
}

// Benchmark multithreaded snowflake generation with Rayon
fn bench_multi_thread(c: &mut Criterion) {
    c.bench_function("snowflake_multi_thread", |b| {
        b.iter(|| {
            (0..100_000).into_par_iter().for_each(|_| {
                black_box(snowflake());
            });
        });
    });
}

criterion_group!(benches, bench_single_thread, bench_multi_thread);
criterion_main!(benches);
