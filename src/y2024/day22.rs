use crate::utils::AdventDay;
use itertools::Itertools;
use std::collections::HashMap;

const MODULO: i64 = 16_777_216;

pub struct Day22 {
    input: String,
}

fn step(number: i64) -> i64 {
    let mut num = number;
    num = (num ^ (num * 64)) % MODULO;
    num = (num ^ (num / 32)) % MODULO;
    num = (num ^ (num * 2048)) % MODULO;
    num
}

fn n_steps(number: i64, n: usize) -> i64 {
    (0..n).fold(number, |num, _| step(num))
}

fn parse_input(input: &str) -> Vec<i64> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn generate_history(input: &[i64]) -> Vec<Vec<i32>> {
    // input
    //     .iter()
    //     .map(|&num| {
    //         (0..2000)
    //             .scan(num, |state, _| {
    //                 *state = step(*state);
    //                 Some((*state % 10) as i32)
    //             })
    //             .collect()
    //     })
    //     .collect()
    input
        .iter()
        .map(|&num| {
            (0..2000)
                .scan(num, |state, _| {
                    *state = step(*state);
                    Some((*state % 10) as i32)
                })
                .collect()
        })
        .collect()
}

fn find_best_sequence(history: &[Vec<i32>]) -> i32 {
    let mut best: HashMap<(i32, i32, i32, i32), i32> = HashMap::new();

    for changes in history {
        let mut seen = HashMap::new();

        for (&a, &b, &c, &d, &e) in changes.iter().tuple_windows() {
            let key = (b - a, c - b, d - c, e - d);

            seen.entry(key).or_insert_with(|| {
                *best.entry(key).or_insert(0) += e;
                e
            });
        }
    }

    *best.values().max().unwrap_or(&0)
}

impl AdventDay for Day22 {
    fn new(input: String) -> Self {
        Self { input }
    }

    fn part_one(&self) -> String {
        let sum = parse_input(&self.input)
            .iter()
            .map(|&secret| n_steps(secret, 2000))
            .sum::<i64>();

        sum.to_string()
    }

    fn part_two(&self) -> String {
        let input = parse_input(&self.input);
        let history = generate_history(&input);
        let best_value = find_best_sequence(&history);

        best_value.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA_1: &str = r#"1
10
100
2024"#;

    const DATA_2: &str = r#"1
2
3
2024"#;

    #[test]
    fn part_one() {
        let day22 = Day22::new(DATA_1.to_string());
        assert_eq!(day22.part_one(), "37327623");
    }

    #[test]
    fn part_two() {
        let day22 = Day22::new(DATA_2.to_string());
        assert_eq!(day22.part_two(), "23");
    }
}
