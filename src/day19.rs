use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::{HashMap, HashSet};

#[aoc_generator(day19)]
fn parse_input(input: &str) -> (HashSet<String>, Vec<String>) {
    use aoc_parse::{parser, prelude::*};

    let parser = parser!(
        section(hash_set(line(repeat_sep(string({'w', 'u', 'b', 'r', 'g'}+), ", ")))) 
        section(lines(string({'w', 'u', 'b', 'r', 'g'}+))));

    parser.parse(input).unwrap()
}

fn is_possible(pattern: &str, towels: &HashSet<String>) -> bool {
    let mut possible = false;

    if pattern.is_empty() {
        return true;
    }

    for i in 1..=pattern.len() {
        let prefix = &pattern[0..i];
        let postfix = &pattern[i..];

        possible = possible || towels.contains(prefix) && is_possible(postfix, towels);

        if possible {
            return true;
        }
    }

    possible
}

#[aoc(day19, part1)]
fn part1((towels, patterns): &(HashSet<String>, Vec<String>)) -> usize {
    patterns
        .iter()
        .filter(|pattern| is_possible(pattern, towels))
        .count()
}

fn count_possibilities(
    pattern: &str,
    towels: &HashSet<String>,
    index: &mut HashMap<String, usize>,
) -> usize {
    if pattern.is_empty() {
        return 1;
    }

    if let Some(possibilities) = index.get(pattern) {
        return *possibilities;
    }

    let mut possibilities = 0;

    for i in 1..=pattern.len() {
        let prefix = &pattern[0..i];
        let postfix = &pattern[i..];

        if towels.contains(prefix) {
            possibilities += count_possibilities(postfix, towels, index);
        }
    }

    index.insert(pattern.to_string(), possibilities);

    possibilities
}

#[aoc(day19, part2)]
fn part2((towels, patterns): &(HashSet<String>, Vec<String>)) -> usize {
    let mut index = HashMap::new();

    patterns
        .iter()
        .map(|pattern| count_possibilities(pattern, towels, &mut index))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 6);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(TEST_INPUT)), 16);
    }
}
