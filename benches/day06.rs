use criterion::{Criterion, criterion_group, criterion_main};

fn day06_benchmark(c: &mut Criterion) {
    let input = include_str!("../input/day06/input.txt");
    c.bench_function("day06part1", |b| {
        b.iter(|| adventofcode2025::day06::solve_part1(input))
    });
    c.bench_function("day06part2", |b| {
        b.iter(|| adventofcode2025::day06::solve_part2(input))
    });
}

criterion_group!(benches, day06_benchmark);
criterion_main!(benches);
