use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use day14::shared::parse;
use day14::soln1::Soln1;

pub fn benchmarks(c: &mut Criterion) {
    let contents: &str = include_str!("../inputs/day14.txt");

    let (poly, rules) = parse(contents);
    let mut group = c.benchmark_group("day14-naive");
    for iters in 5..=12 {
        group.bench_with_input(BenchmarkId::new("total", iters), &iters, |b, &i| {
            b.iter(|| {
                let final_poly = Soln1::apply_n(poly, rules.to_owned(), i);
                Soln1::score(&final_poly)
            })
        });
    }
    group.finish();

    let mut group = c.benchmark_group("day14-v2");
    let mut soln = Soln1::new();
    for iters in 5..=12 {
        group.bench_with_input(BenchmarkId::new("total", iters), &iters, |b, &i| {
            b.iter(|| {
                let final_poly = soln.expand_chunkwise(poly, &rules, i);
                Soln1::score(&final_poly)
            })
        });
    }
    group.finish();
}

criterion_group!(
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = benchmarks
);
criterion_main!(benches);
