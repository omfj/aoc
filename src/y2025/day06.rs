use itertools::Itertools;

use crate::AdventDay;

pub struct Day06 {
    input: String,
}

impl AdventDay for Day06 {
    fn new(input: String) -> Self {
        Self { input }
    }

    fn part_one(&self) -> String {
        let lines = self.input.lines().collect_vec();
        let num_rows = lines.len() - 1; // last line is operators

        let rows: Vec<Vec<usize>> = lines[..num_rows]
            .iter()
            .map(|line| {
                line.split_whitespace()
                    .map(|s| s.parse().unwrap())
                    .collect_vec()
            })
            .collect_vec();

        let operators: Vec<&str> = lines[num_rows].split_whitespace().collect_vec();

        let mut sum: usize = 0;
        for col in 0..operators.len() {
            let col_nums: Vec<usize> = rows.iter().map(|row| row[col]).collect_vec();

            match operators[col] {
                "*" => sum += col_nums.iter().product::<usize>(),
                "+" => sum += col_nums.iter().sum::<usize>(),
                _ => unreachable!(),
            }
        }
        sum.to_string()
    }

    fn part_two(&self) -> String {
        let lines: Vec<&str> = self.input.lines().collect();
        let data_lines = &lines[..lines.len() - 1];
        let op_line = lines[lines.len() - 1];
        let max_len = lines.iter().map(|l| l.len()).max().unwrap_or(0);

        let mut is_separator = vec![true; max_len];
        for line in data_lines {
            for (i, c) in line.chars().enumerate() {
                if c != ' ' {
                    is_separator[i] = false;
                }
            }
        }

        let mut groups: Vec<(usize, usize)> = Vec::new();
        let mut start = None;
        for (i, &col) in is_separator.iter().enumerate().take(max_len) {
            if !col {
                if start.is_none() {
                    start = Some(i);
                }
            } else if let Some(s) = start {
                groups.push((s, i));
                start = None;
            }
        }
        if let Some(s) = start {
            groups.push((s, max_len));
        }

        let mut total = 0;
        let ops: Vec<char> = op_line.chars().collect();

        for (start, end) in groups {
            let op = ops[start..end.min(ops.len())]
                .iter()
                .find(|&&c| c == '*' || c == '+')
                .unwrap_or(&'+');

            let mut nums = Vec::new();
            for col in start..end {
                let digits: String = data_lines
                    .iter()
                    .filter_map(|line| line.chars().nth(col))
                    .filter(|c| c.is_ascii_digit())
                    .collect();
                if !digits.is_empty() {
                    nums.push(digits.parse().unwrap());
                }
            }

            match op {
                '*' => total += nums.iter().product::<usize>(),
                '+' => total += nums.iter().sum::<usize>(),
                _ => unreachable!(),
            }
        }

        total.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = r#"123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +
"#;

    #[test]
    fn part_one() {
        let day06 = Day06::new(DATA.to_string());
        assert_eq!(day06.part_one(), "4277556");
    }

    #[test]
    fn part_two() {
        let day06 = Day06::new(DATA.to_string());
        assert_eq!(day06.part_two(), "3263827");
    }
}
