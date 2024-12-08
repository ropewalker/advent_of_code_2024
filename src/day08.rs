use aoc_runner_derive::{aoc, aoc_generator};
use num::integer::gcd;
use std::collections::{HashMap, HashSet};

struct Map {
    antennas_by_frequency: HashMap<char, Vec<(i32, i32)>>,
    bottom_right: (i32, i32),
}

#[aoc_generator(day8)]
fn parse_input(map: &str) -> Map {
    let mut antennas_by_frequency: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    let bottom_right = (
        map.lines().next().unwrap().len() as i32 - 1,
        map.lines().count() as i32 - 1,
    );

    map.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, character)| {
            if character != '.' {
                antennas_by_frequency
                    .entry(character)
                    .or_default()
                    .push((x as i32, y as i32));
            }
        })
    });

    Map {
        antennas_by_frequency,
        bottom_right,
    }
}

#[aoc(day8, part1)]
fn part1(map: &Map) -> usize {
    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();

    for antennas in map.antennas_by_frequency.values() {
        for (index, (antenna0_x, antenna0_y)) in antennas.iter().enumerate() {
            for (antenna1_x, antenna1_y) in antennas.iter().skip(index + 1) {
                let (dx, dy) = (antenna1_x - antenna0_x, antenna1_y - antenna0_y);

                let (antinode_x, antinode_y) = (antenna0_x - dx, antenna0_y - dy);

                if antinode_x >= 0
                    && antinode_x <= map.bottom_right.0
                    && antinode_y >= 0
                    && antinode_y <= map.bottom_right.1
                {
                    antinodes.insert((antinode_x, antinode_y));
                }

                let (antinode_x, antinode_y) = (antenna1_x + dx, antenna1_y + dy);

                if antinode_x >= 0
                    && antinode_x <= map.bottom_right.0
                    && antinode_y >= 0
                    && antinode_y <= map.bottom_right.1
                {
                    antinodes.insert((antinode_x, antinode_y));
                }
            }
        }
    }

    antinodes.len()
}

#[aoc(day8, part2)]
fn part2(map: &Map) -> usize {
    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();

    for antennas in map.antennas_by_frequency.values() {
        for (index, (antenna0_x, antenna0_y)) in antennas.iter().enumerate() {
            for (antenna1_x, antenna1_y) in antennas.iter().skip(index + 1) {
                let (mut dx, mut dy) = (antenna1_x - antenna0_x, antenna1_y - antenna0_y);
                let gcd = gcd(dx, dy);
                (dx, dy) = (dx / gcd, dy / gcd);

                let (mut antinode_x, mut antinode_y) = (*antenna0_x, *antenna0_y);

                while antinode_x >= 0
                    && antinode_x <= map.bottom_right.0
                    && antinode_y >= 0
                    && antinode_y <= map.bottom_right.1
                {
                    antinodes.insert((antinode_x, antinode_y));
                    antinode_x -= dx;
                    antinode_y -= dy;
                }

                (antinode_x, antinode_y) = (antenna0_x + dx, antenna0_y + dy);

                while antinode_x >= 0
                    && antinode_x <= map.bottom_right.0
                    && antinode_y >= 0
                    && antinode_y <= map.bottom_right.1
                {
                    antinodes.insert((antinode_x, antinode_y));
                    antinode_x += dx;
                    antinode_y += dy;
                }
            }
        }
    }

    antinodes.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 14);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(TEST_INPUT)), 34);
    }
}
