use crate::day21::Button::*;
use crate::day21::Direction::*;
use aoc_runner_derive::{aoc, aoc_generator};
use lazy_static::lazy_static;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Eq, PartialEq, Copy, Clone, Debug, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug, Hash)]
enum Button {
    Number(u8),
    Move(Direction),
    Activate,
}

type Position = (i8, i8);

lazy_static! {
    static ref NUMERIC_KEYPAD_LAYOUT: HashMap<Position, Button> = HashMap::from([
        ((0, 0), Number(7)),
        ((1, 0), Number(8)),
        ((2, 0), Number(9)),
        ((0, 1), Number(4)),
        ((1, 1), Number(5)),
        ((2, 1), Number(6)),
        ((0, 2), Number(1)),
        ((1, 2), Number(2)),
        ((2, 2), Number(3)),
        ((1, 3), Number(0)),
        ((2, 3), Activate),
    ]);
    static ref DIRECTIONAL_KEYPAD_LAYOUT: HashMap<Position, Button> = HashMap::from([
        ((1, 0), Move(Up)),
        ((2, 0), Activate),
        ((0, 1), Move(Left)),
        ((1, 1), Move(Down)),
        ((2, 1), Move(Right)),
    ]);
}

#[aoc_generator(day21)]
fn parse_input(input: &str) -> Vec<Vec<Button>> {
    use aoc_parse::{parser, prelude::*};
    use Button::*;

    let parser = parser!(lines({
        number:digit => Number(number as u8), 
        'A' => Activate
    }+));

    parser.parse(input).unwrap()
}

fn move_in_direction(position: &Position, direction: &Direction) -> Position {
    match direction {
        Up => (position.0, position.1 - 1),
        Down => (position.0, position.1 + 1),
        Left => (position.0 - 1, position.1),
        Right => (position.0 + 1, position.1),
    }
}

fn complexity(code: &[Button], num_robots: usize) -> usize {
    numerical_value(code) * shortest_sequence_len(code, num_robots)
}

#[derive(Eq, PartialEq, Clone, Hash, Debug)]
struct State {
    robot_positions: Vec<Position>,
    output: Vec<Button>,
}

fn shortest_sequence_len(code: &[Button], num_robots: usize) -> usize {
    let mut robot_positions = vec![(2, 0); num_robots - 1];
    robot_positions.push((2, 3));

    let initial_state = State {
        robot_positions,
        output: vec![],
    };

    let mut visited: HashSet<State> = HashSet::from([initial_state.clone()]);
    let mut queue = VecDeque::from([(initial_state, 0)]);

    while let Some((state, buttons_pressed)) = queue.pop_front() {
        if state.output == code {
            return buttons_pressed;
        }

        for button in [Move(Up), Move(Down), Move(Left), Move(Right), Activate] {
            let mut level = 0;
            let mut button = button;

            loop {
                match button {
                    Move(direction) => {
                        let mut new_robot_positions = state.robot_positions.clone();
                        new_robot_positions[level] =
                            move_in_direction(&state.robot_positions[level], &direction);

                        if (level == num_robots - 1
                            && NUMERIC_KEYPAD_LAYOUT.contains_key(&new_robot_positions[level]))
                            || (level < num_robots - 1
                                && DIRECTIONAL_KEYPAD_LAYOUT
                                    .contains_key(&new_robot_positions[level]))
                        {
                            let new_state = State {
                                robot_positions: new_robot_positions,
                                output: state.output.clone(),
                            };

                            if !visited.contains(&new_state) {
                                visited.insert(new_state.clone());
                                queue.push_back((new_state, buttons_pressed + 1));
                            }
                        }

                        break;
                    }
                    Activate => {
                        if level == num_robots - 1 {
                            let new_output_button = *NUMERIC_KEYPAD_LAYOUT
                                .get(&state.robot_positions[level])
                                .unwrap();

                            if code[state.output.len()] == new_output_button {
                                let mut new_output = state.output.clone();
                                new_output.push(new_output_button);

                                let new_state = State {
                                    robot_positions: state.robot_positions.clone(),
                                    output: new_output,
                                };

                                if !visited.contains(&new_state) {
                                    visited.insert(new_state.clone());
                                    queue.push_back((new_state, buttons_pressed + 1));
                                }
                            }

                            break;
                        } else {
                            button = *DIRECTIONAL_KEYPAD_LAYOUT
                                .get(&state.robot_positions[level])
                                .unwrap();

                            level += 1;
                        }
                    }
                    _ => unreachable!(),
                }
            }
        }
    }

    unreachable!()
}

fn numerical_value(code: &[Button]) -> usize {
    code.iter().fold(0, |value, button| match button {
        Number(number) => value * 10 + *number as usize,
        _ => value,
    })
}

#[aoc(day21, part1)]
fn part1(codes: &[Vec<Button>]) -> usize {
    codes.iter().map(|code| complexity(code, 3)).sum()
}

#[aoc(day21, part2)]
fn part2(codes: &[Vec<Button>]) -> usize {
    codes.iter().map(|code| complexity(code, 25 + 1)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"029A
980A
179A
456A
379A";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 126_384);
    }
}
