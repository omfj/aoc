use itertools::Itertools;

use crate::utils::AdventDay;

#[derive(Debug)]
enum Result {
    Win,
    Lose,
    Draw,
}

impl Result {
    fn points(&self) -> i32 {
        match self {
            Self::Win => 6,
            Self::Draw => 3,
            Self::Lose => 0,
        }
    }

    fn from_str(s: &str) -> Self {
        match s {
            "X" => Self::Lose,
            "Y" => Self::Draw,
            "Z" => Self::Win,
            _ => panic!("Invalid result"),
        }
    }
}

#[derive(Clone, Debug)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

impl Choice {
    fn from_str(s: &str) -> Self {
        match s {
            "A" => Self::Rock,
            "B" => Self::Paper,
            "C" => Self::Scissors,
            "X" => Self::Rock,
            "Y" => Self::Paper,
            "Z" => Self::Scissors,
            _ => panic!("Invalid choice"),
        }
    }

    fn points(&self) -> i32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    fn result(&self, other: &Self) -> Result {
        match (self, other) {
            (Self::Rock, Self::Scissors) => Result::Win,
            (Self::Rock, Self::Paper) => Result::Lose,
            (Self::Paper, Self::Rock) => Result::Win,
            (Self::Paper, Self::Scissors) => Result::Lose,
            (Self::Scissors, Self::Paper) => Result::Win,
            (Self::Scissors, Self::Rock) => Result::Lose,
            _ => Result::Draw,
        }
    }
}

fn choice_from_strategy(strategy: &Result, opponent: &Choice) -> Choice {
    match strategy {
        Result::Win => match opponent {
            Choice::Rock => Choice::Paper,
            Choice::Paper => Choice::Scissors,
            Choice::Scissors => Choice::Rock,
        },
        Result::Lose => match opponent {
            Choice::Rock => Choice::Scissors,
            Choice::Paper => Choice::Rock,
            Choice::Scissors => Choice::Paper,
        },
        Result::Draw => opponent.clone(),
    }
}

pub struct Day02 {
    input: String,
}

impl AdventDay for Day02 {
    fn new(input: String) -> Self {
        Self { input }
    }

    fn part_one(&self) -> String {
        self.input
            .lines()
            .fold(0, |acc, line| {
                let parts = line.split_whitespace();
                let (a, b) = parts.collect_tuple().unwrap();
                let opponent = Choice::from_str(a);
                let me = Choice::from_str(b);

                acc + me.result(&opponent).points() + me.points()
            })
            .to_string()
    }

    fn part_two(&self) -> String {
        self.input
            .lines()
            .fold(0, |acc, line| {
                let parts = line.split_whitespace();
                let (a, b) = parts.collect_tuple().unwrap();
                let opponent = Choice::from_str(a);
                let strategy = Result::from_str(b);
                let choice = choice_from_strategy(&strategy, &opponent);

                acc + strategy.points() + choice.points()
            })
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = r#"A Y
B X
C Z"#;

    #[test]
    fn part_one() {
        let day02 = Day02::new(DATA.to_string());
        assert_eq!(day02.part_one(), "15");
    }

    #[test]
    fn part_two() {
        let day02 = Day02::new(DATA.to_string());
        assert_eq!(day02.part_two(), "12");
    }
}
