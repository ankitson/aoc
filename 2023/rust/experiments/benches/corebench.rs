use criterion::{black_box, criterion_group, criterion_main, Criterion};
use experiments::popcnt;

pub fn popcnt(c: &mut Criterion) {
    let mut group = c.benchmark_group("popcnt");
    group.bench_function("popcnt_naive", |b| {
        b.iter(|| popcnt::popcnt_naive(0b10101010))
    });

    group.bench_function("popcnt_split", |b| {
        b.iter(|| popcnt::popcnt_split(0b10101010))
    });
    group.finish();
}

criterion_group!(benches, popcnt);
criterion_main!(benches);
