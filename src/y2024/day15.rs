use crate::utils::AdventDay;
use core::fmt;
use itertools::Itertools;

#[derive(Debug, Clone)]
struct Game {
    board: Vec<Vec<Tile>>,
}

impl Game {
    fn new(board: Vec<Vec<Tile>>) -> Self {
        Self { board }
    }

    fn make_wide(&mut self) {
        let mut wide_board = vec![vec![Tile::Wall; self.board[0].len() * 2]; self.board.len()];

        for (i, row) in self.board.iter().enumerate() {
            for (j, tile) in row.iter().enumerate() {
                let j1 = j * 2;
                let j2 = j1 + 1;
                match tile {
                    Tile::Box => {
                        wide_board[i][j1] = Tile::LeftBox;
                        wide_board[i][j2] = Tile::RightBox;
                    }
                    Tile::Robot => {
                        wide_board[i][j1] = Tile::Robot;
                        wide_board[i][j2] = Tile::Empty;
                    }
                    _ => {
                        wide_board[i][j1] = *tile;
                        wide_board[i][j2] = *tile;
                    }
                }
            }
        }

        self.board = wide_board;
    }

    fn score(&self) -> i32 {
        let mut score = 0;

        for (i, row) in self.board.iter().enumerate() {
            for (j, tile) in row.iter().enumerate() {
                if *tile == Tile::Box || *tile == Tile::LeftBox {
                    score += 100 * i + j;
                }
            }
        }

        score as i32
    }

    fn find_robot(&self) -> (usize, usize) {
        for (y, row) in self.board.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                if *tile == Tile::Robot {
                    return (x, y);
                }
            }
        }

        panic!("Robot not found");
    }

    fn get_next(&self, x: usize, y: usize, direction: Direction) -> (usize, usize) {
        let (dx, dy) = match direction {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        };

        let next_x = (x as i32 + dx) as usize;
        let next_y = (y as i32 + dy) as usize;

        (next_x, next_y)
    }

    fn move_robot(&mut self, direction: Direction) {
        let (x, y) = self.find_robot();
        let (next_x, next_y) = self.get_next(x, y, direction);
        let next_tile = self.board[next_y][next_x];

        match next_tile {
            Tile::Empty => {
                self.board[y][x] = Tile::Empty;
                self.board[next_y][next_x] = Tile::Robot;
            }
            Tile::Box => {
                let offset: usize = self.moveable_tiles(x, y, direction).try_into().unwrap();

                if offset == 0 {
                    return;
                }

                for i in 0..offset {
                    match direction {
                        Direction::Up => {
                            self.board[y - offset + i][x] = self.board[y - offset + i + 1][x];
                            self.board[y - offset + i + 1][x] = Tile::Empty;
                        }
                        Direction::Down => {
                            self.board[y + offset - i][x] = self.board[y + offset - i - 1][x];
                            self.board[y + offset - i - 1][x] = Tile::Empty;
                        }
                        Direction::Left => {
                            self.board[y][x - offset + i] = self.board[y][x - offset + i + 1];
                            self.board[y][x - offset + i + 1] = Tile::Empty;
                        }
                        Direction::Right => {
                            self.board[y][x + offset - i] = self.board[y][x + offset - i - 1];
                            self.board[y][x + offset - i - 1] = Tile::Empty;
                        }
                    }
                }
            }
            _ => (),
        }
    }

    /// Scans the direction to see if it can move.
    ///
    /// @ -> O -> O -> . = 3
    /// @ -> O -> O -> O -> O -> . = 5
    /// @ -> O -> # = 0
    /// @ -> . = 1
    /// @ -> # = 0
    ///
    /// Returns the amount of tiles in the direction that can be moved
    fn moveable_tiles(&self, x: usize, y: usize, direction: Direction) -> i32 {
        let mut count = 0;
        let (mut x, mut y) = (x as i32, y as i32);

        loop {
            let (next_x, next_y) = self.get_next(x as usize, y as usize, direction);
            let next_tile = self.board[next_y][next_x];

            match next_tile {
                Tile::Empty => {
                    return count + 1;
                }
                Tile::Box => {
                    count += 1;
                    x = next_x as i32;
                    y = next_y as i32;
                }
                _ => {
                    return 0;
                }
            }
        }
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = String::new();

        for row in &self.board {
            for tile in row {
                let c = match tile {
                    Tile::Wall => '#',
                    Tile::Empty => '.',
                    Tile::Box => 'O',
                    Tile::Robot => '@',
                    Tile::LeftBox => '[',
                    Tile::RightBox => ']',
                };

                output.push(c);
            }

            output.push('\n');
        }

        write!(f, "{}", output)
    }
}

type Moves = Vec<Direction>;

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
enum Tile {
    Wall,
    Empty,
    Box,
    LeftBox,
    RightBox,
    Robot,
}

#[derive(Clone, Debug, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn from_char(c: char) -> Self {
        match c {
            '^' => Self::Up,
            'v' => Self::Down,
            '<' => Self::Left,
            '>' => Self::Right,
            _ => panic!("Invalid direction"),
        }
    }
}

pub struct Day15 {
    input: String,
}

impl AdventDay for Day15 {
    fn new(input: String) -> Self {
        Self { input }
    }

    fn part_one(&self) -> String {
        let (mut game, moves) = parse_input(&self.input);

        for direction in moves {
            game.move_robot(direction);
        }

        game.score().to_string()
    }

    fn part_two(&self) -> String {
        let (mut game, moves) = parse_input(&self.input);
        game.make_wide();

        for direction in moves {
            game.move_robot(direction);
        }

        game.score().to_string()
    }
}

fn parse_input(input: &str) -> (Game, Moves) {
    let (game, moves) = input.split("\n\n").collect_tuple().unwrap();
    let game = parse_game(game);
    let moves = parse_moves(moves);

    (game, moves)
}

fn parse_game(input: &str) -> Game {
    let board: Vec<Vec<Tile>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '#' => Tile::Wall,
                    '.' => Tile::Empty,
                    'O' => Tile::Box,
                    '@' => Tile::Robot,
                    '[' => Tile::LeftBox,
                    ']' => Tile::RightBox,
                    _ => panic!("Invalid tile"),
                })
                .collect()
        })
        .collect();

    Game::new(board)
}

fn parse_moves(input: &str) -> Moves {
    input
        .lines()
        .join("")
        .chars()
        .map(Direction::from_char)
        .collect()
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
<>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
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
    #[ignore = "not finished"]
    fn part_two() {
        let day15 = Day15::new(DATA_2.to_string());
        assert_eq!(day15.part_two(), "9021");
    }
}
