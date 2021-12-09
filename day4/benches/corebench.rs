use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

include!("../src/soln.rs");

pub fn part1_core(c: &mut Criterion) {
    let contents: &str = include_str!("../inputs/day4.txt");

    c.bench_with_input(BenchmarkId::new("day4", contents.len()), &contents, |b, c| {
        b.iter(|| {
            let (a, b) = Soln1::parse(contents);
            Soln1::part1(a, b)
        });
    });
}

criterion_group!(benches, part1_core);
criterion_main!(benches);
