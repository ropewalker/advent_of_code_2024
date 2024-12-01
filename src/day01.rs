use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

#[aoc_generator(day1)]
fn parse_input(lists_side_by_side: &str) -> (Vec<i32>, Vec<i32>) {
    use aoc_parse::{parser, prelude::*};

    let pairs = parser!(lines(i32 "   " i32));
    let lists: Vec<(i32, i32)> = pairs.parse(lists_side_by_side).unwrap();
    lists.into_iter().unzip()
}

#[aoc(day1, part1)]
fn part1(lists: &(Vec<i32>, Vec<i32>)) -> i32 {
    let mut left_list = lists.0.clone();
    let mut right_list = lists.1.clone();

    left_list.sort_unstable();
    right_list.sort_unstable();

    left_list
        .iter()
        .zip(right_list.iter())
        .map(|(a, b)| (a - b).abs())
        .sum()
}

#[aoc(day1, part2)]
fn part2(lists: &(Vec<i32>, Vec<i32>)) -> i32 {
    let left_list = lists.0.clone();
    let right_list = lists.1.clone();

    let mut occurrences: HashMap<i32, i32> = HashMap::with_capacity(right_list.len());

    for id in right_list.iter() {
        *occurrences.entry(*id).or_default() += 1;
    }

    left_list
        .iter()
        .map(|id| *id * occurrences.get(id).unwrap_or(&0))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 11);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(TEST_INPUT)), 31);
    }
}
