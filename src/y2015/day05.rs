use itertools::Itertools;

use crate::AdventDay;

const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
const NAUGHTY_STRINGS: [&str; 4] = ["ab", "cd", "pq", "xy"];

fn count_vowels(str: &str) -> usize {
    str.chars().filter(|c| VOWELS.contains(c)).count()
}

fn has_double_letter(str: &str) -> bool {
    str.chars().tuple_windows().any(|(a, b)| a == b)
}

fn has_naughty_string(str: &str) -> bool {
    NAUGHTY_STRINGS.iter().any(|&s| str.contains(s))
}

fn is_nice_string(str: &str) -> bool {
    count_vowels(str) >= 3 && has_double_letter(str) && !has_naughty_string(str)
}

fn has_repeating_pair(str: &str) -> bool {
    str.chars().tuple_windows().enumerate().any(|(i, (a, b))| {
        str.chars()
            .skip(i + 2)
            .tuple_windows()
            .any(|(c, d)| a == c && b == d)
    })
}

fn repeat_with_gap(str: &str) -> bool {
    str.chars().tuple_windows().any(|(a, _, c)| a == c)
}

fn is_nice_string_2(str: &str) -> bool {
    has_repeating_pair(str) && repeat_with_gap(str)
}

pub struct Day05 {
    input: String,
}

impl AdventDay for Day05 {
    fn new(input: String) -> Self {
        Self { input }
    }

    fn part_one(&self) -> String {
        self.input
            .lines()
            .filter(|&s| is_nice_string(s))
            .count()
            .to_string()
    }

    fn part_two(&self) -> String {
        self.input
            .lines()
            .filter(|&s| is_nice_string_2(s))
            .count()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = "ugknbfddgicrmopn";

    #[test]
    fn part_one() {
        let day05 = Day05::new(DATA.to_string());
        assert_eq!(day05.part_one(), "1");
    }

    #[test]
    fn part_two() {
        let day05 = Day05::new(DATA.to_string());
        assert_eq!(day05.part_two(), "0");
    }
}
