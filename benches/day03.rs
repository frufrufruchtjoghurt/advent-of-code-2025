use criterion::{Criterion, criterion_group, criterion_main};

fn day03_benchmark(c: &mut Criterion) {
    let input = include_str!("../input/day03/input.txt");
    c.bench_function("day03part1", |b| {
        b.iter(|| adventofcode2025::day03::solve_part1(input))
    });
    c.bench_function("day03part2", |b| {
        b.iter(|| adventofcode2025::day03::solve_part2(input))
    });
}

criterion_group!(benches, day03_benchmark);
criterion_main!(benches);
