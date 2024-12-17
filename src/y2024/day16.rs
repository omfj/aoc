use crate::utils::AdventDay;
use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
};

#[derive(PartialEq, Clone, Copy)]
enum Tile {
    Wall,
    Empty,
    Reindeer,
    Start,
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
enum Direction {
    Vertical,
    Horizontal,
}

#[derive(PartialEq, Eq, Clone)]
struct HeapItem {
    point: Point,
    score: usize,
    path: Vec<Point>,
    prev_direction: Direction,
}

impl Ord for HeapItem {
    fn cmp(&self, other: &Self) -> Ordering {
        other.score.cmp(&self.score)
    }
}

impl PartialOrd for HeapItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

type Point = (usize, usize);
type Map = Vec<Vec<Tile>>;

pub struct Day16 {
    input: String,
}

impl AdventDay for Day16 {
    fn new(input: String) -> Self {
        Self { input }
    }

    fn part_one(&self) -> String {
        let map = parse_input(&self.input);
        let start = find_start(&map);
        let reindeer = find_reindeer(&map);
        let best_paths = dfs_score(&map, start, reindeer);
        let (score, _) = best_paths.first().unwrap();

        score.to_string()
    }

    fn part_two(&self) -> String {
        let map = parse_input(&self.input);
        let start = find_start(&map);
        let reindeer = find_reindeer(&map);
        let best_paths = dfs_score(&map, start, reindeer);
        let mut sitting_points = HashSet::new();
        for (_, path) in best_paths {
            for point in path {
                sitting_points.insert(point);
            }
        }

        sitting_points.len().to_string()
    }
}

fn find_reindeer(map: &Map) -> Point {
    for (y, row) in map.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            if *tile == Tile::Reindeer {
                return (x, y);
            }
        }
    }
    panic!("Reindeer not found");
}

fn find_start(map: &Map) -> Point {
    for (y, row) in map.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            if *tile == Tile::Start {
                return (x, y);
            }
        }
    }
    panic!("Start not found");
}

fn dfs_score(map: &Map, start: Point, end: Point) -> Vec<(usize, Vec<Point>)> {
    let mut best_paths = Vec::new();
    let mut visited = HashSet::new();
    let mut best_score = usize::MAX;

    let mut heap = BinaryHeap::new();
    heap.push(HeapItem {
        point: start,
        score: 0,
        path: vec![start].into_iter().collect(),
        prev_direction: Direction::Horizontal,
    });

    // let mut visited: HashMap<(Point, Point), u32> = HashMap::new();

    // let mut covered_tiles: HashSet<Point> = HashSet::new();

    // let mut min_score = u32::MAX;
    // bfs_queue.push_back(Tile {
    //     point: starting_point,
    //     direction: RIGHT,
    //     score: 0,
    //     previous_points: vec![starting_point],
    // });

    while let Some(state) = heap.pop() {
        let HeapItem {
            point,
            score,
            path,
            prev_direction,
        } = state;

        if point == end {
            if score < best_score {
                best_score = score;
                best_paths.clear();
                best_paths.push((score, path.clone()));
            } else if score == best_score {
                best_paths.push((score, path.clone()));
            }
            continue;
        }

        if !visited.insert((point, prev_direction)) {
            continue;
        }

        let neighbors = [
            (point.0 + 1, point.1),             // Right
            (point.0.wrapping_sub(1), point.1), // Left
            (point.0, point.1 + 1),             // Down
            (point.0, point.1.wrapping_sub(1)), // Up
        ];

        for neighbor in neighbors {
            if !is_valid_point(map, neighbor) {
                continue;
            }

            let current_direction = determine_direction(point, neighbor);
            let mut new_score = score + 1;

            if prev_direction != current_direction {
                new_score += 1000;
            }

            let mut new_path = path.clone();
            new_path.insert(0, neighbor);

            heap.push(HeapItem {
                point: neighbor,
                score: new_score,
                path: new_path,
                prev_direction: current_direction,
            });
        }
    }

    best_paths
}

fn is_valid_point(map: &Map, point: Point) -> bool {
    let (x, y) = point;
    y < map.len() && x < map[0].len() && map[y][x] != Tile::Wall
}

fn determine_direction(prev: Point, current: Point) -> Direction {
    if prev.0 == current.0 {
        Direction::Vertical
    } else {
        Direction::Horizontal
    }
}

fn parse_input(input: &str) -> Map {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '#' => Tile::Wall,
                    '.' => Tile::Empty,
                    'E' => Tile::Reindeer,
                    'S' => Tile::Start,
                    _ => panic!("Invalid tile"),
                })
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA_1: &str = r#"###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############"#;

    const DATA_2: &str = r#"#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################"#;

    #[test]
    fn part_one_example_one() {
        let day16 = Day16::new(DATA_1.to_string());
        assert_eq!(day16.part_one(), "7036");
    }

    #[test]
    fn part_one_example_two() {
        let day16 = Day16::new(DATA_2.to_string());
        assert_eq!(day16.part_one(), "11048");
    }

    #[test]
    #[ignore = "not done"]
    fn part_two_example_one() {
        let day16 = Day16::new(DATA_1.to_string());
        assert_eq!(day16.part_two(), "45");
    }

    #[test]
    #[ignore = "not done"]
    fn part_two_example_two() {
        let day16 = Day16::new(DATA_2.to_string());
        assert_eq!(day16.part_two(), "64");
    }
}
