// use aoc_runner_derive::aoc_main;
// aoc_main! { lib = aoc_2024 }

use aoc_2024::day1;

fn main() {
    let input = include_str!("../input/2024/sample/day1.txt");
    let mut out1 = 0;
    let mut out2 = 0;
    for _i in 0..10_000 {
        out1 = day1::part1(input);
        out2 = day1::part2(input);
    }
    println!("{} {}", out1, out2);
}
