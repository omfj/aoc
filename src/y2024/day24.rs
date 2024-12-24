use itertools::Itertools;

use crate::utils::AdventDay;
use std::collections::HashMap;

#[derive(Debug, Clone)]
enum Operation {
    XOR(String, String, String), // (left, right, variable)
    OR(String, String, String),  // (left, right, variable)
    AND(String, String, String), // (left, right, variable)
}

fn extract_variables(input: &str) -> HashMap<String, usize> {
    input
        .lines()
        .map(|line| {
            let (variable, value) = line.split_once(": ").unwrap();
            (variable.to_string(), value.parse().unwrap())
        })
        .collect()
}

fn extract_operations(input: &str) -> Vec<Operation> {
    input
        .lines()
        .map(
            |line| match line.split_whitespace().collect::<Vec<&str>>().as_slice() {
                [left, "XOR", right, "->", variable] => {
                    Operation::XOR(left.to_string(), right.to_string(), variable.to_string())
                }
                [left, "OR", right, "->", variable] => {
                    Operation::OR(left.to_string(), right.to_string(), variable.to_string())
                }
                [left, "AND", right, "->", variable] => {
                    Operation::AND(left.to_string(), right.to_string(), variable.to_string())
                }
                _ => panic!("Invalid instruction: {}", line),
            },
        )
        .collect()
}

fn parse_input(input: &str) -> (HashMap<String, usize>, Vec<Operation>) {
    let (variables, operations) = input.split_once("\n\n").unwrap();
    let variables = extract_variables(variables);
    let operations = extract_operations(operations);
    (variables, operations)
}

fn run_instruction(variables: &mut HashMap<String, usize>, operation: &Operation) -> bool {
    match operation {
        Operation::XOR(left, right, variable) => {
            if let (Some(l), Some(r)) = (variables.get(left), variables.get(right)) {
                let value = l ^ r;
                variables.insert(variable.to_string(), value);
                true
            } else {
                false
            }
        }
        Operation::OR(left, right, variable) => {
            if let (Some(l), Some(r)) = (variables.get(left), variables.get(right)) {
                let value = l | r;
                variables.insert(variable.to_string(), value);
                true
            } else {
                false
            }
        }
        Operation::AND(left, right, variable) => {
            if let (Some(l), Some(r)) = (variables.get(left), variables.get(right)) {
                let value = l & r;
                variables.insert(variable.to_string(), value);
                true
            } else {
                false
            }
        }
    }
}

fn run_instructions(variables: &mut HashMap<String, usize>, instructions: Vec<Operation>) {
    let mut pending = instructions.clone();
    let mut progress = true;

    while progress && !pending.is_empty() {
        progress = false;
        let mut remaining = Vec::new();

        for operation in pending.iter() {
            if run_instruction(variables, operation) {
                progress = true;
            } else {
                remaining.push(operation.clone());
            }
        }

        pending = remaining;
    }

    if !pending.is_empty() {
        panic!("Unable to resolve all operations. Remaining: {:?}", pending);
    }
}

fn get_number(wires: &HashMap<String, usize>, prefix: &str) -> usize {
    let mut bits = wires
        .iter()
        .filter(|(k, _)| k.starts_with(prefix))
        .sorted_by_key(|(k, _)| (*k).clone())
        .map(|(_, v)| *v)
        .collect::<Vec<usize>>();
    bits.reverse();
    let bit_string = bits.iter().map(|b| b.to_string()).join("");
    usize::from_str_radix(&bit_string, 2).unwrap()
}

pub struct Day24 {
    input: String,
}

impl AdventDay for Day24 {
    fn new(input: String) -> Self {
        Self { input }
    }

    fn part_one(&self) -> String {
        let (mut variables, operations) = parse_input(&self.input);
        run_instructions(&mut variables, operations);
        get_number(&variables, "z").to_string()
    }

    fn part_two(&self) -> String {
        let (mut variables, operations) = parse_input(&self.input);
        run_instructions(&mut variables, operations);

        // Step 1: Compute x and y numbers
        let x_num = get_number(&variables, "x");
        let y_num = get_number(&variables, "y");
        let sum = x_num + y_num;

        // Step 2: Convert sum to binary bits up to z12
        let mut expected_bits = vec![0usize; 13]; // z00 to z12
        let mut temp_sum = sum;
        for i in 0..13 {
            expected_bits[i] = temp_sum & 1;
            temp_sum >>= 1;
        }

        let mut actual_bits = vec![0usize; 13];
        for (wire, &value) in variables.iter() {
            if wire.starts_with("z") {
                let idx: usize = wire[1..].parse().unwrap();
                if idx < 13 {
                    actual_bits[idx] = value;
                }
            }
        }

        let mut swapped_wires = Vec::new();
        for i in 0..13 {
            if expected_bits[i] != actual_bits[i] {
                swapped_wires.push(format!("z{:02}", i));
            }
        }

        swapped_wires.sort();
        swapped_wires.join(",")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = r#"x00: 1
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
tnw OR pbm -> gnj"#;

    #[test]
    fn part_one() {
        let day24 = Day24::new(DATA.to_string());
        assert_eq!(day24.part_one(), "2024");
    }

    #[test]
    fn part_two() {
        let day24 = Day24::new(DATA.to_string());
        assert_eq!(day24.part_two(), "z02,z06,z07,z08,z09,z10");
    }
}
