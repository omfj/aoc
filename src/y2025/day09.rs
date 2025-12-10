use itertools::Itertools;

use crate::AdventDay;

pub struct Day09 {
    input: String,
}

fn parse_input(input: &str) -> Vec<(isize, isize)> {
    input
        .lines()
        .map(|line| {
            line.split(",")
                .map(|s| s.parse().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect()
}

fn is_edge(x: isize, y: isize, poly: &[(isize, isize)]) -> bool {
    let n = poly.len();
    for i in 0..n {
        let (x1, y1) = poly[i];
        let (x2, y2) = poly[(i + 1) % n];

        if x1 == x2 && x == x1 && y >= y1.min(y2) && y <= y1.max(y2) {
            return true;
        }
        if y1 == y2 && y == y1 && x >= x1.min(x2) && x <= x1.max(x2) {
            return true;
        }
    }
    false
}

fn is_point_in_poly(p: (isize, isize), poly: &[(isize, isize)]) -> bool {
    if is_edge(p.0, p.1, poly) {
        return true;
    }

    let n = poly.len();
    let (x, y) = p;
    let mut inside = false;

    for i in 0..n {
        let (x1, y1) = poly[i];
        let (x2, y2) = poly[(i + 1) % n];

        if ((y1 > y) != (y2 > y)) && (x < (x2 - x1) * (y - y1) / (y2 - y1) + x1) {
            inside = !inside;
        }
    }

    inside
}

fn is_inside(x1: isize, y1: isize, x2: isize, y2: isize, poly: &[(isize, isize)]) -> bool {
    let min_x = x1.min(x2);
    let max_x = x1.max(x2);
    let min_y = y1.min(y2);
    let max_y = y1.max(y2);

    let corners = [
        (min_x, min_y),
        (max_x, min_y),
        (min_x, max_y),
        (max_x, max_y),
    ];

    if !corners.iter().all(|&p| is_point_in_poly(p, poly)) {
        return false;
    }

    let n = poly.len();
    for i in 0..n {
        let (px1, py1) = poly[i];
        let (px2, py2) = poly[(i + 1) % n];

        if px1 == px2 {
            // Vertical edge
            let edge_min_y = py1.min(py2);
            let edge_max_y = py1.max(py2);
            if px1 > min_x && px1 < max_x && edge_min_y < max_y && edge_max_y > min_y {
                return false;
            }
        } else if py1 == py2 {
            // Horizontal edge
            let edge_min_x = px1.min(px2);
            let edge_max_x = px1.max(px2);
            if py1 > min_y && py1 < max_y && edge_min_x < max_x && edge_max_x > min_x {
                return false;
            }
        }
    }

    true
}

impl AdventDay for Day09 {
    fn new(input: String) -> Self {
        Self { input }
    }

    fn part_one(&self) -> String {
        let pairs = parse_input(&self.input);

        let mut largest_area = 0;

        for (x1, y1) in pairs.iter() {
            for (x2, y2) in pairs.iter() {
                if x1 == x2 || y1 == y2 {
                    continue;
                }

                let w = (x1 - x2).abs() + 1;
                let h = (y1 - y2).abs() + 1;
                let area = w * h;
                if area > largest_area {
                    largest_area = area;
                }
            }
        }

        largest_area.to_string()
    }

    fn part_two(&self) -> String {
        let pairs = parse_input(&self.input);

        let mut largest_area = 0;

        for (x1, y1) in pairs.iter() {
            for (x2, y2) in pairs.iter() {
                if x1 == x2 || y1 == y2 {
                    continue;
                }

                if is_inside(*x1, *y1, *x2, *y2, &pairs) {
                    let w = (x1 - x2).abs() + 1;
                    let h = (y1 - y2).abs() + 1;
                    let area = w * h;
                    if area > largest_area {
                        largest_area = area;
                    }
                }
            }
        }

        largest_area.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = r#"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
"#;

    #[test]
    fn part_one() {
        let day09 = Day09::new(DATA.to_string());
        assert_eq!(day09.part_one(), "50");
    }

    #[test]
    fn part_two() {
        let day09 = Day09::new(DATA.to_string());
        assert_eq!(day09.part_two(), "24");
    }
}
