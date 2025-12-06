use crate::AdventDay;

fn parse_input(input: &str) -> (Vec<(isize, isize)>, Vec<isize>) {
    let mut fresh = Vec::new();

    let sections: Vec<&str> = input.trim().split("\n\n").collect();

    for line in sections[0].lines() {
        let line: Vec<&str> = line.split('-').collect();
        let start: isize = line[0].parse().unwrap();
        let end: isize = line[1].parse().unwrap();
        fresh.push((start, end));
    }

    let mut queries = Vec::new();
    for line in sections[1].lines() {
        let query: isize = line.parse().unwrap();
        queries.push(query);
    }

    (fresh, queries)
}

pub struct Day05 {
    input: String,
}

impl AdventDay for Day05 {
    fn new(input: String) -> Self {
        Self { input }
    }

    fn part_one(&self) -> String {
        let (fresh, queries) = parse_input(&self.input);
        let mut count = 0;
        for query in queries {
            if fresh
                .iter()
                .any(|(start, end)| query >= *start && query <= *end)
            {
                count += 1;
            }
        }
        count.to_string()
    }

    fn part_two(&self) -> String {
        let (mut fresh, _) = parse_input(&self.input);

        fresh.sort_by_key(|r| r.0);

        let mut merged: Vec<(isize, isize)> = Vec::new();
        for (start, end) in fresh {
            if let Some(last) = merged.last_mut() {
                if start <= last.1 + 1 {
                    last.1 = last.1.max(end);
                } else {
                    merged.push((start, end));
                }
            } else {
                merged.push((start, end));
            }
        }

        let count: isize = merged.iter().map(|(s, e)| e - s + 1).sum();
        count.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = r#"3-5
10-14
16-20
12-18

1
5
8
11
17
32
"#;

    #[test]
    fn part_one() {
        let day05 = Day05::new(DATA.to_string());
        assert_eq!(day05.part_one(), "3");
    }

    #[test]
    fn part_two() {
        let day05 = Day05::new(DATA.to_string());
        assert_eq!(day05.part_two(), "14");
    }
}
