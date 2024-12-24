use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
enum Operation {
    Xor,
    And,
    Or,
}

#[derive(Clone, Debug)]
struct Gate {
    input1: String,
    input2: String,
    output: String,
    operation: Operation,
}

#[aoc_generator(day24)]
fn parse_input(input: &str) -> (HashMap<String, u8>, Vec<Gate>) {
    use aoc_parse::{parser, prelude::*};
    use Operation::*;

    let initial_values = parser!(hash_map(lines(string(alnum+) ": " u8)));
    let operation = parser!({"AND" => And, "OR" => Or, "XOR" => Xor});
    let gates = parser!(lines(
        input1:string(alnum+) " "
        operation:operation " "
        input2:string(alnum+) " -> "
        output:string(alnum+) =>
            Gate {
                input1: String::min(input1.clone(), input2.clone()),
                input2: String::max(input1, input2),
                output,
                operation,
            }
    ));

    let parser = parser!(section(initial_values) section(gates));

    parser.parse(input).unwrap()
}

fn resolve_output_gate(
    output_gate: &str,
    gates_by_output: &HashMap<String, &Gate>,
    resolved_gates: &mut HashMap<String, u8>,
) -> u8 {
    use Operation::*;

    if let Some(value) = resolved_gates.get(output_gate) {
        return *value;
    }

    let gate = gates_by_output.get(output_gate).unwrap();

    let input1 = &gate.input1;
    let input2 = &gate.input2;
    let operation = &gate.operation;

    let input1 = resolve_output_gate(input1, gates_by_output, resolved_gates);
    let input2 = resolve_output_gate(input2, gates_by_output, resolved_gates);

    let result = match operation {
        And => input1 & input2,
        Or => input1 | input2,
        Xor => input1 ^ input2,
    };

    resolved_gates.insert(output_gate.to_string(), result);

    result
}

#[aoc(day24, part1)]
fn part1((initial_values, gates): &(HashMap<String, u8>, Vec<Gate>)) -> u64 {
    let gates_by_output = gates
        .iter()
        .map(|gate| (gate.output.clone(), gate))
        .collect::<HashMap<_, _>>();

    let mut resolved_gates = initial_values.clone();
    let mut result = 0;

    for z in 0u32..=99 {
        let final_gate = format!("z{:02}", z);

        if !gates_by_output.contains_key(&final_gate) {
            break;
        }

        result += 2u64.pow(z)
            * resolve_output_gate(&final_gate, &gates_by_output, &mut resolved_gates) as u64;
    }

    result
}

fn format_result(wires: &[String]) -> String {
    let mut sorted_wires = wires.to_owned();
    sorted_wires.sort_unstable();

    let mut output = String::new();

    if !sorted_wires.is_empty() {
        output.push_str(sorted_wires.first().unwrap());
    }

    sorted_wires.iter().skip(1).for_each(|computer_name| {
        output.push(',');
        output.push_str(computer_name);
    });

    output
}

#[aoc(day24, part2)]
fn part2((initial_values, gates): &(HashMap<String, u8>, Vec<Gate>)) -> String {
    use Operation::*;

    let mut result: Vec<String> = Vec::with_capacity(8);

    result.extend(gates.iter().filter_map(|gate| {
        if gate.operation == Xor
            && gate.input1.starts_with('x')
            && gate.input1 != "x00"
            && gate.output.starts_with('z')
        {
            Some(gate.output.clone())
        } else {
            None
        }
    }));

    result.extend(gates.iter().filter_map(|gate| {
        if gate.operation == Xor
            && gate.input1.starts_with('x')
            && gate.input1 != "x00"
            && gates.iter().any(|or_gate| {
                or_gate.operation == Or
                    && (or_gate.input1 == gate.output || or_gate.input2 == gate.output)
            })
        {
            Some(gate.output.clone())
        } else {
            None
        }
    }));

    result.extend(gates.iter().filter_map(|gate| {
        if gate.operation == Xor && !gate.input1.starts_with('x') && !gate.output.starts_with('z') {
            Some(gate.output.clone())
        } else {
            None
        }
    }));

    result.extend(gates.iter().filter_map(|gate| {
        if gate.operation == And
            && gate.input1.starts_with('x')
            && gate.input1 != "x00"
            && gates.iter().any(|and_or_xor_gate| {
                (and_or_xor_gate.operation == And || and_or_xor_gate.operation == Xor)
                    && (and_or_xor_gate.input1 == gate.output
                        || and_or_xor_gate.input2 == gate.output)
            })
        {
            Some(gate.output.clone())
        } else {
            None
        }
    }));

    result.extend(gates.iter().filter_map(|gate| {
        if gate.operation == And && gate.output.starts_with('z') {
            Some(gate.output.clone())
        } else {
            None
        }
    }));

    result.extend(gates.iter().filter_map(|gate| {
        if gate.operation == Or
            && (gate.input1.starts_with('x')
                || gate.input2.starts_with('y')
                || gate.output.starts_with('z')
                    && gate.output != format!("z{:02}", initial_values.len() / 2))
        {
            Some(gate.output.clone())
        } else {
            None
        }
    }));

    format_result(&result)
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT_1: &str = r"x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02";

    static TEST_INPUT_2: &str = r"x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj";

    #[test]
    fn part1_example_1() {
        assert_eq!(part1(&parse_input(TEST_INPUT_1)), 4);
    }

    #[test]
    fn part1_example_2() {
        assert_eq!(part1(&parse_input(TEST_INPUT_2)), 2_024);
    }
}
