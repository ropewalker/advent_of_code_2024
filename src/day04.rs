use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day4)]
fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

#[aoc(day4, part1)]
fn part1(word_search: &[Vec<char>]) -> usize {
    const XMAS: &str = "XMAS";

    let mut count = 0;

    for (y0, line) in word_search.iter().enumerate() {
        for (x0, letter) in line.iter().enumerate() {
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
                let (mut x, mut y) = (x0 as i32, y0 as i32);

                for xmas_char in XMAS.chars().skip(1) {
                    (x, y) = (x + x_shift, y + y_shift);

                    if x < 0
                        || x as usize >= line.len()
                        || y < 0
                        || y as usize >= word_search.len()
                        || word_search[y as usize][x as usize] != xmas_char
                    {
                        continue 'next_direction;
                    }
                }

                count += 1;
            }
        }
    }

    count
}

#[aoc(day4, part2)]
fn part2(word_search: &[Vec<char>]) -> usize {
    let mut count = 0;

    for y in 1..word_search.len() - 1 {
        for x in 1..word_search[0].len() - 1 {
            if word_search[y][x] == 'A'
                && (word_search[y - 1][x - 1] == 'M' && word_search[y + 1][x + 1] == 'S'
                    || word_search[y - 1][x - 1] == 'S' && word_search[y + 1][x + 1] == 'M')
                && (word_search[y + 1][x - 1] == 'M' && word_search[y - 1][x + 1] == 'S'
                    || word_search[y + 1][x - 1] == 'S' && word_search[y - 1][x + 1] == 'M')
            {
                count += 1
            }
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
