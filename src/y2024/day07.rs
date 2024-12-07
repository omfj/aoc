use itertools::Itertools;

use crate::utils::AdventDay;

#[derive(Clone, Copy, Debug)]
enum Operator {
    Add,
    Multiply,
    Concatenation,
}

pub struct Day07 {
    input: String,
}

fn parse_input(input: &str) -> Vec<(u64, Vec<u64>)> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(": ");
            let id = parts.next().unwrap().parse().unwrap();
            let values = parts
                .next()
                .unwrap()
                .split(' ')
                .map(|v| v.parse().unwrap())
                .collect();
            (id, values)
        })
        .collect()
}

fn is_valid_line(result: u64, values: &[u64], operators: &[Operator]) -> bool {
    let operator_combinations = (0..values.len() - 1)
        .map(|_| operators.iter())
        .multi_cartesian_product();

    for operators in operator_combinations {
        let mut current_result = values[0];
        for (i, &value) in values.iter().skip(1).enumerate() {
            match operators[i] {
                Operator::Add => current_result += value,
                Operator::Multiply => current_result *= value,
                Operator::Concatenation => {
                    current_result = format!("{}{}", current_result, value).parse().unwrap()
                }
            }
        }

        if current_result == result {
            return true;
        }
    }

    false
}

fn sum_valid_lines(data: &[(u64, Vec<u64>)], operators: &[Operator]) -> u64 {
    data.iter()
        .filter(|(result, values)| is_valid_line(*result, values, operators))
        .map(|(result, _)| result)
        .sum()
}

impl AdventDay for Day07 {
    fn new(input: String) -> Self {
        Self { input }
    }

    fn part_one(&self) -> String {
        let data = parse_input(&self.input);
        let operators = vec![Operator::Add, Operator::Multiply];
        sum_valid_lines(&data, &operators).to_string()
    }

    fn part_two(&self) -> String {
        let data = parse_input(&self.input);
        let operators = vec![Operator::Add, Operator::Multiply, Operator::Concatenation];
        sum_valid_lines(&data, &operators).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"#;

    #[test]
    fn part_one() {
        let day07 = Day07::new(DATA.to_string());
        assert_eq!(day07.part_one(), "3749");
    }

    #[test]
    fn part_two() {
        let day07 = Day07::new(DATA.to_string());
        assert_eq!(day07.part_two(), "11387");
    }
}
