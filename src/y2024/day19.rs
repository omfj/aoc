use crate::utils::AdventDay;
use std::collections::HashMap;

pub struct Day19 {
    input: String,
}

impl AdventDay for Day19 {
    fn new(input: String) -> Self {
        Self { input }
    }

    fn part_one(&self) -> String {
        let (towels, desired_patterns) = parse_data(&self.input);
        let mut count = 0;
        let mut cache = HashMap::new();

        for pattern in desired_patterns {
            let valid_patterns = count_valid_patterns(&mut cache, pattern.as_bytes(), &towels);
            if valid_patterns > 0 {
                count += 1;
            }
        }

        count.to_string()
    }

    fn part_two(&self) -> String {
        let (towels, desired_patterns) = parse_data(&self.input);
        let mut count = 0;
        let mut cache = HashMap::new();

        for pattern in desired_patterns {
            let valid_patterns = count_valid_patterns(&mut cache, pattern.as_bytes(), &towels);
            count += valid_patterns;
        }

        count.to_string()
    }
}

fn count_valid_patterns<'a>(
    cache: &mut HashMap<&'a [u8], usize>,
    s: &'a [u8],
    towels: &[&'a [u8]],
) -> usize {
    if s.is_empty() {
        return 1;
    }

    if let Some(&ans) = cache.get(&s) {
        return ans;
    }

    let mut num = 0;
    for t in towels.iter() {
        if s.starts_with(t) {
            num += count_valid_patterns(cache, &s[t.len()..], towels);
        }
    }

    cache.insert(s, num);

    num
}

fn parse_data(input: &str) -> (Vec<&[u8]>, Vec<&str>) {
    let (towels, desired_patterns) = input.split_once("\n\n").unwrap();
    let towels = towels.split(", ").map(|towel| towel.as_bytes()).collect();
    let desired_patterns = desired_patterns.lines().collect();

    (towels, desired_patterns)
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = r#"r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb"#;

    #[test]
    fn part_one() {
        let day19 = Day19::new(DATA.to_string());
        assert_eq!(day19.part_one(), "6");
    }

    #[test]
    fn part_two() {
        let day19 = Day19::new(DATA.to_string());
        assert_eq!(day19.part_two(), "16");
    }
}
