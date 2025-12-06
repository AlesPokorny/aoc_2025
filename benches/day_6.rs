use aoc_2025::{
    day_6::{part_1, part_2},
    utilities::read_lines,
};
use criterion::{Criterion, criterion_group, criterion_main};
use std::time::Duration;

fn bench_day_2(c: &mut Criterion) {
    let mut group = c.benchmark_group("day_6");

    let input = read_lines("inputs/day_6.txt");

    group.bench_with_input("Day 6 part 1", &input, |b, lines| {
        b.iter(|| {
            let _ = part_1(lines);
        });
    });
    group.bench_with_input("Day 6 part 2", &input, |b, lines| {
        b.iter(|| {
            let _ = part_2(lines);
        });
    });
    group.finish()
}

criterion_group! {
    name = benches;
    config = Criterion::default().measurement_time(Duration::from_secs(10));
    targets = bench_day_2
}
criterion_main!(benches);
