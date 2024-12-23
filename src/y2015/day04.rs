use crate::utils::AdventDay;

fn find_hash(input: &str, prefix: &str) -> u64 {
    let mut i = 0;

    loop {
        let hash = format!("{:x}", md5::compute(format!("{}{}", input, i)));
        if hash.starts_with(prefix) {
            return i;
        }

        i += 1;
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
        find_hash(&self.input, "00000").to_string()
    }

    fn part_two(&self) -> String {
        find_hash(&self.input, "000000").to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {
        let day04 = Day04::new("abcdef".to_string());
        assert_eq!(day04.part_one(), "609043");
    }
}
