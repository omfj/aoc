use crate::utils::AdventDay;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
enum BlinkResult {
    One(usize),
    Split(usize, usize),
}

fn has_even_digits(n: usize) -> bool {
    n.to_string().len() % 2 == 0
}

fn split_number(n: usize) -> (usize, usize) {
    let s = n.to_string();
    let mid = s.len() / 2;
    let left = s[..mid].parse().unwrap();
    let right = s[mid..].parse().unwrap();
    (left, right)
}

fn blink(stone: usize) -> BlinkResult {
    if stone == 0 {
        BlinkResult::One(1)
    } else if has_even_digits(stone) {
        let (left, right) = split_number(stone);
        BlinkResult::Split(left, right)
    } else {
        BlinkResult::One(stone * 2024)
    }
}

fn blink_cached(stone: usize, memo: &mut HashMap<usize, BlinkResult>) -> BlinkResult {
    if let Some(&cached) = memo.get(&stone) {
        cached
    } else {
        let result = blink(stone);
        memo.insert(stone, result);
        result
    }
}

fn blink_multiple_one_stone(
    stone: usize,
    n: usize,
    count_memo: &mut HashMap<(usize, usize), usize>,
    result_memo: &mut HashMap<usize, BlinkResult>,
) -> usize {
    if n == 0 {
        return 1;
    }

    if let Some(&count) = count_memo.get(&(stone, n)) {
        return count;
    }

    let result = blink_cached(stone, result_memo);
    let count = match result {
        BlinkResult::One(s) => blink_multiple_one_stone(s, n - 1, count_memo, result_memo),
        BlinkResult::Split(left, right) => {
            blink_multiple_one_stone(left, n - 1, count_memo, result_memo)
                + blink_multiple_one_stone(right, n - 1, count_memo, result_memo)
        }
    };

    count_memo.insert((stone, n), count);
    count
}

fn blink_multiple(stones: &[usize], n: usize) -> usize {
    let mut result_memo = HashMap::new();
    let mut count_memo = HashMap::new();
    stones
        .iter()
        .map(|&s| blink_multiple_one_stone(s, n, &mut count_memo, &mut result_memo))
        .sum()
}

fn collect_stones(input: &str) -> Vec<usize> {
    input
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect()
}

pub struct Day11 {
    input: String,
}

impl AdventDay for Day11 {
    fn new(input: String) -> Self {
        Self { input }
    }

    fn part_one(&self) -> String {
        let stones = collect_stones(&self.input);
        blink_multiple(&stones, 25).to_string()
    }

    fn part_two(&self) -> String {
        let stones = collect_stones(&self.input);
        blink_multiple(&stones, 75).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = "125 17";

    #[test]
    fn part_one_and_two() {
        let day11 = Day11::new(DATA.to_string());
        assert_eq!(day11.part_one(), "55312");
    }
}
