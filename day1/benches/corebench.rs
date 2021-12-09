use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

include!("../src/soln.rs");
use shared::Solution;

pub fn part1_core(c: &mut Criterion) {
    let contents: &str = include_str!("../inputs/day1.txt");

    c.bench_with_input(BenchmarkId::new("day1", contents.len()), &contents, |b, c| {
        b.iter(|| Soln1::part1_core(Soln1::parse(c)));
    });
}

criterion_group!(benches, part1_core);
criterion_main!(benches);
