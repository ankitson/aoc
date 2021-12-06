use criterion::{criterion_group, criterion_main, Criterion};
include!("../src/soln.rs");

pub fn part1_core(c: &mut Criterion) {
    let contents: &str = include_str!("../inputs/day5.txt");

    let mut group = c.benchmark_group("day5-part1");
    group.bench_function("part1-1000", |b| b.iter(|| Soln1::part1(contents, 1000)));
    group.bench_function("part1-2000", |b| b.iter(|| Soln1::part1(contents, 2000)));
    group.finish();

    let mut group = c.benchmark_group("day5-part2");
    group.bench_function("part1-1000", |b| b.iter(|| Soln1::part2(contents, 1000)));
    group.bench_function("part1-2000", |b| b.iter(|| Soln1::part2(contents, 2000)));
    group.finish();
}

criterion_group!(benches, part1_core);
criterion_main!(benches);
