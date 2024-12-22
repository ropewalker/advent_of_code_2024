use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day22)]
fn parse_input(input: &str) -> Vec<i64> {
    use aoc_parse::{parser, prelude::*};

    parser!(lines(i64)).parse(input).unwrap()
}

fn mix(secret_number: &i64, value: i64) -> i64 {
    secret_number ^ value
}

fn prune(secret_number: &i64) -> i64 {
    secret_number & (16_777_216 - 1)
}

fn process(secret_number: &i64) -> i64 {
    let mut secret_number = prune(&mix(secret_number, secret_number << 6));
    secret_number = prune(&mix(&secret_number, secret_number >> 5));
    secret_number = prune(&mix(&secret_number, secret_number << 11));

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
    let mut total_price_per_sequence = vec![0i64; 2usize.pow(20)];

    for secret_number in secret_numbers {
        let mut secret_number = *secret_number;
        let mut price = secret_number % 10;

        let mut sequence = 0;

        let mut vec_encountered_sequences = vec![false; 2usize.pow(20)];

        for i in 0..2_000 {
            let new_secret_number = process(&secret_number);
            let new_price = new_secret_number % 10;

            sequence *= 32;
            sequence += new_price - price + 9;

            if i > 3 {
                sequence &= (2usize.pow(20) - 1) as i64;
            }

            secret_number = new_secret_number;
            price = new_price;

            if i >= 3 && !vec_encountered_sequences[sequence as usize] {
                total_price_per_sequence[sequence as usize] += price;
                vec_encountered_sequences[sequence as usize] = true;
            }
        }
    }

    *total_price_per_sequence.iter().max().unwrap()
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
