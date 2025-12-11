use itertools::Itertools;

use crate::AdventDay;
use std::collections::{HashMap, HashSet, VecDeque};

pub struct Day11 {
    input: String,
}

fn parse_input(input: &str) -> HashMap<String, Vec<String>> {
    input
        .lines()
        .map(|line| {
            let (key, value) = line.split(':').collect_tuple().unwrap();
            (
                key.to_string(),
                value.trim().split(" ").map(|s| s.to_string()).collect(),
            )
        })
        .collect()
}

impl AdventDay for Day11 {
    fn new(input: String) -> Self {
        Self { input }
    }

    fn part_one(&self) -> String {
        let input = parse_input(&self.input);

        let mut paths = 0;

        let mut q = VecDeque::new();
        q.push_back(("you", HashSet::from(["you"])));

        while let Some((node, visited)) = q.pop_front() {
            if input.get(node).unwrap().contains(&"out".to_string()) {
                paths += 1;
                continue;
            }

            for neighbor in input.get(node).unwrap() {
                if !visited.contains(neighbor.as_str()) {
                    let mut new_visited = visited.clone();
                    new_visited.insert(neighbor);
                    q.push_back((neighbor, new_visited));
                }
            }
        }

        paths.to_string()
    }

    fn part_two(&self) -> String {
        let g = parse_input(&self.input);

        fn count_paths<'a>(
            curr: &'a str,
            has_seen_dac: bool,
            has_seen_fft: bool,
            g: &'a HashMap<String, Vec<String>>,
            cache: &mut HashMap<(&'a str, bool, bool), usize>,
        ) -> usize {
            if let Some(&cached) = cache.get(&(curr, has_seen_dac, has_seen_fft)) {
                return cached;
            }

            let neighbors = g.get(curr).unwrap();

            let result = if neighbors.iter().any(|n| n == "out") {
                if has_seen_dac && has_seen_fft {
                    1
                } else {
                    0
                }
            } else {
                neighbors
                    .iter()
                    .map(|neighbor| {
                        count_paths(
                            neighbor,
                            has_seen_dac || neighbor == "dac",
                            has_seen_fft || neighbor == "fft",
                            g,
                            cache,
                        )
                    })
                    .sum()
            };

            cache.insert((curr, has_seen_dac, has_seen_fft), result);
            result
        }

        let mut cache = HashMap::new();
        count_paths("svr", false, false, &g, &mut cache).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {
        let data = r#"aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out
"#;

        let day11 = Day11::new(data.to_string());
        assert_eq!(day11.part_one(), "5");
    }

    #[test]
    fn part_two() {
        let data = r#"svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out
"#;

        let day11 = Day11::new(data.to_string());
        assert_eq!(day11.part_two(), "2");
    }
}
