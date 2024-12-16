use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

#[aoc_generator(day11)]
fn parse_input(input: &str) -> Vec<u64> {
    use aoc_parse::{parser, prelude::*};

    let parser = parser!(repeat_sep(u64, " "));
    parser.parse(input).unwrap()
}

fn count_stones(initial_stones: &[u64], blink_count: usize) -> usize {
    let mut result: HashMap<u64, usize> = initial_stones.iter().fold(
        HashMap::with_capacity(initial_stones.len()),
        |mut result, stone| {
            *result.entry(*stone).or_default() += 1;
            result
        },
    );

    for _ in 1..=blink_count {
        result = result
            .iter()
            .fold(HashMap::new(), |mut result, (&stone, &count)| {
                let digit_count = stone.checked_ilog10().unwrap_or(0) + 1;

                match (stone, digit_count) {
                    (stone, num_digits) if num_digits % 2 == 0 => {
                        let split = 10u64.pow(num_digits / 2);

                        *result.entry(stone / split).or_default() += count;
                        *result.entry(stone % split).or_default() += count;
                    }
                    (0, _) => {
                        *result.entry(1).or_default() += count;
                    }
                    _ => {
                        *result.entry(stone * 2_024).or_default() += count;
                    }
                }

                result
            });
    }

    result.values().sum()
}

fn process_stone(
    stone: u64,
    remaining_steps: usize,
    cache: &mut HashMap<(u64, usize), usize>,
) -> usize {
    if let Some(result) = cache.get(&(stone, remaining_steps)) {
        return *result;
    }

    if remaining_steps == 0 {
        return 1;
    }

    let digit_count = stone.checked_ilog10().unwrap_or(0) + 1;

    let result = match (stone, digit_count) {
        (stone, num_digits) if num_digits % 2 == 0 => {
            let split = 10u64.pow(num_digits / 2);
            process_stone(stone / split, remaining_steps - 1, cache)
                + process_stone(stone % split, remaining_steps - 1, cache)
        }
        (0, _) => process_stone(1, remaining_steps - 1, cache),
        _ => process_stone(stone * 2024, remaining_steps - 1, cache),
    };

    cache.insert((stone, remaining_steps), result);

    result
}

#[aoc(day11, part1)]
fn part1(stones: &[u64]) -> usize {
    count_stones(stones, 25)
}

#[aoc(day11, part2)]
fn part2(stones: &[u64]) -> usize {
    count_stones(stones, 75)
}

#[aoc(day11, part1, cache)]
fn part1_with_cache(stones: &[u64]) -> usize {
    let mut cache = HashMap::new();

    stones
        .iter()
        .map(|stone| process_stone(*stone, 25, &mut cache))
        .sum()
}

#[aoc(day11, part2, cache)]
fn part2_with_cache(stones: &[u64]) -> usize {
    let mut cache = HashMap::new();

    stones
        .iter()
        .map(|stone| process_stone(*stone, 75, &mut cache))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "125 17";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 55_312);
    }

    #[test]
    fn part1_example_with_cache() {
        assert_eq!(part1_with_cache(&parse_input(TEST_INPUT)), 55_312);
    }
}
