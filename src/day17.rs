use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone)]
struct Memory {
    register_a: u64,
    register_b: u64,
    register_c: u64,
}

fn combo_operand(memory: &Memory, operand: u64) -> u64 {
    match operand {
        literal if literal <= 3 => literal,
        4 => memory.register_a,
        5 => memory.register_b,
        6 => memory.register_c,
        _ => unreachable!(),
    }
}

fn run_program(memory: &mut Memory, program: &Vec<u64>) -> Vec<u64> {
    let mut instruction_pointer = 0;
    let mut output = Vec::new();

    while instruction_pointer < program.len() - 1 {
        let opcode = program[instruction_pointer];
        let operand = program[instruction_pointer + 1];

        match opcode {
            0 => {
                memory.register_a >>= combo_operand(&memory, operand);
                instruction_pointer += 2;
            }
            1 => {
                memory.register_b ^= operand;
                instruction_pointer += 2;
            }
            2 => {
                memory.register_b = combo_operand(&memory, operand) & 7;
                instruction_pointer += 2;
            }
            3 => {
                if memory.register_a != 0 {
                    instruction_pointer = operand as usize;
                } else {
                    instruction_pointer += 2;
                }
            }
            4 => {
                memory.register_b ^= memory.register_c;
                instruction_pointer += 2;
            }
            5 => {
                output.push(combo_operand(&memory, operand) & 7);
                instruction_pointer += 2;
            }
            6 => {
                memory.register_b = memory.register_a >> combo_operand(&memory, operand);
                instruction_pointer += 2;
            }
            7 => {
                memory.register_c = memory.register_a >> combo_operand(&memory, operand);
                instruction_pointer += 2;
            }
            _ => unreachable!(),
        }
    }

    output
}

fn print_output(output: &[u64]) -> String {
    let mut printed_output = String::new();

    if !output.is_empty() {
        printed_output.push_str(&output.first().unwrap().to_string());
    }

    output.iter().skip(1).for_each(|number| {
        printed_output.push(',');
        printed_output.push_str(&(*number).to_string());
    });

    printed_output
}

#[aoc_generator(day17)]
fn parse_input(input: &str) -> (Memory, Vec<u64>) {
    use aoc_parse::{parser, prelude::*};

    let parser = parser!(
        registers:section(
            line("Register A: " u64)
            line("Register B: " u64)
            line("Register C: " u64)
        )
        program:section(
            line("Program: " repeat_sep(u64, ","))
        ) => (
            Memory {
                register_a: registers.0,
                register_b: registers.1,
                register_c: registers.2,
            },
            program
        )
    );

    parser.parse(input).unwrap()
}

#[aoc(day17, part1)]
fn part1((memory, program): &(Memory, Vec<u64>)) -> String {
    print_output(&run_program(&mut memory.clone(), program))
}

#[aoc(day17, part2)]
fn part2((memory, program): &(Memory, Vec<u64>)) -> u64 {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT_1: &str = r"Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

    static TEST_INPUT_2: &str = r"Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";

    #[test]
    fn part1_program_example_1() {
        let mut memory = Memory {
            register_a: 0,
            register_b: 0,
            register_c: 9,
        };

        run_program(&mut memory, &vec![2, 6]);

        assert_eq!(memory.register_b, 1);
    }

    #[test]
    fn part1_program_example_2() {
        let mut memory = Memory {
            register_a: 10,
            register_b: 0,
            register_c: 0,
        };

        assert_eq!(
            print_output(&run_program(&mut memory, &vec![5, 0, 5, 1, 5, 4])),
            "0,1,2"
        );
    }

    #[test]
    fn part1_program_example_3() {
        let mut memory = Memory {
            register_a: 2_024,
            register_b: 0,
            register_c: 0,
        };

        let output = run_program(&mut memory, &vec![0, 1, 5, 4, 3, 0]);

        assert_eq!(print_output(&output), "4,2,5,6,7,7,7,7,3,1,0");
        assert_eq!(memory.register_a, 0);
    }

    #[test]
    fn part1_program_example_4() {
        let mut memory = Memory {
            register_a: 0,
            register_b: 29,
            register_c: 0,
        };

        run_program(&mut memory, &vec![1, 7]);

        assert_eq!(memory.register_b, 26);
    }

    #[test]
    fn part1_program_example_5() {
        let mut memory = Memory {
            register_a: 0,
            register_b: 2_024,
            register_c: 43_690,
        };

        run_program(&mut memory, &vec![4, 0]);

        assert_eq!(memory.register_b, 44_354);
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT_1)), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(TEST_INPUT_2)), 117_440);
    }
}
