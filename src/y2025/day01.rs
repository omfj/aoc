use crate::AdventDay;

pub struct Day01 {
    input: String,
}

impl AdventDay for Day01 {
    fn new(input: String) -> Self {
        Self { input }
    }

    fn part_one(&self) -> String {
        todo!()
    }

    fn part_two(&self) -> String {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = r#""#;

    #[test]
    fn part_one() {
        let day01 = Day01::new(DATA.to_string());
        assert_eq!(day01.part_one(), "");
    }

    #[test]
    #[ignore]
    fn part_two() {
        let day01 = Day01::new(DATA.to_string());
        assert_eq!(day01.part_two(), "");
    }
}
