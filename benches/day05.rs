use criterion::{Criterion, criterion_group, criterion_main};

fn day05_benchmark(c: &mut Criterion) {
    let input = include_str!("../input/day05/input.txt");
    c.bench_function("day05part1", |b| {
        b.iter(|| adventofcode2025::day05::solve_part1(input))
    });
    c.bench_function("day05part2", |b| {
        b.iter(|| adventofcode2025::day05::solve_part2(input))
    });
}

criterion_group!(benches, day05_benchmark);
criterion_main!(benches);
