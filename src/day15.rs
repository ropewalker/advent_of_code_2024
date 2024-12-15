use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;

#[derive(Copy, Clone, Eq, PartialEq)]
enum Tile {
    Wall,
    Box,
    Empty,
    Robot,
}

struct WarehouseSetup {
    warehouse_map: Vec<Vec<Tile>>,
    moves: Vec<(i32, i32)>,
}

#[aoc_generator(day15)]
fn parse_input(input: &str) -> WarehouseSetup {
    use aoc_parse::{parser, prelude::*};
    use Tile::*;

    let parser = parser!(
        section(lines({
            '#' => Wall,
            'O' => Box,
            '@' => Robot,
            '.' => Empty
        }+))
        section(moves:lines({
                '<' => (-1, 0),
                '>' => (1, 0),
                '^' => (0 ,-1),
                'v' => (0, 1)
            }+) => moves.into_iter().flatten().collect::<Vec<_>>())
    );

    let (warehouse_map, moves) = parser.parse(input).unwrap();

    WarehouseSetup {
        warehouse_map,
        moves,
    }
}

#[aoc(day15, part1)]
fn part1(warehouse_setup: &WarehouseSetup) -> usize {
    use Tile::*;
    let mut warehouse_map = warehouse_setup.warehouse_map.clone();

    let mut robot_position = (0, 0);

    for (y, _) in warehouse_map.iter().enumerate() {
        for (x, tile) in warehouse_map[y].iter().enumerate() {
            if *tile == Robot {
                robot_position = (x as i32, y as i32);
                break;
            }
        }
    }

    let map_height = warehouse_map.len() as i32;
    let map_width = warehouse_map[0].len() as i32;

    for robot_move in warehouse_setup.moves.iter() {
        let mut to_move = HashSet::new();

        let (start, end, is_x) = match robot_move {
            (0, -1) => (robot_position.1 - 1, 0, false),
            (0, 1) => (robot_position.1 + 1, map_height, false),
            (-1, 0) => (robot_position.0 - 1, 0, true),
            (1, 0) => (robot_position.0 + 1, map_width, true),
            _ => unreachable!(),
        };

        let range = if start < end {
            (start..=end).collect::<Vec<_>>()
        } else {
            (end..=start).rev().collect::<Vec<_>>()
        };

        for x_or_y in range {
            let (x, y) = if is_x {
                (x_or_y, robot_position.1)
            } else {
                (robot_position.0, x_or_y)
            };

            if warehouse_map[y as usize][x as usize] == Box {
                to_move.insert((x, y));
            } else if warehouse_map[y as usize][x as usize] == Wall {
                to_move.clear();
                break;
            } else {
                let (new_x, new_y) = (
                    robot_position.0 + robot_move.0,
                    robot_position.1 + robot_move.1,
                );

                warehouse_map[robot_position.1 as usize][robot_position.0 as usize] = Empty;
                warehouse_map[new_y as usize][new_x as usize] = Robot;

                robot_position = (new_x, new_y);

                break;
            }
        }

        for (x, y) in to_move.iter() {
            let (new_x, new_y) = (x + robot_move.0, y + robot_move.1);
            warehouse_map[new_y as usize][new_x as usize] = Box;
        }
    }

    warehouse_map
        .iter()
        .enumerate()
        .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, tile)| (x, y, tile)))
        .filter_map(|(x, y, tile)| {
            if *tile == Box {
                Some(x + 100 * y)
            } else {
                None
            }
        })
        .sum()
}

#[aoc(day15, part2)]
fn part2(warehouse_setup: &WarehouseSetup) -> usize {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT_LARGE: &str = r"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

    static TEST_INPUT_SMALL: &str = r"########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";

    #[test]
    fn part1_example_large() {
        assert_eq!(part1(&parse_input(TEST_INPUT_LARGE)), 10_092);
    }

    #[test]
    fn part1_example_small() {
        assert_eq!(part1(&parse_input(TEST_INPUT_SMALL)), 2_028);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(TEST_INPUT_LARGE)), 9_021);
    }
}
