use criterion::{black_box, criterion_group, criterion_main, Criterion};
// use day23::shared;
use day23::soln1;

pub fn part1(c: &mut Criterion) {
    let contents: &str = include_str!("../inputs/day23.txt");

    let mut group = c.benchmark_group("day23.part1.realinput");
    // group.bench_function("part1", |b| b.iter(|| soln1::Soln1::part1(black_box(contents))));
    group.finish();
}

pub fn part2(c: &mut Criterion) {
    let contents: &str = include_str!("../inputs/day23.txt");

    let mut group = c.benchmark_group("day23.part2.realinput");
    // group.bench_function("part2", |b| b.iter(|| soln1::Soln1::part2(black_box(contents))));
    group.finish();
}

pub fn parse(c: &mut Criterion) {
    let contents: &str = include_str!("../inputs/day23.txt");

    let mut group = c.benchmark_group("day23.parse.realinput");
    // group.bench_function("parse", |b| b.iter(|| shared::parse(black_box(contents))));
    group.finish();
}

criterion_group!(
    name = benches;
    config = Criterion::default();
    targets = part1, part2, parse
);
criterion_main!(benches);
