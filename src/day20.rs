use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::{HashMap, HashSet};

type Position = (i32, i32);

const DEPRECATED_CHEAT_DURATION: i32 = 2;
const LATEST_CHEAT_DURATION: i32 = 20;

#[derive(Debug)]
struct RaceSetup {
    obstacles: HashSet<Position>,
    start: Position,
    end: Position,
}

#[aoc_generator(day20)]
fn parse_input(input: &str) -> RaceSetup {
    let mut start = (0, 0);
    let mut end = (0, 0);

    let mut obstacles: HashSet<(i32, i32)> = HashSet::new();

    input.lines().enumerate().for_each(|(y, row)| {
        row.chars().enumerate().for_each(|(x, tile)| match tile {
            '#' => {
                obstacles.insert((x as i32, y as i32));
            }
            'S' => {
                start = (x as i32, y as i32);
            }
            'E' => {
                end = (x as i32, y as i32);
            }
            _ => (),
        })
    });

    RaceSetup {
        obstacles,
        start,
        end,
    }
}

fn count_deprecated_cheats(race_setup: &RaceSetup, saved: i32) -> usize {
    let mut path = HashMap::from([(race_setup.start, 0)]);
    let mut position = race_setup.start;

    while position != race_setup.end {
        for next_position in [
            (position.0 - 1, position.1),
            (position.0 + 1, position.1),
            (position.0, position.1 - 1),
            (position.0, position.1 + 1),
        ] {
            if !path.contains_key(&next_position) && !race_setup.obstacles.contains(&next_position)
            {
                path.insert(next_position, *path.get(&position).unwrap() + 1);
                position = next_position;
            }
        }
    }

    let mut count = 0;

    for (position, cost) in path.iter() {
        for next_position in [
            (position.0 - 2, position.1),
            (position.0 + 2, position.1),
            (position.0, position.1 - 2),
            (position.0, position.1 + 2),
        ] {
            if let Some(next_cost) = path.get(&next_position) {
                if next_cost - cost >= saved + DEPRECATED_CHEAT_DURATION {
                    count += 1;
                }
            }
        }
    }

    count
}

#[aoc(day20, part1)]
fn part1(race_setup: &RaceSetup) -> usize {
    count_deprecated_cheats(race_setup, 100)
}

fn count_latest_cheats(race_setup: &RaceSetup, saved: i32) -> usize {
    let mut cost = 0;
    let mut path = vec![(race_setup.start, cost)];
    let mut visited = HashSet::from([race_setup.start]);
    let mut position = race_setup.start;

    while position != race_setup.end {
        for next_position in [
            (position.0 - 1, position.1),
            (position.0 + 1, position.1),
            (position.0, position.1 - 1),
            (position.0, position.1 + 1),
        ] {
            if !visited.contains(&next_position) && !race_setup.obstacles.contains(&next_position) {
                visited.insert(next_position);
                cost += 1;
                path.push((next_position, cost));
                position = next_position;
            }
        }
    }

    let mut count = 0;

    for i in 0..path.len() {
        for j in i + 1..path.len() {
            let (cheat_start, start_cost) = path[i];
            let (cheat_end, end_cost) = path[j];

            if (cheat_end.0 - cheat_start.0).abs() + (cheat_end.1 - cheat_start.1).abs()
                <= LATEST_CHEAT_DURATION
                && end_cost - start_cost
                    >= saved
                        + (cheat_end.0 - cheat_start.0).abs()
                        + (cheat_end.1 - cheat_start.1).abs()
            {
                count += 1;
            }
        }
    }

    count
}

#[aoc(day20, part2)]
fn part2(race_setup: &RaceSetup) -> usize {
    count_latest_cheats(race_setup, 100)
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";

    #[test]
    fn part1_example1() {
        assert_eq!(
            count_deprecated_cheats(&parse_input(TEST_INPUT), 2),
            14 + 14 + 2 + 4 + 2 + 3 + 1 + 1 + 1 + 1 + 1
        );
    }

    #[test]
    fn part1_example2() {
        assert_eq!(
            count_deprecated_cheats(&parse_input(TEST_INPUT), 4),
            14 + 2 + 4 + 2 + 3 + 1 + 1 + 1 + 1 + 1
        );
    }

    #[test]
    fn part1_example3() {
        assert_eq!(
            count_deprecated_cheats(&parse_input(TEST_INPUT), 6),
            2 + 4 + 2 + 3 + 1 + 1 + 1 + 1 + 1
        );
    }

    #[test]
    fn part1_example4() {
        assert_eq!(
            count_deprecated_cheats(&parse_input(TEST_INPUT), 8),
            4 + 2 + 3 + 1 + 1 + 1 + 1 + 1
        );
    }

    #[test]
    fn part1_example5() {
        assert_eq!(
            count_deprecated_cheats(&parse_input(TEST_INPUT), 10),
            2 + 3 + 1 + 1 + 1 + 1 + 1
        );
    }

    #[test]
    fn part1_example6() {
        assert_eq!(
            count_deprecated_cheats(&parse_input(TEST_INPUT), 12),
            3 + 1 + 1 + 1 + 1 + 1
        );
    }

    #[test]
    fn part1_example7() {
        assert_eq!(
            count_deprecated_cheats(&parse_input(TEST_INPUT), 20),
            1 + 1 + 1 + 1 + 1
        );
    }

    #[test]
    fn part1_example8() {
        assert_eq!(
            count_deprecated_cheats(&parse_input(TEST_INPUT), 36),
            1 + 1 + 1 + 1
        );
    }

    #[test]
    fn part1_example9() {
        assert_eq!(
            count_deprecated_cheats(&parse_input(TEST_INPUT), 38),
            1 + 1 + 1
        );
    }

    #[test]
    fn part1_example10() {
        assert_eq!(count_deprecated_cheats(&parse_input(TEST_INPUT), 40), 1 + 1);
    }

    #[test]
    fn part1_example11() {
        assert_eq!(count_deprecated_cheats(&parse_input(TEST_INPUT), 64), 1);
    }

    #[test]
    fn part2_example1() {
        assert_eq!(
            count_latest_cheats(&parse_input(TEST_INPUT), 50),
            32 + 31 + 29 + 39 + 25 + 23 + 20 + 19 + 12 + 14 + 12 + 22 + 4 + 3
        );
    }

    #[test]
    fn part2_example2() {
        assert_eq!(
            count_latest_cheats(&parse_input(TEST_INPUT), 52),
            31 + 29 + 39 + 25 + 23 + 20 + 19 + 12 + 14 + 12 + 22 + 4 + 3
        );
    }

    #[test]
    fn part2_example3() {
        assert_eq!(
            count_latest_cheats(&parse_input(TEST_INPUT), 54),
            29 + 39 + 25 + 23 + 20 + 19 + 12 + 14 + 12 + 22 + 4 + 3
        );
    }

    #[test]
    fn part2_example4() {
        assert_eq!(
            count_latest_cheats(&parse_input(TEST_INPUT), 56),
            39 + 25 + 23 + 20 + 19 + 12 + 14 + 12 + 22 + 4 + 3
        );
    }

    #[test]
    fn part2_example5() {
        assert_eq!(
            count_latest_cheats(&parse_input(TEST_INPUT), 58),
            25 + 23 + 20 + 19 + 12 + 14 + 12 + 22 + 4 + 3
        );
    }

    #[test]
    fn part2_example6() {
        assert_eq!(
            count_latest_cheats(&parse_input(TEST_INPUT), 60),
            23 + 20 + 19 + 12 + 14 + 12 + 22 + 4 + 3
        );
    }

    #[test]
    fn part2_example7() {
        assert_eq!(
            count_latest_cheats(&parse_input(TEST_INPUT), 62),
            20 + 19 + 12 + 14 + 12 + 22 + 4 + 3
        );
    }

    #[test]
    fn part2_example8() {
        assert_eq!(
            count_latest_cheats(&parse_input(TEST_INPUT), 64),
            19 + 12 + 14 + 12 + 22 + 4 + 3
        );
    }

    #[test]
    fn part2_example9() {
        assert_eq!(
            count_latest_cheats(&parse_input(TEST_INPUT), 66),
            12 + 14 + 12 + 22 + 4 + 3
        );
    }

    #[test]
    fn part2_example10() {
        assert_eq!(
            count_latest_cheats(&parse_input(TEST_INPUT), 68),
            14 + 12 + 22 + 4 + 3
        );
    }

    #[test]
    fn part2_example11() {
        assert_eq!(
            count_latest_cheats(&parse_input(TEST_INPUT), 70),
            12 + 22 + 4 + 3
        );
    }

    #[test]
    fn part2_example12() {
        assert_eq!(
            count_latest_cheats(&parse_input(TEST_INPUT), 72),
            22 + 4 + 3
        );
    }

    #[test]
    fn part2_example13() {
        assert_eq!(count_latest_cheats(&parse_input(TEST_INPUT), 74), 4 + 3);
    }

    #[test]
    fn part2_example14() {
        assert_eq!(count_latest_cheats(&parse_input(TEST_INPUT), 76), 3);
    }
}
