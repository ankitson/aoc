use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day01::shared;
use day01::soln1;
use pprof::criterion::{Output, PProfProfiler};

pub fn part1(c: &mut Criterion) {
    let contents: &str = include_str!("../../inputs/day01.txt");
    let mut group = c.benchmark_group("day01.part1.realinput");
    group.bench_function("part1", |b| {
        b.iter(|| soln1::Soln1::part1(black_box(contents)))
    });
    group.finish();
}

pub fn part1_memchr(c: &mut Criterion) {
    let contents: &str = include_str!("../../inputs/day01.txt");
    let mut group = c.benchmark_group("day01.part1_simd.realinput");
    group.bench_function("part1_memchr", |b| {
        b.iter(|| soln1::Soln1::part1_memchr(black_box(contents)))
    });
    group.finish();
}

pub fn part2(c: &mut Criterion) {
    let contents: &str = include_str!("../../inputs/day01.txt");
    let mut group = c.benchmark_group("day01.part2.realinput");
    group.bench_function("part2", |b| {
        b.iter(|| soln1::Soln1::part2(black_box(contents)))
    });
    group.finish();
}

criterion_group!(
    name=benches;
    config=Criterion::default().with_profiler(PProfProfiler::new(100, Output::Flamegraph(None)));
    targets=part1, part1_memchr, part2);
criterion_main!(benches);
