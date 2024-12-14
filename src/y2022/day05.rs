use crate::utils::AdventDay;
use itertools::Itertools;

type Game = Vec<Vec<char>>;
type Moves = Vec<(i32, i32, i32)>;

pub struct Day05 {
    input: String,
}

impl AdventDay for Day05 {
    fn new(input: String) -> Self {
        Self { input }
    }

    fn part_one(&self) -> String {
        let (mut game, moves) = parse_data(&self.input);

        for (count, from, to) in moves {
            for _ in 0..count {
                let c = game[from as usize - 1][0];
                game[to as usize - 1].insert(0, c);
                game[from as usize - 1].remove(0);
            }
        }

        game.iter().map(|stack| stack.first().unwrap()).join("")
    }

    fn part_two(&self) -> String {
        let (mut game, moves) = parse_data(&self.input);

        for (count, from, to) in moves {
            for i in 0..count {
                let c = game[from as usize - 1][0];
                game[to as usize - 1].insert(i as usize, c);
                game[from as usize - 1].remove(0);
            }
        }

        game.iter().map(|stack| stack.first().unwrap()).join("")
    }
}

fn parse_data(input: &str) -> (Game, Moves) {
    let (game, moves) = input.split_once("\n\n").unwrap();
    let game = parse_game(game);
    let moves = parse_moves(moves);
    (game, moves)
}
fn parse_game(game: &str) -> Game {
    let lines: Vec<&str> = game.lines().collect();
    let last_line = lines.last().unwrap();
    let num_stacks = last_line.split_whitespace().count();

    let mut stacks: Game = vec![Vec::new(); num_stacks];

    for &line in &lines[..lines.len() - 1] {
        if !line.contains('[') {
            continue;
        }

        for (i, stack) in stacks.iter_mut().enumerate().take(num_stacks) {
            let start = i * 4;
            if start + 3 <= line.len() {
                let segment = &line[start..start + 3];
                if segment.starts_with('[') && segment.ends_with(']') && segment.len() == 3 {
                    let c = segment.chars().nth(1).unwrap();
                    stack.push(c);
                }
            }
        }
    }

    stacks
}

/// Parse the moves from the input
/// (count, from, to)
fn parse_moves(moves: &str) -> Moves {
    moves
        .lines()
        .map(|line| {
            let parts = line.split(" ").collect::<Vec<&str>>();
            let count = parts.get(1).unwrap().parse().unwrap();
            let from = parts.get(3).unwrap().parse().unwrap();
            let to = parts.get(5).unwrap().parse().unwrap();
            (count, from, to)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = r#"    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"#;

    #[test]
    fn part_one() {
        let day05 = Day05::new(DATA.to_string());
        assert_eq!(day05.part_one(), "CMZ");
    }

    #[test]
    fn part_two() {
        let day05 = Day05::new(DATA.to_string());
        assert_eq!(day05.part_two(), "MCD");
    }
}
