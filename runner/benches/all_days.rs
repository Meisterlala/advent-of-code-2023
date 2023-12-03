use criterion::{criterion_group, criterion_main, Criterion};

use runner::DAYS;

pub fn criterion_benchmark(c: &mut Criterion) {
    for d in DAYS {
        if let Some(p1) = d.part1 {
            c.bench_function(&format!("Day {:2} Part 1", d.day), |b| b.iter(p1));
        }
        if let Some(p2) = d.part2 {
            c.bench_function(&format!("Day {:2} Part 2", d.day), |b| b.iter(p2));
        }
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
