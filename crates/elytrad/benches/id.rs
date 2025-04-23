use criterion::{criterion_group, criterion_main, Criterion};
use elytrad::id::generate;

fn bench_id_generation(c: &mut Criterion) {
    let mut group = c.benchmark_group("ID Generation");

    group.throughput(criterion::Throughput::Elements(1));

    group.bench_function("id::generate", |b| {
        b.iter(|| {
            let _id = generate(0);
        });
    });

    group.finish();
}

fn bench_base62_encoding(c: &mut Criterion) {
    let mut group = c.benchmark_group("ID Generation");

    let id = generate(0);

    group.throughput(criterion::Throughput::Elements(1));

    group.bench_function("base62::encode", |b| {
        b.iter(|| {
            let _ = id.to_string();
        });
    });

    group.finish();
}

criterion_group!(benches, bench_id_generation, bench_base62_encoding);
criterion_main!(benches);
