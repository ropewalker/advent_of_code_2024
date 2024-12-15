use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;

struct WarehouseSetup {
    walls: HashSet<(i32, i32)>,
    boxes: HashSet<(i32, i32)>,
    robot: (i32, i32),
    moves: Vec<(i32, i32)>,
}

#[aoc_generator(day15)]
fn parse_input(input: &str) -> WarehouseSetup {
    use aoc_parse::{parser, prelude::*};

    let parser = parser!(
        section(lines(any_char+))
        section(moves:lines({
                '<' => (-1, 0),
                '>' => (1, 0),
                '^' => (0 ,-1),
                'v' => (0, 1)
            }+) => moves.into_iter().flatten().collect::<Vec<_>>())
    );

    let (warehouse_map, moves) = parser.parse(input).unwrap();

    let mut walls: HashSet<(i32, i32)> = HashSet::new();
    let mut boxes: HashSet<(i32, i32)> = HashSet::new();
    let mut robot: (i32, i32) = (0, 0);

    warehouse_map
        .iter()
        .enumerate()
        .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, tile)| (x, y, tile)))
        .for_each(|(x, y, tile)| match tile {
            '#' => {
                walls.insert((x as i32, y as i32));
            }
            'O' => {
                boxes.insert((x as i32, y as i32));
            }
            '@' => {
                robot = (x as i32, y as i32);
            }
            _ => (),
        });

    WarehouseSetup {
        walls,
        boxes,
        robot,
        moves,
    }
}

#[aoc(day15, part1)]
fn part1(warehouse_setup: &WarehouseSetup) -> i32 {
    let walls = &warehouse_setup.walls;
    let mut boxes = warehouse_setup.boxes.clone();
    let mut robot = warehouse_setup.robot;

    for robot_move in warehouse_setup.moves.iter() {
        let mut boxes_to_move = HashSet::new();

        let mut position = (robot.0 + robot_move.0, robot.1 + robot_move.1);

        loop {
            if boxes.contains(&position) {
                boxes_to_move.insert(position);
                position.0 += robot_move.0;
                position.1 += robot_move.1;
            } else if walls.contains(&position) {
                boxes_to_move.clear();
                break;
            } else {
                robot.0 += robot_move.0;
                robot.1 += robot_move.1;
                break;
            }
        }

        for moved_box in boxes_to_move.iter() {
            boxes.remove(&moved_box);
        }

        for moved_box in boxes_to_move.iter() {
            boxes.insert((moved_box.0 + robot_move.0, moved_box.1 + robot_move.1));
        }
    }

    boxes.iter().map(|(x, y)| 100 * y + x).sum::<i32>()
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
