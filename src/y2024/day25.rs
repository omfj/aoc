use crate::utils::AdventDay;
use itertools::Itertools;

type Schematics = (u32, u32, u32, u32, u32);

#[derive(Debug, Clone, PartialEq)]
enum Scheme {
    Key(Schematics),
    Lock(Schematics),
}

fn parse_schematics(s: &str) -> Scheme {
    let mut schematics: Vec<u32> = vec![0; 5];
    let lines: Vec<&str> = s.lines().collect();

    let is_lock = match lines.first() {
        Some(line) => line.chars().all(|c| c == '#'),
        None => false,
    };

    let col_char = if is_lock { '#' } else { '.' };

    for column in 0..5 {
        let col = lines.iter().map(|line| line.chars().nth(column).unwrap());
        let height = col.take_while(|c| *c == col_char).count() as u32;
        schematics[column] = if is_lock { height - 1 } else { 6 - height };
    }

    let tuple = (
        schematics[0],
        schematics[1],
        schematics[2],
        schematics[3],
        schematics[4],
    );

    if is_lock {
        Scheme::Lock(tuple)
    } else {
        Scheme::Key(tuple)
    }
}

fn parse_input(input: &str) -> Vec<Scheme> {
    input.split("\n\n").map(|s| parse_schematics(s)).collect()
}

fn key_fits_lock(key: Schematics, lock: Schematics) -> bool {
    let (k1, k2, k3, k4, k5) = key;
    let (l1, l2, l3, l4, l5) = lock;

    l1 + k1 <= 5 && l2 + k2 <= 5 && l3 + k3 <= 5 && l4 + k4 <= 5 && l5 + k5 <= 5
}

pub struct Day25 {
    input: String,
}

impl AdventDay for Day25 {
    fn new(input: String) -> Self {
        Self { input }
    }

    fn part_one(&self) -> String {
        let schematics = parse_input(&self.input);
        let mut keys: Vec<Schematics> = Vec::new();
        let mut locks: Vec<Schematics> = Vec::new();

        for scheme in schematics {
            match scheme {
                Scheme::Key(key) => keys.push(key),
                Scheme::Lock(lock) => locks.push(lock),
            }
        }

        keys.iter()
            .cartesian_product(locks.iter())
            .filter(|(k, l)| key_fits_lock(**k, **l))
            .count()
            .to_string()
    }

    fn part_two(&self) -> String {
        "Done :D".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = r#"#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####"#;

    #[test]
    fn part_one() {
        let day25 = Day25::new(DATA.to_string());
        assert_eq!(day25.part_one(), "3");
    }
}
