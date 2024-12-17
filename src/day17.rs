use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone)]
struct Computer {
    register_a: usize,
    register_b: usize,
    register_c: usize,
    program: Vec<usize>,
    instruction_pointer: usize,
    output: Vec<usize>,
}

impl Computer {
    fn run_program(&mut self) {
        while self.instruction_pointer < self.program.len() - 1 {
            self.run_instruction();
        }
    }

    fn print_output(self) -> String {
        let mut output = String::new();

        if !self.output.is_empty() {
            output.push_str(&self.output.first().unwrap().to_string());
        }

        self.output.iter().skip(1).for_each(|number| {
            output.push(',');
            output.push_str(&(*number).to_string());
        });

        output
    }

    fn run_instruction(&mut self) {
        let opcode = self.program[self.instruction_pointer];
        let operand = self.program[self.instruction_pointer + 1];

        match opcode {
            0 => {
                self.register_a >>= self.combo_operand(operand);
                self.instruction_pointer += 2;
            }
            1 => {
                self.register_b ^= operand;
                self.instruction_pointer += 2;
            }
            2 => {
                self.register_b = self.combo_operand(operand) % 8;
                self.instruction_pointer += 2;
            }
            3 => {
                if self.register_a != 0 {
                    self.instruction_pointer = operand;
                } else {
                    self.instruction_pointer += 2;
                }
            }
            4 => {
                self.register_b ^= self.register_c;
                self.instruction_pointer += 2;
            }
            5 => {
                self.output.push(self.combo_operand(operand) % 8);
                self.instruction_pointer += 2;
            }
            6 => {
                self.register_b = self.register_a >> self.combo_operand(operand);
                self.instruction_pointer += 2;
            }
            7 => {
                self.register_c = self.register_a >> self.combo_operand(operand);
                self.instruction_pointer += 2;
            }
            _ => unreachable!(),
        }
    }

    fn combo_operand(&self, operand: usize) -> usize {
        match operand {
            literal if literal <= 3 => literal,
            4 => self.register_a,
            5 => self.register_b,
            6 => self.register_c,
            _ => unreachable!(),
        }
    }
}

#[aoc_generator(day17)]
fn parse_input(input: &str) -> Computer {
    use aoc_parse::{parser, prelude::*};

    let parser = parser!(
        registers:section(
            line("Register A: " usize)
            line("Register B: " usize)
            line("Register C: " usize)
        )
        program:section(
            line("Program: " repeat_sep(usize, ","))
        ) => Computer {
            register_a: registers.0,
            register_b: registers.1,
            register_c: registers.2,
            program,
            instruction_pointer: 0,
            output: vec![],
        }
    );

    parser.parse(input).unwrap()
}

#[aoc(day17, part1)]
fn part1(computer: &Computer) -> String {
    let mut computer = computer.clone();
    computer.run_program();
    computer.print_output()
}

#[aoc(day17, part2)]
fn part2(computer: &Computer) -> usize {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(TEST_INPUT)), 117_440);
    }
}
