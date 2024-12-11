use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day11)]
fn parse_input(input: &str) -> Vec<u64> {
    use aoc_parse::{parser, prelude::*};

    let parser = parser!(repeat_sep(u64, " "));
    parser.parse(input).unwrap()
}

fn blink(stones: &[u64]) -> Vec<u64> {
    stones
        .iter()
        .flat_map(|stone| {
            let num_digits = stone.checked_ilog10().unwrap_or(0) + 1;

            match (stone, num_digits) {
                (0, _) => vec![1],
                (stone, num_digits) if num_digits % 2 == 0 => {
                    let split = 10u64.pow(num_digits / 2);

                    vec![stone / split, stone % split]
                }
                _ => vec![stone * 2024],
            }
        })
        .collect::<Vec<_>>()
}

#[aoc(day11, part1)]
fn part1(stones: &[u64]) -> usize {
    let mut result = stones.to_vec();

    for _ in 1..=25 {
        result = blink(&result);
    }

    result.len()
}

#[aoc(day11, part2)]
fn part2(stones: &[u64]) -> usize {
    let mut result = stones.to_vec();

    for _ in 1..=75 {
        result = blink(&result);
    }

    result.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "125 17";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 55_312);
    }
}
