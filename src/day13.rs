use aoc_runner_derive::{aoc, aoc_generator};

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

fn solve_machine(
    MachineSetup {
        button_a,
        button_b,
        prize_location,
    }: &MachineSetup,
    with_limit: bool,
) -> Option<i64> {
    let a_nom = prize_location.0 * button_b.1 - prize_location.1 * button_b.0;
    let b_nom = button_a.0 * prize_location.1 - button_a.1 * prize_location.0;
    let denom = button_a.0 * button_b.1 - button_a.1 * button_b.0;

    if a_nom % denom == 0 && b_nom % denom == 0 {
        let a_pressed = a_nom / denom;
        let b_pressed = b_nom / denom;

        if !with_limit || a_pressed <= MAX_TIMES_PRESSED && b_pressed <= MAX_TIMES_PRESSED {
            Some(a_pressed * PUSH_A_COST + b_pressed * PUSH_B_COST)
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
        .filter_map(|machine_setup| solve_machine(machine_setup, true))
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
                solve_machine(
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
