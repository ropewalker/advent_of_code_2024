use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::{HashMap, HashSet, VecDeque};

#[aoc_generator(day5)]
fn parse_input(input: &str) -> (Vec<(i32, i32)>, Vec<Vec<i32>>) {
    use aoc_parse::{parser, prelude::*};

    let rules = parser!(lines(i32 "|" i32));
    let updates = parser!(lines(repeat_sep(i32, ",")));
    let parser = parser!(section(rules) section(updates));

    parser.parse(input).unwrap()
}

fn is_in_right_order(update: &[i32], rules: &HashMap<i32, HashSet<i32>>) -> bool {
    for (page_position, page) in update.iter().enumerate() {
        for subsequent_page in rules.get(page).into_iter().flatten() {
            if let Some(subsequent_page_position) = update.iter().position(|x| x == subsequent_page)
            {
                if subsequent_page_position < page_position {
                    return false;
                }
            }
        }
    }

    true
}

#[aoc(day5, part1)]
fn part1((ordered_pairs, updates): &(Vec<(i32, i32)>, Vec<Vec<i32>>)) -> i32 {
    let mut rules: HashMap<i32, HashSet<i32>> = HashMap::new();

    for (preceding, superseding) in ordered_pairs.iter() {
        rules.entry(*preceding).or_default().insert(*superseding);
    }

    updates
        .iter()
        .filter_map(|update| {
            if is_in_right_order(update, &rules) {
                Some(update[update.len() / 2])
            } else {
                None
            }
        })
        .sum()
}

fn fix_order(update: &[i32], rules: &HashMap<i32, HashSet<i32>>) -> Vec<i32> {
    let mut indegrees = HashMap::with_capacity(update.len());

    for preceding_page in update.iter() {
        for superseding_page in rules.get(preceding_page).into_iter().flatten() {
            indegrees
                .entry(superseding_page)
                .and_modify(|indegree| *indegree += 1)
                .or_insert(1);
        }
    }

    let mut result = Vec::with_capacity(update.len());

    let mut queue = update
        .iter()
        .filter(|page| *indegrees.entry(page).or_default() == 0)
        .collect::<VecDeque<_>>();

    while let Some(page) = queue.pop_front() {
        result.push(*page);

        for superseding_page in rules.get(page).into_iter().flatten() {
            if update.contains(superseding_page) {
                indegrees.entry(superseding_page).and_modify(|indegree| {
                    *indegree -= 1;

                    if *indegree == 0 {
                        queue.push_back(superseding_page)
                    }
                });
            }
        }
    }

    result
}

#[aoc(day5, part2)]
fn part2((ordered_pairs, updates): &(Vec<(i32, i32)>, Vec<Vec<i32>>)) -> i32 {
    let mut rules: HashMap<i32, HashSet<i32>> = HashMap::new();

    for (preceding, superseding) in ordered_pairs.iter() {
        rules.entry(*preceding).or_default().insert(*superseding);
    }

    updates
        .iter()
        .filter_map(|update| {
            if !is_in_right_order(update, &rules) {
                Some(fix_order(update, &rules)[update.len() / 2])
            } else {
                None
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 143);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(TEST_INPUT)), 123);
    }
}
