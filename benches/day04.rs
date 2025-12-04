use criterion::{Criterion, criterion_group, criterion_main};

fn day04_benchmark(c: &mut Criterion) {
    let input = include_str!("../input/day04/input.txt");
    c.bench_function("day04part1", |b| {
        b.iter(|| adventofcode2025::day04::solve_part1(input))
    });
    c.bench_function("day04part2", |b| {
        b.iter(|| adventofcode2025::day04::solve_part2(input))
    });
}

criterion_group!(benches, day04_benchmark);
criterion_main!(benches);
