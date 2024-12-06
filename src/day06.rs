use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;

type Position = (i32, i32);

struct Map {
    obstacles: HashSet<Position>,
    bottom_right: Position,
}

#[aoc_generator(day6)]
fn parse_input(input: &str) -> (Map, Position) {
    let mut obstacles = HashSet::new();
    let mut guard_position = (0, 0);

    let bottom_right = (
        input.lines().next().unwrap().len() as i32,
        input.lines().count() as i32,
    );

    input.lines().enumerate().for_each(|(y, line)| {
        line.chars()
            .enumerate()
            .for_each(|(x, character)| match character {
                '#' => {
                    obstacles.insert((x as i32, y as i32));
                }
                '^' => guard_position = (x as i32, y as i32),
                _ => {}
            })
    });

    (
        Map {
            obstacles,
            bottom_right,
        },
        guard_position,
    )
}

#[aoc(day6, part1)]
fn part1((map, guard_position): &(Map, Position)) -> usize {
    let mut current_position = *guard_position;
    let mut visited = HashSet::from([current_position]);

    let mut direction = (0, -1);
    let mut next_position = (
        current_position.0 + direction.0,
        current_position.1 + direction.1,
    );

    while next_position.0 >= 0
        && next_position.0 < map.bottom_right.0
        && next_position.1 >= 0
        && next_position.1 < map.bottom_right.1
    {
        if map.obstacles.contains(&next_position) {
            direction = match direction {
                (0, -1) => (1, 0),
                (1, 0) => (0, 1),
                (0, 1) => (-1, 0),
                (-1, 0) => (0, -1),
                _ => unreachable!(),
            };

            next_position = (
                current_position.0 + direction.0,
                current_position.1 + direction.1,
            );
        } else {
            current_position = next_position;
            visited.insert(current_position);

            next_position = (
                current_position.0 + direction.0,
                current_position.1 + direction.1,
            );
        }
    }

    visited.len()
}

#[aoc(day6, part2)]
fn part2((map, guard_position): &(Map, Position)) -> usize {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 41);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(TEST_INPUT)), 6);
    }
}
