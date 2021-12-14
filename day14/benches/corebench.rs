// use super::shared;

use criterion::{criterion_group, criterion_main, Criterion};
use day14::shared::parse;
use day14::soln1::Soln1;

// include!("../src/shared.rs");
// include!("../src/soln1.rs");

pub fn benchmarks(c: &mut Criterion) {
    let contents: &str = include_str!("../inputs/day14.txt");

    let (poly, rules) = parse(contents);
    // let final_poly = Soln1::apply_n(poly, rules, 10);
    // Self::score(&final_poly)

    let mut group = c.benchmark_group("day14");
    group.bench_function("total (5)", |b| {
        b.iter(|| {
            let final_poly = Soln1::apply_n(poly, rules.to_owned(), 5);
            Soln1::score(&final_poly)
        })
    });
    group.bench_function("total (8)", |b| {
        b.iter(|| {
            let final_poly = Soln1::apply_n(poly, rules.to_owned(), 8);
            Soln1::score(&final_poly)
        })
    });
    group.bench_function("total (12)", |b| {
        b.iter(|| {
            let final_poly = Soln1::apply_n(poly, rules.to_owned(), 12);
            Soln1::score(&final_poly)
        })
    });
    group.finish()
}

criterion_group!(
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = benchmarks
);
criterion_main!(benches);
