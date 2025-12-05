use criterion::{Criterion, criterion_group, criterion_main};

fn day01_benchmark(c: &mut Criterion) {
    let input = include_str!("../input/day01/input.txt");
    c.bench_function("day01part1", |b| {
        b.iter(|| adventofcode2025::day01::solve_part1(input))
    });
    c.bench_function("day01part2", |b| {
        b.iter(|| adventofcode2025::day01::solve_part2(input))
    });
}

criterion_group!(benches, day01_benchmark);
criterion_main!(benches);
