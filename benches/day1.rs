use aoc_2024::day1::{part1, part2};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    let input = match std::env::var("AOC_INPUT_DAY01") {
        Ok(val) => val,
        Err(_) => include_str!("../input/2024/sample/day1.txt").to_owned(),
    };
    let input = input.as_str();

    c.bench_function("part1", |b| b.iter(|| part1(black_box(input))));
    c.bench_function("part2", |b| b.iter(|| part2(black_box(input))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
