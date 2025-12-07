use std::collections::{HashMap, HashSet, VecDeque};

use crate::AdventDay;

const SPLITTER: char = '^';
const START: char = 'S';

pub struct Day07 {
    input: String,
}

fn find_start(grid: &[Vec<char>]) -> (usize, usize) {
    for (i, row) in grid.iter().enumerate() {
        for (j, &c) in row.iter().enumerate() {
            if c == START {
                return (i, j);
            }
        }
    }
    panic!("Start position not found");
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

impl AdventDay for Day07 {
    fn new(input: String) -> Self {
        Self { input }
    }

    fn part_one(&self) -> String {
        let grid: Vec<Vec<char>> = parse_input(&self.input);
        let start = find_start(&grid);

        let mut queue = VecDeque::from([start]);
        let mut visited = HashSet::from([start]);

        while let Some((i, j)) = queue.pop_front() {
            let neighbors: Vec<(usize, usize)> = match grid[i][j] {
                SPLITTER => [(i, j.wrapping_sub(1)), (i, j + 1)].into(),
                _ => [(i + 1, j)].into(),
            };
            for pos in neighbors {
                if pos.0 < grid.len() && pos.1 < grid[0].len() && visited.insert(pos) {
                    queue.push_back(pos);
                }
            }
        }

        // Count splitters visited
        let count = visited
            .iter()
            .filter(|&&(i, j)| grid[i][j] == SPLITTER)
            .count();
        count.to_string()
    }

    fn part_two(&self) -> String {
        let grid: Vec<Vec<char>> = parse_input(&self.input);
        let start = find_start(&grid);

        fn count_paths(
            grid: &[Vec<char>],
            pos: (usize, usize),
            cache: &mut HashMap<(usize, usize), usize>,
        ) -> usize {
            let (i, j) = pos;
            if i >= grid.len() || j >= grid[0].len() {
                return 0;
            }

            if let Some(&cached) = cache.get(&pos) {
                return cached;
            }

            let count = match grid[i][j] {
                SPLITTER => {
                    let left = if j > 0 {
                        count_paths(grid, (i, j - 1), cache)
                    } else {
                        0
                    };
                    let right = count_paths(grid, (i, j + 1), cache);
                    left + right
                }
                _ => {
                    if i + 1 < grid.len() {
                        count_paths(grid, (i + 1, j), cache)
                    } else {
                        1
                    }
                }
            };

            cache.insert(pos, count);
            count
        }

        let mut cache = HashMap::new();

        let count = count_paths(&grid, start, &mut cache);
        count.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = r#".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
"#;

    #[test]
    fn part_one() {
        let day07 = Day07::new(DATA.to_string());
        assert_eq!(day07.part_one(), "21");
    }

    #[test]
    fn part_two() {
        let day07 = Day07::new(DATA.to_string());
        assert_eq!(day07.part_two(), "40");
    }
}
