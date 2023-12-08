use std::arch::asm;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day07::shared;
use day07::soln1;
use itertools::Itertools;
use pprof::criterion::{Output, PProfProfiler};
use rand::prelude::*;

pub fn parse(c: &mut Criterion) {
    let contents: &str = include_str!("../../inputs/day07.txt");
    let mut group = c.benchmark_group("day07.parse.nosum.realinput");

    group.bench_function("parse", |b| b.iter(|| shared::parse(black_box(contents))));
    group.finish();
}

#[inline(never)]
pub fn read_all() -> u32 {
    let contents: &str = include_str!("../../inputs/day07.txt");
    let mut i: usize = 0;
    for _ in contents.chars() {
        i += 1;
    }
    i.count_ones()
}

#[inline(never)]
pub fn sort_10k_str(vec: &mut Vec<String>) {
    // we can use a NOP sled to mark where our code is in the decompiled assembly
    // - but inline(never) works too :)
    // unsafe {
    // asm!("99:"); //the label is renamed in the ASM so search for NOP
    // asm!("NOP");
    // }
    vec.sort();
}

pub fn baselines(c: &mut Criterion) {
    let mut group = c.benchmark_group("day07.baselines.nosum.realinput");
    group.bench_function("read_all", |b| b.iter(|| black_box(read_all())));

    let chars = ('2'..='9').chain(vec!['A', 'J', 'Q', 'K'].into_iter());
    let mut cands = chars.permutations(5).take(10000).map(|t| t.iter().join("")).collect_vec();
    let mut rng = rand::thread_rng();
    cands.shuffle(&mut rng);

    group.bench_function("sort_10k_str", |b| {
        b.iter(|| {
            black_box(sort_10k_str(black_box(&mut cands)));
        });
    });
}

pub fn part1(c: &mut Criterion) {
    let contents: &str = include_str!("../../inputs/day07.txt");
    let mut group = c.benchmark_group("day07.part1.realinput");

    group.bench_function("part1", |b| b.iter(|| soln1::Soln1::part1(black_box(contents))));
    group.finish();
}

pub fn part2(c: &mut Criterion) {
    let contents: &str = include_str!("../../inputs/day07.txt");
    let mut group = c.benchmark_group("day07.part2.realinput");

    group.bench_function("part2", |b| b.iter(|| soln1::Soln1::part2(black_box(contents))));
    group.finish();
}

criterion_group!(
    name=benches;
    config=Criterion::default().with_profiler(PProfProfiler::new(100, Output::Flamegraph(None)));
    targets=parse, baselines, part1, part2,);
criterion_main!(benches);
