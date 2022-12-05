use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use day2::soln1;

pub fn part1(c: &mut Criterion) {
    let contents: &str = include_str!("../inputs/day2.txt");
    let mut group = c.benchmark_group("day2.part1.realinput");
    group.bench_with_input(
        BenchmarkId::new("day2", contents.len()),
        &contents,
        |b, c| {
            b.iter(|| soln1::Soln1::part1_core(soln1::Soln1::parse(contents)));
        },
    );
    group.finish()
}

pub fn part2(c: &mut Criterion) {
    let contents: &str = include_str!("../inputs/day2.txt");
    let mut group = c.benchmark_group("day2.part2.realinput");
    group.bench_with_input(
        BenchmarkId::new("day2", contents.len()),
        &contents,
        |b, c| {
            b.iter(|| soln1::Soln1::part2_core(soln1::Soln1::parse(contents)));
        },
    );
    group.finish()
}
criterion_group!(benches, part1, part2);
criterion_main!(benches);
