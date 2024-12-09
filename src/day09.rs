use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::BTreeMap;
use std::iter::Iterator;

#[aoc_generator(day9)]
fn parse_input(input: &str) -> Vec<usize> {
    use aoc_parse::{parser, prelude::*};

    let parser = parser!(line(digit+));
    parser.parse(input).unwrap()
}

#[aoc(day9, part1)]
fn part1(disk_map: &[usize]) -> usize {
    let mut layout: Vec<usize> = Vec::new();

    let mut head = 0;
    let mut tail = disk_map.len() - 1;

    if tail % 2 != 0 {
        tail -= 1;
    }

    let mut tail_id = tail / 2;
    let mut tail_len = disk_map[tail];
    let mut gap_len;

    'label: while head < tail {
        if head % 2 == 0 {
            let head_id = head / 2;
            let head_len = disk_map[head];

            layout.extend([head_id].into_iter().cycle().take(head_len));
            head += 1;
        } else {
            gap_len = disk_map[head];

            while gap_len >= tail_len {
                layout.extend([tail_id].into_iter().cycle().take(tail_len));

                gap_len -= tail_len;
                tail_len = 0;

                tail -= 2;

                if tail <= head {
                    break 'label;
                }

                tail_id = tail / 2;
                tail_len = disk_map[tail];
            }

            layout.extend([tail_id].into_iter().cycle().take(gap_len));

            tail_len -= gap_len;
            head += 1;
        }
    }

    layout.extend([tail_id].into_iter().cycle().take(tail_len));

    layout
        .iter()
        .enumerate()
        .map(|(address, id)| address * id)
        .sum()
}

#[aoc(day9, part2)]
fn part2(disk_map: &[usize]) -> usize {
    let mut file_index: BTreeMap<usize, (usize, usize)> = BTreeMap::new();
    let mut layout: BTreeMap<usize, usize> = BTreeMap::new();

    let mut address = 0;

    disk_map
        .iter()
        .enumerate()
        .for_each(|(index, length)| match index % 2 {
            0 => {
                let file_id = index / 2;
                file_index.insert(file_id, (address, *length));
                layout.insert(address, file_id);
                address += length;
            }
            1 => {
                address += length;
            }
            _ => unreachable!(),
        });

    let mut file_id = (disk_map.len() - 1) / 2;

    while file_id > 0 {
        let (file_address, file_len) = file_index.get(&file_id).unwrap();
        let mut new_address = *file_address;

        for ((first_address, first_id), (second_address, second_id)) in
            layout.iter().zip(layout.iter().skip(1))
        {
            let first_len = file_index.get(first_id).unwrap().1;
            let gap_len = second_address - (first_address + first_len);

            if gap_len >= *file_len {
                new_address = first_address + first_len;
                break;
            } else if *second_id == file_id {
                break;
            }
        }

        layout.remove(file_address);
        layout.insert(new_address, file_id);
        file_index.insert(file_id, (new_address, *file_len));
        file_id -= 1;
    }

    layout
        .iter()
        .map(|(address, file_id)| {
            let file_len = file_index.get(file_id).unwrap().1;
            file_id * (2 * address + file_len - 1) * file_len / 2
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "2333133121414131402";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 1928);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(TEST_INPUT)), 2858);
    }
}
