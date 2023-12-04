use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day04::shared;
use day04::soln1;
use pprof::criterion::{Output, PProfProfiler};

pub fn part1(c: &mut Criterion) {
    let contents: &str = include_str!("../../inputs/day04.txt");
    let mut group = c.benchmark_group("day04.part1.realinput");
    group.bench_function("part1", |b| b.iter(|| soln1::Soln1::part1(black_box(contents))));
    group.finish();
}

pub fn part1_core(c: &mut Criterion) {
    let contents: &str = include_str!("../../inputs/day04.txt");
    let parsed = shared::parse(contents);
    let mut group = c.benchmark_group("day04.part1.realinput");
    group.bench_function("part1_core.nosum", |b| b.iter(|| soln1::Soln1::part1_core(black_box(&parsed))));
    group.finish();
}

pub fn part2(c: &mut Criterion) {
    let contents: &str = include_str!("../../inputs/day04.txt");
    let mut group = c.benchmark_group("day04.part2.realinput");
    group.bench_function("part2", |b| b.iter(|| soln1::Soln1::part2(black_box(contents))));
    group.finish();
}

pub fn part2_core(c: &mut Criterion) {
    let contents: &str = include_str!("../../inputs/day04.txt");
    let parsed = shared::parse(contents);
    let mut group = c.benchmark_group("day04.part2.realinput");
    group.bench_function("part2_core.nosum", |b| b.iter(|| soln1::Soln1::part2_core(black_box(&parsed))));
    group.finish();
}

criterion_group!(
    name=benches;
    config=Criterion::default().with_profiler(PProfProfiler::new(100, Output::Flamegraph(None)));
    targets=part1, part1_core, part2, part2_core);
criterion_main!(benches);
