use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day{DAY_NUM}::shared;
use day{DAY_NUM}::soln1;

pub fn part1(c: &mut Criterion) {
    let contents: &str = include_str!("../../inputs/day{DAY_NUM}.txt");
    let mut group = c.benchmark_group("day{DAY_NUM}.part1.realinput");
    group.bench_function("part1", |b| b.iter(|| soln1::Soln1::part1(black_box(contents))));
    group.finish();
}

pub fn part1_core(c: &mut Criterion) {
    let contents: &str = include_str!("../../inputs/day{DAY_NUM}.txt");
    let parsed = shared::parse(contents);
    let mut group = c.benchmark_group("day{DAY_NUM}.part1.realinput");
    group.bench_function("part1_core.nosum", |b| b.iter(|| soln1::Soln1::part1(black_box(&parsed))));
    group.finish();
}

pub fn part2(c: &mut Criterion) {
    let contents: &str = include_str!("../../inputs/day{DAY_NUM}.txt");
    let mut group = c.benchmark_group("day{DAY_NUM}.part2.realinput");
    group.bench_function("part2", |b| b.iter(|| soln1::Soln1::part2(black_box(contents))));
    group.finish();
}

pub fn part2_core(c: &mut Criterion) {
    let contents: &str = include_str!("../../inputs/day{DAY_NUM}.txt");
    let parsed = shared::parse(contents);
    let mut group = c.benchmark_group("day{DAY_NUM}.part1.realinput");
    group.bench_function("part2_core.nosum", |b| b.iter(|| soln1::Soln1::part2(black_box(&parsed))));
    group.finish();
}

criterion_group!(benches, part1, part1_core, part2, part2_core);
criterion_main!(benches);
