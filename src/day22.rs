use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::{HashMap, HashSet};

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
    let mut max_prices: Vec<HashMap<[i64; 4], i64>> = Vec::with_capacity(secret_numbers.len());

    for secret_number in secret_numbers {
        let mut secret_number = *secret_number;
        let mut price = secret_number % 10;

        let mut prices = Vec::with_capacity(2_000);
        let mut price_changes = Vec::with_capacity(2_000);

        let mut max_prices_per_sequence: HashMap<[i64; 4], i64> = HashMap::new();

        for _ in 0..2_000 {
            let new_secret_number = process(&secret_number);
            let new_price = new_secret_number % 10;

            prices.push(new_price);
            price_changes.push(new_price - price);

            secret_number = new_secret_number;
            price = new_price;
        }

        price_changes
            .windows(4)
            .enumerate()
            .for_each(|(index, sequence)| {
                let price = prices[index + 3];

                max_prices_per_sequence
                    .entry(sequence.try_into().unwrap())
                    .or_insert(price);
            });

        max_prices.push(max_prices_per_sequence);
    }

    let sequences = max_prices.iter().fold(
        HashSet::new(),
        |mut sequences: HashSet<[i64; 4]>, sequence_prices_per_secret_number| {
            sequences.extend(sequence_prices_per_secret_number.keys());
            sequences
        },
    );

    sequences
        .iter()
        .map(|sequence| {
            max_prices
                .iter()
                .filter_map(|prices_per_sequence| prices_per_sequence.get(sequence))
                .sum::<i64>()
        })
        .max()
        .unwrap()
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
