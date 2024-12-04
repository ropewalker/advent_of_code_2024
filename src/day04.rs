use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;
use std::iter::Iterator;

#[aoc_generator(day4)]
fn parse_input(input: &str) -> HashMap<(i32, i32), char> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, letter)| ((x as i32, y as i32), letter))
                .collect::<Vec<_>>()
        })
        .collect()
}

#[aoc(day4, part1)]
fn part1(word_search: &HashMap<(i32, i32), char>) -> i32 {
    const XMAS: &str = "XMAS";

    let mut count = 0;

    for ((x0, y0), letter) in word_search.iter() {
        if !XMAS.starts_with(*letter) {
            continue;
        }

        'next_direction: for (x_shift, y_shift) in [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ] {
            let (mut x, mut y) = (*x0, *y0);

            for xmas_char in XMAS.chars().skip(1) {
                (x, y) = (x + x_shift, y + y_shift);

                if word_search.get(&(x, y)) != Some(&xmas_char) {
                    continue 'next_direction;
                }
            }

            count += 1;
        }
    }

    count
}

#[aoc(day4, part2)]
fn part2(word_search: &HashMap<(i32, i32), char>) -> i32 {
    let mut count = 0;

    for ((x, y), letter) in word_search.iter() {
        if *letter == 'A'
            && (word_search.get(&(x - 1, y - 1)) == Some(&'M')
                && word_search.get(&(x + 1, y + 1)) == Some(&'S')
                || word_search.get(&(x - 1, y - 1)) == Some(&'S')
                    && word_search.get(&(x + 1, y + 1)) == Some(&'M'))
            && (word_search.get(&(x - 1, y + 1)) == Some(&'M')
                && word_search.get(&(x + 1, y - 1)) == Some(&'S')
                || word_search.get(&(x - 1, y + 1)) == Some(&'S')
                    && word_search.get(&(x + 1, y - 1)) == Some(&'M'))
        {
            count += 1
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 18);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(TEST_INPUT)), 9);
    }
}
