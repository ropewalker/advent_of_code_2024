use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::{HashSet, VecDeque};

#[aoc_generator(day10)]
fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|height| height.to_digit(10).unwrap())
                .collect()
        })
        .collect()
}

fn ratings_sum(map: &[Vec<u32>], count_distinct_trails: bool) -> usize {
    let trailheads = map
        .iter()
        .enumerate()
        .flat_map(|(y, parallel)| {
            parallel
                .iter()
                .enumerate()
                .filter_map(move |(x, height)| if *height == 0 { Some((x, y)) } else { None })
        })
        .collect::<Vec<_>>();

    let mut score_sum = 0;
    let mut visited: HashSet<(usize, usize)> = HashSet::with_capacity(0);

    for (trailhead_x, trailhead_y) in trailheads {
        let mut queue = VecDeque::from(vec![(trailhead_x, trailhead_y)]);

        if !count_distinct_trails {
            visited = HashSet::from([(trailhead_x, trailhead_y)]);
        }

        while let Some((x, y)) = queue.pop_front() {
            for (direction_x, direction_y) in [(-1i32, 0i32), (1, 0), (0, -1), (0, 1)] {
                if x as i32 + direction_x >= 0
                    && x as i32 + direction_x < map[0].len() as i32
                    && y as i32 + direction_y >= 0
                    && y as i32 + direction_y < map.len() as i32
                {
                    let (new_x, new_y) = (
                        (x as i32 + direction_x) as usize,
                        (y as i32 + direction_y) as usize,
                    );

                    if (count_distinct_trails || !visited.contains(&(new_x, new_y)))
                        && map[new_y][new_x] as i32 - map[y][x] as i32 == 1
                    {
                        if map[new_y][new_x] == 9 {
                            score_sum += 1;
                        } else {
                            queue.push_back((new_x, new_y));
                        }

                        if !count_distinct_trails {
                            visited.insert((new_x, new_y));
                        }
                    }
                }
            }
        }
    }

    score_sum
}

#[aoc(day10, part1)]
fn part1(map: &[Vec<u32>]) -> usize {
    ratings_sum(map, false)
}

#[aoc(day10, part2)]
fn part2(map: &[Vec<u32>]) -> usize {
    ratings_sum(map, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 36);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(TEST_INPUT)), 81);
    }
}
