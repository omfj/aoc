use crate::utils::AdventDay;
use itertools::Itertools;
use std::collections::HashMap;

fn is_safe(rules: &HashMap<i32, Vec<i32>>, update: &[i32]) -> bool {
    for (i, value) in update.iter().enumerate() {
        if let Some(values) = rules.get(value) {
            if values.iter().any(|v| update[0..i].contains(v)) {
                return false;
            }
        }
    }

    true
}

fn parse_rules(str: &str) -> HashMap<i32, Vec<i32>> {
    let mut rules_map: HashMap<i32, Vec<i32>> = HashMap::new();
    for rule in str.lines() {
        let (key, value) = rule
            .split('|')
            .map(|value| value.parse::<i32>().unwrap())
            .collect_tuple()
            .unwrap();
        rules_map
            .entry(key)
            .and_modify(|values| values.push(value))
            .or_insert_with(|| vec![value]);
    }

    rules_map
}

fn fix_update(rules: &HashMap<i32, Vec<i32>>, update: &[i32]) -> Vec<i32> {
    let mut fixed = update.to_owned();

    fixed.sort_by(|a, b| {
        let a_deps = rules.get(a).map(|v| v.as_slice()).unwrap_or(&[]);
        let b_deps = rules.get(b).map(|v| v.as_slice()).unwrap_or(&[]);

        if a_deps.contains(b) {
            std::cmp::Ordering::Greater
        } else if b_deps.contains(a) {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Equal
        }
    });

    fixed
}

fn parse_updates(str: &str) -> Vec<Vec<i32>> {
    str.lines()
        .map(|line| {
            line.split(",")
                .map(|value| value.parse::<i32>().unwrap())
                .collect_vec()
        })
        .collect_vec()
}

pub struct Day05 {
    input: String,
}

impl AdventDay for Day05 {
    fn new(input: String) -> Self {
        Self { input }
    }

    fn part_one(&self) -> String {
        let (rules_str, updates_str) = self.input.split("\n\n").collect_tuple().unwrap();
        let rules = parse_rules(rules_str);
        let updates = parse_updates(updates_str);

        updates
            .iter()
            .filter(|update| is_safe(&rules, update))
            .map(|update| update[update.len() / 2])
            .sum::<i32>()
            .to_string()
    }

    fn part_two(&self) -> String {
        let (rules_str, updates_str) = self.input.split("\n\n").collect_tuple().unwrap();
        let rules = parse_rules(rules_str);
        let updates = parse_updates(updates_str);
        updates
            .iter()
            .filter(|update| !is_safe(&rules, update))
            .map(|update| {
                let fixed = fix_update(&rules, update);
                fixed[fixed.len() / 2]
            })
            .sum::<i32>()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#;

    #[test]
    fn part_one() {
        let day05 = Day05::new(DATA.to_string());
        assert_eq!(day05.part_one(), "143");
    }

    #[test]
    fn part_two() {
        let day05 = Day05::new(DATA.to_string());
        assert_eq!(day05.part_two(), "123");
    }
}
