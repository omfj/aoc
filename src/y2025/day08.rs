use std::collections::HashMap;

use itertools::Itertools;

use crate::AdventDay;

pub struct Day08 {
    input: String,
}

fn parse_input(input: &str) -> Vec<Box> {
    input
        .lines()
        .map(|line| {
            let (x, y, z) = line
                .split(',')
                .map(|s| s.parse::<i32>().unwrap())
                .collect_tuple()
                .unwrap();
            Box::new(x, y, z)
        })
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Box {
    x: i32,
    y: i32,
    z: i32,
}

impl Box {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    fn distance(self, other: &Box) -> i64 {
        let dx = (self.x as i64 - other.x as i64).pow(2);
        let dy = (self.y as i64 - other.y as i64).pow(2);
        let dz = (self.z as i64 - other.z as i64).pow(2);
        (dx + dy + dz).isqrt()
    }
}

struct UnionFind {
    parent: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) {
        let px = self.find(x);
        let py = self.find(y);
        if px != py {
            self.parent[px] = py;
        }
    }
}

impl AdventDay for Day08 {
    fn new(input: String) -> Self {
        Self { input }
    }

    fn part_one(&self) -> String {
        let boxes = parse_input(&self.input);
        let len = boxes.len();

        let mut pairs = Vec::new();
        for i in 0..len {
            for j in (i + 1)..len {
                let d = boxes[i].distance(&boxes[j]);
                pairs.push((d, i, j));
            }
        }
        pairs.sort();

        let mut uf = UnionFind::new(len);

        let take = if len == 20 { 10 } else { 1000 };
        for (_, i, j) in pairs.into_iter().take(take) {
            if uf.find(i) != uf.find(j) {
                uf.union(i, j);
            }
        }

        let mut circuit_sizes: HashMap<usize, usize> = HashMap::new();
        for i in 0..len {
            *circuit_sizes.entry(uf.find(i)).or_default() += 1;
        }

        let top3: Vec<usize> = circuit_sizes
            .values()
            .copied()
            .sorted()
            .rev()
            .take(3)
            .collect();
        top3.iter().product::<usize>().to_string()
    }

    fn part_two(&self) -> String {
        let boxes = parse_input(&self.input);
        let n = boxes.len();

        let mut pairs = Vec::new();
        for (i, box1) in boxes.iter().enumerate() {
            for (j, box2) in boxes.iter().enumerate().skip(i + 1) {
                let d = box1.distance(box2);
                pairs.push((d, i, j));
            }
        }
        pairs.sort();

        let mut uf = UnionFind::new(n);
        let mut last_pair = (0, 0);

        for (_, i, j) in pairs {
            if uf.find(i) != uf.find(j) {
                uf.union(i, j);
                last_pair = (i, j);

                let root = uf.find(0);
                if (0..n).all(|k| uf.find(k) == root) {
                    break;
                }
            }
        }

        let (i, j) = last_pair;
        (boxes[i].x as i64 * boxes[j].x as i64).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = r#"162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
"#;

    #[test]
    fn part_one() {
        let day08 = Day08::new(DATA.to_string());
        assert_eq!(day08.part_one(), "40");
    }

    #[test]
    fn part_two() {
        let day08 = Day08::new(DATA.to_string());
        assert_eq!(day08.part_two(), "25272");
    }
}
