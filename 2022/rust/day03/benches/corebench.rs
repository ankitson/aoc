use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day03::soln1;

pub fn part1(c: &mut Criterion) {
    let contents: &str = include_str!("../../inputs/day03.txt");

    let mut group = c.benchmark_group("day03.soln1.realinput");
    group.bench_function("part1", |b| b.iter(|| soln1::Soln1::part1(black_box(contents))));
    group.finish();
}

pub fn part2(c: &mut Criterion) {
    let contents: &str = include_str!("../../inputs/day03.txt");

    let mut group = c.benchmark_group("day03.soln2.realinput");
    group.bench_function("part2", |b| b.iter(|| soln1::Soln1::part2(black_box(contents))));
    group.finish();
}

criterion_group!(benches, part1, part2);
criterion_main!(benches);
