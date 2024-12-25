use crate::utils::AdventDay;
use std::collections::HashMap;

#[derive(Debug, Clone)]
enum Operation {
    Not(String, String),            // (value, variable)
    Assign(String, String),         // (value, variable)
    And(String, String, String),    // (left, right, variable)
    Or(String, String, String),     // (left, right, variable)
    Rshift(String, String, String), // (value, shift, variable)
    Lshift(String, String, String), // (value, shift, variable)
}

struct Circuit {
    operations: Vec<Operation>,
    variables: HashMap<String, u16>,
}

impl Circuit {
    fn new(input: &str) -> Self {
        let operations = input.lines().map(parse_line).collect();
        Self {
            operations,
            variables: HashMap::new(),
        }
    }

    fn set_value(&mut self, variable: &str, value: u16) {
        self.variables.insert(variable.to_string(), value);
    }

    fn get_value(&self, value: &str) -> Option<u16> {
        if let Some(value) = self.variables.get(value) {
            return Some(*value);
        }

        if let Ok(value) = value.parse::<u16>() {
            return Some(value);
        }

        None
    }

    fn run_all(&mut self) {
        let mut pending = self.operations.clone();
        let mut progress = true;

        while progress && !pending.is_empty() {
            progress = false;
            let mut remaining = Vec::new();

            for operation in pending.iter() {
                if self.run_operation(operation) {
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

    fn run_operation(&mut self, operation: &Operation) -> bool {
        match operation {
            Operation::Assign(value, variable) => {
                if let Some(val) = self.get_value(value) {
                    self.variables.insert(variable.clone(), val);
                    return true;
                }
                false
            }
            Operation::Not(value, variable) => {
                if let Some(val) = self.get_value(value) {
                    self.variables.insert(variable.clone(), !val);
                    return true;
                }
                false
            }
            Operation::And(left, right, variable) => {
                if let (Some(l), Some(r)) = (self.get_value(left), self.get_value(right)) {
                    self.variables.insert(variable.clone(), l & r);
                    return true;
                }
                false
            }
            Operation::Or(left, right, variable) => {
                if let (Some(l), Some(r)) = (self.get_value(left), self.get_value(right)) {
                    self.variables.insert(variable.clone(), l | r);
                    return true;
                }
                false
            }
            Operation::Lshift(value, shift, variable) => {
                if let (Some(v), Some(s)) = (self.get_value(value), self.get_value(shift)) {
                    self.variables.insert(variable.clone(), v << s);
                    return true;
                }
                false
            }
            Operation::Rshift(value, shift, variable) => {
                if let (Some(v), Some(s)) = (self.get_value(value), self.get_value(shift)) {
                    self.variables.insert(variable.clone(), v >> s);
                    return true;
                }
                false
            }
        }
    }
}

fn parse_line(line: &str) -> Operation {
    let parts: Vec<&str> = line.split_whitespace().collect();
    match parts.len() {
        3 => Operation::Assign(parts[0].to_string(), parts[2].to_string()),
        4 => Operation::Not(parts[1].to_string(), parts[3].to_string()),
        5 => {
            let operation = parts[1];
            match operation {
                "AND" => Operation::And(
                    parts[0].to_string(),
                    parts[2].to_string(),
                    parts[4].to_string(),
                ),
                "OR" => Operation::Or(
                    parts[0].to_string(),
                    parts[2].to_string(),
                    parts[4].to_string(),
                ),
                "LSHIFT" => Operation::Lshift(
                    parts[0].to_string(),
                    parts[2].to_string(),
                    parts[4].to_string(),
                ),
                "RSHIFT" => Operation::Rshift(
                    parts[0].to_string(),
                    parts[2].to_string(),
                    parts[4].to_string(),
                ),
                _ => panic!("Invalid input"),
            }
        }
        _ => panic!("Invalid input"),
    }
}

pub struct Day07 {
    input: String,
}

impl AdventDay for Day07 {
    fn new(input: String) -> Self {
        Self { input }
    }

    fn part_one(&self) -> String {
        let mut circuit = Circuit::new(&self.input);
        circuit.run_all();
        circuit.get_value("a").unwrap().to_string()
    }

    fn part_two(&self) -> String {
        let mut circuit = Circuit::new(&self.input);
        circuit.run_all();
        let a_signal = circuit.get_value("a").unwrap();

        // Filter out line that assigns b to a value
        let filtered_operations: Vec<Operation> = self
            .input
            .lines()
            .filter(|line| {
                if let Some(variable) = line.split("->").nth(1) {
                    return variable.trim() != "b";
                }
                true
            })
            .map(parse_line)
            .collect();

        let mut new_circuit = Circuit {
            operations: filtered_operations,
            variables: HashMap::new(),
        };

        new_circuit.set_value("b", a_signal);
        new_circuit.run_all();
        new_circuit.get_value("a").unwrap().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = r#"123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> a"#;

    #[test]
    fn part_one() {
        let day07 = Day07::new(DATA.to_string());
        assert_eq!(day07.part_one(), "65079");
    }

    #[test]
    fn part_two() {
        let day07 = Day07::new(DATA.to_string());
        assert_eq!(day07.part_two(), "65079");
    }
}
