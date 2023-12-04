use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day04::shared;
use day04::soln1;
use pprof::criterion::{Output, PProfProfiler};

pub fn part1(c: &mut Criterion) {
    let contents: &str = include_str!("../../inputs/day04.txt");
    let mut group = c.benchmark_group("day04.part1.realinput");
    group.bench_function("part1.nosum", |b| b.iter(|| soln1::Soln1::part1(black_box(contents))));
    group.finish();
}

pub fn part1_core(c: &mut Criterion) {
    let contents: &str = include_str!("../../inputs/day04.txt");
    let parsed = shared::parse(contents);
    let mut group = c.benchmark_group("day04.part1.realinput");
    group.bench_function("part1_core.nosum", |b| b.iter(|| soln1::Soln1::part1_core(black_box(&parsed))));
    group.finish();
}

pub fn part1_streamparse(c: &mut Criterion) {
    let contents: &str = include_str!("../../inputs/day04.txt");
    let mut group = c.benchmark_group("day04.part1.realinput");
    group.bench_function("part1_streamparse.nosum", |b| b.iter(|| soln1::Soln1::part1_streamparse(contents)));
    group.finish();
}

pub fn part1_bitset(c: &mut Criterion) {
    let contents: &str = include_str!("../../inputs/day04.txt");
    let mut group = c.benchmark_group("day04.part1.realinput");
    group.bench_function("part1_bitset", |b| b.iter(|| soln1::Soln1::part1_bitset(contents)));
    group.finish();
}

pub fn part2(c: &mut Criterion) {
    let contents: &str = include_str!("../../inputs/day04.txt");
    let mut group = c.benchmark_group("day04.part2.realinput");
    group.bench_function("part2.nosum", |b| b.iter(|| soln1::Soln1::part2(black_box(contents))));
    group.finish();
}

pub fn part2_core(c: &mut Criterion) {
    let contents: &str = include_str!("../../inputs/day04.txt");
    let parsed = shared::parse(contents);
    let mut group = c.benchmark_group("day04.part2.realinput");
    group.bench_function("part2_core.nosum", |b| b.iter(|| soln1::Soln1::part2_core(black_box(&parsed))));
    group.finish();
}

pub fn part2_streamparse(c: &mut Criterion) {
    let contents: &str = include_str!("../../inputs/day04.txt");
    let mut group = c.benchmark_group("day04.part2.realinput");
    group.bench_function("part2_streamparse", |b| b.iter(|| soln1::Soln1::part2_streamparse(black_box(contents))));
    group.finish();
}

criterion_group!(
    name=benches;
    config=Criterion::default().with_profiler(PProfProfiler::new(100, Output::Flamegraph(None)));
    targets=part1, part1_core, part1_streamparse, part1_bitset, part2, part2_core, part2_streamparse);
criterion_main!(benches);
