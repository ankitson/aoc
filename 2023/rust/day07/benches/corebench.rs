use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day07::shared;
use day07::soln1;
use pprof::criterion::{Output, PProfProfiler};

pub fn parse(c: &mut Criterion) {
    let contents: &str = include_str!("../../inputs/day07.txt");
    let mut group = c.benchmark_group("day07.parse.nosum.realinput");

    group.bench_function("parse", |b| b.iter(|| shared::parse(black_box(contents))));
    group.finish();
}

pub fn part1(c: &mut Criterion) {
    let contents: &str = include_str!("../../inputs/day07.txt");
    let mut group = c.benchmark_group("day07.part1.realinput");

    group.bench_function("part1", |b| b.iter(|| soln1::Soln1::part1(black_box(contents))));
    group.finish();
}

pub fn part2(c: &mut Criterion) {
    let contents: &str = include_str!("../../inputs/day07.txt");
    let mut group = c.benchmark_group("day07.part2.realinput");

    group.bench_function("part2", |b| b.iter(|| soln1::Soln1::part2(black_box(contents))));
    group.finish();
}

criterion_group!(
    name=benches;
    config=Criterion::default().with_profiler(PProfProfiler::new(100, Output::Flamegraph(None)));
    targets=parse, part1, part2,);
criterion_main!(benches);
