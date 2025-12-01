use crate::AdventDay;

enum DialInput {
    Right(u32),
    Left(u32),
}

impl From<&str> for DialInput {
    fn from(s: &str) -> Self {
        let (dir, dist) = s.split_at(1);
        let distance: u32 = dist.parse().unwrap();
        match dir {
            "R" => DialInput::Right(distance),
            "L" => DialInput::Left(distance),
            _ => panic!("Invalid direction"),
        }
    }
}

pub struct Day01 {
    input: String,
}

impl AdventDay for Day01 {
    fn new(input: String) -> Self {
        Self { input }
    }

    // Count zeros landed on after each whole movement
    fn part_one(&self) -> String {
        let inputs = self.parse_input();

        let mut dial = 50;
        let mut zeros = 0;

        for inp in inputs {
            match inp {
                DialInput::Right(dist) => {
                    for _ in 0..dist {
                        dial = (dial + 1) % 100;
                    }
                }
                DialInput::Left(dist) => {
                    for _ in 0..dist {
                        dial = (dial + 99) % 100;
                    }
                }
            }

            if dial == 0 {
                zeros += 1;
            }
        }

        zeros.to_string()
    }

    // Count zeros crossed during the movement
    fn part_two(&self) -> String {
        let inputs = self.parse_input();

        let mut dial = 50;
        let mut zeros = 0;

        for inp in inputs {
            match inp {
                DialInput::Right(dist) => {
                    for _ in 0..dist {
                        dial = (dial + 1) % 100;

                        if dial == 0 {
                            zeros += 1;
                        }
                    }
                }
                DialInput::Left(dist) => {
                    for _ in 0..dist {
                        dial = (dial + 99) % 100;

                        if dial == 0 {
                            zeros += 1;
                        }
                    }
                }
            }
        }

        zeros.to_string()
    }
}

impl Day01 {
    fn parse_input(&self) -> Vec<DialInput> {
        self.input.split_whitespace().map(DialInput::from).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = r#"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"#;

    #[test]
    fn part_one() {
        let day01 = Day01::new(DATA.to_string());
        assert_eq!(day01.part_one(), "3");
    }

    #[test]
    fn part_two() {
        let day01 = Day01::new(DATA.to_string());
        assert_eq!(day01.part_two(), "6");
    }
}
