use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::{HashSet, VecDeque};

#[aoc_generator(day12)]
fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|row| row.chars().collect()).collect()
}

fn region_price(
    map: &[Vec<char>],
    (start_x, start_y): (usize, usize),
    claimed: &mut [Vec<bool>],
) -> usize {
    let (bottom_right_x, bottom_right_y) = (map[0].len() - 1, map.len() - 1);

    let mut area = 0;
    let mut perimeter = 0;

    if claimed[start_y][start_x] {
        return 0;
    }

    let region_id = map[start_y][start_x];

    claimed[start_y][start_x] = true;
    area += 1;

    let mut visited = HashSet::from([(start_x, start_y)]);
    let mut queue = VecDeque::from([(start_x, start_y)]);

    while let Some((x, y)) = queue.pop_front() {
        let mut next_plots: Vec<(usize, usize)> = Vec::with_capacity(4);

        if x > 0 {
            next_plots.push((x - 1, y));
        } else {
            perimeter += 1;
        }

        if x < bottom_right_x {
            next_plots.push((x + 1, y));
        } else {
            perimeter += 1;
        }

        if y > 0 {
            next_plots.push((x, y - 1));
        } else {
            perimeter += 1;
        }

        if y < bottom_right_y {
            next_plots.push((x, y + 1));
        } else {
            perimeter += 1;
        }

        for (next_plot_x, next_plot_y) in next_plots {
            if visited.contains(&(next_plot_x, next_plot_y)) {
                continue;
            }

            if map[next_plot_y][next_plot_x] == region_id {
                area += 1;

                visited.insert((next_plot_x, next_plot_y));
                queue.push_back((next_plot_x, next_plot_y));
                claimed[next_plot_y][next_plot_x] = true;
            } else {
                perimeter += 1;
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
    let (bottom_right_x, bottom_right_y) = (map[0].len() - 1, map.len() - 1);

    let mut area = 0;
    let mut number_of_sides = 0;

    if claimed[start_y][start_x] {
        return 0;
    }

    let region_id = map[start_y][start_x];

    claimed[start_y][start_x] = true;
    area += 1;

    let mut visited = HashSet::from([(start_x, start_y)]);
    let mut queue = VecDeque::from([(start_x, start_y)]);

    let mut vertical_left_fences: HashSet<(usize, usize)> = HashSet::new();
    let mut vertical_right_fences: HashSet<(usize, usize)> = HashSet::new();
    let mut horizontal_up_fences: HashSet<(usize, usize)> = HashSet::new();
    let mut horizontal_down_fences: HashSet<(usize, usize)> = HashSet::new();

    while let Some((x, y)) = queue.pop_front() {
        if x > 0 {
            let (next_plot_x, next_plot_y) = (x - 1, y);

            if !visited.contains(&(next_plot_x, next_plot_y)) {
                if map[next_plot_y][next_plot_x] == region_id {
                    area += 1;

                    visited.insert((next_plot_x, next_plot_y));
                    queue.push_back((next_plot_x, next_plot_y));
                    claimed[next_plot_y][next_plot_x] = true;
                } else {
                    vertical_left_fences.insert((x, y));
                }
            }
        } else {
            vertical_left_fences.insert((x, y));
        }

        if x < bottom_right_x {
            let (next_plot_x, next_plot_y) = (x + 1, y);

            if !visited.contains(&(next_plot_x, next_plot_y)) {
                if map[next_plot_y][next_plot_x] == region_id {
                    area += 1;

                    visited.insert((next_plot_x, next_plot_y));
                    queue.push_back((next_plot_x, next_plot_y));
                    claimed[next_plot_y][next_plot_x] = true;
                } else {
                    vertical_right_fences.insert((x + 1, y));
                }
            }
        } else {
            vertical_right_fences.insert((x + 1, y));
        }

        if y > 0 {
            let (next_plot_x, next_plot_y) = (x, y - 1);

            if !visited.contains(&(next_plot_x, next_plot_y)) {
                if map[next_plot_y][next_plot_x] == region_id {
                    area += 1;

                    visited.insert((next_plot_x, next_plot_y));
                    queue.push_back((next_plot_x, next_plot_y));
                    claimed[next_plot_y][next_plot_x] = true;
                } else {
                    horizontal_up_fences.insert((x, y));
                }
            }
        } else {
            horizontal_up_fences.insert((x, y));
        }

        if y < bottom_right_y {
            let (next_plot_x, next_plot_y) = (x, y + 1);

            if !visited.contains(&(next_plot_x, next_plot_y)) {
                if map[next_plot_y][next_plot_x] == region_id {
                    area += 1;

                    visited.insert((next_plot_x, next_plot_y));
                    queue.push_back((next_plot_x, next_plot_y));
                    claimed[next_plot_y][next_plot_x] = true;
                } else {
                    horizontal_down_fences.insert((x, y + 1));
                }
            }
        } else {
            horizontal_down_fences.insert((x, y + 1));
        }
    }

    let mut counted_fences: HashSet<(usize, usize)> = HashSet::new();

    for (fence_x, fence_y) in horizontal_up_fences.iter() {
        if counted_fences.contains(&(*fence_x, *fence_y)) {
            continue;
        }

        number_of_sides += 1;
        counted_fences.insert((*fence_x, *fence_y));
        let mut queue = vec![(*fence_x, *fence_y)];

        while let Some((fence_x, fence_y)) = queue.pop() {
            if fence_x > 0
                && horizontal_up_fences.contains(&(fence_x - 1, fence_y))
                && !counted_fences.contains(&(fence_x - 1, fence_y))
            {
                queue.push((fence_x - 1, fence_y));
                counted_fences.insert((fence_x - 1, fence_y));
            }

            if fence_x <= bottom_right_x
                && horizontal_up_fences.contains(&(fence_x + 1, fence_y))
                && !counted_fences.contains(&(fence_x + 1, fence_y))
            {
                queue.push((fence_x + 1, fence_y));
                counted_fences.insert((fence_x + 1, fence_y));
            }
        }
    }

    let mut counted_fences: HashSet<(usize, usize)> = HashSet::new();

    for (fence_x, fence_y) in horizontal_down_fences.iter() {
        if counted_fences.contains(&(*fence_x, *fence_y)) {
            continue;
        }

        number_of_sides += 1;
        counted_fences.insert((*fence_x, *fence_y));
        let mut queue = vec![(*fence_x, *fence_y)];

        while let Some((fence_x, fence_y)) = queue.pop() {
            if fence_x > 0
                && horizontal_down_fences.contains(&(fence_x - 1, fence_y))
                && !counted_fences.contains(&(fence_x - 1, fence_y))
            {
                queue.push((fence_x - 1, fence_y));
                counted_fences.insert((fence_x - 1, fence_y));
            }

            if fence_x <= bottom_right_x
                && horizontal_down_fences.contains(&(fence_x + 1, fence_y))
                && !counted_fences.contains(&(fence_x + 1, fence_y))
            {
                queue.push((fence_x + 1, fence_y));
                counted_fences.insert((fence_x + 1, fence_y));
            }
        }
    }

    let mut counted_fences: HashSet<(usize, usize)> = HashSet::new();

    for (fence_x, fence_y) in vertical_left_fences.iter() {
        if counted_fences.contains(&(*fence_x, *fence_y)) {
            continue;
        }

        number_of_sides += 1;
        counted_fences.insert((*fence_x, *fence_y));
        let mut queue = vec![(*fence_x, *fence_y)];

        while let Some((fence_x, fence_y)) = queue.pop() {
            if fence_y > 0
                && vertical_left_fences.contains(&(fence_x, fence_y - 1))
                && !counted_fences.contains(&(fence_x, fence_y - 1))
            {
                queue.push((fence_x, fence_y - 1));
                counted_fences.insert((fence_x, fence_y - 1));
            }

            if fence_y <= bottom_right_y
                && vertical_left_fences.contains(&(fence_x, fence_y + 1))
                && !counted_fences.contains(&(fence_x, fence_y + 1))
            {
                queue.push((fence_x, fence_y + 1));
                counted_fences.insert((fence_x, fence_y + 1));
            }
        }
    }

    let mut counted_fences: HashSet<(usize, usize)> = HashSet::new();

    for (fence_x, fence_y) in vertical_right_fences.iter() {
        if counted_fences.contains(&(*fence_x, *fence_y)) {
            continue;
        }

        number_of_sides += 1;
        counted_fences.insert((*fence_x, *fence_y));
        let mut queue = vec![(*fence_x, *fence_y)];

        while let Some((fence_x, fence_y)) = queue.pop() {
            if fence_y > 0
                && vertical_right_fences.contains(&(fence_x, fence_y - 1))
                && !counted_fences.contains(&(fence_x, fence_y - 1))
            {
                queue.push((fence_x, fence_y - 1));
                counted_fences.insert((fence_x, fence_y - 1));
            }

            if fence_y <= bottom_right_y
                && vertical_right_fences.contains(&(fence_x, fence_y + 1))
                && !counted_fences.contains(&(fence_x, fence_y + 1))
            {
                queue.push((fence_x, fence_y + 1));
                counted_fences.insert((fence_x, fence_y + 1));
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
