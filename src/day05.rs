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

fn is_in_right_order(update: &[i32], rule_violations: &HashMap<i32, HashSet<i32>>) -> bool {
    let mut update: VecDeque<i32> = VecDeque::from(update.to_vec());
    let mut visited: HashSet<i32> = HashSet::with_capacity(rule_violations.len());

    while update.len() > 1 {
        let current_page = update.pop_front().unwrap();

        let mut nodes = VecDeque::from([current_page]);
        visited.insert(current_page);

        while !nodes.is_empty() {
            let current_node = nodes.pop_front().unwrap();

            for next_node in rule_violations.get(&current_node).into_iter().flatten() {
                if visited.contains(next_node) {
                    continue;
                }

                if update.contains(next_node) {
                    return false;
                }

                visited.insert(*next_node);
            }
        }
    }

    true
}

#[aoc(day5, part1)]
fn part1((page_ordering_rules, updates): &(Vec<(i32, i32)>, Vec<Vec<i32>>)) -> i32 {
    let mut rule_violations: HashMap<i32, HashSet<i32>> = HashMap::new();

    for (preceding, superseding) in page_ordering_rules.iter() {
        rule_violations
            .entry(*superseding)
            .or_default()
            .insert(*preceding);
    }

    updates
        .iter()
        .filter_map(|update| {
            if is_in_right_order(update, &rule_violations) {
                Some(update[update.len() / 2])
            } else {
                None
            }
        })
        .sum()
}

fn fix_order(update: &[i32], rule_violations: &HashMap<i32, HashSet<i32>>) -> Option<Vec<i32>> {
    let mut update: VecDeque<i32> = VecDeque::from(update.to_vec());
    let mut right_order = Vec::with_capacity(update.len());
    let mut visited: HashSet<i32> = HashSet::with_capacity(rule_violations.len());
    let mut fixed = false;

    'next_page: while update.len() > 1 {
        let current_page = update.pop_front().unwrap();

        let mut nodes = VecDeque::from([current_page]);

        while !nodes.is_empty() {
            let current_node = nodes.pop_front().unwrap();

            for next_node in rule_violations.get(&current_node).into_iter().flatten() {
                if visited.contains(next_node) {
                    continue;
                }

                if let Some(index) = update.iter().position(|page| *page == *next_node) {
                    update.remove(index);
                    update.push_front(current_page);
                    update.push_front(*next_node);
                    fixed = true;
                    continue 'next_page;
                }

                visited.insert(*next_node);
            }

            visited.insert(current_node);
        }

        right_order.push(current_page);
    }

    if fixed {
        Some(right_order)
    } else {
        None
    }
}

#[aoc(day5, part2)]
fn part2((page_ordering_rules, updates): &(Vec<(i32, i32)>, Vec<Vec<i32>>)) -> i32 {
    let mut rule_violations: HashMap<i32, HashSet<i32>> = HashMap::new();

    for (preceding, superseding) in page_ordering_rules.iter() {
        rule_violations
            .entry(*superseding)
            .or_default()
            .insert(*preceding);
    }

    updates
        .iter()
        .filter_map(|update| {
            fix_order(update, &rule_violations)
                .map(|fixed_update| fixed_update[fixed_update.len() / 2])
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
