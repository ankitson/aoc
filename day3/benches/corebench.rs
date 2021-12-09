use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

include!("../src/soln.rs");

pub fn part1_core(c: &mut Criterion) {
    let contents: &str = include_str!("../inputs/day3.txt");

    c.bench_with_input(BenchmarkId::new("day3", contents.len()), &contents, |b, c| {
        b.iter(|| {
            let (a, b) = Soln1::parse(c);
            let a = a.collect::<Vec<u32>>();
            Soln1::part1(&a, b);
        });
    });
}

criterion_group!(benches, part1_core);
criterion_main!(benches);
