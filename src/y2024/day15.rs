use crate::AdventDay;
use itertools::Itertools;

type Grid = Vec<Vec<Tile>>;
type Point = (usize, usize);

#[derive(Clone, Copy, PartialEq)]
enum Tile {
    Wall,
    Empty,
    Robot,
    Box,
    LeftBox,
    RightBox,
}

impl Tile {
    fn from(c: char) -> Self {
        match c {
            '#' => Tile::Wall,
            '.' => Tile::Empty,
            'O' => Tile::Box,
            '@' => Tile::Robot,
            '[' => Tile::LeftBox,
            ']' => Tile::RightBox,
            _ => panic!("Invalid tile: {}", c),
        }
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn from(c: char) -> Self {
        match c {
            '^' => Direction::Up,
            'v' => Direction::Down,
            '<' => Direction::Left,
            '>' => Direction::Right,
            _ => panic!("Invalid direction: {}", c),
        }
    }

    fn next(&self, x: usize, y: usize) -> Point {
        match self {
            Direction::Up => (x, y - 1),
            Direction::Down => (x, y + 1),
            Direction::Left => (x - 1, y),
            Direction::Right => (x + 1, y),
        }
    }
}

struct Board {
    grid: Grid,
}

impl Board {
    fn new(grid: Vec<Vec<Tile>>) -> Self {
        Self { grid }
    }

    fn find_robot(&self) -> Point {
        for (y, line) in self.grid.iter().enumerate() {
            for (x, c) in line.iter().enumerate() {
                if *c == Tile::Robot {
                    return (x, y);
                }
            }
        }

        panic!("No robot found");
    }

    fn make_wide(&mut self) -> Self {
        let mut grid = vec![];
        for line in self.grid.iter() {
            let mut new_line = vec![];
            for tile in line.iter() {
                match tile {
                    Tile::Robot => {
                        new_line.push(Tile::Robot);
                        new_line.push(Tile::Empty);
                    }
                    Tile::Box => {
                        new_line.push(Tile::LeftBox);
                        new_line.push(Tile::RightBox);
                    }
                    _ => {
                        new_line.push(*tile);
                        new_line.push(*tile);
                    }
                }
            }
            grid.push(new_line);
        }

        Self { grid }
    }

    fn step(&mut self, direction: &Direction) {
        let (x, y) = self.find_robot();
        let (nx, ny) = direction.next(x, y);
        let tile = self.grid[ny][nx];

        match tile {
            Tile::Wall => (),
            Tile::Empty => {
                self.grid[y][x] = Tile::Empty;
                self.grid[ny][nx] = Tile::Robot;
            }
            _ => {
                self.try_push(nx, ny, direction);

                if self.grid[ny][nx] == Tile::Empty {
                    self.grid[y][x] = Tile::Empty;
                    self.grid[ny][nx] = Tile::Robot;
                }
            }
        }
    }

    fn try_push(&mut self, x: usize, y: usize, direction: &Direction) {
        if let Some(moves) = self.push_moves(x, y, direction) {
            let moves = moves.into_iter().unique().collect::<Vec<_>>();

            for (x, y) in moves {
                let (nx, ny) = direction.next(x, y);
                (self.grid[y][x], self.grid[ny][nx]) = (self.grid[ny][nx], self.grid[y][x]);
            }
        }
    }

    fn push_moves(&self, x: usize, y: usize, direction: &Direction) -> Option<Vec<Point>> {
        let (nx, ny) = direction.next(x, y);

        let connected = match (self.grid[y][x], direction) {
            (Tile::LeftBox, Direction::Up | Direction::Down) => Some((x + 1, y)),
            (Tile::RightBox, Direction::Up | Direction::Down) => Some((x - 1, y)),
            _ => None,
        };

        let connected_new = connected.map(|(x, y)| direction.next(x, y));

        match (self.grid[ny][nx], connected, connected_new) {
            (Tile::Wall, _, _) => None,
            (_, _, Some((x, y))) if self.grid[y][x] == Tile::Wall => None,
            (Tile::Empty, _, None) => Some(vec![(x, y)]),
            (Tile::Empty, Some((cn, cy)), Some((cnx, cny)))
                if self.grid[cny][cnx] == Tile::Empty =>
            {
                Some(vec![(x, y), (cn, cy)])
            }
            _ => {
                let mut all_moves = vec![];
                if self.grid[ny][nx] != Tile::Empty {
                    all_moves.extend(self.push_moves(nx, ny, direction)?);
                }
                if let Some((other_new_x, other_new_y)) = connected_new {
                    if self.grid[other_new_y][other_new_x] != Tile::Empty {
                        all_moves.extend(self.push_moves(other_new_x, other_new_y, direction)?);
                    }
                }

                all_moves.push((x, y));
                if let Some((other_x, other_y)) = connected {
                    all_moves.push((other_x, other_y));
                }

                Some(all_moves)
            }
        }
    }

    fn score(&self) -> usize {
        let mut score = 0;

        for (y, line) in self.grid.iter().enumerate() {
            for (x, &tile) in line.iter().enumerate() {
                if tile == Tile::LeftBox || tile == Tile::Box {
                    score += 100 * y + x;
                }
            }
        }

        score
    }
}

fn parse_input(input: &str) -> (Grid, Vec<Direction>) {
    let (grid, directions) = input.split_once("\n\n").unwrap();

    let grid = grid
        .lines()
        .map(|line| line.chars().map(Tile::from).collect::<Vec<Tile>>())
        .collect::<Vec<_>>();

    let directions = directions
        .lines()
        .flat_map(|line| {
            line.chars()
                .map(Direction::from)
                .collect::<Vec<Direction>>()
        })
        .collect::<Vec<_>>();

    (grid, directions)
}

pub struct Day15 {
    input: String,
}

impl AdventDay for Day15 {
    fn new(input: String) -> Self {
        Self { input }
    }

    fn part_one(&self) -> String {
        let (grid, instructions) = parse_input(&self.input);
        let mut board = Board::new(grid);

        for instruction in instructions {
            board.step(&instruction);
        }

        board.score().to_string()
    }

    fn part_two(&self) -> String {
        let (grid, instructions) = parse_input(&self.input);
        let mut board = Board::new(grid).make_wide();

        for instruction in instructions {
            board.step(&instruction);
        }

        board.score().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = r#"########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<"#;

    const DATA_2: &str = r#"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^"#;

    #[test]
    fn part_one() {
        let day15 = Day15::new(DATA.to_string());
        assert_eq!(day15.part_one(), "2028");
    }

    #[test]
    fn part_one_2() {
        let day15 = Day15::new(DATA_2.to_string());
        assert_eq!(day15.part_one(), "10092");
    }

    #[test]
    fn part_two() {
        let day15 = Day15::new(DATA_2.to_string());
        assert_eq!(day15.part_two(), "9021");
    }
}
