use std::collections::{HashSet, VecDeque};

use crate::utils::AdventDay;

type Grid<'a> = [[&'a str; 71]; 71];

fn find_shortest_path(grid: &Grid, is_test: bool) -> u32 {
    let start = (0, 0);
    let end = if is_test { (6, 6) } else { (70, 70) };
    let height_width = if is_test { 7 } else { 71 };

    let mut visited: HashSet<(u32, u32)> = HashSet::new();
    let mut queue: VecDeque<((u32, u32), u32)> = VecDeque::new();
    queue.push_back((start, 0));

    let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    let mut steps = u32::MAX;

    while let Some(((x, y), step_count)) = queue.pop_front() {
        if (x, y) == end {
            steps = step_count;
            break;
        }

        if visited.contains(&(x, y)) {
            continue;
        }

        visited.insert((x, y));

        for &(dx, dy) in &directions {
            let new_x = x as i32 + dx;
            let new_y = y as i32 + dy;

            if new_x >= 0
                && new_x < height_width as i32
                && new_y >= 0
                && new_y < height_width as i32
                && grid[new_y as usize][new_x as usize] != "#"
            {
                queue.push_back(((new_x as u32, new_y as u32), step_count + 1));
            }
        }
    }

    steps
}

pub struct Day18 {
    input: String,
}

impl AdventDay for Day18 {
    fn new(input: String) -> Self {
        Self { input }
    }

    fn part_one(&self) -> String {
        let corupted_bytes: Vec<(u32, u32)> = self
            .input
            .lines()
            .map(|line| {
                let (x, y) = line.split_once(",").unwrap();
                (x.parse::<u32>().unwrap(), y.parse::<u32>().unwrap())
            })
            .collect();

        let is_test = corupted_bytes.len() == 25;

        let mut grid: Grid = [["."; 71]; 71];

        let bytes_fallen = if is_test { 12 } else { 1024 };
        for &(x, y) in corupted_bytes[0..bytes_fallen].iter() {
            grid[y as usize][x as usize] = "#";
        }

        find_shortest_path(&grid, is_test).to_string()
    }

    fn part_two(&self) -> String {
        let corupted_bytes: Vec<(u32, u32)> = self
            .input
            .lines()
            .map(|line| {
                let (x, y) = line.split_once(",").unwrap();
                (x.parse::<u32>().unwrap(), y.parse::<u32>().unwrap())
            })
            .collect();

        let is_test = corupted_bytes.len() == 25;

        let mut grid: Grid = [["."; 71]; 71];

        let mut bytes_fallen = if is_test { 12 } else { 1024 };

        for &(x, y) in corupted_bytes[0..bytes_fallen].iter() {
            grid[y as usize][x as usize] = "#";
        }

        let mut steps = find_shortest_path(&grid, is_test);
        let mut prevented_grid = (0, 0);

        while steps != u32::MAX {
            if bytes_fallen < corupted_bytes.len() {
                let (x, y) = corupted_bytes[bytes_fallen];
                bytes_fallen += 1;
                grid[y as usize][x as usize] = "#";
                steps = find_shortest_path(&grid, is_test);
                prevented_grid = (x, y);
            } else {
                break;
            }
        }

        format!("{},{}", prevented_grid.0, prevented_grid.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = r#"5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0"#;

    #[test]
    fn part_one() {
        let day18 = Day18::new(DATA.to_string());
        assert_eq!(day18.part_one(), "22");
    }

    #[test]
    fn part_two() {
        let day18 = Day18::new(DATA.to_string());
        assert_eq!(day18.part_two(), "6,1");
    }
}
