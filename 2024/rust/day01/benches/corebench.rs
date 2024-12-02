use std::ops::Mul;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day01::shared;
use day01::soln1;
use pprof::criterion::{Output, PProfProfiler};

pub fn parse(c: &mut Criterion) {
    let contents: &str = include_str!("../../inputs/day01.txt");
    let mut group = c.benchmark_group("day01.parse.nosum.realinput");

    group.bench_function("parse", |b| b.iter(|| soln1::parse(black_box(contents))));
    group.finish();
}

pub fn part1(c: &mut Criterion) {
    let contents: &str = include_str!("../../inputs/day01.txt");
    let mut group = c.benchmark_group("day01.part1.realinput");

    group.bench_function("part1", |b| b.iter(|| soln1::part1(black_box(contents))));
    group.bench_function("part1_par", |b| b.iter(|| soln1::part1_par(black_box(contents))));
    group.finish();
}

pub fn part2(c: &mut Criterion) {
    let contents: &str = include_str!("../../inputs/day01.txt");
    let mut group = c.benchmark_group("day01.part2.realinput");

    group.bench_function("part2", |b| b.iter(|| soln1::part2(black_box(contents))));
    group.bench_function("part2_par", |b| b.iter(|| soln1::part2_par(black_box(contents))));
    group.finish();
}

criterion_group!(
    name=benches;
    config=Criterion::default().with_profiler(PProfProfiler::new(100, Output::Protobuf));
    targets=parse, part1, part2,);
criterion_main!(benches);
