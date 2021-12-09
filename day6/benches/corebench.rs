use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

include!("../src/soln.rs");

pub fn part1_core(c: &mut Criterion) {
    let contents: &str = include_str!("../inputs/day6.txt");

    let mut group = c.benchmark_group("day6-part1");
    group.bench_function("part1-80", |b| b.iter(|| Soln1::part1(contents, 80)));
    group.bench_function("part1-256", |b| b.iter(|| Soln1::part1(contents, 256)));
    group.finish();

    // c.bench_with_input(BenchmarkId::new("day1", contents.len()), &contents, |b, c| {
    // b.iter(|| Soln1::part1(c));
    // });
}

criterion_group!(benches, part1_core);
criterion_main!(benches);
