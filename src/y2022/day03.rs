use std::collections::HashSet;

use itertools::Itertools;

use crate::utils::AdventDay;

const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

pub struct Day03 {
    input: String,
}

impl AdventDay for Day03 {
    fn new(input: String) -> Self {
        Self { input }
    }

    fn part_one(&self) -> String {
        self.input
            .lines()
            .map(|line| {
                let (a, b) = line.split_at(line.len() / 2);
                let a: HashSet<char> = a.chars().collect();
                let b: HashSet<char> = b.chars().collect();
                let common = a.intersection(&b).next().unwrap();
                ALPHABET.chars().position(|c| c == *common).unwrap() as i32 + 1
            })
            .sum::<i32>()
            .to_string()
    }

    fn part_two(&self) -> String {
        self.input
            .lines()
            .chunks(3)
            .into_iter()
            .map(|chunk| {
                let [a, b, c] = chunk.collect::<Vec<_>>().try_into().unwrap();
                let a: HashSet<char> = a.chars().collect();
                let b: HashSet<char> = b.chars().collect();
                let c: HashSet<char> = c.chars().collect();
                let common = a
                    .intersection(&b)
                    .cloned()
                    .collect::<HashSet<_>>()
                    .intersection(&c)
                    .cloned()
                    .next()
                    .unwrap();
                ALPHABET.chars().position(|c| c == common).unwrap() as i32 + 1
            })
            .sum::<i32>()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"#;

    #[test]
    fn part_one() {
        let day03 = Day03::new(DATA.to_string());
        assert_eq!(day03.part_one(), "157");
    }

    #[test]
    fn part_two() {
        let day03 = Day03::new(DATA.to_string());
        assert_eq!(day03.part_two(), "70");
    }
}
