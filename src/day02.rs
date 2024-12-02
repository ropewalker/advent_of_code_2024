use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day2)]
fn parse_input(reports: &str) -> Vec<Vec<i32>> {
    use aoc_parse::{parser, prelude::*};

    parser!(lines(repeat_sep(i32, " "))).parse(reports).unwrap()
}

#[aoc(day2, part1)]
fn part1(reports: &[Vec<i32>]) -> usize {
    let mut safe_count = 0;

    'next_report: for report in reports.iter() {
        let diff = report[1] - report[0];

        if diff.abs() > 3 {
            continue;
        }

        let signum = diff.signum();

        for i in 1..report.len() - 1 {
            let diff = report[i + 1] - report[i];

            if diff.signum() != signum || diff.abs() > 3 {
                continue 'next_report;
            }
        }

        safe_count += 1;
    }

    safe_count
}

fn is_safe(report: &[i32], bad_level_tolerated: bool, signum: i32) -> bool {
    for i in 1..report.len() {
        let diff = report[i] - report[i - 1];

        if diff.signum() != signum || diff.abs() > 3 {
            if bad_level_tolerated {
                return false;
            }

            return if i >= 2 {
                is_safe(
                    &[&report[i - 2..i - 1], &report[i..]].concat(),
                    true,
                    signum,
                ) || is_safe(
                    &[&report[i - 2..i], &report[i + 1..]].concat(),
                    true,
                    signum,
                )
            } else {
                is_safe(&report[i..], true, signum)
                    || is_safe(&[&report[0..1], &report[i + 1..]].concat(), true, signum)
            };
        }
    }

    true
}

#[aoc(day2, part2)]
fn part2(reports: &[Vec<i32>]) -> usize {
    reports
        .iter()
        .filter(|report| is_safe(report, false, -1) || is_safe(report, false, 1))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 2);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(TEST_INPUT)), 4);
    }
}
