use crate::utils::AdventDay;
use core::panic;
use std::{cmp::Reverse, collections::BinaryHeap};

fn run(program: &[usize], a: usize, b: usize, c: usize) -> Vec<usize> {
    let mut a = a;
    let mut b = b;
    let mut c = c;

    let mut pointer = 0;
    let mut output: Vec<usize> = Vec::new();

    while pointer < program.len() {
        let opcode = program[pointer];
        let operand = program[pointer + 1];

        if let Some(jumo) = step(&mut output, &mut a, &mut b, &mut c, opcode, operand) {
            pointer = jumo;
        } else {
            pointer += 2;
        }
    }

    output
}

fn step(
    output: &mut Vec<usize>,
    a: &mut usize,
    b: &mut usize,
    c: &mut usize,
    opcode: usize,
    operand: usize,
) -> Option<usize> {
    let get_operand = |operand| match operand {
        0..=3 => operand,
        4 => *a,
        5 => *b,
        6 => *c,
        _ => panic!("Invalid operand"),
    };

    match opcode {
        0 => *a /= 2usize.pow(get_operand(operand) as u32),
        1 => *b ^= operand,
        2 => *b = get_operand(operand) % 8,
        3 => {
            if *a != 0 {
                return Some(operand);
            }
        }
        4 => *b ^= *c,
        5 => output.push(get_operand(operand) % 8),
        6 => *b = *a / 2usize.pow(get_operand(operand) as u32),
        7 => *c = *a / 2usize.pow(get_operand(operand) as u32),
        _ => {}
    }

    None
}

pub struct Day17 {
    input: String,
}

impl AdventDay for Day17 {
    fn new(input: String) -> Self {
        Self { input }
    }

    fn part_one(&self) -> String {
        let (a, program) = parse_data(&self.input);

        let output = run(&program, a, 0, 0);

        output
            .iter()
            .map(|num| num.to_string())
            .collect::<Vec<String>>()
            .join(",")
    }

    fn part_two(&self) -> String {
        let (_, program) = parse_data(&self.input);

        let mut heap: BinaryHeap<Reverse<usize>> = BinaryHeap::new();

        for i in 1..8 {
            heap.push(Reverse(i));
        }

        while let Some(Reverse(a)) = heap.pop() {
            let output = run(&program, a, 0, 0);

            if output == program {
                return a.to_string();
            }

            if output == program[program.len() - output.len()..] {
                for i in 0..8 {
                    heap.push(Reverse((a << 3) + i));
                }
            }
        }

        panic!("No solution found")
    }
}

fn parse_data(input: &str) -> (usize, Vec<usize>) {
    let (registers, program) = input.split_once("\n\n").unwrap();
    let registers = parse_registers(registers);
    let program = parse_instructions(program);

    (registers, program)
}

fn parse_registers(register: &str) -> usize {
    let mut a = 0;

    register.lines().for_each(|line| {
        let (register, value) = line.split_once(": ").unwrap();
        let value = value.parse::<usize>().unwrap();
        match register {
            "Register A" => a = value,
            _ => {}
        }
    });

    a
}

fn parse_instructions(program: &str) -> Vec<usize> {
    program
        .split_once(": ")
        .unwrap()
        .1
        .split(',')
        .map(|num| num.parse::<usize>().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA_1: &str = r#"Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0"#;

    const DATA_2: &str = r#"Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0"#;

    #[test]
    fn part_one() {
        let day17 = Day17::new(DATA_1.to_string());
        assert_eq!(day17.part_one(), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn part_two() {
        let day17 = Day17::new(DATA_2.to_string());
        assert_eq!(day17.part_two(), "117440");
    }
}
