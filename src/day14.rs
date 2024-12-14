use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;

const TIME_PASSED_SECONDS: i32 = 100;
const ROOM_WIDTH: i32 = 101;
const ROOM_HEIGHT: i32 = 103;

#[derive(Clone, Copy, Debug)]
struct Robot {
    position: (i32, i32),
    velocity: (i32, i32),
}

#[aoc_generator(day14)]
fn parse_input(input: &str) -> Vec<Robot> {
    use aoc_parse::{parser, prelude::*};

    let parser = parser!(lines(
        "p=" p_x:i32 "," p_y:i32
        " v=" v_x:i32 "," v_y:i32
            => Robot{
                position: (p_x, p_y),
                velocity: (v_x, v_y)}
    ));

    parser.parse(input).unwrap()
}

fn quadrants_counts(
    robots: &[Robot],
    room_width: i32,
    room_height: i32,
    time: i32,
) -> (usize, usize, usize, usize) {
    robots
        .iter()
        .map(|robot| {
            (
                ((robot.position.0 + robot.velocity.0 * time) % room_width + room_width)
                    % room_width,
                ((robot.position.1 + robot.velocity.1 * time) % room_height + room_height)
                    % room_height,
            )
        })
        .fold((0, 0, 0, 0), |(nw, ne, sw, se), (x, y)| match (x, y) {
            (x, y) if x < room_width / 2 && y < room_height / 2 => (nw + 1, ne, sw, se),
            (x, y) if x > room_width / 2 && y < room_height / 2 => (nw, ne + 1, sw, se),
            (x, y) if x < room_width / 2 && y > room_height / 2 => (nw, ne, sw + 1, se),
            (x, y) if x > room_width / 2 && y > room_height / 2 => (nw, ne, sw, se + 1),
            _ => (nw, ne, sw, se),
        })
}

fn safety_factor(robots: &[Robot], room_width: i32, room_height: i32, time: i32) -> usize {
    let quadrants_counts = quadrants_counts(robots, room_width, room_height, time);
    quadrants_counts.0 * quadrants_counts.1 * quadrants_counts.2 * quadrants_counts.3
}

#[aoc(day14, part1)]
fn part1(robots: &[Robot]) -> usize {
    safety_factor(robots, ROOM_WIDTH, ROOM_HEIGHT, TIME_PASSED_SECONDS)
}

fn move_robots(robots: &[Robot], room_width: i32, room_height: i32, time: i32) -> Vec<Robot> {
    robots
        .iter()
        .map(|robot| Robot {
            position: (
                ((robot.position.0 + robot.velocity.0 * time) % room_width + room_width)
                    % room_width,
                ((robot.position.1 + robot.velocity.1 * time) % room_height + room_height)
                    % room_height,
            ),
            velocity: robot.velocity,
        })
        .collect()
}

fn find_cluster(robots: &[Robot], room_width: i32, room_height: i32) -> Option<(i32, i32)> {
    let cluster_width = room_width / 2;
    let cluster_height = room_height / 2;

    let robots_count = robots.len();

    let robots_map = robots.iter().fold(
        vec![vec![0; room_width as usize]; room_height as usize],
        |mut map, robot| {
            map[robot.position.1 as usize][robot.position.0 as usize] += 1;
            map
        },
    );

    for y in 0..room_height - cluster_height + 1 {
        for x in 0..room_width - cluster_width + 1 {
            let cluster_count = robots_map[y as usize..(y + cluster_height) as usize]
                .iter()
                .map(|row| {
                    row[x as usize..(x + cluster_width) as usize]
                        .iter()
                        .sum::<usize>()
                })
                .sum::<usize>();

            if cluster_count >= robots_count / 2 {
                return Some((x, y));
            }
        }
    }

    None
}

fn print_map(robots: &[Robot], room_width: i32, room_height: i32) {
    let map = robots
        .iter()
        .map(|robot| robot.position)
        .collect::<HashSet<_>>();

    for y in 0..room_height {
        for x in 0..room_width {
            if map.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }

        println!();
    }
}

#[aoc(day14, part2)]
fn part2(robots: &[Robot]) -> i32 {
    let mut time_passed = 0;

    loop {
        time_passed += 1;

        let robots = move_robots(robots, ROOM_WIDTH, ROOM_HEIGHT, time_passed);

        if find_cluster(&robots, ROOM_WIDTH, ROOM_HEIGHT).is_some() {
            print_map(&robots, ROOM_WIDTH, ROOM_HEIGHT);

            return time_passed;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

    #[test]
    fn part1_example() {
        assert_eq!(
            safety_factor(&parse_input(TEST_INPUT), 7, 11, TIME_PASSED_SECONDS),
            12
        );
    }
}
