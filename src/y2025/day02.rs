use crate::AdventDay;

struct IdRange {
    start: usize,
    end: usize,
}

impl From<&str> for IdRange {
    fn from(s: &str) -> Self {
        let parts: Vec<&str> = s.split('-').collect();
        Self {
            start: parts[0].parse().unwrap(),
            end: parts[1].parse().unwrap(),
        }
    }
}

impl Iterator for IdRange {
    type Item = Id;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start > self.end {
            None
        } else {
            let id = Id(self.start);
            self.start += 1;
            Some(id)
        }
    }
}

struct Id(usize);

impl Id {
    #[inline]
    fn value(&self) -> usize {
        self.0
    }

    // An invalid ID has:
    // - An even number of digits
    // - The first half matches the second half
    fn is_invalid_part_one(&self) -> bool {
        let str = self.0.to_string();
        if !str.len().is_multiple_of(2) {
            return false;
        }
        str.split_at(str.len() / 2).0 == str.split_at(str.len() / 2).1
    }

    // An invalid ID has a repeating pattern (11, 121212, 333)
    fn is_invalid_part_two(&self) -> bool {
        let str = self.0.to_string();
        for len in 1..=str.len() / 2 {
            let pattern = &str[0..len];
            let mut repeated = String::new();
            for _ in 0..(str.len() / len) {
                repeated.push_str(pattern);
            }
            if repeated == str {
                return true;
            }
        }
        false
    }
}

fn parse_input(input: &str) -> Vec<IdRange> {
    input.trim().split(',').map(IdRange::from).collect()
}

pub struct Day02 {
    input: String,
}

impl AdventDay for Day02 {
    fn new(input: String) -> Self {
        Self { input }
    }

    fn part_one(&self) -> String {
        let ranges = parse_input(&self.input);
        let mut invalid_ids = Vec::new();

        for range in ranges {
            for id in range {
                if id.is_invalid_part_one() {
                    invalid_ids.push(id.value());
                }
            }
        }

        invalid_ids.iter().sum::<usize>().to_string()
    }

    fn part_two(&self) -> String {
        let ranges = parse_input(&self.input);
        let mut invalid_ids = Vec::new();

        for range in ranges {
            for id in range {
                if id.is_invalid_part_two() {
                    invalid_ids.push(id.value());
                }
            }
        }

        invalid_ids.iter().sum::<usize>().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = r#"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"#;

    #[test]
    fn part_one() {
        let day02 = Day02::new(DATA.to_string());
        assert_eq!(day02.part_one(), "1227775554");
    }

    #[test]
    fn part_two() {
        let day02 = Day02::new(DATA.to_string());
        assert_eq!(day02.part_two(), "4174379265");
    }

    #[test]
    fn is_invalid_part_one() {
        assert!(Id(1212).is_invalid_part_one());
        assert!(Id(3333).is_invalid_part_one());

        assert!(!Id(12321).is_invalid_part_one());
    }

    #[test]
    fn is_invalid_part_two() {
        assert!(Id(121212).is_invalid_part_two());
        assert!(Id(3333).is_invalid_part_two());
        assert!(Id(11).is_invalid_part_two());
    }
}
