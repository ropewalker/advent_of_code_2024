use aoc_runner_derive::{aoc, aoc_generator};
use std::iter::Iterator;

#[aoc_generator(day9)]
fn parse_input(input: &str) -> Vec<usize> {
    use aoc_parse::{parser, prelude::*};

    let parser = parser!(line(digit+));
    parser.parse(input).unwrap()
}

#[aoc(day9, part1)]
fn part1(disk_map: &[usize]) -> usize {
    let mut filesystem_checksum = 0;
    let mut block_count = 0;

    let mut head_index = 0;

    let mut tail_index = disk_map.len() - 1;

    if tail_index % 2 != 0 {
        tail_index -= 1;
    }

    let mut tail_file_id = tail_index / 2;
    let mut tail_file_length = disk_map[tail_index];

    while head_index < tail_index {
        if head_index % 2 == 0 {
            let head_file_id = head_index / 2;
            let head_file_length = disk_map[head_index];

            filesystem_checksum +=
                head_file_id * head_file_length * (2 * block_count + head_file_length - 1) / 2;
            block_count += head_file_length;

            head_index += 1;
        } else {
            let mut free_space_length = disk_map[head_index];

            while free_space_length > 0 && tail_index > head_index {
                let filled_length = free_space_length.min(tail_file_length);

                filesystem_checksum +=
                    tail_file_id * filled_length * (2 * block_count + filled_length - 1) / 2;
                block_count += filled_length;

                free_space_length -= filled_length;
                tail_file_length -= filled_length;

                if tail_file_length == 0 {
                    tail_index -= 2;

                    tail_file_id = tail_index / 2;
                    tail_file_length = disk_map[tail_index];
                }
            }

            head_index += 1;
        }
    }

    if head_index == tail_index {
        filesystem_checksum +=
            tail_file_id * tail_file_length * (2 * block_count + tail_file_length - 1) / 2;
    }

    filesystem_checksum
}

#[derive(Debug)]
struct File {
    id: usize,
    address: usize,
    length: usize,
    adjacent_free_space: usize,
}

#[aoc(day9, part2)]
fn part2(disk_map: &[usize]) -> usize {
    let mut layout: Vec<File> = Vec::new();
    let mut block_count = 0;

    for (index, &length) in disk_map.iter().enumerate() {
        if index % 2 == 0 {
            layout.push(File {
                id: index / 2,
                address: block_count,
                length,
                adjacent_free_space: 0,
            });

            block_count += length;
        } else {
            let last_file = layout.last_mut().unwrap();
            last_file.adjacent_free_space = length;

            block_count += length;
        }
    }

    let mut moved_files: Vec<File> = Vec::with_capacity(layout.len());

    let mut max_length = None;
    let mut last_processed_id = layout.last().unwrap().id;

    'next_file: while let Some(mut file) = layout.pop() {
        if file.id > last_processed_id || max_length.is_some() && file.length >= max_length.unwrap()
        {
            moved_files.push(file);
            continue;
        }

        for i in 0..layout.len() {
            if layout[i].adjacent_free_space >= file.length {
                file.adjacent_free_space = layout[i].adjacent_free_space - file.length;
                file.address = layout[i].address + layout[i].length;
                layout[i].adjacent_free_space = 0;

                last_processed_id = file.id;

                layout.insert(i + 1, file);
                continue 'next_file;
            }
        }

        max_length = Some(file.length).into_iter().chain(max_length).min();
        moved_files.push(file);
    }

    moved_files
        .iter()
        .map(|file| file.id * (2 * file.address + file.length - 1) * file.length / 2)
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
