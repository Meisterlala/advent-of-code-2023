use criterion::{criterion_group, criterion_main, Criterion};
use std::time::Duration;

use advent_of_code_2023::DAYS;

pub fn seperate(c: &mut Criterion) {
    for d in DAYS {
        let mut group = c.benchmark_group(format!("day_{:02}", d.day));
        if let Some(p1) = d.part1 {
            group.bench_function("Part 1", |b| b.iter(|| p1.solve()));
        }
        if let Some(p2) = d.part2 {
            group.bench_function("Part 2", |b| b.iter(|| p2.solve()));
        }
        group.finish();
    }
}

pub fn combined(c: &mut Criterion) {
    for d in DAYS {
        c.bench_function(&format!("day_{:02}", d.day), |b| {
            b.iter(|| {
                if let Some(p1) = d.part1 {
                    p1.solve();
                }
                if let Some(p2) = d.part2 {
                    p2.solve();
                }
            })
        });
    }
}

criterion_group! {
    name = benches;
    config = Criterion::default().measurement_time(Duration::from_secs(30));
    targets = combined
}

criterion_main!(benches);
