use aoc_runner_derive::{aoc, aoc_generator};

const PRUNE_VALUE: u64 = 16_777_216;

#[aoc_generator(day22)]
fn parse_input(input: &str) -> Vec<u64> {
    use aoc_parse::{parser, prelude::*};

    parser!(lines(u64)).parse(input).unwrap()
}

fn mix(secret_number: &u64, value: u64) -> u64 {
    secret_number ^ value
}

fn prune(secret_number: &u64) -> u64 {
    secret_number % PRUNE_VALUE
}

fn process(secret_number: &u64) -> u64 {
    let mut secret_number = prune(&mix(secret_number, secret_number * 64));
    secret_number = prune(&mix(&secret_number, secret_number / 32));
    secret_number = prune(&mix(&secret_number, secret_number * 2_048));

    secret_number
}

fn nth_new_secret_number(secret_number: &u64, steps: usize) -> u64 {
    (0..steps).fold(*secret_number, |secret_number, _| process(&secret_number))
}

#[aoc(day22, part1)]
fn part1(secret_numbers: &[u64]) -> u64 {
    secret_numbers
        .iter()
        .map(|secret_number| nth_new_secret_number(secret_number, 2000))
        .sum()
}

#[aoc(day22, part2)]
fn part2(secret_numbers: &[u64]) -> u64 {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"1
10
100
2024";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 37_327_623);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(TEST_INPUT)), 23);
    }
}
