use crate::AdventDay;

pub struct Day03 {
    input: String,
}

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect::<Vec<u8>>()
        })
        .collect()
}

impl AdventDay for Day03 {
    fn new(input: String) -> Self {
        Self { input }
    }

    fn part_one(&self) -> String {
        let banks = parse_input(&self.input);

        let mut sum = 0;

        for bank in banks.iter() {
            let (start_idx, start) = bank[..bank.len() - 1]
                .iter()
                .enumerate()
                .max_by_key(|&(_, val)| val)
                .expect("No best start found");

            let end = bank[start_idx + 1..]
                .iter()
                .max()
                .expect("No best end found");

            let num_str = format!("{}{}", start, end);
            let max: usize = num_str.parse().unwrap_or(0);

            sum += max;
        }

        sum.to_string()
    }

    fn part_two(&self) -> String {
        let size = 12;
        let banks = parse_input(&self.input);

        let mut sum = 0;

        for bank in banks.iter() {
            let mut selected = Vec::new();
            let mut start = 0;

            for rem in (1..=size).rev() {
                let mut best_idx = start;
                let mut curr = bank[start];

                // Decide window size
                let end = bank.len() - rem + 1;

                // Search in window for best digit
                for (i, num) in bank.iter().enumerate().take(end).skip(start) {
                    if *num > curr {
                        curr = *num;
                        best_idx = i;
                    }
                }

                selected.push(curr);
                start = best_idx + 1; // Move start to one past previous best
            }

            let num_str: String = selected.iter().map(|d| d.to_string()).collect();
            let num: usize = num_str.parse().unwrap_or(0);
            sum += num;
        }

        sum.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = r#"987654321111111
811111111111119
234234234234278
818181911112111"#;

    #[test]
    fn part_one() {
        let day03 = Day03::new(DATA.to_string());
        assert_eq!(day03.part_one(), "357");
    }

    #[test]
    fn part_two() {
        let day03 = Day03::new(DATA.to_string());
        assert_eq!(day03.part_two(), "3121910778619");
    }
}
