use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use day4::soln1;

pub fn part1(c: &mut Criterion) {
    let contents: &str = include_str!("../inputs/day4.txt");

    let mut group = c.benchmark_group("day4.part1.realinput");
    group.bench_with_input(
        BenchmarkId::new("day4", contents.len()),
        &contents,
        |b, c| {
            b.iter(|| {
                let (a, b) = soln1::Soln1::parse(contents);
                soln1::Soln1::part1(a, b)
            });
        },
    );
    group.finish();
}

pub fn part2(c: &mut Criterion) {
    let contents: &str = include_str!("../inputs/day4.txt");

    let mut group = c.benchmark_group("day4.part2.realinput");
    group.bench_with_input(
        BenchmarkId::new("day4", contents.len()),
        &contents,
        |b, c| {
            b.iter(|| {
                let (a, b) = soln1::Soln1::parse(contents);
                soln1::Soln1::part2(a, b)
            });
        },
    );
    group.finish();
}

criterion_group!(benches, part1, part2);
criterion_main!(benches);
