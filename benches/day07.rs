use criterion::{Criterion, criterion_group, criterion_main};

fn day07_benchmark(c: &mut Criterion) {
    let input = include_str!("../input/day07/input.txt");
    c.bench_function("day07part1", |b| {
        b.iter(|| adventofcode2025::day07::solve_part1(input))
    });
    c.bench_function("day07part2", |b| {
        b.iter(|| adventofcode2025::day07::solve_part2(input))
    });
}

criterion_group!(benches, day07_benchmark);
criterion_main!(benches);
