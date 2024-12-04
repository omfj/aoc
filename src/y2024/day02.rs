use std::cmp::Ordering;

use itertools::Itertools;

use crate::utils::AdventDay;

pub struct Day02 {
    input: String,
}

fn is_safe_rate_of_change(vec: Vec<i32>) -> bool {
    let first = vec.first().expect("Expected element on index 0");
    let second = vec.get(1).expect("Expected element on index 1");
    let ordering = first.cmp(second);

    if ordering == Ordering::Equal {
        return false;
    }

    for (&i, &j) in vec.iter().tuple_windows() {
        if i.cmp(&j) != ordering {
            return false;
        }

        if (i - j).abs() > 3 {
            return false;
        }
    }

    true
}

fn vec_combinations(vec: Vec<i32>) -> Vec<Vec<i32>> {
    let mut combinations = Vec::new();

    combinations.push(vec.clone());

    for i in 0..vec.len() {
        let mut new_vec = vec.clone();
        new_vec.remove(i);
        combinations.push(new_vec);
    }

    combinations
}

impl AdventDay for Day02 {
    fn new(input: String) -> Self {
        Self { input }
    }

    fn part_one(&self) -> String {
        let reports: Vec<Vec<i32>> = self
            .input
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .map(|num| num.parse::<i32>().unwrap())
                    .collect()
            })
            .collect();

        let count = reports
            .iter()
            .filter(|report| is_safe_rate_of_change(report.to_vec()))
            .count();

        count.to_string()
    }

    fn part_two(&self) -> String {
        let reports: Vec<Vec<i32>> = self
            .input
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .map(|num| num.parse::<i32>().unwrap())
                    .collect()
            })
            .collect();

        let count = reports
            .iter()
            .filter(|report| {
                let combinations = vec_combinations(report.to_vec());
                combinations
                    .iter()
                    .any(|combination| is_safe_rate_of_change(combination.to_vec()))
            })
            .count();

        count.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;

    #[test]
    fn part_one() {
        let day02 = Day02::new(DATA.to_string());
        assert_eq!(day02.part_one(), "2");
    }

    #[test]
    fn part_two() {
        let day02 = Day02::new(DATA.to_string());
        assert_eq!(day02.part_two(), "4");
    }

    #[test]
    fn is_safe_rate_of_change_test() {
        assert!(
            is_safe_rate_of_change(vec![1, 2, 3, 4, 5]),
            "1, 2, 3, 4, 5"
        );
        assert!(is_safe_rate_of_change(vec![6, 4, 2, 1]), "6, 4, 2, 1");
        assert!(
            !is_safe_rate_of_change(vec![1, 2, 7, 8, 9]),
            "1, 2, 7, 8, 9"
        );
        assert!(
            !is_safe_rate_of_change(vec![9, 7, 6, 2, 1]),
            "9, 7, 6, 2, 1"
        );
        assert!(
            !is_safe_rate_of_change(vec![1, 3, 2, 4, 5]),
            "1, 3, 2, 4, 5"
        );
        assert!(
            !is_safe_rate_of_change(vec![8, 6, 4, 4, 1]),
            "8, 6, 4, 4, 1"
        );
    }
}
