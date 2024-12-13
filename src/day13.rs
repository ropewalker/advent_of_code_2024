use aoc_runner_derive::{aoc, aoc_generator};
use num_rational::*;

const MAX_TIMES_PRESSED: i64 = 100;
const PUSH_A_COST: i64 = 3;
const PUSH_B_COST: i64 = 1;
const EXTRA_DISTANCE: i64 = 10_000_000_000_000;

#[derive(Debug)]
struct MachineSetup {
    button_a: (i64, i64),
    button_b: (i64, i64),
    prize_location: (i64, i64),
}

#[aoc_generator(day13)]
fn parse_input(input: &str) -> Vec<MachineSetup> {
    use aoc_parse::{parser, prelude::*};

    let parser = parser!(sections(
    button_a:line("Button A: X+" i64 ", Y+" i64)
    button_b:line("Button B: X+" b_x:i64 ", Y+" b_y:i64)
    prize_location:line("Prize: X=" prize_x:i64 ", Y=" prize_y:i64)
     => MachineSetup{
         button_a,
         button_b,
         prize_location
     }));

    parser.parse(input).unwrap()
}

fn shortest_path_short(
    MachineSetup {
        button_a,
        button_b,
        prize_location,
    }: &MachineSetup,
    with_limit: bool,
) -> Option<i64> {
    let slope_a = Rational64::new(button_a.1, button_a.0);
    let slope_b = Rational64::new(button_b.1, button_b.0);

    let intersect_x = (Rational64::from_integer(prize_location.1)
        - slope_b * Rational64::from_integer(prize_location.0))
        / (slope_a - slope_b);
    let intersect_y = slope_a * intersect_x;

    if intersect_x.is_integer() && intersect_y.is_integer() {
        let pressed_a = intersect_x / Rational64::from_integer(button_a.0);
        let pressed_b = (Rational64::from_integer(prize_location.0) - intersect_x)
            / Rational64::from_integer(button_b.0);

        if pressed_a.is_integer()
            && pressed_b.is_integer()
            && (!with_limit
                || pressed_a.to_integer() <= MAX_TIMES_PRESSED
                    && pressed_b.to_integer() <= MAX_TIMES_PRESSED)
        {
            Some(pressed_a.to_integer() * PUSH_A_COST + pressed_b.to_integer() * PUSH_B_COST)
        } else {
            None
        }
    } else {
        None
    }
}

#[aoc(day13, part1)]
fn part1(machines: &[MachineSetup]) -> i64 {
    machines
        .iter()
        .filter_map(|machine_setup| shortest_path_short(machine_setup, true))
        .sum()
}

#[aoc(day13, part2)]
fn part2(machines: &[MachineSetup]) -> i64 {
    machines
        .iter()
        .filter_map(
            |MachineSetup {
                 button_a,
                 button_b,
                 prize_location,
             }: &MachineSetup| {
                shortest_path_short(
                    &MachineSetup {
                        button_a: *button_a,
                        button_b: *button_b,
                        prize_location: (
                            prize_location.0 + EXTRA_DISTANCE,
                            prize_location.1 + EXTRA_DISTANCE,
                        ),
                    },
                    false,
                )
            },
        )
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 480);
    }
}
