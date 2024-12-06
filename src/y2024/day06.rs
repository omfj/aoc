use crate::utils::AdventDay;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn next(&self) -> Self {
        match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }
}

struct Guard<'a> {
    map: &'a Vec<Vec<char>>,
    i: i32,
    j: i32,
    moves: Vec<(i32, i32)>,
    direction: Direction,
    done: bool,
}

impl<'a> Guard<'a> {
    fn new(map: &'a Vec<Vec<char>>) -> Self {
        let (i, j) = find_start(map);

        Self {
            map,
            i,
            j,
            direction: Direction::North,
            moves: vec![(i, j)],
            done: false,
        }
    }
}

impl Iterator for Guard<'_> {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        let (new_i, new_j) = match self.direction {
            Direction::North => (self.i - 1, self.j),
            Direction::East => (self.i, self.j + 1),
            Direction::South => (self.i + 1, self.j),
            Direction::West => (self.i, self.j - 1),
        };

        let is_outside = new_i < 0
            || new_j < 0
            || new_i >= self.map.len() as i32
            || new_j >= self.map[0].len() as i32;
        if is_outside {
            self.done = true;
            return None;
        }

        if self.map[new_i as usize][new_j as usize] == '#'
            || self.map[new_i as usize][new_j as usize] == 'O'
        {
            self.direction = self.direction.next();
            return self.next();
        }

        self.i = new_i;
        self.j = new_j;
        self.moves.push((self.i, self.j));

        Some((self.i, self.j))
    }
}

fn parse_map(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec()
}

fn find_start(map: &Vec<Vec<char>>) -> (i32, i32) {
    for (i, row) in map.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if *cell == '^' {
                return (i as i32, j as i32);
            }
        }
    }

    panic!("No start found");
}

pub struct Day06 {
    input: String,
}

impl AdventDay for Day06 {
    fn new(input: String) -> Self {
        Self { input }
    }

    fn part_one(&self) -> String {
        let map = parse_map(&self.input);
        let guard = Guard::new(&map);
        let moves: Vec<(i32, i32)> = guard.collect();
        let distinct_moves: HashSet<(i32, i32)> = moves.into_iter().collect();
        distinct_moves.len().to_string()
    }

    fn part_two(&self) -> String {
        let map = parse_map(&self.input);
        let guard = Guard::new(&map);
        let moves: Vec<(i32, i32)> = guard.collect();
        let distinct_moves: HashSet<(i32, i32)> = moves.into_iter().collect();
        let mut loops = 0;

        for (i, j) in distinct_moves {
            if map[i as usize][j as usize] == '^' {
                continue;
            }

            let mut map_with_o = map.clone();
            map_with_o[i as usize][j as usize] = 'O';
            let guard = Guard::new(&map_with_o);
            let mut visited: HashMap<(i32, i32), i32> = HashMap::new();

            for (k, l) in guard {
                if visited.contains_key(&(k, l)) {
                    let visited_count = visited[&(k, l)];
                    if visited_count > 3 {
                        loops += 1;
                        break;
                    }

                    visited.insert((k, l), visited_count + 1);
                } else {
                    visited.insert((k, l), 1);
                }
            }
        }

        loops.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#;

    #[test]
    fn part_one() {
        let day06 = Day06::new(DATA.to_string());
        assert_eq!(day06.part_one(), "41");
    }

    #[test]
    fn part_two() {
        let day06 = Day06::new(DATA.to_string());
        assert_eq!(day06.part_two(), "6");
    }
}
