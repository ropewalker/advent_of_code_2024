use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::{HashMap, HashSet, VecDeque};

#[aoc_generator(day12)]
fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|row| row.chars().collect()).collect()
}

fn region_price(
    map: &[Vec<char>],
    (start_x, start_y): (usize, usize),
    claimed: &mut [Vec<bool>],
) -> usize {
    let (bottom_right_x, bottom_right_y) = ((map[0].len() - 1) as i32, (map.len() - 1) as i32);

    let mut area = 0;
    let mut perimeter = 0;

    if claimed[start_y][start_x] {
        return 0;
    }

    let region_id = map[start_y][start_x];

    claimed[start_y][start_x] = true;
    area += 1;

    let mut visited = HashSet::from([(start_x as i32, start_y as i32)]);
    let mut queue = VecDeque::from([(start_x as i32, start_y as i32)]);

    while let Some((x, y)) = queue.pop_front() {
        for (direction_x, direction_y) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let (next_plot_x, next_plot_y) = (x + direction_x, y + direction_y);

            if visited.contains(&(next_plot_x, next_plot_y)) {
                continue;
            }

            if next_plot_x < 0
                || next_plot_x > bottom_right_x
                || next_plot_y < 0
                || next_plot_y > bottom_right_y
                || map[next_plot_y as usize][next_plot_x as usize] != region_id
            {
                perimeter += 1;
            } else {
                area += 1;

                visited.insert((next_plot_x, next_plot_y));
                queue.push_back((next_plot_x, next_plot_y));
                claimed[next_plot_y as usize][next_plot_x as usize] = true;
            }
        }
    }

    area * perimeter
}

#[aoc(day12, part1)]
fn part1(map: &[Vec<char>]) -> usize {
    let (bottom_right_x, bottom_right_y) = (map[0].len() - 1, map.len() - 1);
    let mut claimed: Vec<Vec<bool>> = vec![vec![false; bottom_right_x + 1]; bottom_right_y + 1];

    let mut result = 0;

    for y in 0..=bottom_right_y {
        for x in 0..=bottom_right_x {
            result += region_price(map, (x, y), &mut claimed);
        }
    }

    result
}

fn region_discounted_price(
    map: &[Vec<char>],
    (start_x, start_y): (usize, usize),
    claimed: &mut [Vec<bool>],
) -> usize {
    let (bottom_right_x, bottom_right_y) = ((map[0].len() - 1) as i32, (map.len() - 1) as i32);

    let mut area = 0;
    let mut number_of_sides = 0;

    if claimed[start_y][start_x] {
        return 0;
    }

    let region_id = map[start_y][start_x];

    claimed[start_y][start_x] = true;
    area += 1;

    let mut visited = HashSet::from([(start_x as i32, start_y as i32)]);
    let mut queue = VecDeque::from([(start_x as i32, start_y as i32)]);

    let mut fences_by_direction: HashMap<(i32, i32), HashSet<(i32, i32)>> = HashMap::new();

    while let Some((x, y)) = queue.pop_front() {
        for (direction_x, direction_y) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let (next_plot_x, next_plot_y) = (x + direction_x, y + direction_y);

            if visited.contains(&(next_plot_x, next_plot_y)) {
                continue;
            }

            if next_plot_x < 0
                || next_plot_x > bottom_right_x
                || next_plot_y < 0
                || next_plot_y > bottom_right_y
                || map[next_plot_y as usize][next_plot_x as usize] != region_id
            {
                fences_by_direction
                    .entry((direction_x, direction_y))
                    .or_default()
                    .insert((next_plot_x, next_plot_y));
            } else {
                area += 1;

                visited.insert((next_plot_x, next_plot_y));
                queue.push_back((next_plot_x, next_plot_y));
                claimed[next_plot_y as usize][next_plot_x as usize] = true;
            }
        }
    }

    for ((direction_x, direction_y), fences) in fences_by_direction.iter() {
        let mut counted_fences: HashSet<(i32, i32)> = HashSet::new();

        for (fence_x, fence_y) in fences.iter() {
            if counted_fences.contains(&(*fence_x, *fence_y)) {
                continue;
            }

            number_of_sides += 1;
            counted_fences.insert((*fence_x, *fence_y));

            for (shift_x, shift_y) in [
                (*direction_y, *direction_x),
                (-(*direction_y), -(*direction_x)),
            ] {
                let (mut connected_x, mut connected_y) = (fence_x + shift_x, fence_y + shift_y);

                while fences.contains(&(connected_x, connected_y))
                    && !counted_fences.contains(&(connected_x, connected_y))
                {
                    counted_fences.insert((connected_x, connected_y));
                    connected_x += shift_x;
                    connected_y += shift_y;
                }
            }
        }
    }

    area * number_of_sides
}

#[aoc(day12, part2)]
fn part2(map: &[Vec<char>]) -> usize {
    let (bottom_right_x, bottom_right_y) = (map[0].len() - 1, map.len() - 1);
    let mut claimed: Vec<Vec<bool>> = vec![vec![false; bottom_right_x + 1]; bottom_right_y + 1];

    let mut result = 0;

    for y in 0..=bottom_right_y {
        for x in 0..=bottom_right_x {
            result += region_discounted_price(map, (x, y), &mut claimed);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT_1: &str = r"AAAA
BBCD
BBCC
EEEC";

    static TEST_INPUT_2: &str = r"OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";

    static TEST_INPUT_3: &str = r"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

    static TEST_INPUT_4: &str = r"EEEEE
EXXXX
EEEEE
EXXXX
EEEEE";

    static TEST_INPUT_5: &str = r"AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA";

    #[test]
    fn part1_example_1() {
        assert_eq!(part1(&parse_input(TEST_INPUT_1)), 140);
    }

    #[test]
    fn part1_example_2() {
        assert_eq!(part1(&parse_input(TEST_INPUT_2)), 772);
    }

    #[test]
    fn part1_example_3() {
        assert_eq!(part1(&parse_input(TEST_INPUT_3)), 1930);
    }

    #[test]
    fn part2_example_1() {
        assert_eq!(part2(&parse_input(TEST_INPUT_1)), 80);
    }

    #[test]
    fn part2_example_2() {
        assert_eq!(part2(&parse_input(TEST_INPUT_2)), 436);
    }

    #[test]
    fn part2_example_3() {
        assert_eq!(part2(&parse_input(TEST_INPUT_4)), 236);
    }

    #[test]
    fn part2_example_4() {
        assert_eq!(part2(&parse_input(TEST_INPUT_5)), 368);
    }

    #[test]
    fn part2_example_5() {
        assert_eq!(part2(&parse_input(TEST_INPUT_3)), 1206);
    }
}
