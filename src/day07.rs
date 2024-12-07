use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day7)]
fn parse_input(input: &str) -> Vec<(u64, Vec<u64>)> {
    use aoc_parse::{parser, prelude::*};

    let operands = parser!(repeat_sep(u64, " "));
    let calibration_equation = parser!(test_value:u64 ": " operands);

    let parser = parser!(lines(calibration_equation));

    parser.parse(input).unwrap()
}

fn can_be_true(test_value: &u64, operands: &[u64]) -> bool {
    if operands.is_empty() {
        return false;
    }

    evaluate(test_value, &operands[0], &operands[1..], false)
}

fn can_be_true_extended(test_value: &u64, operands: &[u64]) -> bool {
    if operands.is_empty() {
        return false;
    }

    evaluate(test_value, &operands[0], &operands[1..], true)
}

fn evaluate(test_value: &u64, result: &u64, operands: &[u64], extended: bool) -> bool {
    if operands.is_empty() {
        return *result == *test_value;
    }

    if result > test_value {
        return false;
    }

    evaluate(
        test_value,
        &(*result + operands[0]),
        &operands[1..],
        extended,
    ) || evaluate(
        test_value,
        &(*result * operands[0]),
        &operands[1..],
        extended,
    ) || (extended
        && evaluate(
            test_value,
            &((*result) * (10u64.pow(operands[0].ilog10() + 1)) + operands[0]),
            &operands[1..],
            extended,
        ))
}

#[aoc(day7, part1)]
fn part1(calibration_equations: &[(u64, Vec<u64>)]) -> u64 {
    calibration_equations
        .iter()
        .filter_map(|(test_value, operands)| {
            if can_be_true(test_value, operands) {
                Some(test_value)
            } else {
                None
            }
        })
        .sum()
}

#[aoc(day7, part2)]
fn part2(calibration_equations: &[(u64, Vec<u64>)]) -> u64 {
    calibration_equations
        .iter()
        .filter_map(|(test_value, operands)| {
            if can_be_true_extended(test_value, operands) {
                Some(test_value)
            } else {
                None
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 3749);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(TEST_INPUT)), 11387);
    }
}
