use std::collections::HashSet;

use crate::AdventDay;

pub struct Day03 {
    input: String,
}

impl AdventDay for Day03 {
    fn new(input: String) -> Self {
        Self { input }
    }

    fn part_one(&self) -> String {
        let mut coords = HashSet::new();
        let mut current = (0, 0);

        coords.insert(current);

        for c in self.input.chars() {
            match c {
                '^' => current.1 += 1,
                'v' => current.1 -= 1,
                '<' => current.0 -= 1,
                '>' => current.0 += 1,
                _ => panic!("Invalid character"),
            };

            coords.insert(current);
        }

        coords.len().to_string()
    }

    fn part_two(&self) -> String {
        let mut coords = HashSet::new();
        let mut santa = (0, 0);
        let mut robo_santa = (0, 0);

        coords.insert(santa);

        for (i, c) in self.input.chars().enumerate() {
            let current = if i % 2 == 0 {
                &mut santa
            } else {
                &mut robo_santa
            };

            match c {
                '^' => current.1 += 1,
                'v' => current.1 -= 1,
                '<' => current.0 -= 1,
                '>' => current.0 += 1,
                _ => panic!("Invalid character"),
            };

            coords.insert(*current);
        }

        coords.len().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {
        let day03 = Day03::new(">".to_string());
        assert_eq!(day03.part_one(), "2");

        let day03 = Day03::new("^>v<".to_string());
        assert_eq!(day03.part_one(), "4");

        let day03 = Day03::new("^v^v^v^v^v".to_string());
        assert_eq!(day03.part_one(), "2");
    }

    #[test]
    fn part_two() {
        let day03 = Day03::new("^v".to_string());
        assert_eq!(day03.part_two(), "3");
    }
}
