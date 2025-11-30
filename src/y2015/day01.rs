use crate::AdventDay;

pub struct Day01 {
    input: String,
}

impl AdventDay for Day01 {
    fn new(input: String) -> Self {
        Self { input }
    }

    fn part_one(&self) -> String {
        let mut floor = 0;

        for c in self.input.chars() {
            match c {
                '(' => floor += 1,
                ')' => floor -= 1,
                _ => (),
            }
        }

        floor.to_string()
    }

    fn part_two(&self) -> String {
        let mut floor = 0;

        for (i, c) in self.input.chars().enumerate() {
            match c {
                '(' => floor += 1,
                ')' => floor -= 1,
                _ => (),
            }

            if floor < 0 {
                return (i + 1).to_string();
            }
        }

        panic!("No solution found!");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {
        let day01 = Day01::new("(())".to_string());
        assert_eq!(day01.part_one(), "0");

        let day01 = Day01::new("()()".to_string());
        assert_eq!(day01.part_one(), "0");

        let day01 = Day01::new("(((".to_string());
        assert_eq!(day01.part_one(), "3");
    }

    #[test]
    fn part_two() {
        let day01 = Day01::new("()())".to_string());
        assert_eq!(day01.part_two(), "5");
    }
}
