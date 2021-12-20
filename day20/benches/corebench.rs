use criterion::{criterion_group, criterion_main, Criterion};
// use day20::shared;
// use day20::soln1;

pub fn part1(c: &mut Criterion) {
    let contents: &str = include_str!("../inputs/day20.txt");

    let mut group = c.benchmark_group("day20.part1.realinput");
    // group.bench_function("part1", |b| b.iter(|| soln1::Soln1::part1(contents)));
    group.finish();
}

pub fn part2(c: &mut Criterion) {
    let contents: &str = include_str!("../inputs/day20.txt");

    let mut group = c.benchmark_group("day20.part2.realinput");
    // group.bench_function("part2", |b| b.iter(|| soln1::Soln1::part2(contents)));
    group.finish();
}

pub fn parse(c: &mut Criterion) {
    let contents: &str = include_str!("../inputs/day20.txt");

    let mut group = c.benchmark_group("day20.parse.realinput");
    // group.bench_function("parse", |b| b.iter(|| shared::parse(contents)));
    group.finish();
}

criterion_group!(
    name = benches;
    config = Criterion::default();
    targets = part1, part2, parse
);
criterion_main!(benches);
