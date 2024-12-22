use crate::utils::AdventDay;

pub struct Day22 {
    input: String,
}

impl AdventDay for Day22 {
    fn new(input: String) -> Self {
        Self { input }
    }

    fn part_one(&self) -> String {
        // Solve part 1 here
        "".to_string()
    }

    fn part_two(&self) -> String {
        // Solve part 2 here
        "".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = "";

    #[test]
    fn part_one() {
        let day22 = Day22::new(DATA.to_string());
        assert_eq!(day22.part_one(), "");
    }

    #[test]
    #[ignore = "TODO: Implement me!"]
    fn part_two() {
        let day22 = Day22::new(DATA.to_string());
        assert_eq!(day22.part_two(), "");
    }
}
