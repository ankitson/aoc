use criterion::{black_box, criterion_group, criterion_main, Criterion};
use experiments::popcnt;

pub fn popcnt_naive(c: &mut Criterion) {
    let mut group = c.benchmark_group("experiments");
    group.bench_function("popcnt_naive", |b| b.iter(|| popcnt::popcnt(0b10101010)));
    group.finish();
}
criterion_group!(benches, popcnt_naive);
criterion_main!(benches);
