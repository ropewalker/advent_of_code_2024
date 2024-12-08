use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;

type Position = (i32, i32);
type Direction = (i32, i32);

struct Map {
    obstacles: HashSet<Position>,
    bottom_right: Position,
}

#[aoc_generator(day6)]
fn parse_input(input: &str) -> (Map, Position) {
    let mut obstacles = HashSet::new();
    let mut guard_position = (0, 0);

    let bottom_right = (
        input.lines().next().unwrap().len() as i32 - 1,
        input.lines().count() as i32 - 1,
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

fn turn_right(direction: Direction) -> Direction {
    match direction {
        (0, -1) => (1, 0),
        (1, 0) => (0, 1),
        (0, 1) => (-1, 0),
        (-1, 0) => (0, -1),
        _ => unreachable!(),
    }
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
        && next_position.0 <= map.bottom_right.0
        && next_position.1 >= 0
        && next_position.1 <= map.bottom_right.1
    {
        if map.obstacles.contains(&next_position) {
            direction = turn_right(direction);

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
    let mut current_position = *guard_position;

    let mut direction = (0, -1);
    let mut next_position = (
        current_position.0 + direction.0,
        current_position.1 + direction.1,
    );

    let mut visited = HashSet::from([(current_position, direction)]);
    let mut clear_tiles = HashSet::from([current_position]);

    let mut extra_obstacle: Option<Position> = None;
    let mut extra_obstacles: HashSet<Position> = HashSet::new();

    while next_position.0 >= 0
        && next_position.0 <= map.bottom_right.0
        && next_position.1 >= 0
        && next_position.1 <= map.bottom_right.1
    {
        if map.obstacles.contains(&next_position) {
            direction = turn_right(direction);

            next_position = (
                current_position.0 + direction.0,
                current_position.1 + direction.1,
            );
        } else {
            if extra_obstacle.is_none() && !clear_tiles.contains(&next_position) {
                extra_obstacle = Some(next_position);

                let mut new_current_position = current_position;
                let mut extra_visited = HashSet::new();
                let mut new_direction = turn_right(direction);
                let mut new_next_position = (
                    new_current_position.0 + new_direction.0,
                    new_current_position.1 + new_direction.1,
                );

                while new_next_position.0 >= 0
                    && new_next_position.0 <= map.bottom_right.0
                    && new_next_position.1 >= 0
                    && new_next_position.1 <= map.bottom_right.1
                {
                    if map.obstacles.contains(&new_next_position)
                        || extra_obstacle == Some(new_next_position)
                    {
                        new_direction = turn_right(new_direction);

                        new_next_position = (
                            new_current_position.0 + new_direction.0,
                            new_current_position.1 + new_direction.1,
                        );
                    } else if visited.contains(&(new_next_position, new_direction))
                        || extra_visited.contains(&(new_next_position, new_direction))
                    {
                        extra_obstacles.insert(extra_obstacle.unwrap());
                        break;
                    } else {
                        new_current_position = new_next_position;
                        extra_visited.insert((new_current_position, new_direction));

                        new_next_position = (
                            new_current_position.0 + new_direction.0,
                            new_current_position.1 + new_direction.1,
                        );
                    }
                }

                extra_obstacle = None;
            }

            current_position = next_position;
            visited.insert((current_position, direction));
            clear_tiles.insert(current_position);

            next_position = (
                current_position.0 + direction.0,
                current_position.1 + direction.1,
            );
        }
    }

    extra_obstacles.len()
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
