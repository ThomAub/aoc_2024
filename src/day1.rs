use core::str;

use aoc_runner_derive::aoc;
use atoi::FromRadix10;
use fxhash::FxHashMap;
use memchr::{memchr, memmem};

fn parse_input_memchr(input: &str) -> (Vec<u32>, Vec<u32>) {
    let mut data = input[..].as_bytes();
    let mut a_vec = Vec::with_capacity(10_000);
    let mut b_vec = Vec::with_capacity(10_000);
    let finder = memmem::Finder::new("   ");
    loop {
        let Some(separator) = finder.find(data) else {
            break;
        };
        let end = memchr(b'\n', &data[separator..]).unwrap();
        let a = &data[..separator];
        let b = &data[separator + 3..separator + end];
        let a = u32::from_radix_10(a);
        let b = u32::from_radix_10(b);
        a_vec.push(a.0);
        b_vec.push(b.0);
        data = &data[separator + end + 1..];
    }
    (a_vec, b_vec)
}

fn compute_distance(a_vec: &mut [u32], b_vec: &mut [u32]) -> u32 {
    a_vec.sort_unstable();
    b_vec.sort_unstable();

    a_vec.iter().zip(b_vec.iter()).map(|(a, b)| b.abs_diff(*a)).sum()
}

#[aoc(day1, part1, Chars)]
pub fn part1(input: &str) -> u32 {
    let (mut a_vec, mut b_vec) = parse_input_memchr(input);
    compute_distance(&mut a_vec, &mut b_vec)
}

fn compute_similarity_score(a_vec: &[u32], b_vec: &[u32]) -> u32 {
    let mut freq_map = FxHashMap::with_capacity_and_hasher(10_000, Default::default());
    for &num in b_vec {
        *freq_map.entry(num).or_insert(0) += 1;
    }

    a_vec.iter().map(|&num| num * freq_map.get(&num).unwrap_or(&0)).sum()
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> u32 {
    let (a_vec, b_vec) = parse_input_memchr(input);
    compute_similarity_score(&a_vec, &b_vec)
}

fn _parse_input(input: &str) -> (Vec<u32>, Vec<u32>) {
    input
        .split('\n')
        .filter_map(|s| s.split_once("   "))
        .map(|(a, b)| (a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap()))
        .unzip()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"3   4
4   3
2   5
1   3
3   9
3   3
";

    #[test]
    fn sample1() {
        let output = part1(INPUT);
        let expected = 11;
        assert_eq!(output, expected);
    }

    #[test]
    fn sample2() {
        let output = part2(INPUT);
        let expected = 31;
        assert_eq!(output, expected);
    }

    #[test]
    fn test_parse_input() {
        let input = INPUT;
        let expected_a_vec: Vec<u32> = vec![3, 4, 2, 1, 3, 3];
        let expected_b_vec: Vec<u32> = vec![4, 3, 5, 3, 9, 3];
        assert_eq!(_parse_input(input), (expected_a_vec, expected_b_vec));
    }

    #[test]
    fn test_parse_input_memchr() {
        let input = INPUT;
        let expected_a_vec: Vec<u32> = vec![3, 4, 2, 1, 3, 3];
        let expected_b_vec: Vec<u32> = vec![4, 3, 5, 3, 9, 3];
        assert_eq!(parse_input_memchr(input), (expected_a_vec, expected_b_vec));
    }

    #[test]
    fn test_compute_distance() {
        let mut a_vec: Vec<u32> = vec![3, 4, 2, 1, 3, 3];
        let mut b_vec: Vec<u32> = vec![4, 3, 5, 3, 9, 3];
        assert_eq!(compute_distance(&mut a_vec, &mut b_vec), 11);
    }

    #[test]
    fn test_compute_similarity_score() {
        let a_vec: Vec<u32> = vec![3, 4, 2, 1, 3, 3];
        let b_vec: Vec<u32> = vec![4, 3, 5, 3, 9, 3];
        assert_eq!(compute_similarity_score(&a_vec, &b_vec), 31);
    }

    #[test]
    fn test_compute_similarity_score_duplicates_right_list() {
        let a_vec: Vec<u32> = vec![1, 2, 3];
        let b_vec: Vec<u32> = vec![3, 3, 3];
        assert_eq!(compute_similarity_score(&a_vec, &b_vec), 9);
    }
}
