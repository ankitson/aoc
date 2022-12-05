use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use day3::soln1;

pub fn part1(c: &mut Criterion) {
    let contents: &str = include_str!("../inputs/day3.txt");

    let mut group = c.benchmark_group("day3.part1.realinput");
    group.bench_with_input(
        BenchmarkId::new("day3", contents.len()),
        &contents,
        |b, c| {
            b.iter(|| {
                let (a, b) = soln1::Soln1::parse(c);
                let a = a.collect::<Vec<u32>>();
                soln1::Soln1::part1(&a, b);
            });
        },
    );
    group.finish();
}

pub fn part2(c: &mut Criterion) {
    let contents: &str = include_str!("../inputs/day3.txt");

    let mut group = c.benchmark_group("day3.part2.realinput");
    group.bench_with_input(
        BenchmarkId::new("day3", contents.len()),
        &contents,
        |b, c| {
            b.iter(|| {
                let (a, b) = soln1::Soln1::parse(c);
                let a = a.collect::<Vec<u32>>();
                soln1::Soln1::part2(&a, b);
            });
        },
    );
    group.finish();
}

criterion_group!(benches, part1, part2);
criterion_main!(benches);
