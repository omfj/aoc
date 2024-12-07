use itertools::Itertools;

use crate::utils::AdventDay;

#[derive(Debug)]
struct Game {
    id: u32,
    sets: Vec<Vec<Color>>,
}

impl Game {
    fn new(id: u32, sets: Vec<Vec<Color>>) -> Self {
        Self { id, sets }
    }

    fn is_valid(&self) -> bool {
        self.sets
            .iter()
            .all(|set| set.iter().all(|color| color.is_valid()))
    }
}

#[derive(Debug)]
enum Color {
    Green(u32),
    Red(u32),
    Blue(u32),
}

impl Color {
    fn is_valid(&self) -> bool {
        match self {
            Color::Red(count) => *count <= 12,
            Color::Green(count) => *count <= 13,
            Color::Blue(count) => *count <= 14,
        }
    }
}

fn power_of_set(set: &[Vec<Color>]) -> u32 {
    let (red, green, blue) = set.iter().fold((0, 0, 0), |acc, colors| {
        colors
            .iter()
            .fold(acc, |(red, green, blue), color| match color {
                Color::Red(count) => (red.max(*count), green, blue),
                Color::Green(count) => (red, green.max(*count), blue),
                Color::Blue(count) => (red, green, blue.max(*count)),
            })
    });

    red * green * blue
}

fn parse_input(input: &str) -> Vec<Game> {
    input
        .lines()
        .map(|game| {
            let id = game
                .split(":")
                .next()
                .unwrap()
                .split_whitespace()
                .nth(1)
                .unwrap()
                .parse::<u32>()
                .unwrap();

            let sets = game
                .split(": ")
                .nth(1)
                .unwrap()
                .split("; ")
                .map(|set| {
                    set.split(", ")
                        .map(|color| {
                            let (count_str, color_str) =
                                color.split_whitespace().collect_tuple().unwrap();
                            let count = count_str.parse::<u32>().unwrap();
                            match color_str {
                                "red" => Color::Red(count),
                                "green" => Color::Green(count),
                                "blue" => Color::Blue(count),
                                _ => panic!("Invalid color"),
                            }
                        })
                        .collect()
                })
                .collect();

            Game::new(id, sets)
        })
        .collect_vec()
}

pub struct Day02 {
    input: String,
}

impl AdventDay for Day02 {
    fn new(input: String) -> Self {
        Self { input }
    }

    fn part_one(&self) -> String {
        let games = parse_input(&self.input);
        games
            .iter()
            .filter(|game| game.is_valid())
            .map(|game| game.id)
            .sum::<u32>()
            .to_string()
    }

    fn part_two(&self) -> String {
        let games = parse_input(&self.input);
        games
            .iter()
            .map(|game| power_of_set(&game.sets))
            .sum::<u32>()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;

    #[test]
    fn part_one() {
        let day02 = Day02::new(DATA.to_string());
        assert_eq!(day02.part_one(), "8");
    }

    #[test]
    fn part_two() {
        let day02 = Day02::new(DATA.to_string());
        assert_eq!(day02.part_two(), "2286");
    }
}
