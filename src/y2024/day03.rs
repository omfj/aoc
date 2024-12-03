use regex::Regex;

use crate::utils::AdventDay;

pub struct Day03 {
    input: String,
}

impl AdventDay for Day03 {
    fn new(input: String) -> Self {
        Self { input }
    }

    fn part_one(&self) -> String {
        let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

        let mut result = 0;

        for cap in re.captures_iter(&self.input) {
            let a: i32 = cap[1].parse().unwrap();
            let b: i32 = cap[2].parse().unwrap();
            result += a * b;
        }

        result.to_string()
    }

    fn part_two(&self) -> String {
        let re = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();

        let mut result = 0;
        let mut skip = false;

        for cap in re.captures_iter(&self.input) {
            match cap.get(0).map(|m| m.as_str()) {
                Some("do()") => {
                    skip = false;
                }
                Some("don't()") => {
                    skip = true;
                }
                Some(_) if !skip => {
                    let a: i32 = cap[1].parse().unwrap();
                    let b: i32 = cap[2].parse().unwrap();
                    result += a * b;
                }
                _ => {}
            }
        }

        result.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA_1: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const DATA_2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn part_one() {
        let day03 = Day03::new(DATA_1.to_string());
        assert_eq!(day03.part_one(), "161");
    }

    #[test]
    fn part_two() {
        let day03 = Day03::new(DATA_2.to_string());
        assert_eq!(day03.part_two(), "48");
    }
}
