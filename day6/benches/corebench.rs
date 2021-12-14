use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use day6::soln1;

pub fn part1(c: &mut Criterion) {
    let contents: &str = include_str!("../inputs/day6.txt");

    let mut group = c.benchmark_group("day6.part1.realinput");
    for num_days in vec![64, 128] {
        group.bench_with_input(BenchmarkId::new("part1", num_days), &contents, |b, c| {
            b.iter(|| soln1::Soln1::part1(contents, num_days));
        });
    }
    group.finish();
}

pub fn part2(c: &mut Criterion) {
    let contents: &str = include_str!("../inputs/day6.txt");

    let mut group = c.benchmark_group("day6.part2.realinput");
    for num_days in vec![64, 128, 256] {
        group.bench_with_input(BenchmarkId::new("part2", num_days), &contents, |b, c| {
            b.iter(|| soln1::Soln1::part2(contents, num_days));
        });
    }
    group.finish();
}

criterion_group!(
    name = benches;
    config = Criterion::default();
    targets = part1,part2
);
criterion_main!(benches);
