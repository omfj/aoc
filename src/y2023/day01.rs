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
        self.input
            .lines()
            .map(|line| {
                let nums = line.chars().filter(|c| c.is_digit(10)).collect_vec();
                let first = nums.first().unwrap().to_digit(10).unwrap();
                let last = nums.last().unwrap().to_digit(10).unwrap();
                format!("{}{}", first, last).parse::<i32>().unwrap()
            })
            .sum::<i32>()
            .to_string()
    }

    fn part_two(&self) -> String {
        let numbers = [
            ("one", "one1one"),
            ("two", "two2two"),
            ("three", "three3three"),
            ("four", "four4four"),
            ("five", "five5five"),
            ("six", "six6six"),
            ("seven", "seven7seven"),
            ("eight", "eight8eight"),
            ("nine", "nine9nine"),
        ];

        self.input
            .lines()
            .map(|line| {
                let new_line = numbers
                    .iter()
                    .fold(line.to_string(), |acc, (word, num)| acc.replace(word, num));
                let nums = new_line.chars().filter(|c| c.is_digit(10)).collect_vec();
                let first = nums.first().unwrap().to_digit(10).unwrap();
                let last = nums.last().unwrap().to_digit(10).unwrap();
                println!("{}", new_line);
                format!("{}{}", first, last).parse::<i32>().unwrap()
            })
            .sum::<i32>()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA_1: &str = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#;

    const DATA_2: &str = r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"#;

    #[test]
    fn part_one() {
        let day01 = Day01::new(DATA_1.to_string());
        assert_eq!(day01.part_one(), "142");
    }

    #[test]
    fn part_two() {
        let day01 = Day01::new(DATA_2.to_string());
        assert_eq!(day01.part_two(), "281");
    }
}
