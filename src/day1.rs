use std::collections::HashMap;

use aoc_runner_derive::aoc;

fn parse_input(input: &str) -> (Vec<u32>, Vec<u32>) {
    input
        .split('\n')
        .filter_map(|s| s.split_once("   "))
        .map(|(a, b)| (a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap()))
        .unzip()
}

fn compute_distance(a_vec: &[u32], b_vec: &[u32]) -> u32 {
    let mut a_sorted = a_vec.to_vec();
    let mut b_sorted = b_vec.to_vec();
    a_sorted.sort_unstable();
    b_sorted.sort_unstable();

    a_sorted.iter().zip(b_sorted.iter()).map(|(a, b)| b.abs_diff(*a)).sum()
}

#[aoc(day1, part1, Chars)]
pub fn part1(input: &str) -> u32 {
    let (a_vec, b_vec) = parse_input(input);
    compute_distance(&a_vec, &b_vec)
}

fn compute_similarity_score(a_vec: &[u32], b_vec: &[u32]) -> u32 {
    let mut freq_map = HashMap::new();
    for &num in b_vec {
        *freq_map.entry(num).or_insert(0) += 1;
    }

    a_vec.iter().map(|&num| num * freq_map.get(&num).unwrap_or(&0)).sum()
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> u32 {
    let (a_vec, b_vec) = parse_input(input);
    compute_similarity_score(&a_vec, &b_vec)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn sample1() {
        let output = part1(INPUT);
        let expected = 11;
        assert_eq!(output, expected);
    }

    #[test]
    fn test_parse_input() {
        let input = INPUT;
        let expected_a_vec: Vec<u32> = vec![3, 4, 2, 1, 3, 3];
        let expected_b_vec: Vec<u32> = vec![4, 3, 5, 3, 9, 3];
        assert_eq!(parse_input(input), (expected_a_vec, expected_b_vec));
    }

    #[test]
    fn test_compute_distance() {
        let a_vec: Vec<u32> = vec![3, 4, 2, 1, 3, 3];
        let b_vec: Vec<u32> = vec![4, 3, 5, 3, 9, 3];
        assert_eq!(compute_distance(&a_vec, &b_vec), 11);
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
