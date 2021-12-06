use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

include!("../src/soln.rs");

pub fn part1_core(c: &mut Criterion) {
    let contents: &str = include_str!("../inputs/day6.txt");

    // c.bench_with_input(
    // BenchmarkId::new("day1", contents.len()),
    // &Soln1::parse(contents),
    // |b, c| {
    // b.iter(|| Soln1::part1_core(c.clone()));
    // },
    // );
}

criterion_group!(benches, part1_core);
criterion_main!(benches);
