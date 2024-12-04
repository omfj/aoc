use crate::utils::AdventDay;

pub struct Day04 {
    input: String,
}

impl AdventDay for Day04 {
    fn new(input: String) -> Self {
        Self { input }
    }

    fn part_one(&self) -> String {
        let matrix: Vec<Vec<char>> = self
            .input
            .lines()
            .map(|line| line.chars().collect())
            .collect();

        let mut count = 0;

        for (i, row) in matrix.iter().enumerate() {
            for (j, _) in row.iter().enumerate() {
                // Check forward
                if j + 3 < row.len() {
                    if row[j] == 'X' && row[j + 1] == 'M' && row[j + 2] == 'A' && row[j + 3] == 'S'
                    {
                        count += 1;
                    }
                }

                // Check down
                if i + 3 < matrix.len() {
                    if matrix[i][j] == 'X'
                        && matrix[i + 1][j] == 'M'
                        && matrix[i + 2][j] == 'A'
                        && matrix[i + 3][j] == 'S'
                    {
                        count += 1;
                    }
                }

                // Check up
                if i >= 3 {
                    if matrix[i][j] == 'X'
                        && matrix[i - 1][j] == 'M'
                        && matrix[i - 2][j] == 'A'
                        && matrix[i - 3][j] == 'S'
                    {
                        count += 1;
                    }
                }

                // Check backward
                if j >= 3 {
                    if row[j] == 'X' && row[j - 1] == 'M' && row[j - 2] == 'A' && row[j - 3] == 'S'
                    {
                        count += 1;
                    }
                }

                // Check diagonal down-right
                if i + 3 < matrix.len() && j + 3 < row.len() {
                    if matrix[i][j] == 'X'
                        && matrix[i + 1][j + 1] == 'M'
                        && matrix[i + 2][j + 2] == 'A'
                        && matrix[i + 3][j + 3] == 'S'
                    {
                        count += 1;
                    }
                }

                // Check diagonal down-left
                if i + 3 < matrix.len() && j >= 3 {
                    if matrix[i][j] == 'X'
                        && matrix[i + 1][j - 1] == 'M'
                        && matrix[i + 2][j - 2] == 'A'
                        && matrix[i + 3][j - 3] == 'S'
                    {
                        count += 1;
                    }
                }

                // Check diagonal up-right
                if i >= 3 && j + 3 < row.len() {
                    if matrix[i][j] == 'X'
                        && matrix[i - 1][j + 1] == 'M'
                        && matrix[i - 2][j + 2] == 'A'
                        && matrix[i - 3][j + 3] == 'S'
                    {
                        count += 1;
                    }
                }

                // Check diagonal up-left
                if i >= 3 && j >= 3 {
                    if matrix[i][j] == 'X'
                        && matrix[i - 1][j - 1] == 'M'
                        && matrix[i - 2][j - 2] == 'A'
                        && matrix[i - 3][j - 3] == 'S'
                    {
                        count += 1;
                    }
                }
            }
        }

        count.to_string()
    }

    fn part_two(&self) -> String {
        let matrix: Vec<Vec<char>> = self
            .input
            .lines()
            .map(|line| line.chars().collect())
            .collect();

        let mut count = 0;

        for (i, row) in matrix.iter().enumerate() {
            for (j, &c) in row.iter().enumerate() {
                if c != 'A' {
                    continue;
                }

                let is_within = i > 0 && j > 0 && i + 1 < matrix.len() && j + 1 < row.len();
                if !is_within {
                    continue;
                }

                let a = format!("{}{}", matrix[i + 1][j + 1], matrix[i - 1][j - 1]);
                let b = format!("{}{}", matrix[i - 1][j + 1], matrix[i + 1][j - 1]);

                let a_is_x_mas = matches!(a.as_str(), "MS" | "SM");
                let b_is_x_mas = matches!(b.as_str(), "MS" | "SM");

                if a_is_x_mas && b_is_x_mas {
                    count += 1;
                }
            }
        }

        count.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SIMPLE_DATA: &str = r#"..X...
.SAMX.
.A..A.
XMAS.S
.X...."#;

    const DATA: &str = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;

    #[test]
    fn part_one() {
        let day04 = Day04::new(DATA.to_string());
        let day04_simple = Day04::new(SIMPLE_DATA.to_string());
        assert_eq!(day04_simple.part_one(), "4");
        assert_eq!(day04.part_one(), "18");
    }

    #[test]
    fn part_two() {
        let day04 = Day04::new(DATA.to_string());
        assert_eq!(day04.part_two(), "9");
    }
}
