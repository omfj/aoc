use itertools::Itertools;

use crate::utils::AdventDay;

pub struct Day01 {
    input: String,
}

impl AdventDay for Day01 {
    fn new(input: String) -> Self {
        Self { input }
    }

    fn part_one(&self) -> String {
        let tuples: Vec<(i32, i32)> = self
            .input
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(|line| {
                let parts: Vec<&str> = line.split_whitespace().collect();
                let a = parts[0].parse::<i32>().expect("Invalid integer");
                let b = parts[1].parse::<i32>().expect("Invalid integer");
                (a, b)
            })
            .collect();

        let left: Vec<i32> = tuples.iter().map(|(a, _)| *a).sorted().collect();
        let right: Vec<i32> = tuples.iter().map(|(_, b)| *b).sorted().collect();

        let total_diffs: i32 = left
            .iter()
            .zip(right.iter())
            .map(|(a, b)| (*a - *b).abs())
            .sum();

        total_diffs.to_string()
    }

    fn part_two(&self) -> String {
        let (left, right): (Vec<i32>, Vec<i32>) = self
            .input
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(|line| {
                let parts: Vec<&str> = line.split_whitespace().collect();
                let a = parts[0].parse::<i32>().expect("Invalid integer");
                let b = parts[1].parse::<i32>().expect("Invalid integer");
                (a, b)
            })
            .unzip();

        let mut total = 0;

        // For each number in the left list, check how many times it appears in the right list
        for a in left.iter() {
            let count = right.iter().filter(|b| **b == *a).count();
            total += a * count as i32;
        }

        total.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = r#"
3   4
4   3
2   5
1   3
3   9
3   3
        "#;

    #[test]
    fn part_one() {
        let day01 = Day01::new(DATA.to_string());
        assert_eq!(day01.part_one(), "11");
    }

    #[test]
    fn part_two() {
        let day01 = Day01::new(DATA.to_string());
        assert_eq!(day01.part_two(), "31");
    }
}
