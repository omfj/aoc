use crate::AdventDay;
use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
};

#[derive(PartialEq, Clone, Copy)]
enum Tile {
    Wall,
    Empty,
    Reindeer,
    Start,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn all() -> &'static [Direction] {
        &[
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ]
    }

    fn delta(self) -> (isize, isize) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct State {
    point: Point,
    direction: Direction,
}

#[derive(Clone, Eq, PartialEq)]
struct Node {
    cost: u32,
    state: State,
}

impl Ord for Node {
    fn cmp(&self, other: &Node) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Node) -> Option<Ordering> {
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

        let (score, _) = all_best_paths(&map, start, Direction::Right, reindeer);

        score.to_string()
    }

    fn part_two(&self) -> String {
        let map = parse_input(&self.input);
        let start = find_start(&map);
        let reindeer = find_reindeer(&map);

        let (_, best_paths) = all_best_paths(&map, start, Direction::Right, reindeer);
        let mut sitting_points = HashSet::new();
        for path in best_paths {
            for (point, _) in path {
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

fn is_valid_point(map: &Map, point: Point) -> bool {
    let (x, y) = point;
    y < map.len() && x < map[0].len() && map[y][x] != Tile::Wall
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

fn neighbors(map: &Map, (x, y): Point) -> Vec<(Point, Direction)> {
    let mut result = Vec::new();
    for &dir in Direction::all() {
        let (dx, dy) = dir.delta();
        let nx = x as isize + dx;
        let ny = y as isize + dy;
        if nx >= 0 && ny >= 0 {
            let nx = nx as usize;
            let ny = ny as usize;
            if is_valid_point(map, (nx, ny)) {
                result.push(((nx, ny), dir));
            }
        }
    }
    result
}

fn all_best_paths(
    map: &Map,
    start: Point,
    start_direction: Direction,
    end: Point,
) -> (u32, Vec<Vec<(Point, Direction)>>) {
    let mut dist: HashMap<State, u32> = HashMap::new();
    let mut heap: BinaryHeap<Node> = BinaryHeap::new();
    let mut predecessors: HashMap<State, Vec<State>> = HashMap::new();

    let start_state = State {
        point: start,
        direction: start_direction,
    };
    dist.insert(start_state, 0);
    heap.push(Node {
        cost: 0,
        state: start_state,
    });

    let mut minimal_end_cost: Option<u32> = None;
    let mut end_states = Vec::new();

    while let Some(Node { cost, state }) = heap.pop() {
        if let Some(&current_dist) = dist.get(&state) {
            if current_dist < cost {
                continue;
            }
        }

        if state.point == end {
            if minimal_end_cost.is_none() || Some(cost) < minimal_end_cost {
                minimal_end_cost = Some(cost);
                end_states.clear();
                end_states.push(state);
            } else if Some(cost) == minimal_end_cost {
                end_states.push(state);
            }
            continue;
        }

        for (npoint, ndir) in neighbors(map, state.point) {
            let step_cost = if ndir == state.direction { 1 } else { 1001 };
            let next_cost = cost + step_cost;
            let next_state = State {
                point: npoint,
                direction: ndir,
            };

            match dist.get(&next_state) {
                Some(&d) => match next_cost.cmp(&d) {
                    Ordering::Less => {
                        dist.insert(next_state, next_cost);
                        predecessors.insert(next_state, vec![state]);
                        heap.push(Node {
                            cost: next_cost,
                            state: next_state,
                        });
                    }
                    Ordering::Equal => {
                        if let Some(p) = predecessors.get_mut(&next_state) {
                            p.push(state);
                        } else {
                            predecessors.insert(next_state, vec![state]);
                        }
                    }
                    _ => {}
                },
                _ => {
                    dist.insert(next_state, next_cost);
                    predecessors.insert(next_state, vec![state]);
                    heap.push(Node {
                        cost: next_cost,
                        state: next_state,
                    });
                }
            }
        }
    }

    let Some(best_cost) = minimal_end_cost else {
        return (u32::MAX, Vec::new());
    };

    let mut all_paths = Vec::new();
    for es in &end_states {
        let mut path_builder = vec![(es.point, es.direction)];
        build_paths(
            *es,
            start_state,
            &predecessors,
            &mut path_builder,
            &mut all_paths,
        );
    }

    (best_cost, all_paths)
}

fn build_paths(
    current: State,
    start: State,
    predecessors: &HashMap<State, Vec<State>>,
    path_so_far: &mut Vec<(Point, Direction)>,
    all_paths: &mut Vec<Vec<(Point, Direction)>>,
) {
    if current == start {
        let mut full_path = path_so_far.clone();
        full_path.reverse();
        all_paths.push(full_path);
        return;
    }

    if let Some(preds) = predecessors.get(&current) {
        for &p in preds {
            path_so_far.push((p.point, p.direction));
            build_paths(p, start, predecessors, path_so_far, all_paths);
            path_so_far.pop();
        }
    }
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
    fn part_two_example_one() {
        let day16 = Day16::new(DATA_1.to_string());
        assert_eq!(day16.part_two(), "45");
    }

    #[test]
    fn part_two_example_two() {
        let day16 = Day16::new(DATA_2.to_string());
        assert_eq!(day16.part_two(), "64");
    }
}
