use crate::AdventDay;

fn count_data(s: &str) -> i32 {
    let mut count = 0;
    let mut chars = s.chars();

    while let Some(c) = chars.next() {
        if c == '\\' {
            match chars.next() {
                Some('x') => {
                    chars.next();
                    chars.next();
                }
                Some('\\') | Some('"') => {}
                _ => panic!("Invalid escape sequence"),
            }
        }

        count += 1;
    }

    count
}

pub struct Day08 {
    input: String,
}

impl AdventDay for Day08 {
    fn new(input: String) -> Self {
        Self { input }
    }

    fn part_one(&self) -> String {
        self.input
            .lines()
            .map(|line| {
                let code = line.len() as i32;
                let data = count_data(line);

                code - data
            })
            .sum::<i32>()
            .to_string()
    }

    fn part_two(&self) -> String {
        // Solve part 2 here
        "".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = r#"""
"abc"
"aaa\"aaa"
"\x27"
"\x5em\"squulpy""#;

    #[test]
    #[ignore = "TODO: Implement me!"]
    fn part_one() {
        let day08 = Day08::new(DATA.to_string());
        assert_eq!(day08.part_one(), "17");
    }

    #[test]
    #[ignore = "TODO: Implement me!"]
    fn part_two() {
        let day08 = Day08::new("".to_string());
        assert_eq!(day08.part_two(), "");
    }
}
