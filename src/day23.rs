use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::{HashMap, HashSet};

#[aoc_generator(day23)]
fn parse_input(input: &str) -> Vec<(String, String)> {
    use aoc_parse::{parser, prelude::*};

    let parser = parser!(lines(string(lower+) "-" string(lower+)));
    parser.parse(input).unwrap()
}

fn connected_computers(connections: &[(String, String)]) -> HashMap<String, HashSet<String>> {
    connections.iter().fold(
        HashMap::new(),
        |mut connected_computers, (computer_1, computer_2)| {
            connected_computers
                .entry(computer_1.to_owned())
                .or_default()
                .insert(computer_2.to_owned());
            connected_computers
                .entry(computer_2.to_owned())
                .or_default()
                .insert(computer_1.to_owned());

            connected_computers
        },
    )
}

#[aoc(day23, part1)]
fn part1(connections: &[(String, String)]) -> usize {
    let connected_computers: HashMap<String, HashSet<String>> = connected_computers(connections);

    connections
        .iter()
        .map(|(computer_1, computer_2)| {
            connected_computers
                .iter()
                .filter(|(computer, connected_computers)| {
                    connected_computers.contains(computer_1)
                        && connected_computers.contains(computer_2)
                        && (computer.starts_with('t')
                            || computer_1.starts_with('t')
                            || computer_2.starts_with('t'))
                })
                .count()
        })
        .sum::<usize>()
        / 3
}

fn password(sorted_computers: &[&String]) -> String {
    let mut password = String::new();

    if !sorted_computers.is_empty() {
        password.push_str(sorted_computers.first().unwrap());
    }

    sorted_computers.iter().skip(1).for_each(|computer_name| {
        password.push(',');
        password.push_str(computer_name);
    });

    password
}

#[aoc(day23, part2)]
fn part2(connections: &[(String, String)]) -> String {
    let connected_computers: HashMap<String, HashSet<String>> = connected_computers(connections);
    let mut groups: Vec<HashSet<String>> = vec![];

    for computer in connected_computers.keys() {
        if !groups.iter().any(|group| {
            group.iter().all(|group_computer| {
                connected_computers
                    .get(group_computer)
                    .unwrap()
                    .contains(computer)
            })
        }) {
            let mut group = HashSet::from([computer.to_owned()]);
            let mut queue = vec![computer];

            while let Some(computer) = queue.pop() {
                for connected_computer in connected_computers.get(computer).into_iter().flatten() {
                    if !group.contains(connected_computer)
                        && group.iter().all(|group_computer| {
                            connected_computers
                                .get(group_computer)
                                .unwrap()
                                .contains(connected_computer)
                        })
                    {
                        group.insert(connected_computer.to_owned());
                        queue.push(connected_computer);
                    }
                }
            }

            groups.push(group);
        }
    }

    let mut computers = groups
        .iter()
        .max_by(|component1, component2| component1.len().cmp(&component2.len()))
        .unwrap()
        .iter()
        .collect::<Vec<_>>();

    computers.sort();

    password(&computers)
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 7);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(TEST_INPUT)), "co,de,ka,ta".to_string());
    }
}
