use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::{HashMap, HashSet, VecDeque};

const PRUNE_VALUE: i64 = 16_777_216;

#[aoc_generator(day22)]
fn parse_input(input: &str) -> Vec<i64> {
    use aoc_parse::{parser, prelude::*};

    parser!(lines(i64)).parse(input).unwrap()
}

fn mix(secret_number: &i64, value: i64) -> i64 {
    secret_number ^ value
}

fn prune(secret_number: &i64) -> i64 {
    secret_number % PRUNE_VALUE
}

fn process(secret_number: &i64) -> i64 {
    let mut secret_number = prune(&mix(secret_number, secret_number * 64));
    secret_number = prune(&mix(&secret_number, secret_number / 32));
    secret_number = prune(&mix(&secret_number, secret_number * 2_048));

    secret_number
}

fn nth_new_secret_number(secret_number: &i64, steps: usize) -> i64 {
    (0..steps).fold(*secret_number, |secret_number, _| process(&secret_number))
}

#[aoc(day22, part1)]
fn part1(secret_numbers: &[i64]) -> i64 {
    secret_numbers
        .iter()
        .map(|secret_number| nth_new_secret_number(secret_number, 2_000))
        .sum()
}

#[aoc(day22, part2)]
fn part2(secret_numbers: &[i64]) -> i64 {
    let mut total_price_per_sequence: HashMap<VecDeque<i64>, i64> = HashMap::new();

    for secret_number in secret_numbers {
        let mut secret_number = *secret_number;
        let mut price = secret_number % 10;

        let mut sequence = VecDeque::with_capacity(4);

        let mut encountered_sequences: HashSet<VecDeque<i64>> = HashSet::new();

        for _ in 0..2_000 {
            let new_secret_number = process(&secret_number);
            let new_price = new_secret_number % 10;

            sequence.push_back(new_price - price);

            if sequence.len() > 4 {
                sequence.pop_front();
            }

            secret_number = new_secret_number;
            price = new_price;

            if sequence.len() == 4 && !encountered_sequences.contains(&sequence) {
                *total_price_per_sequence
                    .entry(sequence.clone())
                    .or_default() += price;
                encountered_sequences.insert(sequence.clone());
            }
        }
    }

    total_price_per_sequence.values().copied().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT_1: &str = r"1
10
100
2024";

    static TEST_INPUT_2: &str = r"1
2
3
2024";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT_1)), 37_327_623);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(TEST_INPUT_2)), 23);
    }
}
