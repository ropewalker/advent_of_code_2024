use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::{HashSet, VecDeque};

#[aoc_generator(day18)]
fn parse_input(input: &str) -> Vec<(i32, i32)> {
    use aoc_parse::{parser, prelude::*};

    let parser = parser!(lines(i32 "," i32));
    parser.parse(input).unwrap()
}

fn shortest_path(
    corrupted_locations: &[(i32, i32)],
    exit: (i32, i32),
    bytes: usize,
) -> Option<usize> {
    let corrupted_locations: HashSet<_> = corrupted_locations.iter().take(bytes).collect();

    let mut queue = VecDeque::from([((0, 0), 0)]);
    let mut visited = HashSet::from([(0, 0)]);

    while let Some((position, distance)) = queue.pop_front() {
        for next_position in [
            (position.0 + 1, position.1),
            (position.0 - 1, position.1),
            (position.0, position.1 + 1),
            (position.0, position.1 - 1),
        ] {
            if next_position.0 >= 0
                && next_position.0 <= exit.0
                && next_position.1 >= 0
                && next_position.1 <= exit.1
            {
                if next_position == exit {
                    return Some(distance + 1);
                }

                if !corrupted_locations.contains(&next_position)
                    && !visited.contains(&next_position)
                {
                    queue.push_back((next_position, distance + 1));
                    visited.insert(next_position);
                }
            }
        }
    }

    None
}

#[aoc(day18, part1)]
fn part1(corrupted_locations: &[(i32, i32)]) -> Option<usize> {
    shortest_path(corrupted_locations, (70, 70), 1024)
}

fn blocking_byte(corrupted_locations: &[(i32, i32)], exit: (i32, i32)) -> Option<(i32, i32)> {
    unimplemented!()
}

#[aoc(day18, part2)]
fn part2(corrupted_locations: &[(i32, i32)]) -> String {
    let (x, y) = blocking_byte(corrupted_locations, (70, 70)).unwrap();
    format!("{},{}", x, y)
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";

    #[test]
    fn part1_example() {
        assert_eq!(
            shortest_path(&parse_input(TEST_INPUT), (6, 6), 12),
            Some(22)
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(TEST_INPUT)), "6,1");
    }
}
