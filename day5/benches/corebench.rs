use criterion::{criterion_group, criterion_main, Criterion, black_box};
use day5::soln1;
use day5::shared;

pub fn part1(c: &mut Criterion) {
    let contents: &str = include_str!("../inputs/day5.txt");

    let mut group = c.benchmark_group("day5.part1.realinput");
    group.bench_function("part1", |b| b.iter(|| soln1::Soln1::part1(black_box(contents))));
    group.finish();
}

pub fn part1_noparse(c: &mut Criterion) {
    let contents: &str = include_str!("../inputs/day5.txt");

    let parsed = shared::parse(contents);
    let mut group = c.benchmark_group("day5.part1.realinput");
    group.bench_function("part1_core", |b| b.iter(|| soln1::Soln1::part1_core(black_box(parsed.to_owned()))));
    group.finish();
}

pub fn part2(c: &mut Criterion) {
    let contents: &str = include_str!("../inputs/day5.txt");

    let mut group = c.benchmark_group("day5.part2.realinput");
    group.bench_function("part2", |b| b.iter(|| soln1::Soln1::part2(black_box(contents))));
    group.finish();
}

pub fn part2_noparse(c: &mut Criterion) {
    let contents: &str = include_str!("../inputs/day5.txt");

    let parsed = shared::parse(contents);
    let mut group = c.benchmark_group("day5.part2.realinput");
    group.bench_function("part2_core", |b| b.iter(|| soln1::Soln1::part2_core(black_box(parsed.to_owned()))));
    group.finish();
}

criterion_group!(benches, part1, part1_noparse, part2, part2_noparse);
criterion_main!(benches);
