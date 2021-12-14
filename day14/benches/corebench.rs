use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use day14::shared::parse;
use day14::soln1::Soln1;
use day14::soln2;

pub fn input_scaling(c: &mut Criterion) {
    let contents: &str = include_str!("../inputs/day14.txt");
    let mut group = c.benchmark_group("day14.naive.input_scaling");
    for iters in 5..=12 {
        group.bench_with_input(BenchmarkId::new("total", iters), &iters, |b, &i| {
            b.iter(|| {
                let (poly, rules) = parse(contents);
                let final_poly = Soln1::apply_n(poly, rules.to_owned(), i);
                Soln1::score(&final_poly)
            })
        });
    }
    group.finish();

    let mut group = c.benchmark_group("day14.recursive_memo.input_scaling");
    let mut soln = Soln1::new();
    for iters in 5..=12 {
        group.bench_with_input(BenchmarkId::new("total", iters), &iters, |b, &i| {
            b.iter(|| {
                let (poly, rules) = parse(contents);
                let final_poly = soln.expand_chunkwise(poly, &rules, i);
                Soln1::score(&final_poly)
            })
        });
    }
    group.finish();

    let mut group = c.benchmark_group("day14.mapcount.input_scaling");
    for iters in 5..=12 {
        group.bench_with_input(BenchmarkId::new("total", iters), &iters, |b, &i| {
            b.iter(|| {
                soln2::run(contents, i);
            })
        });
    }
    group.finish();
}

pub fn part1(c: &mut Criterion) {
    let contents: &str = include_str!("../inputs/day14.txt");

    let mut group = c.benchmark_group("day14.part1.naive.realinput");
    group.bench_function("part1", |b| {
        b.iter(|| {
            let (poly, rules) = parse(contents);
            let final_poly = Soln1::apply_n(poly, rules.to_owned(), 10);
            Soln1::score(&final_poly)
        })
    });
    group.finish();

    let mut soln = Soln1::new();
    let mut group = c.benchmark_group("day14.part1.recursive_memo.realinput");
    group.bench_function("part1", |b| {
        b.iter(|| {
            let (poly, rules) = parse(contents);
            let final_poly = soln.expand_chunkwise(poly, &rules, 10);
            Soln1::score(&final_poly)
        })
    });
    group.finish();

    let mut group = c.benchmark_group("day14.part1.mapcount.realinput");
    group.bench_function("part1", |b| b.iter(|| soln2::run(contents, 10)));
    group.finish();
}

pub fn part2(c: &mut Criterion) {
    let contents: &str = include_str!("../inputs/day14.txt");

    let mut group = c.benchmark_group("day14.part2.mapcount.realinput");
    group.bench_function("part2", |b| b.iter(|| soln2::run(contents, 40)));
    group.finish();
}

criterion_group!(
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = input_scaling, part1, part2
);
criterion_main!(benches);
