use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day03::shared;
use day03::soln1;
use pprof::criterion::{Output, PProfProfiler};

pub fn part1(c: &mut Criterion) {
    let contents: &str = include_str!("../../inputs/day03.txt");
    let mut group = c.benchmark_group("day03.part1.realinput");
    group.bench_function("part1.nosum", |b| {
        b.iter(|| soln1::Soln1::part1(black_box(contents)))
    });
    group.finish();
}

pub fn part1_core(c: &mut Criterion) {
    let contents: &str = include_str!("../../inputs/day03.txt");
    let parsed = shared::parse(contents);
    let mut group = c.benchmark_group("day03.part1.realinput");
    group.bench_function("part1_core.nosum", |b| {
        b.iter(|| soln1::Soln1::part1_core(black_box(&parsed)))
    });
    group.finish();
}

pub fn part1_fast(c: &mut Criterion) {
    let contents: &str = include_str!("../../inputs/day03.txt");
    let mut group = c.benchmark_group("day03.part1.realinput");
    group.bench_function("part1_fast", |b| {
        b.iter(|| soln1::Soln1::part1_fast(black_box(contents)))
    });
    group.finish();
}

pub fn part2(c: &mut Criterion) {
    let contents: &str = include_str!("../../inputs/day03.txt");
    let mut group = c.benchmark_group("day03.part2.realinput");
    group.bench_function("part2.nosum", |b| {
        b.iter(|| soln1::Soln1::part2(black_box(contents)))
    });
    group.finish();
}

pub fn part2_core(c: &mut Criterion) {
    let contents: &str = include_str!("../../inputs/day03.txt");
    let parsed = shared::parse(contents);
    let mut group = c.benchmark_group("day03.part2.realinput");
    group.bench_function("part2_core.nosum", |b| {
        b.iter(|| soln1::Soln1::part2_core(black_box(&parsed)))
    });
    group.finish();
}

pub fn part2_fast(c: &mut Criterion) {
    let contents: &str = include_str!("../../inputs/day03.txt");
    let mut group = c.benchmark_group("day03.part2.realinput");
    group.bench_function("part2_fast", |b| {
        b.iter(|| soln1::Soln1::part2_fast(black_box(contents)))
    });
    group.finish();
}

criterion_group!(
    name=benches;
    config=Criterion::default().with_profiler(PProfProfiler::new(100, Output::Flamegraph(None)));
    targets=part1, part1_core, part1_fast, part2, part2_core, part2_fast);
criterion_main!(benches);
