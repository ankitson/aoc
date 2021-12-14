use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use day5::soln1;

pub fn part1(c: &mut Criterion) {
    let contents: &str = include_str!("../inputs/day5.txt");

    let mut group = c.benchmark_group("day5.part1.realinput");
    for board_size in vec![1000, 2000, 4000] {
        group.bench_with_input(BenchmarkId::new("part1", board_size), &contents, |b, c| {
            b.iter(|| soln1::Soln1::part1(contents, board_size));
        });
    }
    group.finish();
}

pub fn part2(c: &mut Criterion) {
    let contents: &str = include_str!("../inputs/day5.txt");

    let mut group = c.benchmark_group("day5.part2.realinput");
    for board_size in vec![1000, 2000, 4000] {
        group.bench_with_input(BenchmarkId::new("part2", board_size), &contents, |b, c| {
            b.iter(|| soln1::Soln1::part2(contents, board_size));
        });
    }
    group.finish();
}

criterion_group!(benches, part1, part2);
criterion_main!(benches);
