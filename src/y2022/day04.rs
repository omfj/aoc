use crate::utils::AdventDay;

struct Elf {
    min: i32,
    max: i32,
}

impl Elf {
    fn new(min: i32, max: i32) -> Self {
        Self { min, max }
    }

    fn contains(&self, other: &Self) -> bool {
        self.min <= other.min && other.max <= self.max
    }

    fn overlaps(&self, other: &Self) -> bool {
        self.min <= other.min && other.min <= self.max
    }
}

pub struct Day04 {
    input: String,
}

impl AdventDay for Day04 {
    fn new(input: String) -> Self {
        Self { input }
    }

    fn part_one(&self) -> String {
        self.input
            .lines()
            .map(|line| {
                let (a, b) = {
                    let mut parts = line.split(',').map(|pair| {
                        let mut nums = pair.split('-');
                        let min = nums.next().unwrap().parse::<i32>().unwrap();
                        let max = nums.next().unwrap().parse::<i32>().unwrap();

                        Elf::new(min, max)
                    });

                    (parts.next().unwrap(), parts.next().unwrap())
                };

                a.contains(&b) || b.contains(&a)
            })
            // Count all the true values
            .filter(|&x| x)
            .count()
            .to_string()
    }

    fn part_two(&self) -> String {
        self.input
            .lines()
            .map(|line| {
                let (a, b) = {
                    let mut parts = line.split(',').map(|pair| {
                        let mut nums = pair.split('-');
                        let min = nums.next().unwrap().parse::<i32>().unwrap();
                        let max = nums.next().unwrap().parse::<i32>().unwrap();

                        Elf::new(min, max)
                    });

                    (parts.next().unwrap(), parts.next().unwrap())
                };

                a.overlaps(&b) || b.overlaps(&a)
            })
            // Count all the true values
            .filter(|&x| x)
            .count()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = r#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"#;

    #[test]
    fn part_one() {
        let day04 = Day04::new(DATA.to_string());
        assert_eq!(day04.part_one(), "2");
    }

    #[test]
    fn part_two() {
        let day04 = Day04::new(DATA.to_string());
        assert_eq!(day04.part_two(), "4");
    }
}
