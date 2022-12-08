use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day8::shared;
use day8::soln1;

pub fn part1(c: &mut Criterion) {
    let contents: &str = include_str!("../inputs/day8.txt");

    let mut group = c.benchmark_group("day8.part1.realinput");
    group.bench_function("part1", |b| b.iter(|| soln1::Soln1::part1(black_box(contents))));
    group.finish();
}

pub fn part1_noparse(c: &mut Criterion) {
    let contents: &str = include_str!("../inputs/day8.txt");

    let parsed = shared::parse(contents);
    let mut group = c.benchmark_group("day8.part1.realinput");
    group.bench_function("part1_core", |b| b.iter(|| soln1::Soln1::part1_core(black_box(parsed))));
    group.finish();
}

pub fn part2(c: &mut Criterion) {
    let contents: &str = include_str!("../inputs/day8.txt");

    let mut group = c.benchmark_group("day8.part2.realinput");
    group.bench_function("part2", |b| b.iter(|| soln1::Soln1::part2(black_box(contents))));
    group.finish();
}

criterion_group!(benches, part1, part1_noparse, part2);
criterion_main!(benches);
