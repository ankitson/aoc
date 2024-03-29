use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day02::shared;
use day02::soln1;
use pprof::criterion::{Output, PProfProfiler};

pub fn part1(c: &mut Criterion) {
    let contents: &str = include_str!("../../inputs/day02.txt");
    let mut group = c.benchmark_group("day02.part1.realinput");
    group.bench_function("part1.nosum", |b| {
        b.iter(|| soln1::Soln1::part1(black_box(contents)))
    });
    group.finish();
}

pub fn part1_parsing(c: &mut Criterion) {
    let contents: &str = include_str!("../../inputs/day02.txt");
    let mut group = c.benchmark_group("day02.part1.realinput");
    group.bench_function("part1_parsing", |b| {
        b.iter(|| soln1::Soln1::part1_parsing(black_box(contents)))
    });
    group.finish();
}

pub fn part2(c: &mut Criterion) {
    let contents: &str = include_str!("../../inputs/day02.txt");
    let mut group = c.benchmark_group("day02.part2.realinput");
    group.bench_function("part2.nosum", |b| {
        b.iter(|| soln1::Soln1::part2(black_box(contents)))
    });
    group.finish();
}

pub fn part2_parsing(c: &mut Criterion) {
    let contents: &str = include_str!("../../inputs/day02.txt");
    let mut group = c.benchmark_group("day02.part2.realinput");
    group.bench_function("part2_parsing", |b| {
        b.iter(|| soln1::Soln1::part2_parsing(black_box(contents)))
    });
    group.finish();
}

criterion_group!(
    name=benches;
    config=Criterion::default().with_profiler(PProfProfiler::new(100, Output::Flamegraph(None)));
    targets=part1, part1_parsing, part2, part2_parsing);
criterion_main!(benches);
