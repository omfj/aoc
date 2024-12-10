use crate::utils::AdventDay;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

type Map = HashMap<(u32, u32), u32>;

fn collect_map(input: &str) -> Map {
    input
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            line.chars().enumerate().map(move |(j, c)| {
                let height = c.to_digit(10).unwrap();
                ((i as u32, j as u32), height)
            })
        })
        .collect()
}

fn collect_trailheads(map: &Map) -> HashSet<(u32, u32)> {
    map.iter()
        .filter_map(
            |(&(i, j), height)| {
                if *height == 0 {
                    Some((i, j))
                } else {
                    None
                }
            },
        )
        .collect()
}

fn find_summits(map: &Map, trailhead: (u32, u32)) -> Vec<(u32, u32)> {
    let mut stack = vec![(trailhead, Vec::new())];
    let mut summits = Vec::new();

    while let Some(((i, j), path)) = stack.pop() {
        let height = *map.get(&(i, j)).unwrap();

        if height == 9 {
            summits.push((i, j));
            continue;
        }

        let neighbors = [
            (i.wrapping_sub(1), j),
            (i.wrapping_add(1), j),
            (i, j.wrapping_sub(1)),
            (i, j.wrapping_add(1)),
        ];

        for &(ni, nj) in &neighbors {
            if let Some(&neighbor_height) = map.get(&(ni, nj)) {
                if neighbor_height == height + 1 {
                    let mut new_path = path.clone();
                    new_path.push((i, j));
                    stack.push(((ni, nj), new_path));
                }
            }
        }
    }

    summits
}

fn to_set<T: Eq + Hash + Clone>(vec: Vec<T>) -> Vec<T> {
    let mut seen = HashSet::new();
    vec.into_iter()
        .filter(|item| seen.insert(item.clone()))
        .collect()
}

pub struct Day10 {
    input: String,
}

impl AdventDay for Day10 {
    fn new(input: String) -> Self {
        Self { input }
    }

    fn part_one(&self) -> String {
        let map = collect_map(&self.input);
        let trailheads = collect_trailheads(&map);
        let mut sum = 0;

        for trailhead in trailheads {
            let summits = find_summits(&map, trailhead);
            if !summits.is_empty() {
                sum += to_set(summits).len();
            }
        }

        sum.to_string()
    }

    fn part_two(&self) -> String {
        let map = collect_map(&self.input);
        let trailheads = collect_trailheads(&map);
        let mut sum = 0;

        for trailhead in trailheads {
            let summits = find_summits(&map, trailhead);
            if !summits.is_empty() {
                sum += summits.len();
            }
        }

        sum.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = r#"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"#;

    #[test]
    fn part_one() {
        let day10 = Day10::new(DATA.to_string());
        assert_eq!(day10.part_one(), "36");
    }

    #[test]
    fn part_two() {
        let day10 = Day10::new(DATA.to_string());
        assert_eq!(day10.part_two(), "81");
    }
}
