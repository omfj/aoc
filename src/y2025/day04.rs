use crate::AdventDay;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Cell {
    Paper,
    Air,
}

impl From<char> for Cell {
    fn from(s: char) -> Self {
        match s {
            '.' => Cell::Air,
            '@' => Cell::Paper,
            _ => unreachable!(),
        }
    }
}

struct Grid {
    inner: Vec<Vec<Cell>>,
}

impl Grid {
    fn new(input: &str) -> Self {
        let inner: Vec<Vec<Cell>> = input
            .lines()
            .map(|line| line.chars().map(Cell::from).collect())
            .collect();

        Self { inner }
    }

    fn paper_around(&self, i: usize, j: usize) -> usize {
        let mut paper = Vec::new();

        let dirs = [
            (-1, 0),
            (1, 0),
            (0, -1),
            (0, 1),
            (-1, -1),
            (1, -1),
            (-1, 1),
            (1, 1),
        ];

        for (dx, dy) in dirs {
            let ni = i as isize + dx;
            let nj = j as isize + dy;

            if ni < 0
                || nj < 0
                || ni >= self.inner.len() as isize
                || nj >= self.inner[0].len() as isize
            {
                continue;
            }

            if self.inner[ni as usize][nj as usize] == Cell::Paper {
                paper.push((ni as usize, nj as usize));
            }
        }

        paper.len()
    }

    fn remove_paper(&mut self, i: usize, j: usize) {
        self.inner[i][j] = Cell::Air;
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
        let grid = Grid::new(&self.input);

        let mut accessed = 0;

        for i in 0..grid.inner.len() {
            for j in 0..grid.inner[0].len() {
                if grid.inner[i][j] == Cell::Paper && grid.paper_around(i, j) < 4 {
                    accessed += 1;
                }
            }
        }

        accessed.to_string()
    }

    fn part_two(&self) -> String {
        let mut grid = Grid::new(&self.input);
        let mut accessed = 0;
        let mut did_remove = true;

        while did_remove {
            did_remove = false;
            for i in 0..grid.inner.len() {
                for j in 0..grid.inner[0].len() {
                    if grid.inner[i][j] == Cell::Paper && grid.paper_around(i, j) < 4 {
                        accessed += 1;
                        grid.remove_paper(i, j);
                        did_remove = true;
                    }
                }
            }
        }

        accessed.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = r#"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."#;

    #[test]
    fn part_one() {
        let day04 = Day04::new(DATA.to_string());
        assert_eq!(day04.part_one(), "13");
    }

    #[test]
    fn part_two() {
        let day04 = Day04::new(DATA.to_string());
        assert_eq!(day04.part_two(), "43");
    }
}
