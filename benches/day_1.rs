use aoc_2025::{day_1::solve, utilities::read_lines};
use criterion::{Criterion, criterion_group, criterion_main};
use std::time::Duration;

fn bench_day_1(c: &mut Criterion) {
    let mut group = c.benchmark_group("day_1");

    let input = read_lines("inputs/day_1.txt");

    group.bench_with_input("Day 1 part 1 and 2", &input, |b, lines| {
        b.iter(|| {
            let _ = solve(lines);
        });
    });
    group.finish()
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(100).measurement_time(Duration::from_secs(5));
    targets = bench_day_1
}
criterion_main!(benches);
