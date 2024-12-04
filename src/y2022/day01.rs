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
            .split("\n\n")
            .map(|elf| {
                elf.lines()
                    .map(|line| line.parse::<i32>().unwrap())
                    .sum::<i32>()
            })
            .max()
            .unwrap()
            .to_string()
    }

    fn part_two(&self) -> String {
        self.input
            .split("\n\n")
            .map(|elf| {
                elf.lines()
                    .map(|line| line.parse::<i32>().unwrap())
                    .sum::<i32>()
            })
            .sorted()
            .rev()
            .take(3)
            .sum::<i32>()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = r#"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"#;

    #[test]
    fn part_one() {
        let day01 = Day01::new(DATA.to_string());
        assert_eq!(day01.part_one(), "24000");
    }

    #[test]
    fn part_two() {
        let day01 = Day01::new(DATA.to_string());
        assert_eq!(day01.part_two(), "45000");
    }
}
