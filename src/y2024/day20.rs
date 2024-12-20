use crate::utils::AdventDay;
use itertools::Itertools;
use std::collections::{HashMap, VecDeque};

type Grid = Vec<Vec<char>>;
type Point = (usize, usize);

fn find_start_end(grid: &Grid) -> (Point, Point) {
    let mut start = None;
    let mut end = None;

    for (y, line) in grid.iter().enumerate() {
        for (x, tile) in line.iter().enumerate() {
            match tile {
                'S' => start = Some((x, y)),
                'E' => end = Some((x, y)),
                _ => {}
            }
        }
    }

    match (start, end) {
        (Some(start), Some(end)) => (start, end),
        _ => panic!("Start and end not found"),
    }
}

fn parse_input(input: &str) -> Grid {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn collect_distances(grid: &Grid) -> HashMap<Point, usize> {
    let ((sx, sy), end) = find_start_end(grid);

    let height = grid.len();
    let width = grid[0].len();

    let mut queue = VecDeque::from([(sx, sy, 0)]);
    let mut distances = HashMap::new();

    while let Some((x, y, n)) = queue.pop_front() {
        if distances.contains_key(&(x, y)) {
            continue;
        }

        distances.insert((x, y), n);

        if (x, y) == end {
            continue;
        }

        let neighbours = [
            (x.wrapping_sub(1), y),
            (x.wrapping_add(1), y),
            (x, y.wrapping_sub(1)),
            (x, y.wrapping_add(1)),
        ];

        for (x, y) in neighbours {
            if x >= width || y >= height {
                continue;
            }

            if grid[y][x] == '#' {
                continue;
            }

            queue.push_back((x, y, n + 1));
        }
    }

    distances
}

fn count_possible_cheats(
    distances: &HashMap<Point, usize>,
    max_cheats: usize,
    min_picoseconds: usize,
) -> usize {
    distances
        .iter()
        .tuple_combinations()
        .filter(|((&a, &an), (&b, &bn))| {
            let (ax, ay) = a;
            let (bx, by) = b;

            let dist = ax.abs_diff(bx) + ay.abs_diff(by);

            if dist > max_cheats {
                return false;
            }

            let picoseconds_saved = bn.abs_diff(an) - dist;

            picoseconds_saved >= min_picoseconds
        })
        .count()
}
pub struct Day20 {
    input: String,
}

impl AdventDay for Day20 {
    fn new(input: String) -> Self {
        Self { input }
    }

    fn part_one(&self) -> String {
        let grid = parse_input(&self.input);
        let is_test = grid.len() == 15;
        let picoseconds = if is_test { 40 } else { 100 };

        let distances = collect_distances(&grid);
        count_possible_cheats(&distances, 2, picoseconds).to_string()
    }

    fn part_two(&self) -> String {
        let grid = parse_input(&self.input);
        let is_test = grid.len() == 15;
        let picoseconds = if is_test { 76 } else { 100 };

        let distances = collect_distances(&grid);
        count_possible_cheats(&distances, 20, picoseconds).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = r#"###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############"#;

    #[test]
    fn part_one() {
        let day20 = Day20::new(DATA.to_string());
        assert_eq!(day20.part_one(), "2");
    }

    #[test]
    fn part_two() {
        let day20 = Day20::new(DATA.to_string());
        assert_eq!(day20.part_two(), "3");
    }
}
