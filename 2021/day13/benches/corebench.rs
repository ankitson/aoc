// #![feature(array_zip)]
use criterion::{criterion_group, criterion_main, Criterion};
use day13::soln1;

pub fn part1(c: &mut Criterion) {
    let contents: &str = include_str!("../inputs/day13.txt");
    let mut group = c.benchmark_group("day13.part1.staticgrid.realinput");
    group.bench_function("part1", |b| b.iter(|| soln1::Soln1::part1(contents, 2000)));
    group.finish()
}
pub fn part2(c: &mut Criterion) {
    let contents: &str = include_str!("../inputs/day13.txt");
    let mut group = c.benchmark_group("day13.part2.staticgrid.realinput");
    group.bench_function("part2", |b| b.iter(|| soln1::Soln1::part2(contents, 2000)));
    group.finish()
}

criterion_group!(benches, part1, part2);
criterion_main!(benches);
