#![feature(drain_filter)]
use criterion::{criterion_group, criterion_main, Criterion};
include!("../src/soln1.rs");

pub fn benchmarks(c: &mut Criterion) {
    // let contents: &str = include_str!("../inputs/day10.txt");
}

criterion_group!(benches, benchmarks);
criterion_main!(benches);
