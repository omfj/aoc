use crate::AdventDay;
use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};

fn get_numeric_keys() -> HashMap<(char, char), Vec<String>> {
    let keypad = HashMap::from([
        ('7', vec![('4', 'v'), ('8', '>')]),
        ('8', vec![('7', '<'), ('9', '>'), ('5', 'v')]),
        ('9', vec![('8', '<'), ('6', 'v')]),
        ('4', vec![('1', 'v'), ('5', '>'), ('7', '^')]),
        ('5', vec![('2', 'v'), ('6', '>'), ('8', '^'), ('4', '<')]),
        ('6', vec![('3', 'v'), ('9', '^'), ('5', '<')]),
        ('1', vec![('2', '>'), ('4', '^')]),
        ('2', vec![('3', '>'), ('5', '^'), ('1', '<'), ('0', 'v')]),
        ('3', vec![('6', '^'), ('2', '<'), ('A', 'v')]),
        ('0', vec![('2', '^'), ('A', '>')]),
        ('A', vec![('0', '<'), ('3', '^')]),
    ]);

    keypad
        .keys()
        .cartesian_product(keypad.keys())
        .map(|(a, b)| ((*a, *b), find_shortest_paths(&keypad, *a, *b)))
        .collect()
}

fn get_direction_keys() -> HashMap<(char, char), Vec<String>> {
    let keypad = HashMap::from([
        ('^', vec![('A', '>'), ('v', 'v')]),
        ('A', vec![('^', '<'), ('>', 'v')]),
        ('>', vec![('A', '^'), ('v', '<')]),
        ('<', vec![('v', '>')]),
        ('v', vec![('<', '<'), ('^', '^'), ('>', '>')]),
    ]);

    keypad
        .keys()
        .cartesian_product(keypad.keys())
        .map(|(a, b)| ((*a, *b), find_shortest_paths(&keypad, *a, *b)))
        .collect()
}

fn find_shortest_paths(
    neighbors: &HashMap<char, Vec<(char, char)>>,
    start: char,
    end: char,
) -> Vec<String> {
    let mut queue = VecDeque::new();
    queue.push_back((start, Vec::new(), HashSet::new()));

    let mut paths = Vec::new();
    let mut lowest = usize::MAX;

    while let Some((node, path, mut visited)) = queue.pop_front() {
        if node == end {
            if path.len() <= lowest {
                lowest = path.len();
                paths.push(path.iter().collect::<String>());
            }
            continue;
        }

        if visited.contains(&node) {
            continue;
        }
        visited.insert(node);

        for (next, dir) in neighbors.get(&node).unwrap() {
            let mut path = path.clone();
            path.push(*dir);
            queue.push_back((*next, path, visited.clone()));
        }
    }

    paths
}

fn type_numeric_code(code: &str, depth: i32, memo: &mut HashMap<(String, i32), usize>) -> usize {
    let keys = get_numeric_keys();
    let mut sum = 0;
    let code = "A".to_string() + code;

    if let Some(&value) = memo.get(&(code.clone(), depth)) {
        return value;
    }

    for (a, b) in code.chars().tuple_windows() {
        let paths = keys.get(&(a, b)).unwrap();
        sum += match depth {
            0 => paths[0].len() + 1,
            _ => paths
                .iter()
                .cloned()
                .map(|path| {
                    let path = path + "A";
                    type_direction(&path, depth - 1, memo)
                })
                .min()
                .unwrap(),
        };
    }

    memo.insert((code.clone(), depth), sum);

    sum
}

fn type_direction(code: &str, depth: i32, memo: &mut HashMap<(String, i32), usize>) -> usize {
    let keys = get_direction_keys();
    let mut sum = 0;
    let code = "A".to_string() + code;

    if let Some(&value) = memo.get(&(code.clone(), depth)) {
        return value;
    }

    for (a, b) in code.chars().tuple_windows() {
        let paths = keys.get(&(a, b)).unwrap();

        sum += match depth {
            0 => paths[0].len() + 1,
            _ => paths
                .iter()
                .cloned()
                .map(|path| {
                    let path = path + "A";
                    type_direction(&path, depth - 1, memo)
                })
                .min()
                .unwrap(),
        };
    }

    memo.insert((code.clone(), depth), sum);

    sum
}

pub struct Day21 {
    input: String,
}

impl AdventDay for Day21 {
    fn new(input: String) -> Self {
        Self { input }
    }

    fn part_one(&self) -> String {
        let depth = 2;
        let mut memo = HashMap::new();

        self.input
            .lines()
            .map(|code| {
                type_numeric_code(code, depth, &mut memo)
                    * code.trim_end_matches('A').parse::<usize>().unwrap()
            })
            .sum::<usize>()
            .to_string()
    }

    fn part_two(&self) -> String {
        let depth = 25;
        let mut memo = HashMap::new();

        self.input
            .lines()
            .map(|code| {
                type_numeric_code(code, depth, &mut memo)
                    * code.trim_end_matches('A').parse::<usize>().unwrap()
            })
            .sum::<usize>()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = r#"029A
980A
179A
456A
379A"#;

    #[test]
    fn part_one_and_two() {
        let day21 = Day21::new(DATA.to_string());
        assert_eq!(day21.part_one(), "126384");
    }
}
