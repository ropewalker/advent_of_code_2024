use crate::day21::Button::*;
use crate::day21::Direction::*;
use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::{HashMap, VecDeque};
use std::hash::Hash;

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

fn distance(position_1: &Position, position_2: &Position) -> i8 {
    (position_1.0 - position_2.0).abs() + (position_1.1 - position_2.1).abs()
}

fn shortest_paths(
    start_button: &Button,
    end_button: &Button,
    keypad: &[(Button, Position)],
) -> Vec<Vec<Button>> {
    let start_position = keypad
        .iter()
        .find(|(button, _)| button == start_button)
        .unwrap()
        .1;
    let end_position = keypad
        .iter()
        .find(|(button, _)| button == end_button)
        .unwrap()
        .1;

    let mut distances: HashMap<Position, usize> = HashMap::from([(start_position, 0)]);
    let mut queue: VecDeque<(Position, usize)> = VecDeque::from([(start_position, 0)]);

    while let Some((position, distance)) = queue.pop_front() {
        if position == end_position {
            break;
        }

        for direction in [Up, Down, Left, Right] {
            let new_position = move_in_direction(&position, &direction);

            if keypad.iter().any(|(_, position)| *position == new_position)
                && !distances.contains_key(&new_position)
            {
                distances.insert(new_position, distance + 1);
                queue.push_back((new_position, distance + 1));
            }
        }
    }

    let mut paths: Vec<Vec<Button>> = Vec::new();
    let mut queue: Vec<(Position, Vec<Button>)> = vec![(start_position, vec![])];

    while let Some((position, path)) = queue.pop() {
        if position == end_position {
            let mut path = path.clone();
            path.push(Activate);
            paths.push(path);
        }

        let distance = *distances.get(&position).unwrap();

        for direction in [Up, Down, Left, Right] {
            let new_position = move_in_direction(&position, &direction);

            if let Some(new_distance) = distances.get(&new_position) {
                if *new_distance == distance + 1 {
                    let mut new_path = path.clone();
                    new_path.push(Move(direction));

                    queue.push((new_position, new_path));
                }
            }
        }
    }

    paths
}

fn init_costs(levels: usize) -> HashMap<(Button, Button), usize> {
    let directional_keypad = vec![
        (Move(Up), (1, 0)),
        (Activate, (2, 0)),
        (Move(Left), (0, 1)),
        (Move(Down), (1, 1)),
        (Move(Right), (2, 1)),
    ];

    let mut costs: HashMap<(Button, Button), usize> = HashMap::new();
    let mut paths: HashMap<(Button, Button), Vec<Vec<Button>>> = HashMap::new();

    for (start_button, start_position) in directional_keypad.iter() {
        for (end_button, end_position) in directional_keypad.iter() {
            costs.insert(
                (*start_button, *end_button),
                distance(start_position, end_position) as usize + 1,
            );

            paths.insert(
                (*start_button, *end_button),
                shortest_paths(start_button, end_button, &directional_keypad),
            );
        }
    }

    for _ in 1..levels {
        let mut new_costs: HashMap<(Button, Button), usize> = HashMap::new();

        for (start_button, _) in directional_keypad.iter() {
            for (end_button, _) in directional_keypad.iter() {
                let paths = paths.get(&(*start_button, *end_button)).unwrap();

                let new_cost = paths
                    .iter()
                    .map(|path| {
                        path.iter()
                            .fold((0, Activate), |(cost, prev_button), button| {
                                (cost + costs.get(&(prev_button, *button)).unwrap(), *button)
                            })
                    })
                    .map(|(cost, _)| cost)
                    .min()
                    .unwrap();

                new_costs.insert((*start_button, *end_button), new_cost);
            }
        }

        costs = new_costs;
    }

    costs
}

fn shortest_sequence_len(code: &[Button], robots_count: usize) -> usize {
    let numeric_keypad = vec![
        (Number(7), (0, 0)),
        (Number(8), (1, 0)),
        (Number(9), (2, 0)),
        (Number(4), (0, 1)),
        (Number(5), (1, 1)),
        (Number(6), (2, 1)),
        (Number(1), (0, 2)),
        (Number(2), (1, 2)),
        (Number(3), (2, 2)),
        (Number(0), (1, 3)),
        (Activate, (2, 3)),
    ];

    let costs = init_costs(robots_count - 1);

    code.iter()
        .fold((0, Activate), |(cost, prev_button), button| {
            let cost = cost
                + shortest_paths(&prev_button, button, &numeric_keypad)
                    .iter()
                    .map(|path| {
                        path.iter()
                            .fold((0, Activate), |(cost, prev_button), button| {
                                (cost + costs.get(&(prev_button, *button)).unwrap(), *button)
                            })
                    })
                    .map(|(cost, _)| cost)
                    .min()
                    .unwrap();

            (cost, *button)
        })
        .0
}

fn numerical_value(code: &[Button]) -> usize {
    code.iter().fold(0, |value, button| match button {
        Number(number) => value * 10 + *number as usize,
        _ => value,
    })
}

fn complexity(code: &[Button], num_robots: usize) -> usize {
    numerical_value(code) * shortest_sequence_len(code, num_robots)
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
