#![feature(extract_if)]
use criterion::{criterion_group, criterion_main, Criterion};
use day7::soln1;

pub fn part1(c: &mut Criterion) {
    let contents: &str = include_str!("../inputs/day7.txt");
    let mut group = c.benchmark_group("day7.part1.realinput");
    group.bench_function("part1", |b| b.iter(|| soln1::Soln1::part1(contents)));
    group.finish();
}

pub fn part1_fast(c: &mut Criterion) {
    let contents: &str = include_str!("../inputs/day7.txt");
    let mut group = c.benchmark_group("day7.part1.fast.realinput");
    group.bench_function("part1_fast", |b| b.iter(|| soln1::Soln1::part1_fast(contents)));
    group.finish();
}

pub fn part2(c: &mut Criterion) {
    let contents: &str = include_str!("../inputs/day7.txt");
    let mut group = c.benchmark_group("day7.part2.realinput");
    group.bench_function("part2", |b| b.iter(|| soln1::Soln1::part2(contents)));
    group.finish();
}

pub fn part2_fast(c: &mut Criterion) {
    let contents: &str = include_str!("../inputs/day7.txt");
    let mut group = c.benchmark_group("day7.part2.fast.realinput");
    group.bench_function("part2_fast", |b| b.iter(|| soln1::Soln1::part2_fast(contents)));
    group.finish();
}

criterion_group!(benches, part1, part1_fast, part2, part2_fast);
criterion_main!(benches);
