#![feature(drain_filter)]
use criterion::{criterion_group, criterion_main, Criterion};
include!("../src/soln1.rs");

pub fn benchmarks(c: &mut Criterion) {
    let contents: &str = include_str!("../inputs/day10.txt");

    let mut group = c.benchmark_group("day10");
    group.bench_function("part1", |b| b.iter(|| Soln1::part1(contents)));
    group.bench_function("part2", |b| b.iter(|| Soln1::part2(contents)));
    group.finish()
}

criterion_group!(benches, benchmarks);
criterion_main!(benches);
