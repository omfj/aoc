use std::vec;

use itertools::Itertools;

use crate::utils::AdventDay;

type Program = Vec<Instruction>;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Instruction {
    ADV(Operand),
    BXL(Operand),
    BST(Operand),
    JNZ(Operand),
    BXC(Operand),
    OUT(Operand),
    BDV(Operand),
    CDV(Operand),
}

impl Instruction {
    fn _tuple(&self) -> (u32, u32) {
        match self {
            Self::ADV(operand) => (0, operand.literal()),
            Self::BXL(operand) => (1, operand.literal()),
            Self::BST(operand) => (2, operand.literal()),
            Self::JNZ(operand) => (3, operand.literal()),
            Self::BXC(operand) => (4, operand.literal()),
            Self::OUT(operand) => (5, operand.literal()),
            Self::BDV(operand) => (6, operand.literal()),
            Self::CDV(operand) => (7, operand.literal()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Operand {
    Value(u32),
    RegisterA,
    RegisterB,
    RegisterC,
}

impl Operand {
    fn from(value: u32) -> Self {
        match value {
            0..=3 => Self::Value(value),
            4 => Self::RegisterA,
            5 => Self::RegisterB,
            6 => Self::RegisterC,
            _ => panic!("Panicing... Unknown operand: {}", value),
        }
    }

    fn literal(&self) -> u32 {
        match self {
            Self::Value(value) => *value,
            Self::RegisterA => 4,
            Self::RegisterB => 5,
            Self::RegisterC => 6,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Computer {
    a: u32,
    b: u32,
    c: u32,
}

impl Computer {
    fn new(a: Option<u32>, b: Option<u32>, c: Option<u32>) -> Self {
        let a = a.unwrap_or(0);
        let b = b.unwrap_or(0);
        let c = c.unwrap_or(0);

        Self { a, b, c }
    }

    fn run(&mut self, program: &Program) -> Vec<u32> {
        let mut output: Vec<u32> = Vec::new();
        let mut pointer = 0;

        while pointer < program.len() {
            let instruction = program[pointer];
            let maybe_jump = self.step(&mut output, instruction);

            if let Some(jump) = maybe_jump {
                pointer = jump as usize
            } else {
                pointer += 1;
            }
        }

        output
    }

    fn step(&mut self, output: &mut Vec<u32>, instruction: Instruction) -> Option<u32> {
        if let Instruction::JNZ(operand) = instruction {
            if self.a != 0 {
                return Some(operand.literal());
            }
        }

        match instruction {
            Instruction::ADV(operand) => self.a /= 2u32.pow(self.get_operand(operand)),
            Instruction::BXL(operand) => self.b ^= operand.literal(),
            Instruction::BST(operand) => self.b = self.get_operand(operand) % 8,
            Instruction::BXC(_) => self.b ^= self.c,
            Instruction::OUT(operand) => output.push(self.get_operand(operand) % 8),
            Instruction::BDV(operand) => self.b = self.a / 2u32.pow(self.get_operand(operand)),
            Instruction::CDV(operand) => self.c = self.a / 2u32.pow(self.get_operand(operand)),
            _ => {}
        }

        None
    }

    fn get_operand(&self, operand: Operand) -> u32 {
        match operand {
            Operand::Value(value) => value,
            Operand::RegisterA => self.a,
            Operand::RegisterB => self.b,
            Operand::RegisterC => self.c,
        }
    }
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

        let mut computer = Computer::new(Some(a), None, None);
        let output = computer.run(&program);

        output
            .iter()
            .map(|num| num.to_string())
            .collect::<Vec<String>>()
            .join(",")
    }

    fn part_two(&self) -> String {
        let (_, _) = parse_data(&self.input);
        let a = 0;

        a.to_string()
    }
}

fn parse_data(input: &str) -> (u32, Program) {
    let (registers, program) = input.split_once("\n\n").unwrap();
    let registers = parse_registers(registers);
    let program = parse_instructions(program);

    (registers, program)
}

fn parse_registers(register: &str) -> u32 {
    let mut a = 0;

    register.lines().for_each(|line| {
        let (register, value) = line.split_once(": ").unwrap();
        let value = value.parse::<u32>().unwrap();
        match register {
            "Register A" => a = value,
            _ => {}
        }
    });

    a
}

fn parse_instructions(program: &str) -> Program {
    let mut instructions = Vec::new();
    let program_instructions = program.split_once(": ").unwrap().1.split(',').tuples();

    for (instr, operand) in program_instructions {
        let operand = operand.parse::<u32>().unwrap();

        let instr = match instr {
            "0" => Instruction::ADV(Operand::from(operand)),
            "1" => Instruction::BXL(Operand::from(operand)),
            "2" => Instruction::BST(Operand::from(operand)),
            "3" => Instruction::JNZ(Operand::from(operand)),
            "4" => Instruction::BXC(Operand::from(operand)),
            "5" => Instruction::OUT(Operand::from(operand)),
            "6" => Instruction::BDV(Operand::from(operand)),
            "7" => Instruction::CDV(Operand::from(operand)),
            _ => panic!("Panicing... Unknown instruction: {}", instr),
        };

        instructions.push(instr);
    }

    instructions
}

fn _get_bytes(instructions: Vec<Instruction>) -> Vec<u32> {
    instructions
        .iter()
        .map(|instr| instr._tuple())
        .map(|(a, b)| vec![a, b])
        .flatten()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = r#"Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0"#;

    #[test]
    fn part_one() {
        let day17 = Day17::new(DATA.to_string());
        assert_eq!(day17.part_one(), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    #[ignore = "not implemented"]
    fn part_two() {
        let day17 = Day17::new(DATA.to_string());
        assert_eq!(day17.part_two(), "117440");
    }
}
