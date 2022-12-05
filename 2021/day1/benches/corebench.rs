use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use day1::soln1;

pub fn part1(c: &mut Criterion) {
    let contents: &str = include_str!("../inputs/day1.txt");

    let mut group = c.benchmark_group("day1.part1.realinput");
    group.bench_with_input(
        BenchmarkId::new("day1", contents.len()),
        &contents,
        |b, c| {
            b.iter(|| soln1::Soln1::part1_core(soln1::Soln1::parse(c)));
        },
    );
    group.finish()
}

pub fn part2(c: &mut Criterion) {
    let contents: &str = include_str!("../inputs/day1.txt");

    let mut group = c.benchmark_group("day1.part2.realinput");
    group.bench_with_input(
        BenchmarkId::new("day1", contents.len()),
        &contents,
        |b, c| {
            b.iter(|| soln1::Soln1::part2_core(soln1::Soln1::parse(c)));
        },
    );
    group.finish()
}

criterion_group!(benches, part1, part2);
criterion_main!(benches);
