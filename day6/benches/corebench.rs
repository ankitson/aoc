use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};

include!("../src/soln.rs");

pub fn bench(c: &mut Criterion) {
    let contents: &str = include_str!("../inputs/day6.txt");

    let mut group = c.benchmark_group("day6");
    for num_days in [32, 64, 128, 256, 384, 512, 768, 1024].iter() {
        group.throughput(Throughput::Elements(*num_days as u64));
        group.bench_with_input(BenchmarkId::new("part2", num_days), &num_days, |b, &nd| {
            b.iter(|| Soln1::part2(contents, *nd))
        });
    }
    group.finish();
}

criterion_group!(benches, bench);
criterion_main!(benches);
