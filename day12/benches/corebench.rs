use criterion::{criterion_group, criterion_main, Criterion};
use day12::soln1::Soln1;

pub fn benchmarks(c: &mut Criterion) {
    let contents: &str = include_str!("../inputs/day12.txt");

    let mut group = c.benchmark_group("day12");
    group.bench_function("part1", |b| b.iter(|| Soln1::part1(contents)));
    group.bench_function("part2", |b| b.iter(|| Soln1::part2(contents)));
    group.finish()
}

criterion_group!(benches, benchmarks);
criterion_main!(benches);
