#![feature(drain_filter)]
use criterion::{criterion_group, criterion_main, Criterion};
include!("../src/soln.rs");

pub fn benchmarks(c: &mut Criterion) {
    let contents: &str = include_str!("../inputs/day7.txt");

    let mut group = c.benchmark_group("day7-part1");
    group.bench_function("part1", |b| b.iter(|| Soln1::part1(contents)));
    group.finish();
    //253us

    let mut group = c.benchmark_group("day7-part1_fast");
    group.bench_function("part1", |b| b.iter(|| Soln1::part1_fast(contents)));
    group.finish();
    //59us

    let mut group = c.benchmark_group("day7-part2");
    group.bench_function("part2", |b| b.iter(|| Soln1::part2(contents)));
    group.finish();
    //922us

    let mut group = c.benchmark_group("day7-part2_fast");
    group.bench_function("part2", |b| b.iter(|| Soln1::part2_fast(contents)));
    group.finish();
    //18us
}

criterion_group!(benches, benchmarks);
criterion_main!(benches);
