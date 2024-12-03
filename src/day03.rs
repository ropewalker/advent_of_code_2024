use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day3, part1)]
fn parse_input_part1(corrupted_memory: &str) -> Vec<(i32, i32)> {
    use aoc_parse::{parser, prelude::*};

    enum Instruction {
        Mul(i32, i32),
        Garbage,
    }

    use Instruction::*;

    let mul = parser!("mul(" a:i32 "," b:i32 ")" => Mul(a, b));
    let parser = parser!(({mul, any_char => Garbage})*);

    parser
        .parse(corrupted_memory)
        .unwrap()
        .iter()
        .filter_map(|instruction| match instruction {
            Mul(a, b) => Some((*a, *b)),
            Garbage => None,
        })
        .collect()
}

#[aoc(day3, part1)]
fn part1(multipliers: &[(i32, i32)]) -> i32 {
    multipliers.iter().map(|(a, b)| a * b).sum()
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum ExtendedInstruction {
    Mul(i32, i32),
    Enable,
    Disable,
    Garbage,
}

#[aoc_generator(day3, part2)]
fn parse_input_part2(corrupted_memory: &str) -> Vec<ExtendedInstruction> {
    use aoc_parse::{parser, prelude::*};
    use ExtendedInstruction::*;

    let mul = parser!("mul(" a:i32 "," b:i32 ")" => Mul(a, b));
    let enable = parser!("do()" => Enable);
    let disable = parser!("don't()" => Disable);
    let parser = parser!(({mul, enable, disable, any_char => Garbage})*);

    parser
        .parse(corrupted_memory)
        .unwrap()
        .iter()
        .filter_map(|instruction| {
            if *instruction != Garbage {
                Some(*instruction)
            } else {
                None
            }
        })
        .collect()
}

#[aoc(day3, part2)]
fn part2(instructions: &[ExtendedInstruction]) -> i32 {
    use ExtendedInstruction::*;

    instructions
        .iter()
        .fold((0, true), |(mut sum, mut enabled), instruction| {
            match instruction {
                Mul(a, b) => {
                    if enabled {
                        sum += a * b
                    }
                }
                Enable => enabled = true,
                Disable => enabled = false,
                Garbage => {}
            };
            (sum, enabled)
        })
        .0
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT_1: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    static TEST_INPUT_2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input_part1(TEST_INPUT_1)), 161);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input_part2(TEST_INPUT_2)), 48);
    }
}
