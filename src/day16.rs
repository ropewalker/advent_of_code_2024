use aoc_runner_derive::{aoc, aoc_generator};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};

const TURN_COST: usize = 1_000;
const MOVE_COST: usize = 1;

type Position = (i32, i32);
type Direction = (i32, i32);

#[derive(Debug)]
struct RaceSetup {
    obstacles: HashSet<Position>,
    start: Position,
    start_direction: Direction,
    end: Position,
}

#[aoc_generator(day16)]
fn parse_input(input: &str) -> RaceSetup {
    let mut start = (0, 0);
    let start_direction = (1, 0);
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
        start_direction,
        end,
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    position: Position,
    direction: Direction,
    end: Position,
    score: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .score
            .cmp(&self.score)
            .then_with(|| {
                ((self.position.0 - self.end.0).abs() + (self.position.1 - self.end.1).abs()).cmp(
                    &((other.position.0 - other.end.0).abs()
                        + (other.position.1 - other.end.1).abs()),
                )
            })
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[aoc(day16, part1)]
fn part1(race_setup: &RaceSetup) -> Option<usize> {
    let mut min_scores: HashMap<(Position, Direction), usize> = HashMap::new();

    let mut frontier = BinaryHeap::new();

    frontier.push(State {
        position: race_setup.start,
        direction: race_setup.start_direction,
        end: race_setup.end,
        score: 0,
    });

    while let Some(State {
        position,
        direction,
        end,
        score,
    }) = frontier.pop()
    {
        if position == end {
            return Some(score);
        }

        let min_score = min_scores.entry((position, direction)).or_insert(score);

        if score > *min_score {
            continue;
        }

        for new_state in [
            State {
                position: (position.0 + direction.0, position.1 + direction.1),
                direction,
                end,
                score: score + MOVE_COST,
            },
            State {
                position,
                direction: (direction.1, direction.0),
                end,
                score: score + TURN_COST,
            },
            State {
                position,
                direction: (-direction.1, -direction.0),
                end,
                score: score + TURN_COST,
            },
        ] {
            if race_setup.obstacles.contains(&new_state.position) {
                continue;
            }

            let min_score = min_scores
                .entry((new_state.position, new_state.direction))
                .or_insert(new_state.score + 1);

            if new_state.score < *min_score {
                *min_score = new_state.score;
                frontier.push(new_state);
            }
        }
    }

    None
}

#[aoc(day16, part2)]
fn part2(race_setup: &RaceSetup) -> Option<usize> {
    let mut min_scores: HashMap<(Position, Direction), usize> = HashMap::new();
    let mut best_seats: HashMap<(Position, Direction), HashSet<Position>> = HashMap::new();

    let mut frontier = BinaryHeap::new();

    frontier.push(State {
        position: race_setup.start,
        direction: race_setup.start_direction,
        end: race_setup.end,
        score: 0,
    });

    while let Some(State {
        position,
        direction,
        end,
        score,
    }) = frontier.pop()
    {
        if position == end {
            continue;
        }

        let min_score = min_scores.entry((position, direction)).or_insert(score);

        if score > *min_score {
            continue;
        }

        for new_state in [
            State {
                position: (position.0 + direction.0, position.1 + direction.1),
                direction,
                end,
                score: score + MOVE_COST,
            },
            State {
                position,
                direction: (direction.1, direction.0),
                end,
                score: score + TURN_COST,
            },
            State {
                position,
                direction: (-direction.1, -direction.0),
                end,
                score: score + TURN_COST,
            },
        ] {
            if race_setup.obstacles.contains(&new_state.position) {
                continue;
            }

            let min_score = min_scores
                .entry((new_state.position, new_state.direction))
                .or_insert(new_state.score);

            match new_state.score.cmp(&(*min_score)) {
                Ordering::Less => {
                    let current_best_seats = best_seats
                        .get(&(position, direction))
                        .unwrap_or(&HashSet::new())
                        .clone();
                    let next_best_seats = best_seats
                        .entry((new_state.position, new_state.direction))
                        .or_default();

                    *next_best_seats = current_best_seats.clone();
                    (*next_best_seats).insert(position);

                    *min_score = new_state.score;
                    frontier.push(new_state);
                }
                Ordering::Equal => {
                    let current_best_seats = best_seats
                        .get(&(position, direction))
                        .unwrap_or(&HashSet::new())
                        .clone();
                    let next_best_seats = best_seats
                        .entry((new_state.position, new_state.direction))
                        .or_default();

                    *next_best_seats = next_best_seats
                        .union(&current_best_seats)
                        .cloned()
                        .collect();
                    (*next_best_seats).insert(position);

                    frontier.push(new_state);
                }
                Ordering::Greater => {}
            }
        }
    }

    if let Some(min_score) = min_scores
        .iter()
        .filter_map(|((position, _), score)| {
            if *position == race_setup.end {
                Some(*score)
            } else {
                None
            }
        })
        .min()
    {
        let seats = best_seats
            .iter()
            .filter_map(|((position, direction), seats)| {
                if *position == race_setup.end
                    && *min_scores.get(&(*position, *direction)).unwrap() == min_score
                {
                    Some(seats)
                } else {
                    None
                }
            })
            .flatten()
            .collect::<HashSet<_>>();
        Some(seats.len() + 1)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT_1: &str = r"###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";

    static TEST_INPUT_2: &str = r"#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";

    #[test]
    fn part1_example_1() {
        assert_eq!(part1(&parse_input(TEST_INPUT_1)), Some(7_036));
    }

    #[test]
    fn part1_example_2() {
        assert_eq!(part1(&parse_input(TEST_INPUT_2)), Some(11_048));
    }

    #[test]
    fn part2_example_1() {
        assert_eq!(part2(&parse_input(TEST_INPUT_1)), Some(45));
    }

    #[test]
    fn part2_example_2() {
        assert_eq!(part2(&parse_input(TEST_INPUT_2)), Some(64));
    }
}
