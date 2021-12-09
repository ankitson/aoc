use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

include!("../src/soln.rs");
use shared::Solution;

pub fn part1_core(c: &mut Criterion) {
    let contents: &str = include_str!("../inputs/day2.txt");
    c.bench_with_input(BenchmarkId::new("day2", contents.len()), &contents, |b, c| {
        b.iter(|| Soln1::part1_core(Soln1::parse(contents)));
    });
}

criterion_group!(benches, part1_core);
criterion_main!(benches);
