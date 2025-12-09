use criterion::{Criterion, criterion_group, criterion_main};

fn day08_benchmark(c: &mut Criterion) {
    let input = include_str!("../input/day08/input.txt");
    c.bench_function("day08part1", |b| {
        b.iter(|| adventofcode2025::day08::solve_part1(input))
    });
    c.bench_function("day08part2", |b| {
        b.iter(|| adventofcode2025::day08::solve_part2(input))
    });
}

criterion_group!(benches, day08_benchmark);
criterion_main!(benches);
